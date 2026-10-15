# 架构详解

## 前端架构（Vue 3 / TypeScript）

前端代码位于 `src/`，采用三窗口独立入口 + 共享基础层架构（详见 [ADR-0001](./adr/0001-three-window-isolation.md)）：

```
index.html                # 主窗口 HTML（项目根目录）
desktop-lyric.html        # 桌面歌词 HTML
mini-player.html          # 迷你播放器 HTML

src/
├── main.ts                 # 主窗口入口
├── App.vue                 # 根组件
├── router/                 # 路由（Hash 模式）
├── constants/              # 常量与枚举定义（api/app/auth/list/lyric/play/settings/window）
├── stores/                 # Pinia 状态管理（主窗口专用）
│   ├── music.ts            # 播放状态机（Play/Pause/Stop/加载/重试）
│   ├── list.ts             # 播放列表
│   ├── lyric.ts            # 歌词
│   ├── user.ts             # 用户/登录/VIP 状态
│   ├── setting.ts          # 设置
│   ├── context-menu.ts     # 右键菜单
│   ├── state.ts            # 全局状态（refreshId/模式/窗口状态/字体/子窗口坐标）
│   └── updater.ts          # 更新逻辑
├── directives/             # 自定义指令（copy）
├── components/             # 共享组件（ContextMenu/MusicList/MusicTable/SystemActions 等）
├── composables/            # 子窗口 Bridge 与通用组合式函数
│   ├── useMiniPlayerBridge.ts
│   ├── useDesktopLyricBridge.ts
│   ├── useObserver.ts
│   └── usePlayingCover.ts
├── layout/                 # 布局组件
│   ├── Aside/              # 侧边栏
│   ├── Header/             # 头部（登录/搜索/菜单）
│   ├── LyricPage/          # 歌词页
│   ├── Main/               # 主体内容
│   └── Playbar/            # 播放栏
├── views/                  # 页面视图
├── windows/                # 子窗口业务代码与入口
│   ├── desktop-lyric/
│   │   ├── desktop-lyric.ts
│   │   ├── DesktopLyric.vue
│   │   └── stores/
│   └── mini-player/
│       ├── mini-player.ts
│       └── MiniPlayer.vue
├── utils/                  # 工具函数（icons/music/tools）
├── types/                  # 类型定义
├── styles/                 # 全局样式
└── assets/                 # 静态资源（图片/字体）
```

### 路由

使用 `createWebHashHistory`，主窗口路由嵌套在 `Layout` 组件下，子窗口路由独立。

### 状态管理

- 主窗口使用 Pinia + `pinia-plugin-persistedstate` 持久化
- 子窗口使用组件局部 `ref()` 管理状态，通过 Tauri events 与主窗口同步
- 子窗口**禁止**直接 import 主窗口 store（`src/stores/*`）

## 后端架构（Rust / Tauri）

后端代码位于 `src-tauri/src/`，分为五个模块：

```
src-tauri/src/
├── main.rs         # 应用入口
├── lib.rs          # Tauri 库入口：Tauri command 注册
├── plugins.rs      # 插件注册（托盘图标/快捷键/自启/更新等）
├── api/            # 酷狗音乐 API 封装
│   ├── types.rs    # API 公共类型
│   ├── login.rs    # 登录（二维码/微信/验证码/手机号/Token）
│   ├── search.rs   # 搜索（单曲/复杂搜索）
│   ├── song.rs     # 歌曲 URL 获取
│   ├── playlist.rs # 歌单（标签/用户/详情/增删/曲目）
│   ├── album.rs    # 专辑歌曲
│   ├── artist.rs   # 歌手列表/音频
│   ├── audio.rs    # 音频信息
│   ├── lyric.rs    # 歌词搜索/获取
│   ├── rank.rs     # 排行榜
│   ├── top.rs      # 排行榜/歌单/专辑
│   ├── personal.rs # 私人 FM
│   ├── privilege.rs# 权限
│   ├── user.rs     # 用户详情
│   ├── youth.rs    # 概念版 VIP
│   ├── register.rs # 设备注册
│   └── music.rs    # 每日推荐
├── app/            # 应用配置层
│   ├── config.rs   # 应用配置
│   ├── dir.rs      # 目录管理
│   └── mode.rs     # 应用模式
├── http/           # HTTP 层
│   ├── client.rs   # 客户端封装
│   ├── config.rs   # HTTP 配置（Cookie/Header 管理）
│   ├── cookie.rs   # Cookie 管理
│   └── request.rs  # 统一请求封装
├── music/          # 音频处理层
│   ├── player.rs   # 播放器核心（rodio 封装，播放/暂停/停止/跳转/音量）
│   ├── audio.rs    # 音频设备管理
│   ├── file.rs     # 音频文件解析（lofty 元数据）
│   ├── lyric.rs    # 歌词解析（KRC/LRC 解码）
│   ├── media.rs    # 系统媒体控制（Windows SMTC / macOS NowPlaying / Linux MPRIS）
│   ├── scan.rs     # 本地音乐扫描
│   └── stream.rs   # 流式播放
└── utils/          # 工具层
    ├── crypto.rs   # 加解密（AES-CBC/RSA/MD5/SHA1）
    ├── helper.rs   # 通用辅助函数
    ├── logger.rs   # 日志初始化
    └── tools.rs    # 工具函数
```

