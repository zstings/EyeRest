mod commands;
mod state;
mod storage;
mod timer;
mod tray;
mod window;

use parking_lot::Mutex;
use state::{AppState, AppStateWrapper};
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .setup(|app| {
            // 加载统计数据
            let (today_completed, last_reset_date) = storage::load_stats(app.handle());

            // 初始化应用状态，使用固定时长
            let app_state = AppState {
                timer_state: state::TimerState::Stopped,
                work_duration: 20 * 60,  // 固定 20 分钟
                rest_duration: 20,       // 固定 20 秒
                remaining_seconds: 20 * 60,
                today_completed,
                last_reset_date,
            };

            let state_wrapper: AppStateWrapper = Arc::new(Mutex::new(app_state));

            // 管理状态
            app.manage(state_wrapper.clone());

            // 启动定时器循环
            timer::start_timer_loop(app.handle().clone(), state_wrapper.clone());

            // 设置系统托盘
            if let Err(e) = tray::setup_tray(app.handle()) {
                eprintln!("Failed to setup tray: {}", e);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_timer,
            commands::pause_timer,
            commands::resume_timer,
            commands::skip_rest,
            commands::get_state,
            commands::get_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
