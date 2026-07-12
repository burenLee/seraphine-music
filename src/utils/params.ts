/** 列表类型 */
export const enum ListType {
  Local = 'local',
  Show = 'show',
  Play = 'play'
}

/** 屏幕像素断点 */
export const enum BreakPoint {
  MD = 1280,
  LG = 1536
}

/** 列显示数量 */
export const enum ColCount {
  SM = 3,
  MD = 4,
  LG = 5
}

/** 播放来源 */
export const enum PlayingOrigin {
  /** 本地 */
  Local,
  /** 在线 */
  Online
}

/** 播放模式 */
export const enum PlayingMode {
  OrderPlay,
  SinglePlay,
  OrderLoop,
  SingleLoop,
  RandomPlay
}

/** 播放音质 */
export const enum PlayingQuality {
  MagicPiano = 'piano',
  MagicAcappella = 'acappella',
  MagicSubwoofer = 'subwoofer',
  MagicAncient = 'ancient',
  MagicSurnay = 'surnay',
  MagicDj = 'dj',

  Bitrate128 = '128',
  Bitrate320 = '320',
  BitrateFlac = 'flac',
  BitrateHigh = 'high',

  ViperAtmos = 'viper_atmos',
  ViperClear = 'viper_clear',
  ViperTape = 'viper_tape',

  SuperBsd = 'super'
}

/** 排序方式 */
export const enum SortType {
  Default,
  Title,
  Artist,
  Album,
  Duration
}

/** 排序顺序 */
export const enum SortOrder {
  ASC,
  DESC
}

/** 歌词页显示模式 */
export const enum LyricPageMode {
  Cover,
  Record,
  Photo
}

/** 歌词类型 */
export const enum LyricFormat {
  Krc = 'Krc',
  Lrc = 'Lrc'
}

/** 歌词字体大小范围 */
export const enum LyricFontSize {
  Step = 2,
  Default = 24,
  Min = 16,
  Max = 32
}

/** 歌词文本颜色 */
export const enum LyricTextColor {
  Red = '#ef4444',
  Orange = '#f97316',
  Yellow = '#eab308',
  Green = '#22c55e',
  Cyan = '#06b6d4',
  Blue = '#3b82f6',
  Purple = '#a855f7'
}

/** 歌词对齐模式 */
export const enum LyricTextAlign {
  Left = 'text-left',
  Center = 'text-center',
  Right = 'text-right'
}

/** 歌词翻译模式 */
export const enum LyricTransMode {
  Off,
  Roman,
  Trans
}

export const enum LyricOffset {
  Step = 0.2,
  Default = 0
}

/** 添加音频方式 */
export const enum AddMusicType {
  Add,
  Scan
}

/** 播放列表类型 */
export const enum PlaylistType {
  User,
  Collection
}

/** 添加播放列表方式 */
export const enum AddPlaylistType {
  Add,
  Import
}

/** 帧间隔时间 */
export const enum Interval {
  Short = 16,
  Medium = 33,
  Long = 100,
  Sec = 1000,
  /** 上/下一首 */
  PoN = 2000
}

/** 二维码类型 */
export const enum QrcodeType {
  KG,
  QQ,
  WX
}

/** 二维码状态 */
export const enum QrcodeStatus {
  Ready,
  Scan,
  Timeout,
  Confirm,
  Fail
}

/** 分页尺寸 */
export const enum PageSize {
  Min = 10,
  Default = 30,
  Max = 300
}

/** 登录页模式 */
export const enum LoginMode {
  Code,
  Form
}

/** 用户操作 */
export const enum UserAction {
  Info,
  Logout,
  Vip
}

/** 菜单操作 */
export const enum MenuAction {
  Restore,
  Update,
  Setting,
  Logout,
  Exit
}

/** 桌面歌词窗口通信类型 */
export const enum LyricEmitType {
  /** 获取歌词 */
  GetLyric,
  /** 发送歌词 */
  SendLyric,
  /** 打开主界面 */
  Main,
  /** 上一曲 */
  Prev,
  /** 下一曲 */
  Next,
  /** 播放 */
  Play,
  /** 暂停 */
  Pause,
  /** 关闭窗口 */
  Close
}

/** 扫描状态 */
export const enum ScanStatus {
  Ready,
  Loading,
  Error,
  Success
}

/** 关闭按钮状态 */
export const enum CloseStatus {
  Hide,
  Exit
}

/** 自启模式 */
export const enum AutoStartMode {
  Foreground,
  Background
}

/** 查询类型 */
export const enum SearchType {
  Song = 'song',
  Album = 'album',
  Author = 'author',
  Mv = 'mv',
  Lyric = 'lyric',
  Special = 'special',
  Collect = 'collect'
}

