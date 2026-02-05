use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::sync::{Mutex, Notify};
use tokio::time::Duration;

use crate::wsl::command::WslCommandExecutor;
use crate::wsl::models::{WslDistro, WslCommandResult, WslStatus};

// WSL state manager, responsible for managing and monitoring the status of WSL subsystems
//
// Main functions:
// 1. Periodically monitor the running status of WSL subsystems
// 2. Provide interfaces to operate WSL subsystems (start, stop, restart, etc.)
// 3. Maintain the list of WSL subsystems and their status information
// 4. Provide a status change notification mechanism
#[derive(Clone)]
pub struct WslDashboard {
    // WSL command executor, responsible for executing actual WSL commands
    executor: WslCommandExecutor,
    // List of currently installed WSL subsystems, using Mutex for concurrent safety
    pub distros: Arc<Mutex<Vec<WslDistro>>>,
    // Status refresh interval, default 5 seconds
    refresh_interval: Duration,
    // Status change notifier, notifies listeners when WSL status changes
    state_changed: Arc<Notify>,
    // Manual operation counter, prevents conflict between background monitoring and manual operations.
    // > 0 means a manual operation is in progress.
    manual_operation: Arc<std::sync::atomic::AtomicI32>,
    // Heavy operation lock, ensures only one heavy WSL operation (delete, move, etc.) runs at a time
    heavy_op_lock: Arc<Mutex<()>>,
}

impl WslDashboard {
    // Create a new WSL state manager instance
    //
    // Initialize with default configuration:
    // - Use default WSL command executor
    // - Empty subsystem list
    // - Default refresh interval of 5 seconds
    // - Initialize status change notifier
    pub fn new() -> Self {
        Self {
            executor: WslCommandExecutor::new(),
            distros: Arc::new(Mutex::new(Vec::new())),
            refresh_interval: Duration::from_secs(5),
            state_changed: Arc::new(Notify::new()),
            manual_operation: Arc::new(std::sync::atomic::AtomicI32::new(0)),
            heavy_op_lock: Arc::new(Mutex::new(())),
        }
    }

    // Get WSL command executor
    pub fn executor(&self) -> &WslCommandExecutor {
        &self.executor
    }

    // Set status refresh interval
    #[allow(dead_code)]
    pub fn set_refresh_interval(&mut self, interval: Duration) {
        self.refresh_interval = interval;
    }

    // Get status change notifier
    pub fn state_changed(&self) -> &Arc<Notify> {
        &self.state_changed
    }

    // Check if manual operation is in progress
    pub fn is_manual_operation(&self) -> bool {
        self.manual_operation.load(Ordering::SeqCst) > 0
    }

    // Explicitly set manual operation (use with caution, better use inc/dec)
    pub fn set_manual_operation(&self, value: bool) {
        if value {
            self.increment_manual_operation();
        } else {
            self.decrement_manual_operation();
        }
    }

