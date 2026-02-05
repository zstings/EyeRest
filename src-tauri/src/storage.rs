use tauri::{AppHandle, Manager};
use tauri_plugin_store::{Store, StoreExt};
use std::sync::Arc;

const STORE_FILENAME: &str = "stats.json";

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
