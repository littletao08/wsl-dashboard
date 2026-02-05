use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::Mutex;
use tracing::{info, warn};
use slint::{ComponentHandle, Model};
use crate::{AppWindow, AppState, i18n};
use crate::ui::data::refresh_distros_ui;

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    let ah = app_handle.clone();
    let _as_ptr_open = app_state.clone();
    app.on_open_move_dialog(move |name| {
        info!("Operation: Open move dialog - {}", name);
        if let Some(app) = ah.upgrade() {
            if app.get_is_installing() || app.get_is_exporting() || app.get_is_cloning() || app.get_is_moving() {
                app.set_current_message(i18n::t("dialog.operation_in_progress").into());
                app.set_show_message_dialog(true);
                return;
            }

            let name_str = name.to_string();
            let distro_location = app.get_distro_location().to_string();
            let target_path = std::path::Path::new(&distro_location)
                .join(&name_str)
                .to_string_lossy()
                .to_string();

            app.set_move_source_name(name_str.clone().into());
            // Note: move_target_name is still used internally but the input is removed from UI
            app.set_move_target_name(name_str.clone().into());
            app.set_move_target_path(target_path.into());
            app.set_move_original_path("".into());
            app.set_move_error("".into());
            app.set_show_move_dialog(true);
        }
    });

    let ah = app_handle.clone();
    app.on_cancel_move_confirm(move || {
        if let Some(app) = ah.upgrade() {
            app.set_show_move_confirm(false);
            info!("Operation: Move confirm cancelled");
        }
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_confirm_move_action(move || {
        let ah = match ah.upgrade() {
            Some(a) => a,
            None => return,
        };
        ah.set_show_move_confirm(false);

        let source_name = ah.get_move_source_name().to_string();
        let target_name = ah.get_move_target_name().to_string();
        let target_path = ah.get_move_target_path().to_string();

        info!("Operation: Move confirmed - Starting WSL2 Move for {}", source_name);
        
        run_move_process(
            ah.as_weak(), 
            as_ptr.clone(), 
            source_name, 
            target_name, 
            target_path, 
            "2".to_string()
        );
    });

    let ah = app_handle.clone();
    app.on_select_move_folder(move || {
        if let Some(path) = rfd::FileDialog::new()
            .set_title(i18n::t("dialog.select_move_dir"))
            .pick_folder()
        {
            if let Some(app) = ah.upgrade() {
                app.set_move_target_path(path.to_string_lossy().to_string().into());
            }
        }
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_confirm_move(move |source_name, _target_name, target_path| {
        info!("Operation: Confirm move - Source: {}, Target: {}, Path: {}", source_name, _target_name, target_path);
        let ah = match ah.upgrade() {
            Some(a) => a,
            None => return,
        };

        if ah.get_is_installing() || ah.get_is_exporting() || ah.get_is_cloning() || ah.get_is_moving() {
            return;
        }

        // 1. Sync Validations
        let p = std::path::Path::new(target_path.as_str());
        if p.exists() {
            if p.is_dir() {
                if let Ok(entries) = std::fs::read_dir(p) {
                    if entries.count() > 0 {
                        ah.set_move_error(i18n::t("dialog.dir_not_empty").into());
                        return;
                    }
                }
            } else {
                ah.set_move_error(i18n::t("dialog.path_is_not_dir").into());
                return;
            }
        } else {
            if let Err(e) = std::fs::create_dir_all(p) {
                ah.set_move_error(i18n::tr("dialog.mkdir_failed", &[e.to_string()]).into());
                return;
            }
        }

        // Get distro version (Sync)
        let mut version = "2".to_string();
        let distros = ah.get_distros();
        for i in 0..distros.row_count() {
            if let Some(d) = distros.row_data(i) {
                if d.name == source_name {
                    version = d.version.to_string();
                    break;
                }
            }
        }

        ah.set_move_error("".into());
        // Do not close dialog yet, we need to check current path asynchronously
        
        let ah_move = ah.as_weak(); // Still used if we pass to run_move_process
        let as_ptr = as_ptr.clone();
        let source_name = source_name.to_string();
        let target_path = target_path.to_string();
        let target_name = _target_name.to_string();

        if version == "2" {
            // Check if there are other running distros
            let distros = ah.get_distros();
            let mut running_names = Vec::new();
            
            for i in 0..distros.row_count() {
                if let Some(d) = distros.row_data(i) {
                    if d.status.as_str() == "Running" && d.name != source_name {
                        running_names.push(d.name.to_string());
                    }
                }
            }
            
            // Show confirmation dialog for WSL2 move
            let warning_msg = if running_names.is_empty() {
                i18n::t("dialog.move_wsl2_shutdown_warning_no_running")
            } else {
                let other_distros = running_names.join(", ");
                i18n::tr("dialog.move_wsl2_shutdown_warning", &[other_distros])
            };
            
            ah.set_move_confirm_message(warning_msg.into());
            ah.set_show_move_confirm(true);
            ah.set_show_move_dialog(false); 
        } else {
             ah.set_show_move_dialog(false);
             run_move_process(
                ah_move,
                as_ptr,
                source_name,
                target_name,
                target_path,
                version
             );
        }
    });
}

fn run_move_process(
    ah_move: slint::Weak<AppWindow>, 
    as_ptr: Arc<Mutex<AppState>>, 
    source_name: String, 
    target_name: String, 
    target_path: String, 
    version: String
) {
    let _ = slint::spawn_local(async move {
        if let Some(app) = ah_move.upgrade() {
            app.set_is_moving(true);
            app.set_task_status_visible(true);
        }

        let dashboard = {
            let state = as_ptr.lock().await;
            state.wsl_dashboard.clone()
        };
        dashboard.set_manual_operation(true);

        // Show pending status immediately
        if let Some(app) = ah_move.upgrade() {
            let msg = i18n::tr("operation.moving_wsl2_msg", &[source_name.clone(), i18n::t("dialog.processing")]);
            app.set_task_status_text(msg.into());
        }

        // Fetch size before shutdown/stop operations to ensure we have it for the UI
        let mut size_str = "0 MB".to_string();
        if version == "2" {
             let info_res = crate::wsl::ops::info::get_distro_information(dashboard.executor(), &source_name).await;
             if info_res.success {
                 if let Some(info) = info_res.data {
                     if !info.vhdx_size.is_empty() {
                         size_str = info.vhdx_size;
                     }
                 }
             }
        }
        
        // Update size info if available before shutdown delay
        if let Some(app) = ah_move.upgrade() {
            let msg = i18n::tr("operation.moving_wsl2_msg", &[source_name.clone(), size_str.clone()]);
            app.set_task_status_text(msg.into());
        }

        // 3. Stop WSL system (for WSL2 only)
        // We use shutdown instead of terminate because it's more reliable 
        // at releasing VHDX file handles for WSL2 move operations.
        if version == "2" {
            let _ = dashboard.shutdown_wsl().await;
            // Give WSL service sufficient time to release file locks after shutdown
            tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
        } else {
            // For WSL1, just terminate the specific distro
            let _ = dashboard.stop_distro(&source_name).await;
            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        }

        let result = if version == "2" {
            // WSL 2 Move
            if let Some(app) = ah_move.upgrade() {
                let msg = i18n::tr("operation.moving_wsl2_msg", &[source_name.clone(), size_str]);
                app.set_task_status_text(msg.into());
            }

            let mut move_res = crate::wsl::models::WslCommandResult::error("".into(), "".into());
            for attempt in 1..=5 {
                if attempt > 1 {
                    info!("WSL 2 Move: Retry attempt {} after delay...", attempt);
                    // Increase delay between retries to give WSL service more time
                    tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
                }
                
                move_res = dashboard.move_distro(&source_name, &target_path).await;
                
                if move_res.success {
                    // Auxiliary Verification: check if the .vhdx file actually exists in the target path
                    let vhdx_check = std::path::Path::new(&target_path).join("ext4.vhdx");
                    if vhdx_check.exists() {
                        info!("Auxiliary verification: Found VHDX at {}", vhdx_check.display());
                    } else {
                        warn!("Auxiliary verification WARNING: Command reported success, but 'ext4.vhdx' not found at expected location: {}", vhdx_check.display());
                    }
                    break;
                }
                
                // If it's a sharing violation, we should definitely retry
                if move_res.output.contains("ERROR_SHARING_VIOLATION") || move_res.output.contains("0x80070020") {
                    continue;
                }
            }
            move_res
        } else {
            // WSL 1 Move
            move_wsl1(ah_move.clone(), as_ptr.clone(), &source_name, &target_name, &target_path).await
        };

        if let Some(app) = ah_move.upgrade() {
            app.set_task_status_visible(false);
            app.set_is_moving(false);
            if result.success {
                app.set_current_message(i18n::tr("dialog.move_success", &[source_name, target_path]).into());
            } else {
                let err = result.error.unwrap_or_else(|| i18n::t("dialog.error"));
                if result.output == "BACKUP_SAVED" {
                    app.set_current_message(i18n::tr("dialog.move_failed_backup", &[err.clone()]).into());
                    app.set_current_message_link(i18n::t("distro.explorer").into());
                    // Set URL to the parent directory of the backup file for opening in explorer
                    let backup_path = std::path::Path::new(&err);
                    if let Some(parent) = backup_path.parent() {
                            app.set_current_message_url(parent.to_string_lossy().to_string().into());
                    } else {
                            app.set_current_message_url(err.into());
                    }
                } else {
                    app.set_current_message(i18n::tr("dialog.move_failed", &[err]).into());
                }
            }
            app.set_show_message_dialog(true);
            // Restore manual operation flag
            dashboard.set_manual_operation(false);
            refresh_distros_ui(ah_move.clone(), as_ptr.clone()).await;
        } else {
            // Also restore if app upgrade failed (though unlikely here)
            dashboard.set_manual_operation(false);
        }
    });
}

async fn move_wsl1(
    _ah: slint::Weak<AppWindow>, 
    as_ptr: Arc<Mutex<AppState>>, 
    source_name: &str, 
    target_name: &str, 
    target_path: &str
) -> crate::wsl::models::WslCommandResult<String> {
    use crate::wsl::models::WslCommandResult;
    
    // 1. Export
    let (temp_dir, temp_file_str) = {
        let state = as_ptr.lock().await;
        let temp_dir = PathBuf::from(&state.config_manager.get_settings().temp_location);
        let temp_file = temp_dir.join(format!("wsl_move_{}.tar", uuid::Uuid::new_v4()));
        (temp_dir, temp_file.to_string_lossy().to_string())
    };

    let _ = std::fs::create_dir_all(&temp_dir);

    info!("WSL1 Move: Exporting '{}' to '{}'...", source_name, temp_file_str);
    let stop_signal = Arc::new(std::sync::atomic::AtomicBool::new(false));
    
    // (1/2) Exporting
    if let Some(app) = _ah.upgrade() {
        let msg = i18n::tr("operation.moving_wsl1_step1", &[source_name.to_string(), "0 MB".to_string()]);
        app.set_task_status_text(msg.into());
    }

    super::spawn_file_size_monitor(
        _ah.clone(),
        temp_file_str.clone(),
        source_name.to_string(),
        "operation.moving_wsl1_step1".into(),
        stop_signal.clone()
    );

    let export_result = {
        let dashboard = {
            let state = as_ptr.lock().await;
            state.wsl_dashboard.clone()
        };
        dashboard.export_distro(source_name, &temp_file_str).await
    };

    stop_signal.store(true, std::sync::atomic::Ordering::Relaxed);

    if !export_result.success {
        let _ = std::fs::remove_file(&temp_file_str);
        return export_result;
    }

    // Verify file size
    if let Ok(metadata) = std::fs::metadata(&temp_file_str) {
        if metadata.len() == 0 {
            let _ = std::fs::remove_file(&temp_file_str);
            return WslCommandResult::error("".into(), "Exported file is empty".into());
        }
    } else {
        return WslCommandResult::error("".into(), "Failed to verify exported file".into());
    }

    // 2. Unregister
    info!("WSL1 Move: Unregistering '{}'...", source_name);
    let unregister_result = {
        let executor = {
            let state = as_ptr.lock().await;
            state.wsl_dashboard.executor().clone()
        };
        // user requested NOT to delete appx, so we use execute_command directly instead of dashboard.delete_distro
        executor.execute_command(&["--unregister", source_name]).await
    };

    if !unregister_result.success {
        let _ = std::fs::remove_file(&temp_file_str);
        return unregister_result;
    }

    // 3. Import
    info!("WSL1 Move: Importing to '{}' at '{}'...", target_name, target_path);
    // (2/2) Importing
    if let Some(app) = _ah.upgrade() {
        let msg = i18n::tr("operation.moving_wsl1_step2", &[source_name.to_string()]);
        app.set_task_status_text(msg.into());
    }
    let import_result = {
        let dashboard = {
            let state = as_ptr.lock().await;
            state.wsl_dashboard.clone()
        };
        dashboard.import_distro(target_name, target_path, &temp_file_str).await
    };

    if !import_result.success {
        // Return success=false but with a special flag and the backup path
        return WslCommandResult {
            success: false,
            output: "BACKUP_SAVED".into(),
            error: Some(temp_file_str),
            data: None,
        };
    }

    // 4. Verify exist
    let verify_result = {
        let executor = {
            let state = as_ptr.lock().await;
            state.wsl_dashboard.executor().clone()
        };
        executor.execute_command(&["-l", "-v"]).await
    };

    if verify_result.success && verify_result.output.contains(target_name) {
        info!("WSL1 Move: Success, cleaning up temp file");
        let _ = std::fs::remove_file(&temp_file_str);
        WslCommandResult::success("Move successful".into(), None)
    } else {
        warn!("WSL1 Move: Import appeared successful but distro not found in list");
        WslCommandResult {
            success: false,
            output: "BACKUP_SAVED".into(),
            error: Some(temp_file_str),
            data: None,
        }
    }
}
