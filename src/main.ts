import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// DOM 元素
let timerDisplay: HTMLElement | null;
let statusText: HTMLElement | null;
let progressBar: HTMLElement | null;
let todayCount: HTMLElement | null;
let themeToggleBtn: HTMLButtonElement | null;

// 定时器配置（秒）
const WORK_DURATION = 5; // 工作时长：5秒

// 当前状态
let remainingSeconds = WORK_DURATION;
let timerInterval: number | null = null;

// LocalStorage 键
const STATS_KEY = "eyerest_stats";

// 统计数据接口
interface Stats {
  date: string;
  count: number;
}

// 初始化
async function init() {
  // 获取 DOM 元素
  timerDisplay = document.getElementById("timer-display");
  statusText = document.getElementById("status-text");
  progressBar = document.getElementById("progress-bar");
  todayCount = document.getElementById("today-count");
  themeToggleBtn = document.getElementById("theme-toggle") as HTMLButtonElement;

  // 加载统计数据
  loadStats();

  // 初始化显示
  updateTimerDisplay(remainingSeconds);
  updateProgressBar(remainingSeconds);
  updateStatusText();

  // 监听提示窗口关闭事件
  await listen("reminder-closed", () => {
    // 提示窗口关闭后，重新开始倒计时
    startTimer();
  });

  // 绑定事件
  themeToggleBtn?.addEventListener("click", toggleTheme);

  // 加载保存的主题
  loadTheme();

  // 自动开始倒计时
  startTimer();
}

// 加载统计数据（从 localStorage）
function loadStats() {
  try {
    const stats = getStats();

    if (todayCount) {
      todayCount.textContent = stats.count.toString();
    }
  } catch (error) {
    console.error("Failed to load stats:", error);
  }
}

// 获取今天的日期字符串 (YYYY-MM-DD)
function getTodayDate(): string {
  const now = new Date();
  return now.toISOString().split("T")[0];
}

// 获取统计数据
function getStats(): Stats {
  const today = getTodayDate();
  const statsJson = localStorage.getItem(STATS_KEY);

  if (statsJson) {
    const stats: Stats = JSON.parse(statsJson);
    // 如果是新的一天，重置计数
    if (stats.date !== today) {
      return { date: today, count: 0 };
    }
    return stats;
  }

  return { date: today, count: 0 };
}

// 保存统计数据
function saveStats(stats: Stats) {
  localStorage.setItem(STATS_KEY, JSON.stringify(stats));
}

// 增加完成次数（由 reminder 窗口调用）
export function incrementCount() {
  const stats = getStats();
  stats.count++;
  saveStats(stats);
  loadStats();
}

// 开始计时
function startTimer() {
  remainingSeconds = WORK_DURATION;
  updateTimerDisplay(remainingSeconds);
  updateProgressBar(remainingSeconds);
  updateStatusText();
  startTicking();
}

// 开始倒计时
function startTicking() {
  if (timerInterval !== null) {
    clearInterval(timerInterval);
  }

  timerInterval = window.setInterval(() => {
    if (remainingSeconds > 0) {
      remainingSeconds--;
      updateTimerDisplay(remainingSeconds);
      updateProgressBar(remainingSeconds);

      // 倒计时结束
      if (remainingSeconds === 0) {
        onWorkComplete();
      }
    }
  }, 1000);
}

// 停止倒计时
function stopTicking() {
  if (timerInterval !== null) {
    clearInterval(timerInterval);
    timerInterval = null;
  }
}

// 工作完成
async function onWorkComplete() {
  stopTicking();

  // 重置倒计时（准备下一轮）
  remainingSeconds = WORK_DURATION;
  updateTimerDisplay(remainingSeconds);
  updateProgressBar(remainingSeconds);

  // 打开提示窗口
  try {
    await invoke("show_rest_window");
  } catch (error) {
    console.error("Failed to show rest window:", error);
    // 如果打开窗口失败，继续下一轮倒计时
    startTimer();
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
  if (!progressBar) return;

  const percentage = (seconds / WORK_DURATION) * 100;
  progressBar.style.width = `${percentage}%`;
}

// 更新状态文本
function updateStatusText() {
  if (!statusText) return;
  statusText.textContent = "正在工作";
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
}

// 加载主题
function loadTheme() {
  const savedTheme = localStorage.getItem("theme") || "light";
  document.documentElement.setAttribute("data-theme", savedTheme);

  const themeIcon = themeToggleBtn?.querySelector(".theme-icon");
  if (themeIcon) {
    themeIcon.textContent = savedTheme === "light" ? "🌙" : "☀️";
  }
}

// DOM 加载完成后初始化
window.addEventListener("DOMContentLoaded", init);
