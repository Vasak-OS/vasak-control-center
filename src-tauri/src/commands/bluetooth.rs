use serde::Serialize;
use std::process::Command;
use std::str::from_utf8;

#[derive(Debug, Serialize, Clone)]
pub struct BluetoothState {
    enabled: bool,
    connected_devices: Vec<String>,
}

impl Default for BluetoothState {
    fn default() -> Self {
        BluetoothState {
            enabled: false,
            connected_devices: Vec::new(),
        }
    }
}

#[tauri::command]
pub fn get_bluetooth_state() -> Result<BluetoothState, String> {
    // Verificar si bluetooth está habilitado
    let enabled = is_bluetooth_enabled()?;
    
    if !enabled {
        return Ok(BluetoothState::default());
    }

    // Obtener dispositivos conectados
    let output = Command::new("bluetoothctl")
        .args(["devices"])
        .output()
        .map_err(|e| format!("Error executing bluetoothctl: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "bluetoothctl command failed: {}",
            from_utf8(&output.stderr).unwrap_or("Unknown error")
        ));
    }

    let devices = from_utf8(&output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in bluetoothctl output: {}", e))?
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                Some(parts[2..].join(" "))
            } else {
                None
            }
        })
        .collect();

    Ok(BluetoothState {
        enabled,
        connected_devices: devices,
    })
}

fn is_bluetooth_enabled() -> Result<bool, String> {
    let output = Command::new("bluetoothctl")
        .args(["show"])
        .output()
        .map_err(|e| format!("Error checking bluetooth status: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to check bluetooth status: {}",
            from_utf8(&output.stderr).unwrap_or("Unknown error")
        ));
    }

    let status = from_utf8(&output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in output: {}", e))?;

    Ok(status.contains("Powered: yes"))
}

#[tauri::command]
pub fn toggle_bluetooth(enable: bool) -> Result<(), String> {
    let action = if enable { "on" } else { "off" };
    
    let output = Command::new("bluetoothctl")
        .args(["power", action])
        .output()
        .map_err(|e| format!("Error toggling bluetooth: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to toggle bluetooth: {}",
            from_utf8(&output.stderr).unwrap_or("Unknown error")
        ));
    }

    Ok(())
} 