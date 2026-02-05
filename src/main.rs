#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, debug};
use slint::{ComponentHandle, Model};

// Define application modules
mod wsl;
mod utils;
mod ui;
mod config;
mod app;
mod i18n;

// Import Slint UI components
slint::include_modules!();

use app::{AppState, APP_NAME, APP_ID, COMPANY_NAME, LEGAL_COPYRIGHT, GITHUB_URL, GITHUB_ISSUES};
use ui::data::{refresh_data, refresh_distros_ui};
use ui::handlers;

#[tokio::main]
async fn main() {
    // Initialize configuration manager to get log path
    let config_manager = config::ConfigManager::new().await;
    let settings = config_manager.get_settings();
    // Load i18n based on settings
    let lang = if settings.ui_language == "auto" {
        &config_manager.get_config().system.system_language
    } else {
        &settings.ui_language
    };
    i18n::load_resources(lang);
    #[cfg(debug_assertions)]
    i18n::verify_translations();
    
    let initial_logs_location = settings.logs_location.clone();
    let log_level = settings.log_level;
    let timezone = config_manager.get_config().system.timezone.clone();

    // Set up tracing logs
    let logging_system = utils::logging::init_logging(&initial_logs_location, log_level, &timezone);

    // Cleanup expired logs
    cleanup_expired_logs(&initial_logs_location, settings.log_days);

    info!("Starting {} (ID: {})...", APP_NAME, APP_ID);

    // Create application state
    let app_state = Arc::new(Mutex::new(AppState::new(config_manager, logging_system)));
    
    // Create Slint application
    let app = AppWindow::new().expect("Failed to create app");
    
    // Register i18n callback
    app.global::<AppI18n>().on_t(|key, args| {
        let args_vec: Vec<String> = args.iter().map(|s: slint::SharedString| s.to_string()).collect();
        i18n::tr(&key, &args_vec).into()
    });

    // Trigger initial evaluation of all i18n properties
    app.global::<AppI18n>().set_version(1);
    
    // Set version number and URLs - homepage and issues always use GITHUB_URL
    app.global::<AppInfo>().set_version(env!("CARGO_PKG_VERSION").into());
    app.global::<AppInfo>().set_homepage(GITHUB_URL.into());
    app.global::<AppInfo>().set_issues_url(format!("{}{}", GITHUB_URL, GITHUB_ISSUES).into());
    
    debug!("App Metadata - Company: {}, Copyright: {}", COMPANY_NAME, LEGAL_COPYRIGHT);
    
    let app_handle = app.as_weak();

    // Initialize data refresh
    refresh_data(app_handle.clone(), app_state.clone()).await;

    // Load configuration to UI
    load_settings_to_ui(&app, &app_state).await;

    // Set release_url based on timezone
    {
        let state = app_state.lock().await;
        let timezone = &state.config_manager.get_config().system.timezone;
        let release_base_url = if timezone == app::ZH_TIMEZONE {
            app::GITEE_URL
        } else {
            GITHUB_URL
        };
        app.global::<AppInfo>().set_release_url(format!("{}{}", release_base_url, app::GITHUB_RELEASES).into());
    }

    // Set up all UI callbacks
    handlers::common::setup(&app, app_handle.clone(), app_state.clone());
    handlers::window::setup(&app, app_handle.clone());
    handlers::distro::setup(&app, app_handle.clone(), app_state.clone());
    handlers::settings::setup(&app, app_handle.clone(), app_state.clone());
    handlers::update::setup(&app, app_handle.clone(), app_state.clone());
    handlers::instance::setup(&app, app_handle.clone(), app_state.clone());


    // Start WSL status monitoring
    spawn_wsl_monitor(app_state.clone());

    // Listen for distribution state changes and automatically update UI
    spawn_state_listener(app_handle.clone(), app_state.clone());

    // Automatically check for updates and expiration at startup
    app::startup::spawn_check_task(app_handle.clone(), app_state.clone());

    // Show window and center it
    app::window::show_and_center(&app);

    // Run application
    app.run().expect("Failed to run app");

    // Processing after application exit
    handle_app_exit(&app, &app_state).await;
}

