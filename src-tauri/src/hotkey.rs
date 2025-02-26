use crate::session::ActiveSession;
use crate::storage::Storage;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter as _};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

// Shortcut definitions
const START_STOP_SHORTCUT: (&str, Modifiers, Code) = (
    "start_stop",
    Modifiers::SHIFT,
    Code::KeyS,
);

const MARKER_SHORTCUT: (&str, Modifiers, Code) = (
    "marker",
    Modifiers::SHIFT,
    Code::KeyM,
);

pub fn setup_global_hotkeys(
    app_handle: AppHandle,
    active_session: Arc<Mutex<Option<ActiveSession>>>,
    storage: Arc<Mutex<Storage>>,
) -> Result<()> {
    let active_session_clone = active_session.clone();
    let storage_clone = storage.clone();

    // Define the start/stop shortcut
    let start_stop_shortcut = Shortcut::new(Some(START_STOP_SHORTCUT.1), START_STOP_SHORTCUT.2);

    // Define the marker shortcut
    let marker_shortcut = Shortcut::new(Some(MARKER_SHORTCUT.1), MARKER_SHORTCUT.2);

    // Register the shortcuts
    app_handle
        .global_shortcut()
        .register(start_stop_shortcut.clone())?;
    app_handle
        .global_shortcut()
        .register(marker_shortcut.clone())?;

    // Set up the global shortcut handler

    let app_handle_for_startstop = app_handle.clone();
    let _ = app_handle.global_shortcut().on_shortcut(start_stop_shortcut, move |_app, _shortcut, _window_id| {
        handle_start_stop(
            &app_handle_for_startstop,
            &active_session_clone,
            &storage_clone,
        );
    });
    
    // Clone again for the second shortcut
    let app_handle_for_marker = app_handle.clone();
    let active_session_for_marker = active_session.clone();
    let _ = app_handle.global_shortcut().on_shortcut(marker_shortcut, move |_app, _shortcut, _window_id | {
        handle_marker(
            &app_handle_for_marker,
            &active_session_for_marker,
        );
    });

    Ok(())
}

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
