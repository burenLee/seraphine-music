/** 歌词页显示模式 */
export const enum LyricPageMode {
  Cover,
  Record,
  Photo,
}

/** 歌词类型 */
export const enum LyricFormat {
  Krc = 'krc',
  Lrc = 'lrc',
}

/** 歌词字体大小范围 */
export const enum LyricFontSize {
  Step = 2,
  Min = 16,
  Default = 26,
  Max = 36,
}

/** 歌词偏移步进(s) */
export const LyricOffsetStep = 0.1;

/** 歌词对齐模式 */
export const enum LyricTextAlign {
  Left = 'text-left',
  Center = 'text-center',
  Right = 'text-right',
}

export const AlignTitles = {
  [LyricTextAlign.Left]: '左对齐',
  [LyricTextAlign.Center]: '居中对齐',
  [LyricTextAlign.Right]: '右对齐',
};

/** 歌词翻译模式 */
export const enum LyricTransMode {
  /** 音译/罗马音 */
  Roman,
  /** 翻译 */
  Trans,
  Off,
}

/** 歌词底色 */
export const enum LyricBaseColor {
  Red = '#ef4444',
  Orange = '#f97316',
  Yellow = '#eab308',
  Green = '#22c55e',
  Cyan = '#06b6d4',
  Blue = '#3b82f6',
  Purple = '#a855f7',
}

/** 歌词着重色 */
export const enum LyricAccentColor {
  Red = '#fca5a5',
  Orange = '#fdba74',
  Yellow = '#fde047',
  Green = '#86efac',
  Cyan = '#67e8f9',
  Blue = '#93c5fd',
  Purple = '#d8b4fe',
}

/** 预设歌词配色方案 */
export const PresetsColors = [
  [LyricBaseColor.Red, LyricAccentColor.Red],
  [LyricBaseColor.Orange, LyricAccentColor.Orange],
  [LyricBaseColor.Yellow, LyricAccentColor.Yellow],
  [LyricBaseColor.Green, LyricAccentColor.Green],
  [LyricBaseColor.Cyan, LyricAccentColor.Cyan],
  [LyricBaseColor.Blue, LyricAccentColor.Blue],
  [LyricBaseColor.Purple, LyricAccentColor.Purple],
] as const;
