use tauri::{AppHandle, Manager};

/// 显示提醒窗口（全屏、置顶）
pub fn show_reminder_window(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("reminder") {
        window.show().map_err(|e| format!("Failed to show reminder window: {}", e))?;
        window.set_focus().map_err(|e| format!("Failed to focus reminder window: {}", e))?;
        window.set_fullscreen(true).map_err(|e| format!("Failed to set fullscreen: {}", e))?;
        window.set_always_on_top(true).map_err(|e| format!("Failed to set always on top: {}", e))?;
        Ok(())
    } else {
        Err("Reminder window not found".to_string())
    }
}

/// 隐藏提醒窗口
pub fn hide_reminder_window(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("reminder") {
        window.set_fullscreen(false).map_err(|e| format!("Failed to unset fullscreen: {}", e))?;
        window.set_always_on_top(false).map_err(|e| format!("Failed to unset always on top: {}", e))?;
        window.hide().map_err(|e| format!("Failed to hide reminder window: {}", e))?;
        Ok(())
    } else {
        Err("Reminder window not found".to_string())
    }
}
