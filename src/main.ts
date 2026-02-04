import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { StateInfo, Settings, Stats, TimerState } from "./types";

// DOM 元素
let timerDisplay: HTMLElement | null;
let statusText: HTMLElement | null;
let progressBar: HTMLElement | null;
let todayCount: HTMLElement | null;
let startBtn: HTMLButtonElement | null;
let pauseResumeBtn: HTMLButtonElement | null;
let workMinutesInput: HTMLInputElement | null;
let restSecondsInput: HTMLInputElement | null;
let autoStartCheckbox: HTMLInputElement | null;
let saveSettingsBtn: HTMLButtonElement | null;
let themeToggleBtn: HTMLButtonElement | null;

// 当前状态
let currentState: StateInfo | null = null;
let currentSettings: Settings | null = null;

// 初始化
async function init() {
  // 获取 DOM 元素
  timerDisplay = document.getElementById("timer-display");
  statusText = document.getElementById("status-text");
  progressBar = document.getElementById("progress-bar");
  todayCount = document.getElementById("today-count");
  startBtn = document.getElementById("start-btn") as HTMLButtonElement;
  pauseResumeBtn = document.getElementById("pause-resume-btn") as HTMLButtonElement;
  workMinutesInput = document.getElementById("work-minutes") as HTMLInputElement;
  restSecondsInput = document.getElementById("rest-seconds") as HTMLInputElement;
  autoStartCheckbox = document.getElementById("auto-start") as HTMLInputElement;
  saveSettingsBtn = document.getElementById("save-settings-btn") as HTMLButtonElement;
  themeToggleBtn = document.getElementById("theme-toggle") as HTMLButtonElement;

  // 加载初始状态
  await loadState();
  await loadSettings();
  await loadStats();

  // 监听事件
  await listen<number>("timer-tick", (event) => {
    if (currentState) {
      currentState.remaining_seconds = event.payload;
      updateTimerDisplay(event.payload);
      updateProgressBar(event.payload);
    }
  });

  await listen("timer-work-complete", () => {
    if (statusText) {
      statusText.textContent = "休息时间！";
    }
  });

  await listen("timer-rest-complete", () => {
    if (statusText) {
      statusText.textContent = "休息结束";
    }
    loadStats();
  });

  await listen<string>("timer-state-changed", (event) => {
    if (currentState) {
      currentState.timer_state = event.payload as TimerState;
      updateButtonStates(event.payload as TimerState);
      updateStatusText(event.payload as TimerState);
    }
  });

  // 绑定事件
  startBtn?.addEventListener("click", startTimer);
  pauseResumeBtn?.addEventListener("click", togglePauseResume);
  saveSettingsBtn?.addEventListener("click", saveSettings);
  themeToggleBtn?.addEventListener("click", toggleTheme);

  // 加载保存的主题
  loadTheme();
}

// 加载状态
async function loadState() {
  try {
    currentState = await invoke<StateInfo>("get_state");
    if (currentState) {
      updateTimerDisplay(currentState.remaining_seconds);
      updateProgressBar(currentState.remaining_seconds);
      updateButtonStates(currentState.timer_state);
      updateStatusText(currentState.timer_state);
    }
  } catch (error) {
    console.error("Failed to load state:", error);
  }
}

// 加载设置
async function loadSettings() {
  try {
    currentSettings = await invoke<Settings>("get_settings");
    if (currentSettings && workMinutesInput && restSecondsInput && autoStartCheckbox) {
      workMinutesInput.value = currentSettings.work_minutes.toString();
      restSecondsInput.value = currentSettings.rest_seconds.toString();
      autoStartCheckbox.checked = currentSettings.auto_start;
    }
  } catch (error) {
    console.error("Failed to load settings:", error);
  }
}

// 加载统计
async function loadStats() {
  try {
    const stats = await invoke<Stats>("get_stats");
    if (stats && todayCount) {
      todayCount.textContent = stats.today_completed.toString();
    }
  } catch (error) {
    console.error("Failed to load stats:", error);
  }
}

