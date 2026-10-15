import { describe, expect, it } from 'vitest';

import {
  LyricFontSize,
  LyricFormat,
  LyricOffsetStep,
  LyricTextAlign,
  LyricTransMode,
} from '@/constants';

// 这些枚举值同时是前端 UI、Tauri invoke 参数与窗口事件契约，断言值域防止意外漂移

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

  it('LyricFontSize 范围约束 Step < Min < Default < Max', () => {
    expect(LyricFontSize.Step).toBe(2);
    expect(LyricFontSize.Step).toBeGreaterThan(0);
    expect(LyricFontSize.Step).toBeLessThan(LyricFontSize.Min);
    expect(LyricFontSize.Min).toBeLessThan(LyricFontSize.Default);
    expect(LyricFontSize.Default).toBeLessThan(LyricFontSize.Max);
  });

  it('LyricOffsetStep 偏移步进为 0.1 秒', () => {
    expect(LyricOffsetStep).toBe(0.1);
  });

  it('LyricTextAlign 值为 Tailwind 类名', () => {
    expect(LyricTextAlign.Left).toBe('text-left');
    expect(LyricTextAlign.Center).toBe('text-center');
    expect(LyricTextAlign.Right).toBe('text-right');
  });
});
