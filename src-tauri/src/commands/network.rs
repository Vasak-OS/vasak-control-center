use serde::Serialize;
use std::process::Command;
use std::str::from_utf8;

#[derive(Debug, Serialize, Clone)]
pub enum NetworkType {
    Ethernet,
    Wifi,
    Disconnected,
}

#[derive(Debug, Serialize, Clone)]
pub struct NetworkState {
    connected: bool,
    network_type: NetworkType,
    interface_name: String,
    enabled: bool,
}

impl Default for NetworkState {
    fn default() -> Self {
        NetworkState {
            connected: false,
            network_type: NetworkType::Disconnected,
            interface_name: String::new(),
            enabled: false,
        }
    }
}

#[tauri::command]
pub fn get_network_state() -> Result<NetworkState, String> {
    // Primero verificamos si la red está habilitada
    let enabled = is_network_enabled()?;

    if !enabled {
        return Ok(NetworkState {
            enabled: false,
            ..Default::default()
        });
    }

    // Si está habilitada, obtenemos el estado detallado
    let output = Command::new("nmcli")
        .args(["device", "status"])
        .output()
        .map_err(|e| format!("Error executing nmcli: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "nmcli command failed: {}",
            from_utf8(&output.stderr).unwrap_or("Unknown error")
        ));
    }

    let status =
        from_utf8(&output.stdout).map_err(|e| format!("Invalid UTF-8 in nmcli output: {}", e))?;

    let mut state = NetworkState {
        enabled: true,
        ..Default::default()
    };

    // Buscar la primera conexión activa
    for line in status.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let device_type = parts[1];
            let connection_status = parts[2];

            if connection_status == "connected" {
                state.connected = true;
                state.interface_name = parts[0].to_string();
                state.network_type = match device_type {
                    "ethernet" => NetworkType::Ethernet,
                    "wifi" => NetworkType::Wifi,
                    _ => NetworkType::Disconnected,
                };
                break;
            }
        }
    }

    Ok(state)
}

fn is_network_enabled() -> Result<bool, String> {
    let output = Command::new("nmcli")
        .args(["networking"])
        .output()
        .map_err(|e| format!("Error checking network status: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to check network status: {}",
            from_utf8(&output.stderr).unwrap_or("Unknown error")
        ));
    }

    let status = from_utf8(&output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in output: {}", e))?
        .trim();

    Ok(status == "enabled")
}

#[tauri::command]
pub fn toggle_network(enable: bool) -> Result<(), String> {
    let action = if enable { "on" } else { "off" };

    let output = Command::new("nmcli")
        .args(["networking", action])
        .output()
        .map_err(|e| format!("Error toggling network: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to toggle network: {}",
            from_utf8(&output.stderr).unwrap_or("Unknown error")
        ));
    }

    Ok(())
}
