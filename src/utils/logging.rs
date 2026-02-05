use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::{
    fmt,
    prelude::*,
    layer::{Layer, Filter},
};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU8, Ordering};
use std::io::{Write, Result};
use tracing::{Metadata, Level, Subscriber};

pub struct SwapWriter {
    inner: Arc<Mutex<NonBlocking>>,
}

impl Write for SwapWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner.lock().unwrap().write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.inner.lock().unwrap().flush()
    }
}

impl<'a> fmt::MakeWriter<'a> for SwapWriter {
    type Writer = Self;
    fn make_writer(&'a self) -> Self::Writer {
        Self { inner: self.inner.clone() }
    }
}

// Custom dynamic log level filter
pub struct DynamicLevelFilter {
    current_level: Arc<AtomicU8>,
}

impl DynamicLevelFilter {
    fn get_level(&self) -> Level {
        map_level(self.current_level.load(Ordering::Relaxed))
    }
}

impl<S: Subscriber> Filter<S> for DynamicLevelFilter {
    fn enabled(&self, metadata: &Metadata<'_>, _ctx: &tracing_subscriber::layer::Context<'_, S>) -> bool {
        metadata.level() <= &self.get_level()
    }
}

pub struct LoggingSystem {
    pub writer: SwapWriter,
    pub guard: Arc<Mutex<WorkerGuard>>,
    pub level: Arc<AtomicU8>,
}

use tracing_subscriber::fmt::time::FormatTime;
use chrono::FixedOffset;

#[derive(Clone, Copy)]
struct LocalTimer {
    offset: FixedOffset,
}

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = chrono::Utc::now().with_timezone(&self.offset);
        write!(w, "{}", now.format("%Y-%m-%dT%H:%M:%S%.6f%:z"))
    }
}

fn parse_timezone(tz_str: &str) -> FixedOffset {
    // Expected format from settings: "UTC+08:00"
    let clean_tz = tz_str.trim_start_matches("UTC");
    if clean_tz.is_empty() {
        return FixedOffset::east_opt(0).unwrap();
    }

    // Attempt to parse offset part like "+08:00" or "+0800"
    match chrono::DateTime::parse_from_rfc3339(&format!("2024-01-01T00:00:00{}", clean_tz)) {
        Ok(dt) => *dt.offset(),
        Err(_) => {
            // Fallback: search for sign and try parsing hours
            FixedOffset::east_opt(0).unwrap()
        }
    }
}

pub fn init_logging(log_dir: &str, level_num: u8, timezone_str: &str) -> LoggingSystem {
    let level_atomic = Arc::new(AtomicU8::new(level_num));
    let timer = LocalTimer { offset: parse_timezone(timezone_str) };
    
    // Terminal output
    let stdout_layer = fmt::layer()
        .with_timer(timer)
        .with_target(false)
        .with_filter(DynamicLevelFilter { current_level: level_atomic.clone() });
    
    // File output
    let file_appender = RollingFileAppender::builder()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix("wsl-dashboard")
        .filename_suffix("log")
        .build(log_dir)
        .expect("Failed to create log appender");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    
    let swap_writer = SwapWriter { inner: Arc::new(Mutex::new(non_blocking)) };
    let guard_holder = Arc::new(Mutex::new(guard));

    let file_layer = fmt::layer()
        .with_timer(timer)
        .with_writer(swap_writer.clone())
        .with_ansi(false)
        .with_target(true)
        .with_filter(DynamicLevelFilter { current_level: level_atomic.clone() });

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    LoggingSystem { 
        writer: swap_writer, 
        guard: guard_holder,
        level: level_atomic,
    }
}

fn map_level(level_num: u8) -> Level {
    match level_num {
        1 => Level::ERROR,
        2 => Level::WARN,
        3 => Level::INFO,
        4 => Level::DEBUG,
        5 => Level::TRACE,
        _ => Level::INFO,
    }
}

impl LoggingSystem {
    pub fn update_path(&self, log_dir: &str) {
        let file_appender = RollingFileAppender::builder()
            .rotation(tracing_appender::rolling::Rotation::DAILY)
            .filename_prefix("wsl-dashboard")
            .filename_suffix("log")
            .build(log_dir)
            .expect("Failed to create log appender");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        
        *self.writer.inner.lock().unwrap() = non_blocking;
        *self.guard.lock().unwrap() = guard;
        tracing::info!("Log directory updated to: {}", log_dir);
    }

    pub fn update_level(&self, level_num: u8) {
        self.level.store(level_num, Ordering::Relaxed);
        let level = map_level(level_num);
        tracing::info!("Log level updated to: {:?}", level);
    }
}

impl Clone for SwapWriter {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}
