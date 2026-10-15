/** 播放器Mode */
export const enum Mode {
  KgMobile = 'KgMobile',
  KgLite = 'KgLite',
}

/** 播放器Mode标题 */
export const enum ModeTitle {
  KgMobile = '酷狗移动版',
  KgLite = '酷狗概念版',
}

/** 主题模式 */
export const enum ThemeMode {
  Light = 'light',
  Dark = 'dark',
  Auto = 'auto',
}

/** 菜单操作 */
export const enum MenuAction {
  Restore,
  Update,
  Setting,
  Logout,
  Exit,
}

/** 用户操作 */
export const enum UserAction {
  Vip,
  Info,
  Logout,
}

/** 帧间隔时间 */
export const enum Interval {
  Short = 16,
  Medium = 33,
  Long = 100,
  Sec = 1000,
  Min = 60000,
  /** 上/下一首 */
  PoN = 2000,
}

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
  ['Fira Code', 'Fira Code'],
] as const;

/** 文件大小单位 */
export const SizeUnits = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'] as const;