    pub fn increment_manual_operation(&self) {
        self.manual_operation.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_manual_operation(&self) {
        let old = self.manual_operation.fetch_sub(1, Ordering::SeqCst);
        if old <= 0 {
            // Safety: don't go below 0
            self.manual_operation.store(0, Ordering::SeqCst);
        }
    }

    // Manually refresh WSL subsystem list
    //
    // Get the latest subsystem list information from WSL system and update local cache
    // If subsystem status changes, notify all listeners
    //
    // # Return value
    // - Success: Returns `WslCommandResult` containing the latest subsystem list
    // - Failure: Returns `WslCommandResult` containing error information
    pub async fn refresh_distros(&self) -> WslCommandResult<Vec<WslDistro>> {
        let result = self.executor.list_distros().await;
        if result.success {
            if let Some(distros) = result.data.clone() {
                let mut distros_lock = self.distros.lock().await;
                let old_distros = distros_lock.clone();
                *distros_lock = distros.clone();
                
                // Check if status changed (using business logic equality comparison)
                let mut has_changes = false;
                if old_distros.len() != distros.len() {
                    has_changes = true;
                } else {
                    for (old, new) in old_distros.iter().zip(distros.iter()) {
                        if !old.business_equals(new) {
                            has_changes = true;
                            break;
                        }
                    }
                }
                
                // If changed, notify status change
                if has_changes {
                    tracing::debug!("WSL distribution list changed, notifying UI update");
                    self.state_changed.notify_one();
                } else {
                    tracing::debug!("No changes detected in WSL distribution list");
                }
            }
        }
        result
    }

    // Get current WSL subsystem list
    pub async fn get_distros(&self) -> Vec<WslDistro> {
        let distros_lock = self.distros.lock().await;
        distros_lock.clone()
    }

    // Start WSL subsystem status monitoring task
    //
    // Create a periodic monitoring task in the background to check WSL subsystem status
    // at the configured refresh interval
    // If status changes are detected, update local cache and notify listeners
    // Pause monitoring when manual operations are in progress to avoid conflicts
    pub async fn start_monitoring(&self) {
        let manager = self.clone();
        tokio::spawn(async move {
            loop {
                // Check if manual operation is being executed
                if !manager.is_manual_operation() {
                    // Only perform automatic refresh when not in manual operation
                    let _ = manager.refresh_distros().await;
                }
                
                tokio::time::sleep(manager.refresh_interval).await;
            }
        });
    }

    // Get WSL subsystem with specified name
    #[allow(dead_code)]
    pub async fn get_distro(&self, name: &str) -> Option<WslDistro> {
        let distros_lock = self.distros.lock().await;
        distros_lock.iter().find(|d| d.name == name).cloned()
    }

    // Check if WSL subsystem is running
    #[allow(dead_code)]
    pub async fn is_distro_running(&self, name: &str) -> bool {
        if let Some(distro) = self.get_distro(name).await {
            return matches!(distro.status, WslStatus::Running);
        }
        false
    }

    // Start the specified WSL subsystem
    //
    // Start the WSL subsystem with the specified name and refresh status after successful startup
    // Set manual operation flag during operation to prevent conflicts with background monitoring
    //
    // # Parameters
    // - `name`: Name of the WSL subsystem to start
    //
    // # Return value
    // - Success: Returns `WslCommandResult` containing success message
    // - Failure: Returns `WslCommandResult` containing error information
    pub async fn start_distro(&self, name: &str) -> WslCommandResult<String> {
        // Set manual operation flag to prevent background monitoring interference
        self.increment_manual_operation();
        
        let result = self.executor.start_distro(name).await;
        if result.success {
            tracing::info!("WSL distro '{}' startup command executed, waiting for status update", name);
            
            // Immediately refresh status to get latest subsystem state
            let _ = self.refresh_distros().await;
            
            // Refresh status again after 3-second delay to give WSL enough time to start
            let manager_clone = self.clone();
            let name_clone = name.to_string();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(3)).await;
                tracing::info!("Delayed refresh of WSL distro '{}' status after startup", name_clone);
                let _ = manager_clone.refresh_distros().await;
                
                // Clear manual operation flag after operation completes
                manager_clone.decrement_manual_operation();
                tracing::info!("Manual operation completed, resumed automatic monitoring");
            });
        } else {
            // Also clear manual operation flag on operation failure
            self.decrement_manual_operation();
        }
        result
    }

    // Stop the specified WSL subsystem
    //
    // Stop the WSL subsystem with the specified name and refresh status after successful stoppage
    // Set manual operation flag during operation to prevent conflicts with background monitoring
    //
    // # Parameters
    // - `name`: Name of the WSL subsystem to stop
    //
    // # Return value
    // - Success: Returns `WslCommandResult` containing success message
    // - Failure: Returns `WslCommandResult` containing error information
    pub async fn stop_distro(&self, name: &str) -> WslCommandResult<String> {
        // Set manual operation flag to prevent background monitoring interference
        self.increment_manual_operation();
        
        let result = self.executor.stop_distro(name).await;
        if result.success {
            tracing::info!("WSL distro '{}' termination command executed, waiting for status update", name);
            
            // Immediately refresh status to get latest subsystem state
            let _ = self.refresh_distros().await;
            
            // Refresh status again after 3-second delay to give WSL enough time to stop
            let manager_clone = self.clone();
            let name_clone = name.to_string();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(3)).await;
                tracing::info!("Delayed refresh of WSL distro '{}' status after termination", name_clone);
                let _ = manager_clone.refresh_distros().await;
                
                // Clear manual operation flag after operation completes
                manager_clone.decrement_manual_operation();
                tracing::info!("Manual operation completed, resumed automatic monitoring");
            });
        } else {
            // Also clear manual operation flag on operation failure
            self.decrement_manual_operation();
        }
        result
    }

    // Restart the specified WSL subsystem
    //
    // Restart the WSL subsystem with the specified name by first stopping then starting it
    // Wait appropriately during restart to ensure successful operation
    //
    // # Parameters
    // - `name`: Name of the WSL subsystem to restart
    //
    // # Return value
    // - Success: Returns `WslCommandResult` containing success message
    // - Failure: Returns `WslCommandResult` containing error information
    pub async fn restart_distro(&self, name: &str) -> WslCommandResult<String> {
        tracing::info!("WSL distro '{}' restart initiated", name);
        
        // First stop
        tracing::info!("Stopping WSL distro '{}' as part of restart process", name);
        let stop_result = self.stop_distro(name).await;
        if !stop_result.success {
            tracing::info!("Failed to stop WSL distro '{}' during restart: {:?}", name, stop_result.error);
            return stop_result;
        }
        tracing::info!("Successfully stopped WSL distro '{}'", name);
        
        // Wait for stop to complete, then start
        tracing::info!("Waiting for 4 seconds to ensure distro '{}' has fully stopped", name);
        tokio::time::sleep(Duration::from_secs(4)).await;
        
        // Then start
        tracing::info!("Starting WSL distro '{}' as part of restart process", name);
        let start_result = self.start_distro(name).await;
        if start_result.success {
            tracing::info!("WSL distro '{}' restart completed successfully", name);
        } else {
            tracing::warn!("Failed to start WSL distro '{}' during restart: {:?}", name, start_result.error);
        }
        start_result
    }

    // Shutdown the entire WSL system
    //
    // Execute WSL system-level shutdown command to stop all running WSL subsystems
    // Refresh subsystem status list after successful shutdown
    //
    // # Return value
    // - Success: Returns `WslCommandResult` containing success message
    // - Failure: Returns `WslCommandResult` containing error information
    pub async fn shutdown_wsl(&self) -> WslCommandResult<String> {
        self.increment_manual_operation();
        tracing::info!("Initiating WSL system shutdown");
        let result = self.executor.shutdown_wsl().await;
        
        if result.success {
            tracing::info!("WSL system shutdown successful");
            // Refresh status to update all distros to stopped state
            tracing::info!("Refreshing WSL distros status after system shutdown");
            let refresh_result = self.refresh_distros().await;
            if refresh_result.success {
                tracing::info!("Successfully refreshed WSL distros status");
            } else {
                tracing::warn!("Failed to refresh WSL distros status after shutdown: {:?}", refresh_result.error);
            }
        } else {
            tracing::warn!("WSL system shutdown failed: {:?}", result.error);
        }
        
        self.decrement_manual_operation();
        result
    }

    // Delete the specified WSL subsystem
    //
    // Uninstall and delete the WSL subsystem with the specified name, refresh subsystem list after deletion
    // This operation is irreversible, please use with caution
    //
    // # Parameters
    // - `name`: Name of the WSL subsystem to delete
    //
    // # Return value
    // - Success: Returns `WslCommandResult` containing success message
    // - Failure: Returns `WslCommandResult` containing error information
    pub async fn delete_distro(&self, config_manager: &crate::config::ConfigManager, name: &str) -> WslCommandResult<String> {
        let _heavy_lock = self.heavy_op_lock.lock().await;
        // Set manual operation flag to prevent background monitoring interference
        self.increment_manual_operation();

        tracing::warn!("Initiating deletion of WSL distro '{}' (irreversible operation)", name);
        let result = self.executor.delete_distro(config_manager, name).await;
        
        if result.success {
            tracing::info!("Successfully deleted WSL distro '{}'", name);
            // Refresh status to remove the deleted distro from the list
            tracing::info!("Refreshing WSL distros list after deletion (with 1s delay)");
            
            let manager = self.clone();
            tokio::spawn(async move {
                // Wait a moment for WSL service to stabilize after unregister
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                
                // Refresh with timeout
                let _ = tokio::time::timeout(
                    std::time::Duration::from_secs(5),
                    manager.refresh_distros()
                ).await;
                
                // Clear manual operation flag after operation AND refresh completes
                manager.decrement_manual_operation();
                tracing::info!("Manual operation (delete) completed, resumed automatic monitoring");
            });
        } else {
            tracing::warn!("Failed to delete WSL distro '{}': {:?}", name, result.error);
            // Clear manual operation flag on failure
            self.decrement_manual_operation();
        }

        result
    }

    // Export the specified WSL subsystem
    //
    // Export the WSL subsystem with the specified name as a WSL image file (.tar format)
    //
    // # Parameters
    // - `name`: Name of the WSL subsystem to export
    // - `file_path`: Save path for the export file
    //
    // # Return value
    // - Success: Returns `WslCommandResult` containing success message
    // - Failure: Returns `WslCommandResult` containing error information
    pub async fn export_distro(&self, name: &str, file_path: &str) -> WslCommandResult<String> {
        let _heavy_lock = self.heavy_op_lock.lock().await;
        // Set manual operation flag
        self.increment_manual_operation();

        tracing::info!("Initiating export of WSL distro '{}' to file '{}'", name, file_path);
        let result = self.executor.export_distro(name, file_path).await;
        
        if result.success {
            tracing::info!("Successfully exported WSL distro '{}' to file '{}'", name, file_path);
        } else {
            tracing::warn!("Failed to export WSL distro '{}': {:?}", name, result.error);
        }
        
        // Clear manual operation flag
        self.decrement_manual_operation();

        result
    }

    // Import a WSL subsystem
    //
    // Import a new WSL subsystem from the specified WSL image file
    //
    // # Parameters
    // - `name`: Name of the newly imported WSL subsystem
    // - `install_location`: Installation location for the WSL subsystem
    // - `file_path`: Path to the import file (.tar format)
    //
    // # Return value
    // - Success: Returns `WslCommandResult` containing success message
    // - Failure: Returns `WslCommandResult` containing error information
    pub async fn import_distro(&self, name: &str, install_location: &str, file_path: &str) -> WslCommandResult<String> {
        let _heavy_lock = self.heavy_op_lock.lock().await;
        // Set manual operation flag
        self.increment_manual_operation();

        tracing::info!("Initiating import of WSL distro '{}' from file '{}' to location '{}'", 
                      name, file_path, install_location);
        let result = self.executor.import_distro(name, install_location, file_path).await;
        
        if result.success {
            tracing::info!("Successfully imported WSL distro '{}'", name);
            // Refresh status to include the newly imported distro
            tracing::info!("Refreshing WSL distros list after import");
            let _ = self.refresh_distros().await;
        } else {
            tracing::warn!("Failed to import WSL distro '{}': {:?}", name, result.error);
        }
        
        // Clear manual operation flag
        self.decrement_manual_operation();

        result
    }

    // Move the specified WSL subsystem
    //
    // Move the WSL subsystem to a new location (VHDX move for WSL2, export/import for WSL1)
    //
    // # Parameters
    // - `name`: Name of the WSL subsystem to move
    // - `new_path`: New installation path
    //
    // # Return value
    // - Success: Returns `WslCommandResult` containing success message
    // - Failure: Returns `WslCommandResult` containing error information
    pub async fn move_distro(&self, name: &str, new_path: &str) -> WslCommandResult<String> {
        let _heavy_lock = self.heavy_op_lock.lock().await;
        // Set manual operation flag
        self.increment_manual_operation();

        tracing::info!("Initiating move of WSL distro '{}' to '{}'", name, new_path);
        let result = self.executor.move_distro(name, new_path).await;
        
        if result.success {
            tracing::info!("Successfully moved WSL distro '{}' to '{}'", name, new_path);
            // Refresh status
            let _ = self.refresh_distros().await;
        } else {
            tracing::warn!("Failed to move WSL distro '{}': {:?}", name, result.error);
        }
        
        // Clear manual operation flag
        self.decrement_manual_operation();

        result
    }

    // Open .bashrc file in distribution for editing
    pub async fn open_distro_bashrc(&self, name: &str) -> WslCommandResult<String> {
        let _path = format!(r"\\wsl$\{}\home", name);
        // Simplified handling here: try to open home directory, specific .bashrc editing usually requires determining username
        // Can edit bashrc, we'll first implement opening the corresponding distribution's home directory, or try default path
        self.executor.open_distro_folder_path(name, "~").await
    }

    // Open distribution's folder
    #[allow(dead_code)]
    pub async fn open_distro_folder(&self, distro_name: &str) -> WslCommandResult<String> {
        let _path = format!(r"\\wsl$\{}", distro_name);
        todo!("Open folder functionality to be implemented");
    }
}

impl Default for WslDashboard {
    fn default() -> Self {
        Self::new()
    }
}