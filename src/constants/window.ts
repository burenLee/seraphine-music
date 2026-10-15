/** 关闭按钮状态 */
export const enum CloseMode {
  Hide,
  Exit,
}

/** 窗口名称 */
export const enum WindowTarget {
  Main = 'main',
  MiniPlayer = 'mini-player',
  DesktopLyric = 'desktop-lyric',
}

/** 窗口事件 */
export const enum WindowEvent {
  MiniPlayer = 'mini-player:handler',
  DesktopLyric = 'desktop-lyric:handler',
}

/** 迷你播放器窗口通信类型 */
export const enum MiniPlayerEmit {
  /** 初始化数据 */
  Init,
  /** 发送坐标 */
  Pos,
  /** 获取音频 */
  Audio,
  /** 获取歌词 */
  Lyric,
  /** 获取播放列表 */
  Playlist,
  /** 播放 */
  Play,
  /** 暂停 */
  Pause,
  /** 上一首 */
  Prev,
  /** 下一首 */
  Next,
  /** 播放指定音频 */
  Set,
  /** 关闭窗口 */
  Close,
}

/** 桌面歌词窗口通信类型 */
export const enum DesktopLyricEmit {
  /** 初始化数据 */
  Init,
  /** 发送坐标 */
  Pos,
  /** 获取音频信息 */
  Audio,
  /** 获取歌词 */
  Lyric,
  /** 获取进度 */
  Progress,
  /** 获取可用字体 */
  Fonts,
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
  Close,
}

/** mini播放器默认尺寸(+border:1px) */
export const MiniPlayerSize = { width: 298, height: 66 } as const;

/** 桌面歌词默认尺寸 */
export const DesktopLyricSize = { width: 608, height: 112 } as const;
