use crate::state::{AppStateWrapper, TimerState};
use crate::window::{show_reminder_window, hide_reminder_window};
use crate::storage::save_stats;
use tauri::{AppHandle, Emitter};
use tokio::time::{interval, Duration};

/// 启动定时器循环（后台任务）
pub fn start_timer_loop(app_handle: AppHandle, state: AppStateWrapper) {
    tauri::async_runtime::spawn(async move {
        let mut interval_timer = interval(Duration::from_secs(1));

        loop {
            interval_timer.tick().await;

            // 检查日期是否变化，如果跨天则重置统计
            check_and_reset_daily_stats(&app_handle, &state);

            let mut state_guard = state.lock();
            let current_state = state_guard.timer_state.clone();

            match current_state {
                TimerState::Running => {
                    if state_guard.remaining_seconds > 0 {
                        state_guard.remaining_seconds -= 1;

                        // 发送 tick 事件
                        let _ = app_handle.emit("timer-tick", state_guard.remaining_seconds);

                        // 如果工作时间结束
                        if state_guard.remaining_seconds == 0 {
                            state_guard.timer_state = TimerState::Resting;
                            state_guard.remaining_seconds = state_guard.rest_duration;

                            // 发送工作完成事件
                            let _ = app_handle.emit("timer-work-complete", ());

                            // 显示提醒窗口
                            if let Err(e) = show_reminder_window(&app_handle) {
                                eprintln!("Failed to show reminder window: {}", e);
                            }
                        }
                    }
                }
                TimerState::Resting => {
                    if state_guard.remaining_seconds > 0 {
                        state_guard.remaining_seconds -= 1;

                        // 发送 tick 事件
                        let _ = app_handle.emit("timer-tick", state_guard.remaining_seconds);

                        // 如果休息时间结束
                        if state_guard.remaining_seconds == 0 {
                            // 增加完成次数
                            state_guard.today_completed += 1;

                            // 保存统计数据
                            let _ = save_stats(
                                &app_handle,
                                state_guard.today_completed,
                                &state_guard.last_reset_date,
                            );

                            // 发送休息完成事件
                            let _ = app_handle.emit("timer-rest-complete", ());

                            // 隐藏提醒窗口
                            if let Err(e) = hide_reminder_window(&app_handle) {
                                eprintln!("Failed to hide reminder window: {}", e);
                            }

                            // 休息结束后停止计时器
                            state_guard.timer_state = TimerState::Stopped;
                            state_guard.remaining_seconds = state_guard.work_duration;
                            let _ = app_handle.emit("timer-state-changed", "Stopped");
                        }
                    }
                }
                TimerState::Stopped | TimerState::Paused => {
                    // 不做任何事
                }
            }
        }
    });
}

/// 检查并重置每日统计
fn check_and_reset_daily_stats(app: &AppHandle, state: &AppStateWrapper) {
    let mut state_guard = state.lock();
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    if state_guard.last_reset_date != today {
        state_guard.today_completed = 0;
        state_guard.last_reset_date = today.clone();

        // 保存重置后的统计数据
        let _ = save_stats(app, 0, &today);
    }
}
