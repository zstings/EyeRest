import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

let restCountdown: HTMLElement | null;
let skipBtn: HTMLButtonElement | null;

async function init() {
  restCountdown = document.getElementById("rest-countdown");
  skipBtn = document.getElementById("skip-btn") as HTMLButtonElement;

  // 监听倒计时更新
  await listen<number>("timer-tick", (event) => {
    updateCountdown(event.payload);
  });

  // 监听休息结束
  await listen("timer-rest-complete", () => {
    // 窗口会被后端自动隐藏
  });

  // 绑定跳过按钮
  skipBtn?.addEventListener("click", skipRest);
}

async function skipRest() {
  try {
    await invoke("skip_rest");
  } catch (error) {
    console.error("Failed to skip rest:", error);
  }
}

function updateCountdown(seconds: number) {
  if (restCountdown) {
    restCountdown.textContent = seconds.toString();
  }
}

window.addEventListener("DOMContentLoaded", init);
