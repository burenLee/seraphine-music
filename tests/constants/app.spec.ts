import { describe, expect, it } from 'vitest';

import { Mode, ModeTitle } from '@/constants';

// 这些枚举值同时是前端 UI、Tauri invoke 参数与窗口事件契约，断言值域防止意外漂移

describe('应用模式常量', () => {
  it('Mode 与 ModeTitle 成对对应', () => {
    expect(Mode.KgMobile).toBe('KgMobile');
    expect(Mode.KgLite).toBe('KgLite');
    expect(ModeTitle.KgMobile).toBe('酷狗移动版');
    expect(ModeTitle.KgLite).toBe('酷狗概念版');
  });
});
