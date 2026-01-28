use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, warn};
use slint::{ModelRc, VecModel};
use crate::{AppWindow, AppState, RootFSHelpItem};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct RootFSHelpData {
    pub name: String,
    pub url: String,
}

// Use the VSCodeExtensionData from app::state
use crate::app::VSCodeExtensionData;

#[derive(Debug, Serialize, Deserialize)]
pub struct RootFSHelpResponse {
    #[serde(rename = "rootfs-help")]
    pub rootfs_help: Vec<RootFSHelpData>,
    #[serde(rename = "vscode-extension")]
    pub vscode_extension: Option<VSCodeExtensionData>,
}

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, _app_state: Arc<Mutex<AppState>>) {
    let ah = app_handle.clone();
    let as_ptr = _app_state.clone();
    app.on_show_rootfs_help_clicked(move || {
        debug!("RootFS help icon clicked, showing dialog and fetching latest data");
        
        // 1. Immediately show the dialog (with existing/default data)
        if let Some(app) = ah.upgrade() {
            app.set_show_rootfs_help(true);
        }

        // 2. Fetch latest data
        let ah_fetch = ah.clone();
        let as_fetch = as_ptr.clone();
        tokio::spawn(async move {
            fetch_latest_instance_data(ah_fetch, as_fetch).await;
        });
    });

    // Initial fetch on startup
    let ah_init = app_handle.clone();
    let as_init = _app_state.clone();
    tokio::spawn(async move {
        // Delay slightly to not compete with initial UI load
        tokio::time::sleep(Duration::from_secs(1)).await;
        fetch_latest_instance_data(ah_init, as_init).await;
    });
}

pub async fn fetch_latest_instance_data(ah: slint::Weak<AppWindow>, as_ptr: Arc<Mutex<AppState>>) {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (url, _timezone) = {
        let state = as_ptr.lock().await;
        let tz = state.config_manager.get_config().system.timezone.clone();
        let base_url = if tz == crate::app::constants::ZH_TIMEZONE {
            crate::app::constants::STATIC_API
        } else {
            crate::app::constants::STATIC_API_FREE
        };
        (format!("{}{}?t={}", base_url, crate::app::constants::INSTANCE_API, ts), tz)
    };

    debug!("Fetching Latest Instance data from: {}", url);

    let fetch_result = tokio::task::spawn_blocking(move || {
        match ureq::get(&url).timeout(Duration::from_secs(5)).call() {
            Ok(resp) => {
                match resp.into_json::<RootFSHelpResponse>() {
                    Ok(data) => Ok(data),
                    Err(e) => Err(format!("Failed to parse RootFS JSON: {}", e)),
                }
            }
            Err(e) => Err(format!("RootFS request error: {}", e)),
        }
    }).await;

    match fetch_result {
        Ok(Ok(data)) => {
            if !data.rootfs_help.is_empty() || data.vscode_extension.is_some() {
                debug!("Successfully fetched {} RootFS help items and VS Code extension info", data.rootfs_help.len());
                
                // Update VS Code extension info in AppState
                if let Some(ext) = data.vscode_extension.clone() {
                    let mut state = as_ptr.lock().await;
                    state.vscode_extension = Some(ext);
                }

                let items: Vec<RootFSHelpItem> = data.rootfs_help.into_iter().map(|d| {
                    RootFSHelpItem {
                        name: d.name.into(),
                        url: d.url.into(),
                    }
                }).collect();

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = ah.upgrade() {
                        let model = VecModel::from(items);
                        app.set_rootfs_help_list(ModelRc::from(std::rc::Rc::new(model)));
                        debug!("RootFS help list updated in UI");
                    }
                });
            }
        }
        Ok(Err(e)) => {
            warn!("Silent fail: fetch RootFS help failed: {}", e);
        }
        Err(e) => {
            warn!("Silent fail: RootFS fetch task panicked: {}", e);
        }
    }
}
