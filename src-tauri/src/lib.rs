pub mod commands;
mod window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(tauri::generate_handler![
            commands::icons::get_icon_base64,
            commands::icons::get_symbol_base64,
            window::initialize_window,
            commands::audio::get_volume,
            commands::audio::set_volume,
            commands::audio::toggle_mute,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
