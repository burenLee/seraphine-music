import { describe, expect, it } from 'vitest';

import {
  DesktopLyricEmit,
  DesktopLyricSize,
  MiniPlayerEmit,
  MiniPlayerSize,
  WindowEvent,
  WindowTarget,
} from '@/constants';

// 这些枚举值同时是前端 UI、Tauri invoke 参数与窗口事件契约，断言值域防止意外漂移

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

  it('子窗口默认尺寸', () => {
    expect(MiniPlayerSize).toStrictEqual({ width: 298, height: 66 });
    expect(DesktopLyricSize).toStrictEqual({ width: 608, height: 112 });
  });
});
