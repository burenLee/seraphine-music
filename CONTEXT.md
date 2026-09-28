# Seraphine Music

基于 Tauri 2 + Vue 3 + TypeScript 的跨平台音乐播放器，后端酷狗接口从 KuGouMusicApi 移植到 Rust。

- **本地播放**：扫描本地音频文件（flac/mp3/wav 等）并读取元数据与封面，经 rodio 解码播放。
- **在线播放**：直连酷狗接口获取音频流，边下边播，支持多音质与 VIP 权限校验。
- **无额外服务**：酷狗接口在 Rust 端直接发起请求（含参数签名与 key 加密），不依赖独立后端服务。

## Language

### 窗口与架构

**Main Window（主窗口）**:
承载完整功能的应用主窗口，包含路由、Pinia store、布局与所有视图。
_Avoid_: 主界面、主屏

**Mini Player（迷你播放器）**:
紧凑型播放控制子窗口，仅展示当前音频、歌词与播放列表，通过 Tauri events 与主窗口同步状态。
_Avoid_: 迷你模式、小窗

**Desktop Lyric（桌面歌词）**:
独立悬浮于桌面的歌词显示子窗口，独立 store 管理字体/颜色/对齐等样式配置。
_Avoid_: 桌面歌词窗口、悬浮歌词

**Shared Layer（共享基础层）**:
三窗口共用的基础代码集合，位于 `src/components/`、`src/utils/`、`src/types/`、`src/styles/`。
_Avoid_: 公共层、common 层

**Bridge（桥接）**:
子窗口与主窗口之间的通信层（`src/composables/use*Bridge.ts`），封装 Tauri events 的双向同步逻辑。
_Avoid_: 通信器、connector

### 音频与播放

**Music（音频）**:
应用中播放的基本单元，可能是本地文件或在线流。
_Avoid_: 歌曲、track、song（在代码类型中使用 ListMusic/PlayingMusic 而非通用 song）

**ListMusic（列表音频）**:
播放列表中条目的数据结构，包含 id、hash、path、cover、title、artist、album、duration、sort 等字段。
_Avoid_: PlaylistItem、Track

**PlayingMusic（播放音频）**:
当前正在播放的音频载荷，是 ListMusic 的精简子集（不含 sort、tags 等）。
_Avoid_: CurrentSong、NowPlaying

**Hash**:
酷狗生态中音频的全局唯一字符串标识，用于在线播放、歌词获取、权限校验。
_Avoid_: audio_id、fileid（这些是不同维度的标识）

**PlayingOrigin（播放来源）**:
音频来源枚举，取值 `Local`（本地文件）或 `Online`（在线流）。
_Avoid_: Source、Origin

**PlayingMode（播放模式）**:
播放列表遍历策略枚举：OrderPlay（顺序播放，末尾停止）、SinglePlay（单曲播放后停止）、OrderLoop（列表循环）、SingleLoop（单曲循环）、RandomPlay（随机播放）。
_Avoid_: PlayMode、RepeatMode

**PlayingQuality（播放音质）**:
在线音频码流规格枚举，包含标准（128/320/Flac/High）、蝰蛇（ViperAtmos/ViperClear/ViperTape）、SuperBsd、魔声音效（MagicPiano 等）。
_Avoid_: Bitrate、Format

**Privilege（权限）**:
酷狗平台对音频施加的播放/下载限制标记，通过 `api_privilege_lite` 查询，决定某音频是否可播。
_Avoid_: Permission、Right

### 列表与歌单

**Playlist（歌单）**:
用户在酷狗平台创建或收藏的音频集合，通过 `list_create_gid`/`list_create_listid` 标识。
_Avoid_: 歌单列表、SongList

**ListType（列表类型）**:
应用内列表的分类枚举：`Local`（本地音乐）、`Show`（展示用，如搜索结果）、`Play`（当前播放队列）。
_Avoid_: ListCategory

**PlaylistType（播放列表类型）**:
酷狗歌单归属枚举：`User`（用户自建）或 `Collection`（收藏）。
_Avoid_: PlaylistCategory

### 歌词

**Lyric（歌词）**:
伴随音频播放的文本内容，可能是行级（LRC）或逐字级（KRC）。
_Avoid_: 歌词文本、LyricsText

**LyricFormat（歌词格式）**:
歌词存储格式枚举：`Krc`（逐字加密格式）或 `Lrc`（行级明文格式）。
_Avoid_: LyricType