// Load configuration to UI
async fn load_settings_to_ui(app: &AppWindow, app_state: &Arc<Mutex<AppState>>) {
    let state = app_state.lock().await;
    let settings = state.config_manager.get_settings();
    app.set_ui_language(settings.ui_language.clone().into());
    app.set_distro_location(settings.distro_location.clone().into());
    app.set_new_instance_path(settings.distro_location.clone().into());
    app.set_logs_location(settings.logs_location.clone().into());
    app.set_auto_shutdown(settings.auto_shutdown);
    app.set_log_level(settings.log_level as i32);
    
    // Validate and set log retention days
    let mut log_days = settings.log_days;
    if !vec![7, 15, 30].contains(&log_days) {
        info!("Invalid log-days value ({}), resetting to 7", log_days);
        log_days = 7;
        // Note: we'll update it below along with check_update if needed
    }
    app.set_log_days(log_days as i32);

    // Validate and set check_update interval
    let mut check_update = settings.check_update;
    if !vec![1, 7, 15, 30].contains(&check_update) {
        info!("Invalid check-update value ({}), resetting to 7", check_update);
        check_update = 7;
    }
    app.set_check_update_interval(check_update as i32);

    // Update settings if any were invalid
    if log_days != settings.log_days || check_update != settings.check_update {
        let mut state_mut = app_state.lock().await;
        let mut settings_mut = state_mut.config_manager.get_settings().clone();
        settings_mut.log_days = log_days;
        settings_mut.check_update = check_update;
        let _ = state_mut.config_manager.update_settings(settings_mut);
    }
    
    app.global::<Theme>().set_dark_mode(settings.dark_mode);
    info!("Configuration loaded to UI (Language: {}, Mode: {}, LogLevel: {}, LogDays: {})", 
          settings.ui_language, if settings.dark_mode { "Dark" } else { "Light" }, settings.log_level, log_days);
}

// Start WSL status monitoring task
fn spawn_wsl_monitor(app_state: Arc<Mutex<AppState>>) {
    tokio::spawn(async move {
        let manager = {
            let app_state = app_state.lock().await;
            app_state.wsl_dashboard.clone()
        };
        manager.start_monitoring().await;
    });
}

// Listen for distribution state changes and automatically update UI
fn spawn_state_listener(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    tokio::spawn(async move {
        loop {
            {
                let state = app_state.lock().await;
                let state_changed = state.wsl_dashboard.state_changed().clone();
                drop(state);
                state_changed.notified().await;
            }
            
            debug!("WSL state changed, updating UI...");
            let _ = refresh_distros_ui(app_handle.clone(), app_state.clone()).await;
        }
    });
}

// Processing after application exit
async fn handle_app_exit(app: &AppWindow, app_state: &Arc<Mutex<AppState>>) {
    let auto_shutdown = app.get_auto_shutdown();
    if auto_shutdown {
        debug!("Auto-shutdown on exit is enabled, shutting down WSL...");
        let manager = {
            let state = app_state.lock().await;
            state.wsl_dashboard.clone()
        };
        manager.shutdown_wsl().await;
        debug!("WSL shut down completed");
    }
}

// Cleanup expired log files
fn cleanup_expired_logs(log_dir: &str, log_days: u8) {
    let log_path = std::path::Path::new(log_dir);
    if !log_path.exists() || !log_path.is_dir() {
        return;
    }

    let now = chrono::Local::now().date_naive();
    let expiration_date = now - chrono::TimeDelta::days(log_days as i64);

    if let Ok(entries) = std::fs::read_dir(log_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    // Pattern: wsl-dashboard.YYYY-MM-DD.log
                    if file_name.starts_with("wsl-dashboard.") && file_name.ends_with(".log") {
                        let date_part = file_name
                            .trim_start_matches("wsl-dashboard.")
                            .trim_end_matches(".log");
                        // Sometimes there might be a .1, .2 etc if multiple restarts in one day? 
                        // Actually tracing-appender DAILY should just be YYYY-MM-DD
                        if let Ok(file_date) = chrono::NaiveDate::parse_from_str(date_part, "%Y-%m-%d") {
                            if file_date <= expiration_date {
                                info!("Deleting expired log file: {:?}", file_name);
                                let _ = std::fs::remove_file(path);
                            }
                        }
                    }
                }
            }
        }
    }
}