// 开始计时
async function startTimer() {
  try {
    await invoke("start_timer");
    await loadState();
  } catch (error) {
    console.error("Failed to start timer:", error);
  }
}

// 切换暂停/恢复
async function togglePauseResume() {
  if (!currentState) return;

  try {
    if (currentState.timer_state === "Running") {
      await invoke("pause_timer");
    } else if (currentState.timer_state === "Paused") {
      await invoke("resume_timer");
    }
    await loadState();
  } catch (error) {
    console.error("Failed to toggle pause/resume:", error);
  }
}

// 保存设置
async function saveSettings() {
  if (!workMinutesInput || !restSecondsInput || !autoStartCheckbox || !currentSettings) {
    return;
  }

  const settings: Settings = {
    work_minutes: parseInt(workMinutesInput.value),
    rest_seconds: parseInt(restSecondsInput.value),
    auto_start: autoStartCheckbox.checked,
    theme: currentSettings.theme,
  };

  try {
    await invoke("update_settings", { settings });
    currentSettings = settings;
    await loadState();
    alert("设置已保存！");
  } catch (error) {
    console.error("Failed to save settings:", error);
    alert("保存设置失败");
  }
}

// 更新倒计时显示
function updateTimerDisplay(seconds: number) {
  if (!timerDisplay) return;

  const minutes = Math.floor(seconds / 60);
  const secs = seconds % 60;
  timerDisplay.textContent = `${minutes.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
}

// 更新进度条
function updateProgressBar(seconds: number) {
  if (!progressBar || !currentState) return;

  let total: number;
  if (currentState.timer_state === "Resting") {
    total = currentState.rest_duration;
  } else {
    total = currentState.work_duration;
  }

  const percentage = (seconds / total) * 100;
  progressBar.style.width = `${percentage}%`;
}

// 更新按钮状态
function updateButtonStates(state: TimerState) {
  if (!startBtn || !pauseResumeBtn) return;

  switch (state) {
    case "Stopped":
      startBtn.disabled = false;
      pauseResumeBtn.disabled = true;
      pauseResumeBtn.textContent = "暂停";
      break;
    case "Running":
      startBtn.disabled = true;
      pauseResumeBtn.disabled = false;
      pauseResumeBtn.textContent = "暂停";
      break;
    case "Paused":
      startBtn.disabled = true;
      pauseResumeBtn.disabled = false;
      pauseResumeBtn.textContent = "继续";
      break;
    case "Resting":
      startBtn.disabled = true;
      pauseResumeBtn.disabled = true;
      pauseResumeBtn.textContent = "暂停";
      break;
  }
}

// 更新状态文本
function updateStatusText(state: TimerState) {
  if (!statusText) return;

  switch (state) {
    case "Stopped":
      statusText.textContent = "准备开始";
      break;
    case "Running":
      statusText.textContent = "正在工作";
      break;
    case "Paused":
      statusText.textContent = "已暂停";
      break;
    case "Resting":
      statusText.textContent = "休息时间";
      break;
  }
}

// 切换主题
function toggleTheme() {
  const html = document.documentElement;
  const currentTheme = html.getAttribute("data-theme") || "light";
  const newTheme = currentTheme === "light" ? "dark" : "light";

  html.setAttribute("data-theme", newTheme);
  localStorage.setItem("theme", newTheme);

  // 更新主题图标
  const themeIcon = themeToggleBtn?.querySelector(".theme-icon");
  if (themeIcon) {
    themeIcon.textContent = newTheme === "light" ? "🌙" : "☀️";
  }

  // 保存到设置
  if (currentSettings) {
    currentSettings.theme = newTheme;
    invoke("update_settings", { settings: currentSettings }).catch(console.error);
  }
}

// 加载主题
function loadTheme() {
  if (currentSettings) {
    const theme = currentSettings.theme || "light";
    document.documentElement.setAttribute("data-theme", theme);

    const themeIcon = themeToggleBtn?.querySelector(".theme-icon");
    if (themeIcon) {
      themeIcon.textContent = theme === "light" ? "🌙" : "☀️";
    }
  }
}

// DOM 加载完成后初始化
window.addEventListener("DOMContentLoaded", init);
