import { describe, expect, it } from 'vitest';

import { DefaultSystemFonts, LyricAccentColor, LyricBaseColor, PresetsColors } from '@/constants';

// 这些枚举值同时是前端 UI、Tauri invoke 参数与窗口事件契约，断言值域防止意外漂移

describe('歌词配色与字体', () => {
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

  it('DefaultSystemFonts：首项为默认字体，每项为 [中文名, 英文名] 二元组', () => {
    expect(DefaultSystemFonts.length).toBeGreaterThan(0);
    expect(DefaultSystemFonts[0]).toStrictEqual(['默认字体', 'system-ui']);
    for (const item of DefaultSystemFonts) {
      expect(item).toHaveLength(2);
    }
  });
});
