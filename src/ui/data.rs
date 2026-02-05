use std::sync::Arc;
use std::rc::Rc;
use tokio::sync::Mutex;
use tracing::{warn, trace, debug};
use slint::{ModelRc, VecModel, Model};

// Import Slint UI components
use crate::{AppState, AppWindow, Distro, InstallableDistro, wsl};

// Refresh all core data
pub async fn refresh_data(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    debug!("refresh_data: Starting background data refresh");
    let ah = app_handle.clone();
    let as_ptr = app_state.clone();
    
    // Only refresh installed list at startup, it's lightweight
    refresh_distros_ui(ah, as_ptr).await;
    
    debug!("refresh_data: Background data refresh complete");
}

// Refresh UI list of installed distributions
pub async fn refresh_distros_ui(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    debug!("refresh_distros_ui: Locking app_state");
    let distros = {
        let app_state_lock = app_state.lock().await;
        
        let mut distros = app_state_lock.wsl_dashboard.get_distros().await;
        // Sort by name A-Z
        distros.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        
        debug!("refresh_distros_ui: Found {} distributions", distros.len());
        for distro in &distros {
            debug!("refresh_distros_ui: {} status: {:?}", distro.name, distro.status);
        }
        distros
    };

    debug!("refresh_distros_ui: Starting model conversion");
    // get executor and manual operation status outside loop
    let (executor, is_manual_op) = {
        let app_state_lock = app_state.lock().await;
        (
            app_state_lock.wsl_dashboard.executor().clone(),
            app_state_lock.wsl_dashboard.is_manual_operation()
        )
    };

    let mut intermediate_distros = Vec::new();
    for d in distros {
        let mut icon_key: Option<&'static str> = crate::utils::icon_mapper::map_name_to_icon_key(&d.name);
        
        // If not found and running, try os-release (skip if manual operation is in progress to avoid contention)
        if !is_manual_op && icon_key.is_none() && d.status == wsl::models::WslStatus::Running {
             let result = executor.execute_command(&["-d", d.name.as_str(), "--exec", "cat", "/etc/os-release"]).await;
             if result.success {
                 for line in result.output.lines() {
                     if line.starts_with("ID=") {
                         let id = line.trim_start_matches("ID=").trim_matches('"');
                         let mapped = crate::utils::icon_mapper::map_name_to_icon_key(id);
                         if mapped.is_some() { 
                             icon_key = mapped;
                             break; 
                         }
                     }
                 }
             }
        }

        intermediate_distros.push((
            d.name.clone(),
            match d.status {
                wsl::models::WslStatus::Running => "Running",
                wsl::models::WslStatus::Stopped => "Stopped",
            },
            match d.version {
                wsl::models::WslVersion::V1 => "1",
                wsl::models::WslVersion::V2 => "2",
            },
            d.is_default,
            icon_key,
            crate::utils::icon_mapper::get_initial(&d.name),
        ));
    }

    debug!("refresh_distros_ui: Preparing UI update, model count: {}", intermediate_distros.len());

    let result = slint::invoke_from_event_loop(move || {
        trace!("refresh_distros_ui: Executing in UI thread");
        let slint_distros: Vec<Distro> = intermediate_distros.into_iter().map(|(name, status, version, is_default, icon_key, initial)| {
            let mut image = slint::Image::default();
            let mut has_icon = false;
            
            if let Some(key) = icon_key {
                if let Some(img) = crate::utils::icon_mapper::load_icon(key) {
                    image = img;
                    has_icon = true;
                }
            }

            Distro {
                name: name.into(),
                status: status.into(),
                version: version.into(),
                is_default,
                icon: image,
                has_icon,
                initial: initial.into(),
                distro_display_name: crate::utils::icon_mapper::get_display_name(icon_key).into(),
            }
        }).collect();

        let model = VecModel::from(slint_distros);
        let model_rc = ModelRc::from(Rc::new(model));
        
        if let Some(app) = app_handle.upgrade() {
            trace!("refresh_distros_ui: App instance acquired, setting model");
            app.set_distros(model_rc);
            trace!("refresh_distros_ui: UI model update complete");
        } else {
            warn!("refresh_distros_ui: Could not acquire app instance");
        }
    });
    
    match result {
        Ok(_) => {
            debug!("refresh_distros_ui: UI update successful");
        }
        Err(e) => {
            warn!("refresh_distros_ui: UI update failed: {}", e);
        }
    }
}

// Refresh UI list of installable distributions
pub async fn refresh_installable_distros(app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    let result = {
        let app_state = app_state.lock().await;
        app_state.wsl_dashboard.executor().list_available_distros().await
    };

    if result.success {
        let mut available = wsl::parser::parse_available_distros(&result.output);
        // Sort by distribution name Z-A (Reverse order as requested)
        available.sort_by(|a, b| b.1.to_lowercase().cmp(&a.1.to_lowercase()));
        
        let slint_installables: Vec<InstallableDistro> = available.iter().map(|(name, friendly)| {
            InstallableDistro {
                name: name.clone().into(),
                friendly_name: friendly.clone().into(),
            }
        }).collect();

        let friendly_names: Vec<slint::SharedString> = available.iter().map(|(_, friendly)| {
            friendly.clone().into()
        }).collect();

        let _ = slint::invoke_from_event_loop(move || {
            let model = VecModel::from(slint_installables);
            let model_rc = ModelRc::from(Rc::new(model));
            
            let names_model = VecModel::from(friendly_names);
            let names_rc = ModelRc::from(Rc::new(names_model));
            
            if let Some(app) = app_handle.upgrade() {
                app.set_installable_distros(model_rc);
                app.set_installable_distro_names(names_rc);
                
                // If no selection, default to first item and sync UI fields
                if app.get_selected_install_distro().is_empty() && app.get_installable_distro_names().row_count() > 0 {
                    if let Some(first) = app.get_installable_distro_names().row_data(0) {
                        app.set_selected_install_distro(first.clone());
                        // Trigger the synchronization logic for Instance Name and Path
                        app.invoke_distro_selected(first);
                    }
                }
            }
        });
    }
}