### 关键设计

- **Tauri 插件**：`single-instance`（单实例）、`global-shortcut`（全局快捷键）、`autostart`（开机自启）、`clipboard-manager`、`dialog`、`store`（持久化）、`http`（网络请求）、`log`（日志）、`opener`（外部打开）、`process`（进程控制）、`updater`（在线更新）
- **播放器**：基于 `rodio` 封装，播放控制使用 `tokio` 异步调度，本地扫描使用 `rayon` 并行处理
- **系统媒体控制**：Windows（SMTC）/ macOS（mediaplayer）/ Linux（MPRIS over dbus）

### 三窗口 Capabilities

| 文件                 | 窗口          | 权限范围                     |
| -------------------- | ------------- | ---------------------------- |
| `main.json`          | main          | 核心 API + 文件系统 + 剪贴板 |
| `desktop-lyric.json` | desktop-lyric | 歌词显示 + 窗口控制          |
| `mini-player.json`   | mini-player   | 播放控制 + 窗口控制          |

## 依赖版本

### 前端依赖（package.json）

#### 运行时依赖（dependencies）

| 包名                                 | 版本    |
| ------------------------------------ | ------- |
| @tauri-apps/api                      | ^2.11.1 |
| @tauri-apps/plugin-autostart         | ^2.5.1  |
| @tauri-apps/plugin-clipboard-manager | ^2.3.3  |
| @tauri-apps/plugin-dialog            | ^2.7.3  |
| @tauri-apps/plugin-global-shortcut   | ^2.3.2  |
| @tauri-apps/plugin-log               | ^2.9.2  |
| @tauri-apps/plugin-opener            | ~2.5.5  |
| @tauri-apps/plugin-process           | ^2.3.1  |
| @tauri-apps/plugin-updater           | ^2.11.0 |
| @vueuse/components                   | ^14.4.0 |
| @vueuse/core                         | ^14.4.0 |
| clsx                                 | ^2.1.1  |
| pinia                                | ^3.0.4  |
| pinia-plugin-persistedstate          | ^4.7.1  |
| qrcode.vue                           | ^3.11.0 |
| tailwind-merge                       | ^3.7.0  |
| vue                                  | ^3.5.43 |
| vue-router                           | ^5.3.1  |

### 后端依赖（Cargo.toml）

#### 运行时依赖（dependencies）

| Crate                          | 版本 | features                                           |
| ------------------------------ | ---- | -------------------------------------------------- |
| tauri                          | 2    | tray-icon, protocol-asset（dev 环境另含 devtools） |
| tauri-plugin-autostart         | 2    | -                                                  |
| tauri-plugin-clipboard-manager | 2    | -                                                  |
| tauri-plugin-dialog            | 2    | -                                                  |
| tauri-plugin-global-shortcut   | 2    | -                                                  |
| tauri-plugin-http              | 2    | cookies, gzip, json, stream                        |
| tauri-plugin-log               | 2    | -                                                  |
| tauri-plugin-opener            | 2    | -                                                  |
| tauri-plugin-process           | 2    | -                                                  |
| tauri-plugin-single-instance   | 2    | -                                                  |
| tauri-plugin-store             | 2    | -                                                  |
| tauri-plugin-updater           | 2    | -                                                  |
| aes                            | 0.8  | -                                                  |
| anyhow                         | 1.0  | -                                                  |
| base64                         | 0.22 | -                                                  |
| bytes                          | 1.11 | -                                                  |
| cbc                            | 0.1  | block-padding, alloc                               |
| chrono                         | 0.4  | -                                                  |
| flate2                         | 1.1  | -                                                  |
| hex                            | 0.4  | -                                                  |
| lofty                          | 0.24 | -                                                  |
| log                            | 0.4  | -                                                  |
| md-5                           | 0.11 | -                                                  |
| rand                           | 0.10 | -                                                  |
| rayon                          | 1.11 | -                                                  |
| rodio                          | 0.22 | -                                                  |
| rsa                            | 0.9  | hazmat                                             |
| serde                          | 1.0  | -                                                  |
| serde_json                     | 1.0  | -                                                  |
| sha1                           | 0.11 | -                                                  |
| thiserror                      | 1.0  | -                                                  |
| tokio                          | 1.51 | -                                                  |
| tokio-stream                   | 0.1  | -                                                  |
| uuid                           | 1.23 | v4                                                 |
| walkdir                        | 2    | -                                                  |

#### 平台特定依赖

| 平台    | Crate                  | 版本      | features                                                                   |
| ------- | ---------------------- | --------- | -------------------------------------------------------------------------- |
| macOS   | mediaplayer            | 0.5       | -                                                                          |
| Windows | windows                | 0.58      | Foundation, Media, Media_Control, Media_Playback, Storage, Storage_Streams |
| Linux   | dbus / dbus-crossroads | 0.9 / 0.5 | -                                                                          |

#### 开发依赖（dev-dependencies）

| Crate    | 版本 |
| -------- | ---- |
| tempfile | 3    |
