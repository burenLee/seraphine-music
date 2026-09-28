import { describe, expect, it } from 'vitest';

import { IconMap } from '@/utils/icons';

describe('IconMap', () => {
  it('包含全部 79 个图标映射', () => {
    expect(Object.keys(IconMap)).toHaveLength(79);
  });

  it('每个映射的值都是已定义的组件', () => {
    for (const [name, component] of Object.entries(IconMap)) {
      expect(component, `图标 ${name} 未定义`).toBeDefined();
      expect(['object', 'function']).toContain(typeof component);
    }
  });

  it('播放控制核心图标存在', () => {
    for (const name of [
      'Play',
      'PlayBold',
      'Pause',
      'PauseBold',
      'Next',
      'Previous',
      'Heart',
      'Search',
      'Setting',
      'Ring',
    ] as const) {
      expect(IconMap[name]).toBeDefined();
    }
  });

  it('五种播放模式均有对应图标（PlayingMode UI 依赖）', () => {
    for (const name of [
      'OrderPlay',
      'SinglePlay',
      'RepeatAll',
      'RepeatOne',
      'RandomPlay',
    ] as const) {
      expect(IconMap[name]).toBeDefined();
    }
  });

  it('图标名无重复（对象键天然去重，断言与 import 数量一致）', () => {
    const keys = Object.keys(IconMap);
    expect(new Set(keys).size).toBe(keys.length);
  });
});
