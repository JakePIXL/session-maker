mod hotkey;
mod session;
mod storage;
mod tray;

use session::ActiveSession;
use storage::Storage;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use std::sync::{Arc, Mutex};
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

pub struct AppState {
    pub storage: Arc<Mutex<Storage>>,
    pub active_session: Arc<Mutex<Option<ActiveSession>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            storage: Arc::new(Mutex::new(Storage::new())),
            active_session: Arc::new(Mutex::new(None)),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    log::info!("Starting Dibikaandaagozi");

    // Initialize application state
    let app_state = AppState {
        storage: Arc::new(Mutex::new(Storage::new())),
        active_session: Arc::new(Mutex::new(None)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_global_shortcut::Builder::default().build())
        .manage(app_state)
        .setup(|app| {
            let app_handle = app.handle();

            // Hide the window on startup (runs in background)
            #[allow(unused_variables)]
            if let Some(window) = app.get_webview_window("main") {
                #[cfg(not(target_os = "macos"))]
                window.hide().unwrap();
            }

            // Set up system tray
            let tray_menu = tray::create_tray_menu(&app_handle);

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .on_menu_event(move |app, event| {
                    let app_state = app.state::<AppState>();
                    tray::handle_tray_event(app, event, &app_state);
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.center().unwrap();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            let start_stop_shortcut = Shortcut::new(Some(hotkey::DEFAULT_START_STOP_SHORTCUT.1), hotkey::DEFAULT_START_STOP_SHORTCUT.2);

            let marker_shortcut = Shortcut::new(Some(hotkey::DEFAULT_MARKER_SHORTCUT.1), hotkey::DEFAULT_MARKER_SHORTCUT.2);

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new().with_handler(move |app_handle, shortcut, event| {
                    let state = app_handle.state::<AppState>();
                    let move_session = state.active_session.clone();
                    let move_storage = state.storage.clone();

                    if shortcut == &start_stop_shortcut {
                        if event.state() == ShortcutState::Pressed {
                            hotkey::handle_start_stop(&app_handle, &move_session, &move_storage);
                        }
                    } else if shortcut == &marker_shortcut {
                        if event.state() == ShortcutState::Pressed {
                            // println!("Marker Button hit!");
                            hotkey::handle_marker(&app_handle, &move_session);
                        }
                    }
                })
                .build(),
            )?;

            
            app.global_shortcut().register(start_stop_shortcut)?;

            
            app.global_shortcut().register(marker_shortcut)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            session::start_session,
            session::stop_session,
            session::add_marker,
            session::get_active_session,
            session::get_sessions,
            session::get_session_by_id,
            storage::export_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
