use serde::{Deserialize, Serialize};
use parking_lot::Mutex;
use std::sync::Arc;

/// 定时器状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimerState {
    Stopped,
    Running,
    Paused,
    Resting,
}

/// 用户设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub work_minutes: u32,      // 工作时长（分钟），默认 20
    pub rest_seconds: u32,      // 休息时长（秒），默认 20
    pub auto_start: bool,       // 休息后自动开始
    pub theme: String,          // "light" 或 "dark"
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            work_minutes: 20,
            rest_seconds: 20,
            auto_start: false,
            theme: "light".to_string(),
        }
    }
}

/// 应用状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub timer_state: TimerState,
    pub work_duration: u64,        // 工作时长（秒），默认 1200（20 分钟）
    pub rest_duration: u64,        // 休息时长（秒），默认 20
    pub remaining_seconds: u64,    // 剩余秒数
    pub settings: Settings,
    pub today_completed: u32,      // 今日完成次数
    pub last_reset_date: String,   // 上次重置日期（用于每日统计重置）
}

impl Default for AppState {
    fn default() -> Self {
        let settings = Settings::default();
        Self {
            timer_state: TimerState::Stopped,
            work_duration: (settings.work_minutes as u64) * 60,
            rest_duration: settings.rest_seconds as u64,
            remaining_seconds: (settings.work_minutes as u64) * 60,
            settings,
            today_completed: 0,
            last_reset_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
        }
    }
}

/// 应用状态包装器，用于在 Tauri 中共享状态
pub type AppStateWrapper = Arc<Mutex<AppState>>;

/// 状态信息（用于返回给前端）
#[derive(Debug, Clone, Serialize)]
pub struct StateInfo {
    pub timer_state: TimerState,
    pub remaining_seconds: u64,
    pub work_duration: u64,
    pub rest_duration: u64,
}

/// 统计信息（用于返回给前端）
#[derive(Debug, Clone, Serialize)]
pub struct Stats {
    pub today_completed: u32,
}
