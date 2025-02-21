use tauri::{Manager, Window};
use tauri_plugin_positioner::{WindowExt, Position};

#[tauri::command]
pub async fn initialize_window(window: Window) {
    let monitor = window.current_monitor().unwrap().unwrap();
    let monitor_size = monitor.size();
    
    // Configuramos el tamaño inicial de la ventana
    window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
        width: 300,  // Ancho del panel lateral
        height: monitor_size.height - 100, // Alto de la pantalla menos 100px
    }))
    .unwrap();

    // Posicionamos la ventana en el lado derecho
    window
        .move_window(Position::RightCenter)
        .unwrap();

    // Ajustamos la posición Y para centrar verticalmente
    let y_position = (monitor_size.height - (monitor_size.height - 100)) / 2;
    window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
        x: (monitor_size.width - 300) as i32,
        y: y_position as i32,
    }))
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
                // window_clone.close().unwrap_or_else(|e| {
                //     eprintln!("Error closing window: {}", e);
                // });
            }
        }
    });
} 