<p align="center">
  <img src="./src-tauri/icons/128x128.png" width="96" height="96" alt="Seraphine Music" />
</p>
<p align="center">
  <strong>Seraphine Music</strong>
</p>

> 一个体积小、启动快的简易音乐播放器

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Powered%20by-Tauri-50c8d8)](https://tauri.app/zh-cn/)
[![Vue](https://img.shields.io/badge/Vue-3.5-58v781)](https://cn.vuejs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.6-blue.svg)](https://www.typescriptlang.org/)

## ✨ 功能特性

- 🎵 **本地播放** - 支持 flac/mp3/wav 等所有主流格式
- 🌐 **在线播放** - 支持酷狗音源
- 📦 **包体积小** - 打包后体积仅 3mb ~ 10mb
- ⚡ **启动快速** - 只有必要配置，保证应用打开速度
- 🌐 **无额外服务** - 酷狗音源接口从 KuGouMusicApi 移植到 Rust，不依赖独立后端服务

## 📸 预览

- 主界面
  ![主界面](./screenshots/main.png)
- 播放列表
  ![播放列表](./screenshots/playlist.png)
- 歌词页
  ![歌词页](./screenshots/lyric.png)

## 🛠️ 技术栈

- **[Tauri](https://tauri.app/zh-cn/)** - 创建小型、快速、安全、跨平台的应用程序
- **[Vue](https://cn.vuejs.org/)** - 易学易用，性能出色，适用场景丰富的 Web 前端框架
- **[TypeScript](https://www.typescriptlang.org/)** - 类型安全的 JavaScript 超集
- **[Tailwind CSS](https://tailwindcss.com/)** - 原子化 CSS 框架
- **[Pinia](https://pinia.vuejs.org/zh/)** - Vue 官方推荐的状态管理库
- **[Rust](https://www.rust-lang.org/)** + **[rodio](https://github.com/RustAudio/rodio)** - 音频解码与播放

## 🚀 快速开始

### 前置要求

- [Node.js](https://nodejs.org/) 与 [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install) 稳定版工具链
- 各平台 WebView 依赖同 [Tauri](https://tauri.app/zh-cn/start/prerequisites/)

### 开发环境运行

```bash
pnpm install        # 安装依赖
pnpm tauri dev      # 启动开发环境
```

### 运行测试

```bash
pnpm test                            # 前端测试（Vitest + happy-dom）
cd src-tauri && cargo test           # Rust 后端测试
```

### 代码检查

```bash
pnpm lint:fix        # oxlint 检查并自动修复
pnpm fmt             # oxfmt 格式化
pnpm tsc-check       # vue-tsc 类型检查
```

### 构建发布

```bash
pnpm tauri:build     # 完整构建（前端 + Rust + 打包）
```

## 📁 项目结构

```
SeraphineMusic/
├── index.html            # 主窗口入口 HTML
├── desktop-lyric.html    # 桌面歌词入口 HTML
├── mini-player.html      # 迷你播放器入口 HTML
├── src/                  # 前端 Vue 代码
│   ├── assets/           # 静态资源（图片、字体等）
│   ├── components/       # 可复用组件
│   ├── composables/      # 组合式函数（窗口 Bridge 等）
│   ├── constants/        # 常量与枚举定义
│   ├── directives/       # 自定义指令
│   ├── layout/           # 布局组件
│   │   ├── Aside/        # 侧边栏
│   │   ├── Header/       # 头部栏
│   │   ├── LyricPage/    # 播放页
│   │   ├── Main/         # 主体内容
│   │   └── Playbar/      # 播放栏
│   ├── router/           # 路由配置
│   ├── stores/           # 状态管理
│   ├── styles/           # 全局样式
│   ├── types/            # TypeScript 类型定义
│   ├── utils/            # 工具函数
│   ├── views/            # 页面视图
│   ├── windows/          # 子窗口业务代码与入口
│   │   ├── desktop-lyric/  # 桌面歌词窗口
│   │   └── mini-player/    # 迷你播放器窗口
│   └── main.ts           # 主窗口入口文件
├── src-tauri/            # Tauri 后端
│   ├── icons/            # 应用图标
│   ├── src/              # Rust 源码
│   │   ├── api/          # 酷狗接口
│   │   ├── app/          # 应用配置
│   │   ├── http/         # http客户端
│   │   ├── music/        # 音频处理
│   │   ├── utils/        # 工具函数
│   │   ├── lib.rs        # Tauri command 注册
│   │   ├── plugins.rs    # 插件注册
│   │   └── main.rs       # 应用入口
│   └── tauri.conf.json
├── screenshots/          # 项目截图
└── package.json
```

更多架构与开发细节见 [docs/](./docs/)：

- [架构详解](./docs/architecture.md) - 三窗口架构、前后端模块与依赖版本
- [关键约定](./docs/conventions.md) - 共享层、Commit 规范、版本同步
- [验证流程](./docs/testing.md) - 测试策略与质量检查
- [发布流程](./docs/release.md) - 从打 tag 到用户收到更新的完整链路

## 📄 许可证

本项目采用 [MIT 许可证](LICENSE)。

## 🙏 致谢

感谢以下开源项目：

- [KuGouMusicApi](https://github.com/MakcRe/KuGouMusicApi) - 酷狗音乐 Node.js API service

## 免责声明

> 1. 本项目仅供学习使用，请尊重版权，请勿利用此项目从事商业行为及非法用途!
> 2. 使用本项目的过程中可能会产生版权数据。对于这些版权数据，本项目不拥有它们的所有权。为了避免侵权，使用者务必在 24 小时内清除使用本项目的过程中所产
>    生的版权数据。
> 3. 由于使用本项目产生的包括由于本协议或由于使用或无法使用本项目而引起的任何性质的任何直接、间接、特殊、偶然或结果性损害（包括但不限于因商誉损失、停
>    工、计算机故障或故障引起的损害赔偿，或任何及所有其他商业损害或损失）由使用者负责。
> 4. **禁止在违反当地法律法规的情况下使用本项目。** 对于使用者在明知或不知当地法律法规不允许的情况下使用本项目所造成的任何违法违规行为由使用者承担，本
>    项目不承担由此造成的任何直接、间接、特殊、偶然或结果性责任。
> 5. 音乐平台不易，请尊重版权，支持正版。
> 6. 本项目仅用于对技术可行性的探索及研究，不接受任何商业（包括但不限于广告等）合作及捐赠。
> 7. 如果官方音乐平台觉得本项目不妥，可联系本项目更改或移除。
