import { describe, expect, it } from 'vitest';

import {
  DesktopLyricEmit,
  FORWARD_DURATION,
  Interval,
  LyricBaseColor,
  LyricAccentColor,
  LyricFontSize,
  LyricFormat,
  LyricTextAlign,
  LyricTransMode,
  MiniPlayerEmit,
  Mode,
  PicSize,
  PlayingMode,
  PlayingOrigin,
  PlayingQuality,
  PresetsColors,
  ListType,
  ShortcutKey,
  SizeUnits,
  WindowEvent,
  WindowTarget,
  YouthVip,
  desktopLyricSize,
  miniPlayerSize,
  DefaultSystemFonts,
} from '@/utils/params';

// 这些枚举值同时是前端 UI、Tauri invoke 参数与窗口事件契约，断言值域防止意外漂移

describe('播放相关枚举', () => {
  it('PlayingOrigin：Local=0, Online=1', () => {
    expect(PlayingOrigin.Local).toBe(0);
    expect(PlayingOrigin.Online).toBe(1);
  });

  it('PlayingMode：5 种播放模式按序定义', () => {
    expect(PlayingMode.OrderPlay).toBe(0);
    expect(PlayingMode.SinglePlay).toBe(1);
    expect(PlayingMode.OrderLoop).toBe(2);
    expect(PlayingMode.SingleLoop).toBe(3);
    expect(PlayingMode.RandomPlay).toBe(4);
  });

  it('PlayingQuality：音质字符串与后端约定一致', () => {
    expect(PlayingQuality.Bitrate128).toBe('128');
    expect(PlayingQuality.Bitrate320).toBe('320');
    expect(PlayingQuality.BitrateFlac).toBe('flac');
    expect(PlayingQuality.BitrateHigh).toBe('high');
  });
});

describe('歌词相关枚举', () => {
  it('LyricFormat 与后端 invoke 参数一致', () => {
    expect(LyricFormat.Krc).toBe('krc');
    expect(LyricFormat.Lrc).toBe('lrc');
  });

  it('LyricTransMode：Roman=0, Trans=1, Off=2（作为 translations 的键）', () => {
    expect(LyricTransMode.Roman).toBe(0);
    expect(LyricTransMode.Trans).toBe(1);
    expect(LyricTransMode.Off).toBe(2);
  });

  it('LyricFontSize 范围约束 Min < Default < Max', () => {
    expect(LyricFontSize.Min).toBeLessThan(LyricFontSize.Default);
    expect(LyricFontSize.Default).toBeLessThan(LyricFontSize.Max);
    expect(LyricFontSize.Step).toBeGreaterThan(0);
  });

  it('LyricTextAlign 值为 Tailwind 类名', () => {
    expect(LyricTextAlign.Left).toBe('text-left');
    expect(LyricTextAlign.Center).toBe('text-center');
    expect(LyricTextAlign.Right).toBe('text-right');
  });
});

describe('窗口通信契约', () => {
  it('WindowTarget：三窗口 label', () => {
    expect(WindowTarget.Main).toBe('main');
    expect(WindowTarget.MiniPlayer).toBe('mini-player');
    expect(WindowTarget.DesktopLyric).toBe('desktop-lyric');
  });

  it('WindowEvent：事件名', () => {
    expect(WindowEvent.MiniPlayer).toBe('mini-player:handler');
    expect(WindowEvent.DesktopLyric).toBe('desktop-lyric:handler');
  });

  it('MiniPlayerEmit：11 种通信类型按序定义', () => {
    expect(MiniPlayerEmit.Init).toBe(0);
    expect(MiniPlayerEmit.Pos).toBe(1);
    expect(MiniPlayerEmit.Audio).toBe(2);
    expect(MiniPlayerEmit.Lyric).toBe(3);
    expect(MiniPlayerEmit.Playlist).toBe(4);
    expect(MiniPlayerEmit.Play).toBe(5);
    expect(MiniPlayerEmit.Pause).toBe(6);
    expect(MiniPlayerEmit.Prev).toBe(7);
    expect(MiniPlayerEmit.Next).toBe(8);
    expect(MiniPlayerEmit.Set).toBe(9);
    expect(MiniPlayerEmit.Close).toBe(10);
  });

  it('DesktopLyricEmit：12 种通信类型按序定义', () => {
    expect(DesktopLyricEmit.Init).toBe(0);
    expect(DesktopLyricEmit.Pos).toBe(1);
    expect(DesktopLyricEmit.Audio).toBe(2);
    expect(DesktopLyricEmit.Lyric).toBe(3);
    expect(DesktopLyricEmit.Progress).toBe(4);
    expect(DesktopLyricEmit.Fonts).toBe(5);
    expect(DesktopLyricEmit.Main).toBe(6);
    expect(DesktopLyricEmit.Prev).toBe(7);
    expect(DesktopLyricEmit.Next).toBe(8);
    expect(DesktopLyricEmit.Play).toBe(9);
    expect(DesktopLyricEmit.Pause).toBe(10);
    expect(DesktopLyricEmit.Close).toBe(11);
  });
});

describe('其他常量', () => {
  it('ListType：三种列表', () => {
    expect(ListType.Local).toBe('local');
    expect(ListType.Show).toBe('show');
    expect(ListType.Play).toBe('play');
  });

  it('Interval：播放相关帧间隔', () => {
    expect(Interval.PoN).toBe(2000);
    expect(Interval.Long).toBe(100);
  });

  it('FORWARD_DURATION 歌词提前量', () => {
    expect(FORWARD_DURATION).toBe(150);
  });

  it('PicSize 与酷狗图片尺寸约定一致', () => {
    expect(PicSize.Sm).toBe('64');
    expect(PicSize.Md).toBe('120');
    expect(PicSize.Lg).toBe('400');
  });

  it('SizeUnits 从 B 到 YB 共 9 个单位', () => {
    expect(SizeUnits).toHaveLength(9);
    expect(SizeUnits[0]).toBe('B');
    expect(SizeUnits[8]).toBe('YB');
  });

  it('PresetsColors：7 组同色系 [底色, 着重色] 配对', () => {
    expect(PresetsColors).toStrictEqual([
      [LyricBaseColor.Red, LyricAccentColor.Red],
      [LyricBaseColor.Orange, LyricAccentColor.Orange],
      [LyricBaseColor.Yellow, LyricAccentColor.Yellow],
      [LyricBaseColor.Green, LyricAccentColor.Green],
      [LyricBaseColor.Cyan, LyricAccentColor.Cyan],
      [LyricBaseColor.Blue, LyricAccentColor.Blue],
      [LyricBaseColor.Purple, LyricAccentColor.Purple],
    ]);
  });

  it('子窗口默认尺寸', () => {
    expect(miniPlayerSize).toStrictEqual({ width: 298, height: 66 });
    expect(desktopLyricSize).toStrictEqual({ width: 608, height: 112 });
  });

  it('Mode / YouthVip / ShortcutKey 关键值', () => {
    expect(Mode.KgLite).toBe('KgLite');
    expect(YouthVip.Not).toBe(0);
    expect(ShortcutKey.PlayOrPause).toBe('playOrPause');
  });

  it('DefaultSystemFonts：首项为默认字体，每项为 [中文名, 英文名] 二元组', () => {
    expect(DefaultSystemFonts.length).toBeGreaterThan(0);
    expect(DefaultSystemFonts[0]).toStrictEqual(['默认字体', 'system-ui']);
    for (const item of DefaultSystemFonts) {
      expect(item).toHaveLength(2);
    }
  });
});
