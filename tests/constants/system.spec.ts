import { describe, expect, it } from 'vitest';

import { ShortcutKey, SizeUnits } from '@/constants';

// 这些枚举值同时是前端 UI、Tauri invoke 参数与窗口事件契约，断言值域防止意外漂移

describe('设置相关常量', () => {
  it('SizeUnits 从 B 到 YB 共 9 个单位', () => {
    expect(SizeUnits).toHaveLength(9);
    expect(SizeUnits[0]).toBe('B');
    expect(SizeUnits[8]).toBe('YB');
  });

  it('ShortcutKey 关键值', () => {
    expect(ShortcutKey.PlayOrPause).toBe('playOrPause');
  });
});
