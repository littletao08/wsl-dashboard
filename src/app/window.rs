#[cfg(target_os = "windows")]
use tracing::{debug, warn};
use crate::AppWindow;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, RECT};
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowRect, GetWindowThreadProcessId, IsWindowVisible, 
    SetWindowPos, GetWindow, GW_OWNER, SWP_NOSIZE, SWP_NOZORDER, HWND_TOP
};
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Gdi::{MonitorFromWindow, GetMonitorInfoW, MONITORINFO, MONITOR_DEFAULTTOPRIMARY};

// Data structure to pass to EnumWindows callback
#[cfg(target_os = "windows")]
struct EnumWindowData {
    target_pid: u32,
    best_hwnd: Option<HWND>,
    max_area: i32,
}

// Callback for EnumWindows
#[cfg(target_os = "windows")]
unsafe extern "system" fn enum_window_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // Safety requirement: This function is unsafe because it's an FFI callback.
    // The body must handle unsafe pointers correctly.
    // Rust 2024 requires explicit unsafe blocks even inside unsafe fn
    unsafe {
        let data = &mut *(lparam.0 as *mut EnumWindowData);
        
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        if pid == data.target_pid {
            // Must be visible
            if IsWindowVisible(hwnd).as_bool() {
                // Check dimensions
                let mut rect = RECT::default();
                if GetWindowRect(hwnd, &mut rect).is_ok() {
                    let w = rect.right - rect.left;
                    let h = rect.bottom - rect.top;
                    let area = w * h;

                    // Heuristic: The main window is likely the largest visible window
                    // Filter out small windows (< 100x100)
                    if w > 100 && h > 100 && area > data.max_area {
                        // Check ownership (optional but good practice)
                        // We interpret 'main window' as one without an owner, usually.
                        // But if that fails, size is a strong indicator.
                        // Let's prioritize unowned windows, but frankly, size is king for Slint apps.
                        let is_owned = if let Ok(owner) = GetWindow(hwnd, GW_OWNER) {
                            !owner.0.is_null()
                        } else {
                            false
                        };

                        // Only pick if unowned OR significantly larger than current best
                        // Actually, let's keep it simple: Largest visible window wins.
                        if !is_owned || area > data.max_area * 2 {
                            data.max_area = area;
                            data.best_hwnd = Some(hwnd);
                            // debug!("Found candidate window: {:?}, Size: {}x{}, Area: {}", hwnd, w, h, area);
                        }
                    }
                }
            }
        }
        
        BOOL(1) // Continue enumeration
    }
}

// Windows platform window centering function implementation
// Returns true if successful, false otherwise
#[cfg(target_os = "windows")]
fn center_window_impl() -> bool {
    let current_pid = std::process::id();
    let mut data = EnumWindowData {
        target_pid: current_pid,
        best_hwnd: None,
        max_area: 0,
    };

    unsafe {
        // Enumerate windows to find ours by PID
        let _ = EnumWindows(Some(enum_window_proc), LPARAM(&mut data as *mut _ as _));
        
        if let Some(hwnd) = data.best_hwnd {
            // Logic to center the found window
            let mut window_rect = RECT::default();
            if GetWindowRect(hwnd, &mut window_rect).is_ok() {
                let window_width = window_rect.right - window_rect.left;
                let window_height = window_rect.bottom - window_rect.top;

                // Get the monitor where the window is located (or default)
                let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY);
                
                // Get monitor information
                let mut monitor_info = MONITORINFO {
                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                    ..Default::default()
                };
                
                if GetMonitorInfoW(hmonitor, &mut monitor_info).as_bool() {
                    let monitor_rect = monitor_info.rcWork; // Use work area (excluding taskbar)
                    
                    let monitor_width = monitor_rect.right - monitor_rect.left;
                    let monitor_height = monitor_rect.bottom - monitor_rect.top;
                    
                    // Calculate centered position
                    let x = monitor_rect.left + (monitor_width - window_width) / 2;
                    let y = monitor_rect.top + (monitor_height - window_height) / 2;
                    
                    // Set window position
                    // SWP_FRAMECHANGED causes the frame to be redrawn (useful if non-client area changed)
                    let result = SetWindowPos(
                        hwnd, 
                        HWND_TOP, 
                        x, y, 
                        0, 0, 
                        SWP_NOSIZE | SWP_NOZORDER
                    );

                    if result.is_ok() {
                        debug!("Window centered (Largest Area: {}) at ({}, {}) on monitor {}x{} (Process: {})", 
                               data.max_area, x, y, monitor_width, monitor_height, current_pid);
                        return true;
                    } else {
                        warn!("SetWindowPos failed for window {:?}", hwnd);
                    }
                }
            }
        }
    }
    false
}

// Show window and center it
pub fn show_and_center(app: &AppWindow) {
    #[cfg(target_os = "windows")]
    {
        use slint::ComponentHandle;
        app.show().unwrap();
        
        // Execute centering logic in background thread with polling
        // Using PID-based "Largest Visible Window" lookup for reliability
        std::thread::spawn(|| {
            // Try for up to 500ms (50 * 10ms)
            for i in 0..50 {
                if center_window_impl() {
                    // Success!
                    break;
                }
                // Log warning if we retry many times
                if i == 10 {
                    debug!("Still looking for main window to center...");
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        });
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        use slint::ComponentHandle;
        app.show().unwrap();
    }
}
