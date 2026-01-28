use std::sync::Arc;
use tokio::sync::Mutex;
use rand::{Rng, distr::Alphanumeric};
use crate::{AppWindow, AppState};

pub mod lifecycle;
pub mod manage;
pub mod export;
pub mod clone;
pub mod install;
pub mod move_distro;

pub fn sanitize_instance_name(name: &str) -> String {
    let mut sanitized: String = name.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
        .collect();
    
    if sanitized.len() > 25 {
        sanitized.truncate(25);
    }
    sanitized
}

pub fn generate_random_suffix(name: &str) -> String {
    let random_suffix: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();
    format!("{}_{}", name, random_suffix)
}

pub fn setup(app: &AppWindow, app_handle: slint::Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    lifecycle::setup(app, app_handle.clone(), app_state.clone());
    manage::setup(app, app_handle.clone(), app_state.clone());
    export::setup(app, app_handle.clone(), app_state.clone());
    clone::setup(app, app_handle.clone(), app_state.clone());
    install::setup(app, app_handle.clone(), app_state.clone());
    move_distro::setup(app, app_handle.clone(), app_state.clone());
}
