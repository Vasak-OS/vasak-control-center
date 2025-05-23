pub mod commands;

use gtk::prelude::*;
use tauri::Manager;
use tauri_plugin_network_manager;
use tauri_plugin_positioner::{Position, WindowExt};
use tauri_plugin_shell;
use tauri_plugin_user_data;
use tauri_plugin_vicons;

fn setup_main_window(window: &tauri::WebviewWindow) -> Result<(), Box<dyn std::error::Error>> {
    let _ = window.move_window(Position::TopRight);
    let gtk_window = window.gtk_window()?;

    gtk_window.set_resizable(false);
    gtk_window.set_type_hint(gtk::gdk::WindowTypeHint::Utility);
    gtk_window.set_urgency_hint(true);
    gtk_window.set_skip_taskbar_hint(true);
    gtk_window.set_skip_pager_hint(true);
    gtk_window.stick();

    Ok(())
}

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
        .plugin(tauri_plugin_network_manager::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .invoke_handler(tauri::generate_handler![
            commands::audio::get_volume,
            commands::audio::set_volume,
            commands::audio::toggle_mute,
            commands::notifications::get_unread_notifications,
            commands::notifications::mark_notification_as_seen,
            commands::bluetooth::get_bluetooth_state,
            commands::bluetooth::toggle_bluetooth,
            commands::brightness::get_brightness,
            commands::brightness::set_brightness,
            commands::theme::toggle_system_theme
        ])
        .setup(move |app| {
            let window = app
                .get_webview_window("main")
                .expect("main window not found");

            setup_main_window(&window)?;

            // Crear un clon de la ventana para el closure
            let window_clone = window.clone();

            // Agregar evento para cerrar cuando pierde el foco
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Focused(focused) = event {
                    if !focused {
                        // Si la ventana pierde el foco, la ocultamos
                        window_clone.close().unwrap_or_else(|e| {
                            eprintln!("Error closing window: {}", e);
                        });
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
