use crate::state::{AppStateWrapper, Settings, StateInfo, Stats, TimerState};
use crate::storage::save_settings;
use crate::window::hide_reminder_window;
use tauri::{AppHandle, Emitter, State};

/// 开始计时
#[tauri::command]
pub fn start_timer(state: State<AppStateWrapper>, app: AppHandle) -> Result<(), String> {
    let mut state_guard = state.lock();

    if state_guard.timer_state == TimerState::Stopped {
        state_guard.timer_state = TimerState::Running;
        state_guard.remaining_seconds = state_guard.work_duration;

        let _ = app.emit("timer-state-changed", "Running");

        Ok(())
    } else {
        Err("Timer is already running or paused".to_string())
    }
}

/// 暂停计时
#[tauri::command]
pub fn pause_timer(state: State<AppStateWrapper>, app: AppHandle) -> Result<(), String> {
    let mut state_guard = state.lock();

    if state_guard.timer_state == TimerState::Running {
        state_guard.timer_state = TimerState::Paused;

        let _ = app.emit("timer-state-changed", "Paused");

        Ok(())
    } else {
        Err("Timer is not running".to_string())
    }
}

/// 恢复计时
#[tauri::command]
pub fn resume_timer(state: State<AppStateWrapper>, app: AppHandle) -> Result<(), String> {
    let mut state_guard = state.lock();

    if state_guard.timer_state == TimerState::Paused {
        state_guard.timer_state = TimerState::Running;

        let _ = app.emit("timer-state-changed", "Running");

        Ok(())
    } else {
        Err("Timer is not paused".to_string())
    }
}

/// 跳过休息
#[tauri::command]
pub fn skip_rest(state: State<AppStateWrapper>, app: AppHandle) -> Result<(), String> {
    let mut state_guard = state.lock();

    if state_guard.timer_state == TimerState::Resting {
        state_guard.timer_state = TimerState::Stopped;
        state_guard.remaining_seconds = state_guard.work_duration;

        let _ = app.emit("timer-state-changed", "Stopped");

        // 隐藏提醒窗口
        hide_reminder_window(&app)?;

        Ok(())
    } else {
        Err("Not in resting state".to_string())
    }
}

/// 获取当前状态
#[tauri::command]
pub fn get_state(state: State<AppStateWrapper>) -> StateInfo {
    let state_guard = state.lock();

    StateInfo {
        timer_state: state_guard.timer_state.clone(),
        remaining_seconds: state_guard.remaining_seconds,
        work_duration: state_guard.work_duration,
        rest_duration: state_guard.rest_duration,
    }
}

/// 更新设置
#[tauri::command]
pub fn update_settings(
    state: State<AppStateWrapper>,
    app: AppHandle,
    settings: Settings,
) -> Result<(), String> {
    let mut state_guard = state.lock();

    // 更新工作时长和休息时长
    state_guard.work_duration = (settings.work_minutes as u64) * 60;
    state_guard.rest_duration = settings.rest_seconds as u64;

    // 如果当前是停止状态，更新剩余时间
    if state_guard.timer_state == TimerState::Stopped {
        state_guard.remaining_seconds = state_guard.work_duration;
    }

    // 更新设置
    state_guard.settings = settings.clone();

    // 保存设置
    save_settings(&app, &settings)?;

    Ok(())
}

/// 获取设置
#[tauri::command]
pub fn get_settings(state: State<AppStateWrapper>) -> Settings {
    let state_guard = state.lock();
    state_guard.settings.clone()
}

/// 获取统计信息
#[tauri::command]
pub fn get_stats(state: State<AppStateWrapper>) -> Stats {
    let state_guard = state.lock();
    Stats {
        today_completed: state_guard.today_completed,
    }
}
