use dbus::{
    blocking::Connection,
    message::{MatchRule, Message},
    arg::ReadAll,
    channel::MatchingReceiver,
};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::thread;

static NOTIFICATIONS: Lazy<Mutex<Vec<Notification>>> = Lazy::new(|| Mutex::new(Vec::new()));
const MAX_NOTIFICATIONS: usize = 5;

#[derive(Debug, Serialize, Clone)]
pub struct Notification {
    id: u32,
    app_name: String,
    app_icon: String,
    summary: String,
    body: String,
    timestamp: u64,
    seen: bool,
}

#[tauri::command]
pub fn get_unread_notifications() -> Result<Vec<Notification>, String> {
    let notifications = NOTIFICATIONS.lock().map_err(|e| e.to_string())?;
    Ok(notifications.clone())
}

#[tauri::command]
pub fn mark_notification_as_seen(id: u32) -> Result<(), String> {
    let mut notifications = NOTIFICATIONS.lock().map_err(|e| e.to_string())?;
    notifications.retain(|n| n.id != id);
    Ok(())
}

fn add_notification(
    app_name: String,
    app_icon: String,
    summary: String,
    body: String,
) -> Result<(), String> {
    let mut notifications = NOTIFICATIONS.lock().map_err(|e| e.to_string())?;
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let notification = Notification {
        id: timestamp as u32,
        app_name,
        app_icon,
        summary,
        body,
        timestamp,
        seen: false,
    };

    notifications.insert(0, notification);
    if notifications.len() > MAX_NOTIFICATIONS {
        notifications.truncate(MAX_NOTIFICATIONS);
    }

    Ok(())
}

pub fn initialize_notifications() -> Result<(), String> {

    // Iniciar el hilo de escucha
    thread::spawn(|| {
        if let Err(e) = listen_for_notifications() {
            eprintln!("Error listening for notifications: {}", e);
        }
    });

    Ok(())
}

fn listen_for_notifications() -> Result<(), String> {
    let conn = Connection::new_session().map_err(|e| e.to_string())?;
    
    let rule = MatchRule::new()
        .with_interface("org.freedesktop.Notifications")
        .with_member("Notify");
    
    conn.start_receive(
        rule,
        Box::new(move |msg, _| {
            let mut iter = msg.iter_init();
            
            if let (Ok(app_name), Ok(id), Ok(app_icon), Ok(summary), Ok(body)) = (
                iter.read::<&str>(),
                iter.read::<u32>(),
                iter.read::<&str>(),
                iter.read::<&str>(),
                iter.read::<&str>(),
            ) {
                if let Err(e) = add_notification(
                    app_name.to_string(),
                    app_icon.to_string(),
                    summary.to_string(),
                    body.to_string(),
                ) {
                    eprintln!("Error adding notification: {}", e);
                }
            }
            true
        }),
    );

    // Mantener el hilo vivo y procesar mensajes
    loop {
        conn.process(std::time::Duration::from_millis(1000))
            .map_err(|e| e.to_string())?;
    }
} 