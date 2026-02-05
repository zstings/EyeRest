export type TimerState = "Stopped" | "Running" | "Paused" | "Resting";

export interface StateInfo {
  timer_state: TimerState;
  remaining_seconds: number;
  work_duration: number;
  rest_duration: number;
}

export interface Stats {
  today_completed: number;
}
