import { describe, expect, it } from 'vitest';

import { ListType, PicSize } from '@/constants';

// 这些枚举值同时是前端 UI、Tauri invoke 参数与窗口事件契约，断言值域防止意外漂移

describe('列表与搜索常量', () => {
  it('ListType：三种列表', () => {
    expect(ListType.Local).toBe('local');
    expect(ListType.Show).toBe('show');
    expect(ListType.Play).toBe('play');
  });

  it('PicSize 与酷狗图片尺寸约定一致', () => {
    expect(PicSize.Sm).toBe('64');
    expect(PicSize.Md).toBe('120');
    expect(PicSize.Lg).toBe('240');
  });
});
