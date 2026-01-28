use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::Mutex;
use tracing::info;
use slint::Model;
use crate::{AppWindow, AppState, i18n};
use crate::ui::data::{refresh_distros_ui, refresh_installable_distros};
use super::{sanitize_instance_name, generate_random_suffix};

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    // Folder selection
    let ah = app_handle.clone();
    app.on_select_folder(move || {
        if let Some(path) = rfd::FileDialog::new()
            .set_title(i18n::t("dialog.select_install_dir"))
            .pick_folder()
        {
            if let Some(app) = ah.upgrade() {
                let path_str = path.display().to_string();
                app.set_new_instance_path(path_str.clone().into());
                
                let p = std::path::Path::new(&path_str);
                if p.exists() {
                    if let Ok(entries) = std::fs::read_dir(p) {
                        if entries.count() > 0 {
                            app.set_path_error(i18n::t("dialog.dir_not_empty").into());
                        } else {
                            app.set_path_error("".into());
                        }
                    }
                } else {
                    app.set_path_error("".into());
                }
            }
        }
    });

    let ah = app_handle.clone();
    app.on_check_install_path(move |path| {
        if let Some(app) = ah.upgrade() {
            if path.is_empty() {
                app.set_path_error("".into());
                return;
            }
            let p = std::path::Path::new(path.as_str());
            if p.exists() && p.is_dir() {
                if let Ok(entries) = std::fs::read_dir(p) {
                    if entries.count() > 0 {
                        app.set_path_error(i18n::t("dialog.dir_not_empty").into());
                        return;
                    }
                }
            }
            app.set_path_error("".into());
        }
    });

    let ah = app_handle.clone();
    app.on_select_install_file(move |source_idx| {
        let mut dialog = rfd::FileDialog::new()
            .set_title(i18n::t("dialog.select_install_file"));
        
        dialog = match source_idx {
            0 => dialog.add_filter(i18n::t("dialog.archive"), &["tar", "tar.gz", "tar.xz", "wsl"]),
            1 => dialog.add_filter(i18n::t("dialog.vhdx"), &["vhdx"]),
            _ => dialog,
        };

        if let Some(path) = dialog.pick_file() {
            if let Some(app) = ah.upgrade() {
                app.set_install_file_path(path.display().to_string().into());
                
                if let Some(name_os) = path.file_name() {
                    let mut full_stem = name_os.to_string_lossy().to_string();

                    // Optimize: Remove specific suffixes first to get clean name
                    if full_stem.ends_with(".tar.gz") {
                        full_stem.truncate(full_stem.len() - 7);
                    } else if full_stem.ends_with(".tar.xz") {
                        full_stem.truncate(full_stem.len() - 7);
                    } else if full_stem.ends_with(".tar") {
                        full_stem.truncate(full_stem.len() - 4);
                    } else if full_stem.ends_with(".wsl") {
                        full_stem.truncate(full_stem.len() - 4);
                    } else if full_stem.ends_with(".vhdx") {
                        full_stem.truncate(full_stem.len() - 5);
                    }
                    // Remove "rootfs" case-insensitively
                    while let Some(idx) = full_stem.to_lowercase().find("rootfs") {
                        full_stem.replace_range(idx..idx+6, "");
                    }

                    let parts: Vec<&str> = full_stem.split('-').collect();
                    let mut filtered_parts = Vec::new();
                    let stop_keywords = ["wsl", "amd64", "arm64", "x86_64", "with", "docker", "vhdx", "image"];
                    
                    for part in parts {
                        let lower_part = part.to_lowercase();
                        if stop_keywords.iter().any(|&k| lower_part.contains(k)) {
                            break;
                        }
                        if !part.is_empty() && part != "." {
                             filtered_parts.push(part);
                        }
                    }
                    
                    let suggested_name = if filtered_parts.is_empty() {
                        full_stem
                    } else {
                        filtered_parts.join("-")
                    };
                    
                    let mut sanitized = sanitize_instance_name(&suggested_name);
                    
                    while sanitized.ends_with(|c| c == '-' || c == '_' || c == '.') {
                        sanitized.pop();
                    }

                    app.set_new_instance_name(sanitized.clone().into());
                    
                    // Sync path
                    let distro_location = app.get_distro_location().to_string();
                    let new_path = std::path::Path::new(&distro_location)
                        .join(&sanitized)
                        .to_string_lossy()
                        .to_string();
                    app.set_new_instance_path(new_path.into());
                }
            }
        }
    });

    let ah = app_handle.clone();
    app.on_distro_selected(move |val| {
        if let Some(app) = ah.upgrade() {
            let sanitized = sanitize_instance_name(&val);
            app.set_new_instance_name(sanitized.clone().into());
            
            let distro_location = app.get_distro_location().to_string();
            let new_path = std::path::Path::new(&distro_location)
                .join(&sanitized)
                .to_string_lossy()
                .to_string();
            app.set_new_instance_path(new_path.into());
        }
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_source_selected(move |idx| {
        if let Some(app) = ah.upgrade() {
            app.set_name_error("".into());
            app.set_path_error("".into());
            app.set_install_status("".into());
            app.set_terminal_output("".into());
        }

        if idx == 2 {
             let ah_inner = ah.clone();
             let as_ptr = as_ptr.clone();
             let _ = slint::spawn_local(async move {
                 if let Some(app) = ah_inner.upgrade() {
                     if app.get_installable_distro_names().row_count() == 0 {
                        app.set_operation_text(i18n::t("operation.fetching_distros").into());
                        app.set_show_operation(true);

                        refresh_installable_distros(ah_inner.clone(), as_ptr).await;

                        if let Some(app) = ah_inner.upgrade() {
                            app.set_show_operation(false);
                        }
                     } else {
                        if let Some(first) = app.get_installable_distro_names().row_data(0) {
                            app.set_selected_install_distro(first.clone());
                            app.invoke_distro_selected(first);
                        }
                     }
                 }
             });
        }
    });

    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    app.on_install_distro(move |source_idx, name, friendly_name, install_path, file_path| {
        let name = name.to_string();
        let friendly_name = friendly_name.to_string();
        let install_path = install_path.to_string();
        let file_path = file_path.to_string();
        let ah = ah.clone();
        let as_ptr = as_ptr.clone();
        
        info!("Operation: Install distribution - SourceIdx: {}, Name: {}, File: {}", source_idx, name, file_path);
        
        let _ = slint::spawn_local(async move {
            let app = match ah.upgrade() {
                Some(a) => a,
                None => return,
            };

            app.set_is_installing(true);
            app.set_install_status(i18n::t("install.checking").into());
            app.set_install_success(false);
            app.set_terminal_output("".into());
            app.set_name_error("".into());

            let mut final_name = name.clone();
            if final_name.is_empty() {
                if source_idx == 2 {
                    final_name = friendly_name.clone();
                } else if !file_path.is_empty() {
                    if let Some(stem) = std::path::Path::new(&file_path).file_stem() {
                        final_name = stem.to_string_lossy().to_string();
                    }
                }
            }

            if final_name.is_empty() {
                app.set_name_error(i18n::t("dialog.name_required").into());
                app.set_is_installing(false);
                app.set_install_status(i18n::t("install.error").into());
                return;
            }

            let is_valid_chars = final_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.');
            if !is_valid_chars || final_name.len() > 25 {
                app.set_name_error(i18n::t("dialog.install_name_invalid").into());
                app.set_is_installing(false);
                app.set_install_status(i18n::t("install.error").into());
                return;
            }

            let distros = app.get_distros();
            let mut name_exists = false;
            for i in 0..distros.row_count() {
                if let Some(d) = distros.row_data(i) {
                    if d.name == final_name {
                        name_exists = true;
                        break;
                    }
                }
            }

            if name_exists {
                let new_suggested_name = sanitize_instance_name(&generate_random_suffix(&final_name));
                app.set_new_instance_name(new_suggested_name.clone().into());
                
                let distro_location = app.get_distro_location().to_string();
                let new_path = std::path::Path::new(&distro_location)
                    .join(&new_suggested_name)
                    .to_string_lossy()
                    .to_string();
                app.set_new_instance_path(new_path.into());

                app.set_name_error(i18n::tr("dialog.install_name_exists", &[final_name.clone()]).into());
                app.set_is_installing(false);
                app.set_install_status(i18n::t("install.conflict_error").into());
                return;
            }

            let ah_inner = ah.clone();
            let executor = {
                let state = as_ptr.lock().await;
                state.wsl_dashboard.executor().clone()
            };

            let mut success = false;
            let mut error_msg = String::new();

            match source_idx {
                2 => { // Store
                    let mut real_id = String::new();
                    let installables = app.get_installable_distros();
                    for i in 0..installables.row_count() {
                        if let Some(d) = installables.row_data(i) {
                            if d.friendly_name == friendly_name {
                                real_id = d.name.to_string();
                                break;
                            }
                        }
                    }

                    if real_id.is_empty() {
                        app.set_install_status(i18n::t("install.unknown_distro").into());
                        app.set_is_installing(false);
                        return;
                    }

                    app.set_install_status(i18n::t("install.installing").into());
                    app.set_terminal_output(format!("{}\n", i18n::tr("install.step_1", &[real_id.clone()])).into());
                    info!("Starting store installation for distribution ID: {}", real_id);
                    
                    let _ = {
                        let state = as_ptr.lock().await;
                        executor.delete_distro(&state.config_manager, &real_id).await
                    };
                    
                    let mut current = app.get_terminal_output().to_string();
                    current.push_str(&format!("{}\n", i18n::t("install.step_2")));
                    app.set_terminal_output(current.into());

                    let use_web_download = executor.detect_fastest_source().await;
                    
                    let mut install_args = vec!["--install", "-d", &real_id, "--no-launch"];
                    if use_web_download {
                        install_args.push("--web-download");
                    }
                    let cmd_str = format!("wsl {}", install_args.join(" "));
                    
                    let mut current = app.get_terminal_output().to_string();
                    current.push_str(&format!("{}\n", i18n::tr("install.step_3", &[cmd_str.clone()])));
                    app.set_terminal_output(current.into());

                    let source_text = if use_web_download { "GitHub" } else { "Microsoft" };
                    let mut current = app.get_terminal_output().to_string();
                    current.push_str(&i18n::tr("install.step_4", &[source_text.to_string()])); 
                    app.set_terminal_output(current.into());

                    let timer = slint::Timer::default();
                    let ah_timer = ah_inner.clone();
                    let start_time = std::time::Instant::now();
                    let base_content = app.get_terminal_output().to_string();
                    let mut frame = 0;

                    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(500), move || {
                         if let Some(app) = ah_timer.upgrade() {
                             let elapsed = start_time.elapsed().as_secs();
                             let max_dots = (3 + (elapsed / 5)) as usize;
                             let max_dots = max_dots.min(18);
                             
                             frame += 1;
                             let cycle = 2 * (max_dots - 1);
                             let current_dots = if cycle == 0 {
                                 1
                             } else {
                                 let x = frame % cycle;
                                 (max_dots as isize - (max_dots as isize - 1 - x as isize).abs()) as usize
                             };

                             let mut new_output = base_content.clone();
                             new_output.push_str(&".".repeat(current_dots));
                             app.set_terminal_output(new_output.into());
                         }
                    });
                    
                    let result = executor.execute_command_streaming(&install_args, |_| {}).await;
                    timer.stop();
                    let mut current = app.get_terminal_output().to_string();
                    current.push('\n');
                    app.set_terminal_output(current.into());

                    if result.success {
                        let mut distro_registered = false;
                        app.set_install_status(i18n::t("install.verifying").into());
                        for _ in 0..15 {
                             let list_result = executor.list_distros().await;
                             if let Some(distros) = list_result.data {
                                if distros.iter().any(|d| d.name == real_id) {
                                    distro_registered = true;
                                    break;
                                }
                             }
                             tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        }

                        if !distro_registered {
                             error_msg = i18n::tr("install.verify_failed", &[real_id.clone()]);
                        } else {
                            let mut current = app.get_terminal_output().to_string();
                             current.push_str(&format!("{}\n", i18n::t("install.step_5")));
                            app.set_terminal_output(current.into());

                            if final_name != real_id || !install_path.is_empty() {
                                info!("Customization requested (Name: {}, Path: {}). Starting relocation...", final_name, install_path);
                                 app.set_install_status(i18n::t("install.customizing").into());
                                 
                                 let mut current = app.get_terminal_output().to_string();
                                 current.push_str(&format!("{}\n", i18n::t("install.step_6")));
                                app.set_terminal_output(current.into());

                                let (temp_dir, temp_file_str) = {
                                    let state = as_ptr.lock().await;
                                    let temp_dir = PathBuf::from(&state.config_manager.get_settings().temp_location);
                                    let temp_file = temp_dir.join(format!("wsl_move_{}.tar", uuid::Uuid::new_v4()));
                                    (temp_dir, temp_file.to_string_lossy().to_string())
                                };
                                
                                let _ = std::fs::create_dir_all(&temp_dir);
                                let target_path = install_path.clone();

                                executor.execute_command(&["--export", &real_id, &temp_file_str]).await;
                                
                                let mut current = app.get_terminal_output().to_string();
                                current.push_str(&format!("{}\n", i18n::t("install.step_7")));
                                app.set_terminal_output(current.into());

                                executor.execute_command(&["--unregister", &real_id]).await;
                                
                                let final_path = if target_path.is_empty() {
                                    let state = as_ptr.lock().await;
                                    let base = PathBuf::from(&state.config_manager.get_settings().distro_location);
                                    base.join(&final_name).to_string_lossy().to_string()
                                } else {
                                    target_path
                                };
                                let _ = std::fs::create_dir_all(&final_path);
                                info!("Importing to final path: {}", final_path);
                                
                                let import_res = executor.execute_command(&["--import", &final_name, &final_path, &temp_file_str]).await;
                                let _ = std::fs::remove_file(&temp_file_str);
                                
                                success = import_res.success;
                                if success {
                                    let mut current = app.get_terminal_output().to_string();
                                    current.push_str(&format!("{}\n", i18n::t("install.step_8")));
                                    app.set_terminal_output(current.into());

                                    let list_result = executor.list_distros().await;
                                     if let Some(distros) = list_result.data {
                                        if distros.iter().any(|d| d.name == final_name) {
                                             let mut current = app.get_terminal_output().to_string();
                                             current.push_str(&format!("{}\n", i18n::t("install.step_9")));
                                             app.set_terminal_output(current.into());
                                        }
                                     }
                                } else {
                                    error_msg = import_res.error.unwrap_or_else(|| i18n::t("install.import_failed_custom"));
                                }
                            } else {
                                success = true;
                                let mut current = app.get_terminal_output().to_string();
                                current.push_str(&format!("{}\n", i18n::t("install.step_9")));
                                app.set_terminal_output(current.into());
                            }
                        }
                    } else {
                        if !result.output.trim().is_empty() {
                             let mut current = app.get_terminal_output().to_string();
                             current.push_str(&format!("\n[WSL Output]\n{}\n", result.output));
                             app.set_terminal_output(current.into());
                        }
                        error_msg = result.error.unwrap_or_else(|| i18n::t("install.install_failed"));
                    }
                },
                0 | 1 => { // RootFS or VHDX
                    if file_path.is_empty() {
                        error_msg = i18n::t("install.select_file");
                    } else {
                        app.set_terminal_output(format!("{}\n", i18n::t("install.step_1_3")).into());
                        
                        let mut target_path = install_path.clone();
                        if target_path.is_empty() {
                            let state = as_ptr.lock().await;
                            let base = PathBuf::from(&state.config_manager.get_settings().distro_location);
                            target_path = base.join(&final_name).to_string_lossy().to_string();
                        }
                        if let Err(e) = std::fs::create_dir_all(&target_path) {
                            error_msg = format!("Failed to create directory: {}", e);
                            app.set_install_success(false);
                            app.set_install_status(format!("Error: {}", error_msg).into());
                            app.set_is_installing(false);
                            return;
                        }

                        info!("Importing file '{}' to '{}' as '{}'...", file_path, target_path, final_name);

                        let mut import_args = vec!["--import", &final_name, &target_path, &file_path];
                        if source_idx == 1 {
                            import_args.push("--vhd");
                        }

                        app.set_install_status(i18n::t("install.importing").into());
                        
                        let cmd_str = format!("wsl {}", import_args.join(" "));
                        let mut current = app.get_terminal_output().to_string();
                        current.push_str(&format!("{}\n", i18n::tr("install.step_2_3", &[cmd_str.clone()])));
                        app.set_terminal_output(current.into());

                        let result = executor.execute_command_streaming(&import_args, |_| {}).await;

                        success = result.success;
                        if !success {
                             if !result.output.trim().is_empty() {
                                 let mut current = app.get_terminal_output().to_string();
                                 current.push_str(&format!("\n[WSL Output]\n{}\n", result.output));
                                 app.set_terminal_output(current.into());
                            }
                            error_msg = result.error.unwrap_or_else(|| i18n::t("install.import_failed"));
                        } else {
                            let mut current = app.get_terminal_output().to_string();
                            current.push_str(&format!("{}\n", i18n::tr("install.step_3_3", &[final_name.clone()])));
                            app.set_terminal_output(current.into());
                        }
                    }
                },
                _ => {
                    error_msg = i18n::t("install.unknown_source");
                }
            }

            if success {
                app.set_install_success(true);
                app.set_install_status(i18n::tr("install.created_success", &[final_name.clone()]).into());
                refresh_distros_ui(ah.clone(), as_ptr.clone()).await;
            } else {
                app.set_install_success(false);
                app.set_install_status(format!("{}: {}", i18n::t("install.error"), error_msg).into());
            }
            app.set_is_installing(false);
        });
    });
}