**LyricLine（歌词行）**:
KRC/LRC 解析后的单行歌词结构，包含 offset、duration、words、translations。
_Avoid_: LyricEntry、LyricRow

**LyricWord（歌词单词）**:
KRC 逐字歌词中的最小单元，包含 offset、duration、text。
_Avoid_: LyricToken、LyricChar

**LyricTransMode（翻译模式）**:
歌词翻译展示模式枚举：`Roman`（音译/罗马音）、`Trans`（翻译）、`Off`（关闭）。
_Avoid_: TranslationMode

### 用户与登录

**User（用户）**:
登录到酷狗账号的使用者，由 `UserInfo`（userid/nickname/pic）描述。
_Avoid_: Account、Member

**VIP**:
用户的酷狗会员状态，通过 `api_youth_union_vip` 查询，影响 Privilege 与音质可用性。
_Avoid_: Membership、Subscription

### 通信

**WindowEvent（窗口事件）**:
Tauri events 通道命名，采用 `<window-name>:handler` 格式（如 `mini-player:handler`、`desktop-lyric:handler`），子窗口与主窗口通过该通道双向同步状态。
_Avoid_: WindowMessage、WindowChannel

**WindowTarget（窗口目标）**:
窗口标识枚举：`Main`、`MiniPlayer`（值 `'mini-player'`）、`DesktopLyric`（值 `'desktop-lyric'`）。
_Avoid_: WindowName（与 HTML 入口/Capabilities/Specs 前缀保持一致，使用窗口名而非别名）

**MiniPlayerEmit（迷你播放器通信类型）**:
迷你播放器窗口事件载荷枚举：Init/Pos/Audio/Lyric/Playlist/Play/Pause/Prev/Next/Set/Close。
_Avoid_: MiniPlayerAction

**DesktopLyricEmit（桌面歌词通信类型）**:
桌面歌词窗口事件载荷枚举：Init/Pos/Audio/Lyric/Progress/Fonts/Main/Prev/Next/Play/Pause/Close。
_Avoid_: DesktopLyricAction

### 设置与系统

**前端错误（Frontend Error）**:
产生于 WebView 侧的错误，包括 UI 渲染异常、Bridge 事件处理失败、JS 运行时异常；不含 invoke 调用从后端返回的错误。
_Avoid_: 客户端错误、UI 错误

**后端错误（Backend Error）**:
产生于 Rust 侧的错误，包括 command 内部失败、酷狗接口请求失败、音频解码/文件读写失败；前端 invoke 收到的错误一律视为后端错误。
_Avoid_: 服务端错误、Rust 错误

**AutoStartMode（开机自启模式）**:
应用开机自启的启动方式枚举：`Foreground`（前台启动）或 `Background`（后台启动）。
_Avoid_: StartupMode、LaunchMode

**CloseStatus（关闭行为）**:
点击窗口关闭按钮时的行为枚举：`Hide`（隐藏到托盘）或 `Exit`（退出应用）。
_Avoid_: CloseAction、CloseBehavior

**ShortcutKey（全局快捷键）**:
媒体控制全局快捷键枚举：播放/暂停、音量增减、静音、上一首/下一首、快进/快退。
_Avoid_: Hotkey、MediaKey

**ThemeMode（主题模式）**:
界面主题枚举：`Light`（浅色）、`Dark`（深色）、`Auto`（跟随系统）。
_Avoid_: Theme、Appearance

### 网络与接口

**EncryptType（加密类型）**:
酷狗接口的参数加密方式枚举：`Web`、`Android`、`Register`，决定请求参数的签名算法与 key 加密策略。
_Avoid_: EncryptMode、CipherType

**RequestOptions（请求选项）**:
后端 HTTP 请求的统一配置结构，包含 base_url/method/header/params/data，以及签名与加密开关（should_signature/should_encrypt）。
_Avoid_: RequestConfig、HttpOptions

### 流式播放与设备

**StreamFile（流式文件）**:
在线音频边下边播的文件读取器；当播放读取位置超过已下载大小时阻塞等待下载，超时则返回错误。
_Avoid_: StreamBuffer、StreamingReader

**DeviceInfo（播放设备）**:
音频输出设备描述（id/name），由 rodio 枚举，用于在多设备间切换音频输出。
_Avoid_: AudioDevice、OutputDevice
