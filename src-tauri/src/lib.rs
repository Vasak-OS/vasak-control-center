#[path = "./commands/icons.rs"]
mod icons;
mod window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(tauri::generate_handler![
            icons::get_icon_base64,
            icons::get_symbol_base64,
            window::initialize_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
