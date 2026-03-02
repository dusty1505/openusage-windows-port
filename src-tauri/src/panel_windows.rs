use tauri::{AppHandle, Manager};

pub fn init(_app_handle: &AppHandle) -> tauri::Result<()> {
    Ok(())
}

pub fn show_panel(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub fn toggle_panel(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        match window.is_visible() {
            Ok(true) => {
                let _ = window.hide();
            }
            _ => {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
    }
}

pub fn hide_panel(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.hide();
    }
}
