use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use crate::{AppWindow, AppState, i18n};

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    // Start
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_start_distro(move |name| {
        info!("Operation: Start distribution - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        if let Some(app) = ah.upgrade() {
            app.set_task_status_text(i18n::t("operation.starting").into());
            app.set_task_status_visible(true);
        }
        tokio::spawn(async move {
            let manager = {
                let app_state = as_ptr.lock().await;
                app_state.wsl_dashboard.clone()
            };
            manager.start_distro(&name).await;
            let ah_res = ah.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_res.upgrade() {
                    app.set_task_status_visible(false);
                }
            });
        });
    });

    // Stop
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_stop_distro(move |name| {
        info!("Operation: Stop distribution - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        if let Some(app) = ah.upgrade() {
            app.set_task_status_text(i18n::t("operation.stopping").into());
            app.set_task_status_visible(true);
        }
        tokio::spawn(async move {
            let manager = {
                let app_state = as_ptr.lock().await;
                app_state.wsl_dashboard.clone()
            };
            manager.stop_distro(&name).await;
            let ah_res = ah.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_res.upgrade() {
                    app.set_task_status_visible(false);
                }
            });
        });
    });

    // Restart
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_restart_distro(move |name| {
        info!("Operation: Restart distribution - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        if let Some(app) = ah.upgrade() {
            app.set_task_status_text(i18n::t("operation.restarting").into());
            app.set_task_status_visible(true);
        }
        tokio::spawn(async move {
            let manager = {
                let app_state = as_ptr.lock().await;
                app_state.wsl_dashboard.clone()
            };
            manager.restart_distro(&name).await;
            let ah_res = ah.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_res.upgrade() {
                    app.set_task_status_visible(false);
                }
            });
        });
    });

    // Delete
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_delete_distro(move |name| {
        info!("Operation: Delete distribution - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        
        // 1. Immediate UI update on main thread
        if let Some(app) = ah.upgrade() {
            app.set_task_status_text(i18n::t("operation.deleting").into());
            app.set_task_status_visible(true);
        }

        // 2. Offload heavy work to background thread pool
        tokio::spawn(async move {
            let (dashboard, config_manager) = {
                let app_state = as_ptr.lock().await;
                (app_state.wsl_dashboard.clone(), app_state.config_manager.clone())
            };
            
            // Perform actual WSL deletion
            dashboard.delete_distro(&config_manager, &name).await;
            
            // 3. UI update back on main thread
            let ah_final = ah.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(app) = ah_final.upgrade() {
                    app.set_task_status_visible(false);
                }
            });
        });
    });

    // Delete confirmation log
    app.on_delete_clicked(move |name| {
        info!("Operation: Open delete confirmation - {}", name);
    });
}
