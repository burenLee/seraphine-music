import { beforeEach, describe, expect, it } from 'vitest';

import {
  LyricAccentColor,
  LyricBaseColor,
  LyricFontSize,
  LyricOffset,
  LyricTransMode,
} from '@/utils/params';
import { useDesktopLyricStore } from '@/windows/desktop-lyric/stores/desktop-lyric';

import { createTestPinia } from '../../../helpers/factories';

describe('useDesktopLyricStore 桌面歌词局部状态', () => {
  beforeEach(() => createTestPinia());

  it('初始默认值', () => {
    const store = useDesktopLyricStore();
    expect(store.fontFamily).toBe('system-ui');
    expect(store.fontSize).toBe(LyricFontSize.Default);
    expect(store.textBaseColor).toBe(LyricBaseColor.Blue);
    expect(store.textAccentColor).toBe(LyricAccentColor.Blue);
    expect(store.transMode).toBe(LyricTransMode.Off);
    expect(store.offsetMap).toEqual({});
  });

  it('setFontFamily 更新字体', () => {
    const store = useDesktopLyricStore();
    store.setFontFamily('SimHei');
    expect(store.fontFamily).toBe('SimHei');
  });

  it('setFontSize 加减与重置', () => {
    const store = useDesktopLyricStore();
    store.setFontSize('add');
    expect(store.fontSize).toBe(LyricFontSize.Default + LyricFontSize.Step);
    store.setFontSize('sub');
    expect(store.fontSize).toBe(LyricFontSize.Default);
    store.setFontSize('sub');
    expect(store.fontSize).toBe(LyricFontSize.Default - LyricFontSize.Step);
    store.setFontSize('restart');
    expect(store.fontSize).toBe(LyricFontSize.Default);
  });

  it('setTextColors 同时更新基色与高亮色', () => {
    const store = useDesktopLyricStore();
    store.setTextColors([LyricBaseColor.Green, LyricAccentColor.Green]);
    expect(store.textBaseColor).toBe(LyricBaseColor.Green);
    expect(store.textAccentColor).toBe(LyricAccentColor.Green);
  });

  it('setTransMode 切换翻译模式', () => {
    const store = useDesktopLyricStore();
    store.setTransMode(LyricTransMode.Trans);
    expect(store.transMode).toBe(LyricTransMode.Trans);
  });

  it('setOffsetMap 按 id 增减偏移并重置，空 id 直接返回', () => {
    const store = useDesktopLyricStore();
    store.setOffsetMap('add', 'lyric-1');
    expect(store.offsetMap['lyric-1']).toBe(LyricOffset.Default + LyricOffset.Step);
    store.setOffsetMap('sub', 'lyric-1');
    expect(store.offsetMap['lyric-1']).toBe(LyricOffset.Default);
    store.setOffsetMap('sub', 'lyric-1');
    expect(store.offsetMap['lyric-1']).toBe(LyricOffset.Default - LyricOffset.Step);
    store.setOffsetMap('restart', 'lyric-1');
    expect(store.offsetMap['lyric-1']).toBe(LyricOffset.Default);

    store.setOffsetMap('add', '');
    expect(store.offsetMap).toEqual({ 'lyric-1': LyricOffset.Default });
  });
});
