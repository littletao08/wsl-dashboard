use tokio::fs;
use std::time::Duration;
use tracing::{info, warn};

pub fn get_startup_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let path = dirs::data_dir()
        .ok_or("Could not find AppData directory")?
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("Startup");
    Ok(path)
}

pub fn get_vbs_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    Ok(get_startup_dir()?.join("wsl-dashboard.vbs"))
}

/// Writes to a file with a timeout mechanism to avoid hanging for a long time if intercepted by anti-virus software
/// 
/// If the write operation does not complete within 5 seconds, it returns a timeout error
async fn write_with_timeout(
    path: &std::path::Path,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.to_path_buf();
    
    // Use tokio::time::timeout to set a 5-second timeout
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        fs::write(&path, content)
    ).await;
    
    match result {
        Ok(Ok(())) => {
            // Write succeeded
            Ok(())
        }
        Ok(Err(e)) => {
            // Write failed
            Err(Box::new(e))
        }
        Err(_) => {
            // Timeout
            warn!("File write timed out (5s), possibly intercepted by anti-virus software");
            Err("File write timed out, possibly intercepted by anti-virus software. Please check your anti-virus settings.".into())
        }
    }
}

/// Updates the Windows startup VBS file to add or remove autostart for a WSL distribution.
pub async fn update_windows_autostart(distro_name: &str, enable: bool) -> Result<(), Box<dyn std::error::Error>> {
    let startup_dir = get_startup_dir()?;
    
    if !startup_dir.exists() {
        fs::create_dir_all(&startup_dir).await?;
    }
    
    let vbs_path = get_vbs_path()?;
    let line_to_manage = format!("ws.run \"wsl -d {} -u root /etc/init.wsl-dashboard start\", vbhide", distro_name);
    let header = "Set ws = WScript.CreateObject(\"WScript.Shell\")";

    let mut lines: Vec<String> = if vbs_path.exists() {
        let content = fs::read_to_string(&vbs_path).await?;
        content.lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        vec![header.to_string()]
    };

    // Ensure header is always at the top
    if !lines.iter().any(|l: &String| l.contains("WScript.CreateObject")) {
        lines.insert(0, header.to_string());
    }

    if enable {
        // Add if not present
        if !lines.iter().any(|l| l == &line_to_manage) {
            lines.push(line_to_manage);
            info!("✅ Added autostart line for '{}' to VBS", distro_name);
        }
    } else {
        // Remove strictly matching lines
        let old_count = lines.len();
        lines.retain(|l| l != &line_to_manage);
        if lines.len() < old_count {
            info!("✅ Removed autostart line for '{}' from VBS", distro_name);
        }
    }

    // Write back with timeout protection
    let content = lines.join("\r\n");
    write_with_timeout(&vbs_path, content).await?;
    info!("📂 Updated autostart VBS: {}", vbs_path.display());

    Ok(())
}

pub fn is_autostart_enabled(distro_name: &str) -> bool {
    let vbs_path = match get_vbs_path() {
        Ok(p) => p,
        Err(_) => return false,
    };

    if !vbs_path.exists() {
        return false;
    }

    let line_to_check = format!("ws.run \"wsl -d {} -u root /etc/init.wsl-dashboard start\", vbhide", distro_name);
    
    // Explicitly use std::fs for sync read to avoid any tokio::fs async/sync confusion
    if let Ok(content) = std::fs::read_to_string(&vbs_path) {
        content.lines().any(|l| l.trim() == line_to_check)
    } else {
        false
    }
}
