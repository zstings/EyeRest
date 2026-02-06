use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

/// 显示提醒窗口（全屏、置顶）
pub fn show_reminder_window(app: &AppHandle) -> Result<(), String> {
    let window = if let Some(win) = app.get_webview_window("reminder") {
        win
    } else {
        // 如果窗口不存在（被关闭了），重新创建
        WebviewWindowBuilder::new(app, "reminder", WebviewUrl::App("reminder.html".into()))
            .title("休息时间")
            .inner_size(800.0, 600.0)
            .center()
            .resizable(false)
            .visible(false)
            .decorations(true)
            .skip_taskbar(false)
            .build()
            .map_err(|e| format!("Failed to create reminder window: {}", e))?
    };

    window.show().map_err(|e| format!("Failed to show reminder window: {}", e))?;
    window.set_focus().map_err(|e| format!("Failed to focus reminder window: {}", e))?;
    window.set_fullscreen(true).map_err(|e| format!("Failed to set fullscreen: {}", e))?;
    window.set_always_on_top(true).map_err(|e| format!("Failed to set always on top: {}", e))?;
    Ok(())
}

/// 关闭提醒窗口（彻底关闭，不是隐藏）
pub fn close_reminder_window(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("reminder") {
        window.close().map_err(|e| format!("Failed to close reminder window: {}", e))?;
    }

    // 发送事件通知主窗口，提示窗口已关闭，可以重新开始倒计时
    let _ = app.emit("reminder-closed", ());

    Ok(())
}
