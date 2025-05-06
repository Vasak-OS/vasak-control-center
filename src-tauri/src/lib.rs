pub mod commands;
mod window;

use tauri_plugin_vicons;
use tauri_plugin_user_data;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Inicializar notificaciones
    if let Err(e) = commands::notifications::initialize_notifications() {
        eprintln!("Error initializing notifications: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_vicons::init())
        .plugin(tauri_plugin_user_data::init())
        .invoke_handler(tauri::generate_handler![
            commands::audio::get_volume,
            commands::audio::set_volume,
            commands::audio::toggle_mute,
            commands::notifications::get_unread_notifications,
            commands::notifications::mark_notification_as_seen,
            commands::network::get_network_state,
            commands::network::toggle_network,
            commands::bluetooth::get_bluetooth_state,
            commands::bluetooth::toggle_bluetooth,
            commands::brightness::get_brightness,
            commands::brightness::set_brightness,
            commands::theme::toggle_system_theme,
            window::initialize_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