/** 按键映射 */
export const enum ShortcutKey {
  PlayOrPause = 'playOrPause',
  AddVolumn = 'addVolumn',
  SubVolumn = 'subVolumn',
  Mute = 'mute',
  Prev = 'prev',
  Next = 'next',
  Forward = 'forward',
  Backward = 'backward'
}

export const AreaTypes = [
  { type: 1, musician: 0, title: '华语' },
  { type: 2, musician: 0, title: '欧美' },
  { type: 5, musician: 0, title: '日本' },
  { type: 6, musician: 0, title: '韩国' },
  { type: 7, musician: 0, title: '粤语' },
  { type: 8, musician: 0, title: '闽南语' },
  { type: 0, musician: 3, title: '音乐人' },
  { type: 4, musician: 0, title: '其他' }
] as const

export const SexTypes = [
  { key: 0, value: '全部' },
  { key: 1, value: '男' },
  { key: 2, value: '女' },
  { key: 3, value: '组合' }
] as const

export const AlbumTypes: Array<{ title: string; type: keyof TopAlbum }> = [
  { title: '华语', type: 'chn' },
  { title: '欧美', type: 'eur' },
  { title: '日本', type: 'jpn' },
  { title: '韩国', type: 'kor' }
] as const

// TODO: 暂时使用预设, 后续改成从后端获取全部字体
// 常见系统字体列表 [中文显示名称, 英文名称]
export const DefaultSystemFonts = [
  ['默认字体', 'system-ui'],

  // Windows 中文
  ['微软雅黑', 'Microsoft YaHei'],
  ['微软雅黑 UI', 'Microsoft YaHei UI'],
  ['微软正黑体', 'Microsoft JhengHei'],
  ['宋体', 'SimSun'],
  ['新宋体', 'NSimSun'],
  ['黑体', 'SimHei'],
  ['楷体', 'KaiTi'],
  ['华文楷体', 'STKaiti'],
  ['仿宋', 'FangSong'],
  ['华文仿宋', 'STFangsong'],
  ['幼圆', 'YouYuan'],
  ['隶书', 'LiSu'],
  ['细明体', 'MingLiU'],
  ['新细明体', 'PMingLiU'],

  // macOS 中文
  ['苹方-简', 'PingFang SC'],
  ['苹方-繁', 'PingFang TC'],
  ['苹方-港', 'PingFang HK'],
  ['冬青黑体简体中文', 'Hiragino Sans GB'],
  ['冬青角黑 ProN', 'Hiragino Kaku Gothic ProN'],
  ['华文黑体', 'STHeiti'],
  ['华文中宋', 'STSong'],
  ['华文细黑', 'STXihei'],
  ['华文行楷', 'STXingkai'],
  ['华文新魏', 'STXinwei'],

  // Linux 中文
  ['Noto 无衬线体简体中文', 'Noto Sans CJK SC'],
  ['Noto 无衬线体繁体中文', 'Noto Sans CJK TC'],
  ['Noto 衬线体简体中文', 'Noto Serif CJK SC'],
  ['Noto 衬线体繁体中文', 'Noto Serif CJK TC'],
  ['文泉驿微米黑', 'WenQuanYi Micro Hei'],
  ['文泉驿正黑', 'WenQuanYi Zen Hei'],
  ['Droid Sans Fallback', 'Droid Sans Fallback'],
  ['文鼎PL简中明体', 'AR PL UMing CN'],
  ['文鼎PL简中楷体', 'AR PL UKai CN'],

  // Windows 英文
  ['Arial', 'Arial'],
  ['Arial Black', 'Arial Black'],
  ['Times New Roman', 'Times New Roman'],
  ['Georgia', 'Georgia'],
  ['Verdana', 'Verdana'],
  ['Courier New', 'Courier New'],
  ['Trebuchet MS', 'Trebuchet MS'],
  ['Impact', 'Impact'],
  ['Comic Sans MS', 'Comic Sans MS'],
  ['Palatino Linotype', 'Palatino Linotype'],
  ['Tahoma', 'Tahoma'],
  ['Segoe UI', 'Segoe UI'],
  ['Calibri', 'Calibri'],
  ['Cambria', 'Cambria'],
  ['Consolas', 'Consolas'],

  // macOS 英文
  ['Helvetica', 'Helvetica'],
  ['Helvetica Neue', 'Helvetica Neue'],
  ['Avenir', 'Avenir'],
  ['Futura', 'Futura'],
  ['Optima', 'Optima'],
  ['Baskerville', 'Baskerville'],
  ['Didot', 'Didot'],
  ['American Typewriter', 'American Typewriter'],

  // 通用等宽字体
  ['Monaco', 'Monaco'],
  ['Menlo', 'Menlo'],
  ['Lucida Console', 'Lucida Console'],
  ['Source Code Pro', 'Source Code Pro'],
  ['Fira Code', 'Fira Code']
] as const
