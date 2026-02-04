export type TimerState = "Stopped" | "Running" | "Paused" | "Resting";

export interface StateInfo {
  timer_state: TimerState;
  remaining_seconds: number;
  work_duration: number;
  rest_duration: number;
}

export interface Settings {
  work_minutes: number;
  rest_seconds: number;
  auto_start: boolean;
  theme: string;
}

export interface Stats {
  today_completed: number;
}
