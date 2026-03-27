# Quick Copy

輕量級桌面快速複製工具，常駐系統列，一鍵複製常用文字片段。

## 功能特色

- **快速複製** — 點擊即複製到剪貼簿，視窗自動隱藏
- **類別管理** — 自訂類別分組，標籤列快速篩選
- **項目編輯** — 新增、編輯、刪除，支援拖曳排序
- **深色模式** — 一鍵切換，偏好自動記憶
- **全域快捷鍵** — `Cmd+Shift+V`（macOS）/ `Ctrl+Shift+V`（Windows）呼出/隱藏視窗
- **系統列常駐** — 左鍵點擊系統列圖示切換視窗顯示

## 技術架構

- **前端** — Vue 3 + TypeScript + Vite
- **樣式** — UnoCSS（Tailwind 預設）
- **桌面框架** — Tauri 1.5（Rust）
- **資料儲存** — localStorage

## 開發指南

### 環境需求

- [Node.js](https://nodejs.org/) 18+
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install)

### 安裝與啟動

```bash
# 安裝依賴
pnpm install

# 啟動開發模式（Vite + Tauri）
pnpm tauri dev

# 打包桌面應用程式
pnpm tauri:build
```

### 建議 IDE 設定

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 專案結構

```
Quick_Copy/
├── src/                  # Vue 前端原始碼
│   ├── App.vue           # 主要元件（所有 UI 邏輯）
│   ├── main.ts           # 進入點
│   └── assets/           # 圖示資源
├── src-tauri/            # Rust 後端原始碼
│   ├── src/main.rs       # 系統列、全域快捷鍵、視窗管理
│   └── tauri.conf.json   # Tauri 應用設定
├── uno.config.ts         # UnoCSS 設定
└── vite.config.ts        # Vite 建置設定
```
