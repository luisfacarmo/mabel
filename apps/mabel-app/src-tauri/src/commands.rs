use mabel_transport::{ConnectionDescriptor, RfcommTransport, WindowsRfcommTransport};
use serde::Serialize;
use std::sync::Arc;
use tauri::{Emitter, Manager, State};
use tokio::sync::Mutex;

/// Shared app state holding the transport and discovered devices.
pub struct AppState {
    transport: WindowsRfcommTransport,
    pub devices: Mutex<Vec<ConnectionDescriptor>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            transport: WindowsRfcommTransport::default(),
            devices: Mutex::new(Vec::new()),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct DeviceInfo {
    pub name: String,
    pub mac_address: String,
    pub is_soundcore: bool,
}

/// Discover connected Bluetooth devices. Returns list with Soundcore flag.
#[tauri::command]
pub async fn discover_devices(state: State<'_, Arc<AppState>>) -> Result<Vec<DeviceInfo>, String> {
    let devices = state.transport.discover().await.map_err(|e| e.to_string())?;

    let result: Vec<DeviceInfo> = devices
        .iter()
        .map(|d| DeviceInfo {
            name: d.name.clone(),
            mac_address: d.mac_address.clone(),
            is_soundcore: d.name.contains("soundcore") || d.name.contains("Space One"),
        })
        .collect();

    // Store discovered devices
    *state.devices.lock().await = devices;

    Ok(result)
}

/// Check if any Soundcore device is currently connected.
#[tauri::command]
pub async fn check_device_online(state: State<'_, Arc<AppState>>) -> Result<bool, String> {
    let devices = state.transport.discover().await.map_err(|e| e.to_string())?;
    let found = devices
        .iter()
        .any(|d| d.name.contains("soundcore") || d.name.contains("Space One"));
    Ok(found)
}
