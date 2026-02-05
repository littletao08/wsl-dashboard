use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use crate::{AppWindow, AppState, i18n};
use crate::ui::data::refresh_distros_ui;

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    // Start
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_start_distro(move |name| {
        info!("Operation: Start distribution - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                app.set_task_status_text(i18n::t("operation.starting").into());
                app.set_task_status_visible(true);
            }
            {
                let app_state = as_ptr.lock().await;
                let manager = app_state.wsl_dashboard.clone();
                drop(app_state);
                manager.start_distro(&name).await;
            }
            if let Some(app) = ah.upgrade() {
                app.set_task_status_visible(false);
            }
            refresh_distros_ui(ah, as_ptr).await;
        });
    });

    // Stop
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_stop_distro(move |name| {
        info!("Operation: Stop distribution - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                app.set_task_status_text(i18n::t("operation.stopping").into());
                app.set_task_status_visible(true);
            }
            {
                let app_state = as_ptr.lock().await;
                let manager = app_state.wsl_dashboard.clone();
                drop(app_state);
                manager.stop_distro(&name).await;
            }
            if let Some(app) = ah.upgrade() {
                app.set_task_status_visible(false);
            }
            refresh_distros_ui(ah, as_ptr).await;
        });
    });

    // Restart
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_restart_distro(move |name| {
        info!("Operation: Restart distribution - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                app.set_task_status_text(i18n::t("operation.restarting").into());
                app.set_task_status_visible(true);
            }
            {
                let app_state = as_ptr.lock().await;
                let manager = app_state.wsl_dashboard.clone();
                drop(app_state);
                manager.restart_distro(&name).await;
            }
            if let Some(app) = ah.upgrade() {
                app.set_task_status_visible(false);
            }
            refresh_distros_ui(ah, as_ptr).await;
        });
    });

    // Delete
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_delete_distro(move |name| {
        info!("Operation: Delete distribution - {}", name);
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        let _ = slint::spawn_local(async move {
            if let Some(app) = ah.upgrade() {
                app.set_task_status_text(i18n::t("operation.deleting").into());
                app.set_task_status_visible(true);
            }
            let (dashboard, config_manager) = {
                let app_state = as_ptr.lock().await;
                (app_state.wsl_dashboard.clone(), app_state.config_manager.clone())
            };
            // Perform actual WSL deletion (includes config and autostart cleanup)
            // Lock is released before this await
            dashboard.delete_distro(&config_manager, &name).await;
            
            if let Some(app) = ah.upgrade() {
                app.set_task_status_visible(false);
            }
            refresh_distros_ui(ah, as_ptr).await;
        });
    });

    // Delete confirmation log
    app.on_delete_clicked(move |name| {
        info!("Operation: Open delete confirmation - {}", name);
    });
}
