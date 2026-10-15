/** 播放来源 */
export const enum PlayingOrigin {
  Local,
  Online,
}

/** 播放模式 */
export const enum PlayingMode {
  OrderPlay,
  SinglePlay,
  OrderLoop,
  SingleLoop,
  RandomPlay,
}

/** 播放音质 */
export const enum PlayingQuality {
  Bitrate128 = '128',
  Bitrate320 = '320',
  BitrateFlac = 'flac',
  BitrateHigh = 'high',

  ViperAtmos = 'viper_atmos',
  ViperClear = 'viper_clear',
  ViperTape = 'viper_tape',

  SuperBsd = 'super',

  MagicPiano = 'piano',
  MagicAcappella = 'acappella',
  MagicSubwoofer = 'subwoofer',
  MagicAncient = 'ancient',
  MagicSurnay = 'surnay',
  MagicDj = 'dj',
}

/** 图片尺寸 */
export const enum PicSize {
  Sm = '64',
  Md = '120',
  Lg = '240',
}

/** 歌词提前间隔 */
export const ForwardDuration = 150;
