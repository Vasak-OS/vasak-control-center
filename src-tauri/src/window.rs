use tauri::Window;
use tauri_plugin_positioner::{WindowExt, Position};

#[tauri::command]
pub async fn initialize_window(window: Window) {
    let monitor = window.current_monitor().unwrap().unwrap();
    let monitor_size = monitor.size();

    // Set the window size
    let window_width = 300;
    let window_height = monitor_size.height - 100;
    
    // Configuramos el tamaño inicial de la ventana
    window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
        width: window_width,
        height: window_height,
    }))
    .unwrap();

    // Calculate the manual position
    let x = monitor_size.width;
    let y = (monitor_size.height - window_height) / 2;

    // Set the window position
    window
        .set_position(tauri::Position::Physical(tauri::PhysicalPosition { x: x as i32, y: y as i32 }))
        .unwrap();

    // Configuramos que la ventana no sea redimensionable por el usuario
    window.set_resizable(false).unwrap();

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
} 