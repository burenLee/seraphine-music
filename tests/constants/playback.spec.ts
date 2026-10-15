import { describe, expect, it } from 'vitest';

import { ForwardDuration, Interval, PlayingMode, PlayingOrigin, PlayingQuality } from '@/constants';

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

  it('Interval：播放相关帧间隔', () => {
    expect(Interval.PoN).toBe(2000);
    expect(Interval.Long).toBe(100);
    expect(Interval.Sec).toBe(1000);
    expect(Interval.Min).toBe(60000);
  });

  it('ForwardDuration 歌词提前量', () => {
    expect(ForwardDuration).toBe(150);
  });
});
