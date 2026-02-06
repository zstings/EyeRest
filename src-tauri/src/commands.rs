use crate::window::{close_reminder_window, show_reminder_window};
use tauri::AppHandle;

/// 显示休息窗口
#[tauri::command]
pub fn show_rest_window(app: AppHandle) -> Result<(), String> {
    show_reminder_window(&app).map_err(|e| e.to_string())
}

/// 关闭休息窗口
#[tauri::command]
pub fn close_rest_window(app: AppHandle) -> Result<(), String> {
    close_reminder_window(&app).map_err(|e| e.to_string())
}
