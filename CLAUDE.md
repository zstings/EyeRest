# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

EyeRest 是一个基于 Tauri 2 的桌面应用程序，使用 Vite + TypeScript (Vanilla) 作为前端，Rust 作为后端。

## 开发命令

### 前端开发
- `pnpm dev` - 启动 Vite 开发服务器（端口 1420）
- `pnpm build` - 构建前端（TypeScript 编译 + Vite 打包）
- `pnpm preview` - 预览生产构建

### Tauri 开发
- `pnpm tauri dev` - 启动 Tauri 开发模式（同时运行前端和后端）
- `pnpm tauri build` - 构建完整的 Tauri 应用程序
- `pnpm tauri info` - 显示 Tauri 环境信息
- `pnpm tauri icon` - 生成应用图标

### Rust 后端
- `cd src-tauri && cargo build` - 构建 Rust 后端
- `cd src-tauri && cargo test` - 运行 Rust 测试
- `cd src-tauri && cargo clippy` - 运行 Rust linter
- `cd src-tauri && cargo fmt` - 格式化 Rust 代码

## 项目架构

### 目录结构
```
EyeRest/
├── src/                    # 前端源代码
│   ├── main.ts            # 前端入口文件
│   ├── styles.css         # 样式文件
│   └── assets/            # 静态资源
├── src-tauri/             # Tauri/Rust 后端
│   ├── src/
│   │   ├── main.rs       # Rust 入口点
│   │   └── lib.rs        # Tauri 命令和应用逻辑
│   ├── Cargo.toml        # Rust 依赖配置
│   ├── tauri.conf.json   # Tauri 配置
│   └── icons/            # 应用图标
├── index.html             # HTML 入口
├── vite.config.ts         # Vite 配置
└── package.json           # Node.js 依赖
```

### 前后端通信

前端通过 `@tauri-apps/api/core` 的 `invoke` 函数调用 Rust 后端命令：

```typescript
// 前端 (src/main.ts)
import { invoke } from "@tauri-apps/api/core";
const result = await invoke("command_name", { arg: value });
```

```rust
// 后端 (src-tauri/src/lib.rs)
#[tauri::command]
fn command_name(arg: Type) -> ReturnType {
    // 实现逻辑
}

// 在 run() 函数中注册命令
.invoke_handler(tauri::generate_handler![command_name])
```

### 构建流程

1. **开发模式** (`pnpm tauri dev`):
   - 执行 `beforeDevCommand`: `pnpm dev` 启动 Vite 开发服务器
   - Tauri 启动 Rust 后端并创建 WebView 窗口
   - 前端通过 `http://localhost:1420` 加载

2. **生产构建** (`pnpm tauri build`):
   - 执行 `beforeBuildCommand`: `pnpm build` 构建前端到 `dist/`
   - Cargo 编译 Rust 后端
   - Tauri 将前端和后端打包成平台特定的安装包

## 技术栈

- **前端**: Vite 6 + TypeScript 5.6 + Vanilla JS
- **后端**: Rust + Tauri 2
- **包管理器**: pnpm
- **插件**: tauri-plugin-opener (用于打开 URL/文件)

## 配置文件

- `tauri.conf.json` - Tauri 应用配置（窗口大小、标题、构建命令等）
- `Cargo.toml` - Rust 依赖和项目元数据
- `vite.config.ts` - Vite 构建配置
- `tsconfig.json` - TypeScript 编译选项
