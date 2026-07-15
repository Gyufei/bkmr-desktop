# bkmrx

[bkmr](https://github.com/sysid/bkmr) 书签管理命令行工具的桌面 GUI，基于 Tauri v2 构建。

## 功能

- **书签浏览与搜索** — 通过 bkmr CLI 读取书签数据库，支持全文模糊搜索（Fuse.js）和标签筛选
- **标签面板** — 按标签聚合浏览，点击快速过滤
- **Markdown 笔记** — 集成 [Milkdown](https://milkdown.dev) / Crepe 编辑器，支持笔记的创建、编辑、删除，自动按目录扫描组织
- **内置 HTTP 分享服务** — 启动后通过 Axum HTTP 服务器分享书签，支持 REST API 新增书签
- **设置持久化** — 备份目录、笔记目录等配置保存至 `~/.bkmr/settings.json`
- **深色/浅色主题** — 跟随系统外观

## 快速开始

### 前置依赖

- [bkmr](https://github.com/sysid/bkmr) CLI — 后端依赖
- Rust toolchain（推荐通过 [rustup](https://rustup.rs) 安装）
- Node.js >= 18

### 安装与运行

```bash
# 安装前端依赖
npm install

# 开发模式（热更新）
npm run tauri dev

# 构建生产版本
npm run tauri build
```

## 技术栈

| 层 | 技术 |
|---|---|
| 前端框架 | React 18 + TypeScript |
| 构建工具 | Vite 5 |
| 样式 | TailwindCSS 3 |
| 桌面框架 | Tauri 2 (Rust) |
| HTTP 服务 | Axum (内嵌) |
| 搜索 | Fuse.js |
| 编辑器 | Milkdown / Crepe |
| 后端工具 | bkmr CLI |

## 项目结构

```
bkmrx/
├── src/                    # React 前端
│   ├── components/         # UI 组件
│   │   ├── SearchBar.tsx
│   │   ├── TagPanel.tsx
│   │   ├── ResultList.tsx
│   │   ├── NotesPanel.tsx
│   │   ├── NoteEditor.tsx
│   │   ├── FolderTree.tsx
│   │   ├── Pagination.tsx
│   │   ├── SettingsModal.tsx
│   │   └── SettingsPage.tsx
│   ├── hooks/              # React Hooks
│   │   ├── useBkmr.ts
│   │   ├── useNotes.ts
│   │   └── useSettings.ts
│   ├── utils/              # 工具函数
│   ├── types.ts            # 类型定义
│   └── App.tsx             # 主应用组件
├── src-tauri/              # Rust 后端
│   └── src/
│       ├── main.rs         # 入口
│       ├── lib.rs          # 模块声明
│       ├── bkmr.rs         # bkmr CLI 交互
│       ├── commands.rs     # Tauri IPC 命令
│       ├── http_server.rs  # 内嵌 HTTP 服务
│       ├── notes.rs        # 笔记文件管理
│       └── settings.rs     # 配置持久化
├── package.json
├── vite.config.ts
└── tailwind.config.js
```

## 许可证

MIT
