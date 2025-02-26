use crate::AppState;
use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    AppHandle, Manager,
};

pub fn create_tray_menu(app: &AppHandle) -> tauri::menu::Menu<tauri::Wry> {
    let show_item =
        MenuItem::with_id(app, "show", "Open Dibikaandaagozi", true, None::<&str>).unwrap();
    let start_item = MenuItem::with_id(app, "start", "Start Session", true, None::<&str>).unwrap();
    let stop_item = MenuItem::with_id(app, "stop", "Stop Session", true, None::<&str>).unwrap();
    let marker_item = MenuItem::with_id(app, "marker", "Add Marker", true, None::<&str>).unwrap();
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();

    let menu = Menu::with_items(
        app,
        &[
            &show_item,
            &start_item,
            &stop_item,
            &marker_item,
            &quit_item,
        ],
    )
    .unwrap();

    menu
}

pub fn handle_tray_event(app: &AppHandle, event: MenuEvent, app_state: &AppState) {
    match event.id.as_ref() {
        "quit" => {
            std::process::exit(0);
        }
        "show" => {
            if let Some(window) = app.get_webview_window("main") {
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        }
        "start" => {
            crate::hotkey::handle_start_stop(app, &app_state.active_session, &app_state.storage);
        }
        "stop" => {
            crate::hotkey::handle_start_stop(app, &app_state.active_session, &app_state.storage);
        }
        "marker" => {
            crate::hotkey::handle_marker(app, &app_state.active_session);
        }
        _ => {}
    }
}
