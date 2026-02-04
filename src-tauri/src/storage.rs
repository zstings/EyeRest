use crate::state::Settings;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::{Store, StoreExt};
use std::sync::Arc;

const STORE_FILENAME: &str = "settings.json";

/// 获取 Store 实例
fn get_store<R: tauri::Runtime>(app: &AppHandle<R>) -> Result<Arc<Store<R>>, String> {
    let store_path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join(STORE_FILENAME);

    let store = app.store(store_path)
        .map_err(|e| format!("Failed to create store: {}", e))?;

    Ok(store)
}

/// 保存设置
pub fn save_settings<R: tauri::Runtime>(app: &AppHandle<R>, settings: &Settings) -> Result<(), String> {
    let store = get_store(app)?;

    store.set("work_minutes", serde_json::json!(settings.work_minutes));
    store.set("rest_seconds", serde_json::json!(settings.rest_seconds));
    store.set("auto_start", serde_json::json!(settings.auto_start));
    store.set("theme", serde_json::json!(settings.theme));

    store.save().map_err(|e| format!("Failed to save settings: {}", e))?;

    Ok(())
}

/// 加载设置
pub fn load_settings<R: tauri::Runtime>(app: &AppHandle<R>) -> Settings {
    let store = match get_store(app) {
        Ok(s) => s,
        Err(_) => return Settings::default(),
    };

    let work_minutes = store.get("work_minutes")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(20);

    let rest_seconds = store.get("rest_seconds")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(20);

    let auto_start = store.get("auto_start")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let theme = store.get("theme")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "light".to_string());

    Settings {
        work_minutes,
        rest_seconds,
        auto_start,
        theme,
    }
}

/// 保存统计数据
pub fn save_stats<R: tauri::Runtime>(app: &AppHandle<R>, today_completed: u32, last_reset_date: &str) -> Result<(), String> {
    let store = get_store(app)?;

    store.set("today_completed", serde_json::json!(today_completed));
    store.set("last_reset_date", serde_json::json!(last_reset_date));

    store.save().map_err(|e| format!("Failed to save stats: {}", e))?;

    Ok(())
}

/// 加载统计数据
pub fn load_stats<R: tauri::Runtime>(app: &AppHandle<R>) -> (u32, String) {
    let store = match get_store(app) {
        Ok(s) => s,
        Err(_) => return (0, chrono::Local::now().format("%Y-%m-%d").to_string()),
    };

    let today_completed = store.get("today_completed")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(0);

    let last_reset_date = store.get("last_reset_date")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string());

    (today_completed, last_reset_date)
}
