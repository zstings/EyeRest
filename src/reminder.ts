import { invoke } from "@tauri-apps/api/core";

let restCountdown: HTMLElement | null;
let skipBtn: HTMLButtonElement | null;
let countdownInterval: number | null = null;
let remainingSeconds = 20;

// LocalStorage 键
const STATS_KEY = "eyerest_stats";

// 统计数据接口
interface Stats {
  date: string;
  count: number;
}

function init() {
  restCountdown = document.getElementById("rest-countdown");
  skipBtn = document.getElementById("skip-btn") as HTMLButtonElement;

  // 绑定跳过按钮
  skipBtn?.addEventListener("click", skipRest);

  // 开始倒计时
  startCountdown();
}

function startCountdown() {
  remainingSeconds = 20;
  updateCountdown(remainingSeconds);

  countdownInterval = window.setInterval(() => {
    remainingSeconds--;
    updateCountdown(remainingSeconds);

    if (remainingSeconds <= 0) {
      stopCountdown();
      completeRest();
    }
  }, 1000);
}

function stopCountdown() {
  if (countdownInterval !== null) {
    clearInterval(countdownInterval);
    countdownInterval = null;
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

// 增加完成次数
function incrementCount() {
  const stats = getStats();
  stats.count++;
  saveStats(stats);
}

// 休息完成（倒计时自然结束）
async function completeRest() {
  // 增加完成次数
  incrementCount();

  // 关闭窗口
  try {
    await invoke("close_rest_window");
  } catch (error) {
    console.error("Failed to close rest window:", error);
  }
}

// 跳过休息（点击跳过按钮）
async function skipRest() {
  stopCountdown();

  // 增加完成次数
  incrementCount();

  // 关闭窗口
  try {
    await invoke("close_rest_window");
  } catch (error) {
    console.error("Failed to close rest window:", error);
  }
}

function updateCountdown(seconds: number) {
  if (restCountdown) {
    restCountdown.textContent = seconds.toString();
  }
}

window.addEventListener("DOMContentLoaded", init);
