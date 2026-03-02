#[cfg(target_os = "macos")]
use crate::panel as backend;
#[cfg(not(target_os = "macos"))]
use crate::panel_windows as backend;

use tauri::AppHandle;

pub fn init(app: &AppHandle) -> tauri::Result<()> {
    backend::init(app)
}

pub fn show(app: &AppHandle) {
    backend::show_panel(app);
}

pub fn hide(app: &AppHandle) {
    backend::hide_panel(app);
}

pub fn toggle(app: &AppHandle) {
    backend::toggle_panel(app);
}
