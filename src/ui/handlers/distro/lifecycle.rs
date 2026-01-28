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
                app.set_operation_text(i18n::t("operation.starting").into());
                app.set_show_operation(true);
            }
            {
                let app_state = as_ptr.lock().await;
                let manager = app_state.wsl_dashboard.clone();
                drop(app_state);
                manager.start_distro(&name).await;
            }
            if let Some(app) = ah.upgrade() {
                app.set_show_operation(false);
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
                app.set_operation_text(i18n::t("operation.stopping").into());
                app.set_show_operation(true);
            }
            {
                let app_state = as_ptr.lock().await;
                let manager = app_state.wsl_dashboard.clone();
                drop(app_state);
                manager.stop_distro(&name).await;
            }
            if let Some(app) = ah.upgrade() {
                app.set_show_operation(false);
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
                app.set_operation_text(i18n::t("operation.restarting").into());
                app.set_show_operation(true);
            }
            {
                let app_state = as_ptr.lock().await;
                let manager = app_state.wsl_dashboard.clone();
                drop(app_state);
                manager.restart_distro(&name).await;
            }
            if let Some(app) = ah.upgrade() {
                app.set_show_operation(false);
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
            {
                let app_state = as_ptr.lock().await;
                // Perform actual WSL deletion (includes config and autostart cleanup)
                app_state.wsl_dashboard.delete_distro(&app_state.config_manager, &name).await;
            }
            refresh_distros_ui(ah, as_ptr).await;
        });
    });

    // Delete confirmation log
    app.on_delete_clicked(move |name| {
        info!("Operation: Open delete confirmation - {}", name);
    });
}
