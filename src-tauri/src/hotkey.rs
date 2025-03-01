use crate::session::ActiveSession;
use crate::storage::Storage;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter as _};
use tauri_plugin_global_shortcut::{Code, Modifiers};

// Shortcut definitions
pub const DEFAULT_START_STOP_SHORTCUT: (&str, Modifiers, Code) = (
    "start_stop",
    Modifiers::ALT,
    Code::Numpad2,
);

pub const DEFAULT_MARKER_SHORTCUT: (&str, Modifiers, Code) = (
    "marker",
    Modifiers::ALT,
    Code::Numpad3,
);

pub fn handle_start_stop(
    app_handle: &AppHandle,
    active_session: &Arc<Mutex<Option<ActiveSession>>>,
    storage: &Arc<Mutex<Storage>>,
) {
    let mut active_session_guard = active_session.lock().unwrap();

    match *active_session_guard {
        Some(ref active) => {
            // Stop the session
            let completed_session = active.to_session();

            // Save the session to storage
            if let Ok(mut storage_guard) = storage.lock() {
                if let Err(err) = storage_guard.save_session(&completed_session) {
                    log::error!("Failed to save session: {:?}", err);
                    // Notify user of error
                    show_notification(app_handle, "Error", "Failed to save session");
                } else {
                    // Notify user of successful stop
                    show_notification(
                        app_handle,
                        "Session Stopped",
                        &format!(
                            "Session duration: {}m",
                            (completed_session.end_time - completed_session.start_time)
                                .num_minutes()
                        ),
                    );

                    // Emit event to UI
                    app_handle
                        .emit("session-stopped", completed_session)
                        .unwrap_or_else(|e| {
                            log::error!("Failed to emit session-stopped event: {:?}", e)
                        });
                }
            }

            // Clear active session
            *active_session_guard = None;
        }
        None => {
            // Start a new session
            let new_session = ActiveSession::new();

            // Notify user
            show_notification(
                app_handle,
                "Session Started",
                "Use Ctrl+Shift+M to add markers",
            );

            // Emit event to UI
            app_handle
                .emit("session-started", &new_session.id)
                .unwrap_or_else(|e| log::error!("Failed to emit session-started event: {:?}", e));

            // Set as active session
            *active_session_guard = Some(new_session);
        }
    }
}

pub fn handle_marker(app_handle: &AppHandle, active_session: &Arc<Mutex<Option<ActiveSession>>>) {
    let mut active_session_guard = active_session.lock().unwrap();

    match *active_session_guard {
        Some(ref mut active) => {
            // Add marker to active session
            active.add_marker("Marker");

            // Notify user
            show_notification(
                app_handle,
                "Marker Added",
                "Marker added to current session",
            );

            // Emit event to UI
            app_handle
                .emit("marker-added", active.markers.last().unwrap())
                .unwrap_or_else(|e| log::error!("Failed to emit marker-added event: {:?}", e));
        }
        None => {
            // No active session
            show_notification(
                app_handle,
                "No Active Session",
                "Start a session first with Ctrl+Shift+S",
            );
        }
    }
}

fn show_notification(app_handle: &AppHandle, title: &str, message: &str) {
    app_handle
        .emit("notification", (title, message))
        .unwrap_or_else(|e| log::error!("Failed to emit notification event: {:?}", e));
}
