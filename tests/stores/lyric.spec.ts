import { beforeEach, describe, expect, it } from 'vitest';

import {
  LyricBaseColor,
  LyricFontSize,
  LyricFormat,
  LyricPageMode,
  LyricTextAlign,
  LyricTransMode,
} from '@/constants';
import { useLyricStore } from '@/stores/lyric';

import {
  createTestPinia,
  makeLyricCandidate,
  makeLyricInfo,
  makePlayingMusic,
} from '../helpers/factories';
import { flushAsync, mockInvoke, notifyMock } from '../helpers/mocks';

beforeEach(() => {
  createTestPinia();
});

describe('初始默认值', () => {
  it('歌词页与外观字段均为初始值', () => {
    const store = useLyricStore();

    expect(store.pageVisible).toBe(false);
    expect(store.pageMode).toBe(LyricPageMode.Cover);
    expect(store.isLoading).toBe(false);
    expect(store.lyric).toStrictEqual({ id: '', fmt: LyricFormat.Krc, lines: [] });
    expect(store.fontFamily).toBe('system-ui');
    expect(store.fontSize).toBe(LyricFontSize.Default);
    expect(store.textColor).toBe(LyricBaseColor.Blue);
    expect(store.textAlign).toBe(LyricTextAlign.Left);
    expect(store.transMode).toBe(LyricTransMode.Off);
    expect(store.matchedMap).toStrictEqual({});
    expect(store.offsetMap).toStrictEqual({});
  });
});

describe('外观设置', () => {
  it('togglePageVisible / setPageMode / setLyric', () => {
    const store = useLyricStore();

    store.togglePageVisible();
    expect(store.pageVisible).toBe(true);

    store.setPageMode(LyricPageMode.Record);
    expect(store.pageMode).toBe(LyricPageMode.Record);

    const lyric = makeLyricInfo();
    store.setLyric(lyric);
    expect(store.lyric).toStrictEqual(lyric);
  });

  it('setFontSize 步进增减与重置', () => {
    const store = useLyricStore();

    store.setFontSize('add');
    expect(store.fontSize).toBe(LyricFontSize.Default + 2);

    store.setFontSize('sub');
    store.setFontSize('sub');
    expect(store.fontSize).toBe(LyricFontSize.Default - 2);

    store.setFontSize('restart');
    expect(store.fontSize).toBe(LyricFontSize.Default);
  });

  it('setTextColor / setTextAlign / setTransMode / setMatchedLyric', () => {
    const store = useLyricStore();

    store.setTextColor(LyricBaseColor.Red);
    store.setTextAlign(LyricTextAlign.Center);
    store.setTransMode(LyricTransMode.Trans);
    store.setMatchedLyric(1, 'l1', LyricFormat.Krc);

    expect(store.textColor).toBe(LyricBaseColor.Red);
    expect(store.textAlign).toBe(LyricTextAlign.Center);
    expect(store.transMode).toBe(LyricTransMode.Trans);
    expect(store.matchedMap[1]).toStrictEqual({ id: 'l1', fmt: LyricFormat.Krc });
  });
});

describe('setOffsetMap 歌词偏移', () => {
  it('默认歌词(id为空)时仍会写入 offsetMap', () => {
    const store = useLyricStore();

    store.setOffsetMap('restart');

    expect(store.offsetMap).toStrictEqual({ '': 0 });
  });

  it('按步进调整并重置', () => {
    const store = useLyricStore();
    store.setLyric(makeLyricInfo({ id: 'l1' }));
    store.offsetMap = { l1: 0 };

    store.setOffsetMap('add');
    expect(store.offsetMap['l1']).toBeCloseTo(0.1);

    store.setOffsetMap('sub');
    expect(store.offsetMap['l1']).toBeCloseTo(0);

    store.setOffsetMap('add');
    store.setOffsetMap('restart');
    expect(store.offsetMap['l1']).toBe(0);
  });
});

describe('load 歌词加载（核心路径）', () => {
  it('本地命中：按 matchedMap 调 music_lyric_get 并解析 KRC', async () => {
    const store = useLyricStore();
    const music = makePlayingMusic({ id: 7 });
    store.setMatchedLyric(7, 'krc-1', LyricFormat.Krc);
    mockInvoke({
      music_lyric_get: '[0,1000]<0,500,0>Hello',
    });

    await store.load(music);

    expect(store.lyric?.id).toBe('krc-1');
    expect(store.lyric?.fmt).toBe(LyricFormat.Krc);
    expect(store.lyric?.lines).toHaveLength(1);
    expect(store.lyric?.lines[0].words[0].text).toBe('Hello');
    expect(store.matchedMap[7]).toStrictEqual({ id: 'krc-1', fmt: LyricFormat.Krc });
    expect(store.isLoading).toBe(false);
  });

  it('在线搜索：优先官方推荐，解析 LRC 并记录匹配', async () => {
    const store = useLyricStore();
    const music = makePlayingMusic({ id: 8, hash: 'h8' });
    const normal = makeLyricCandidate({ id: 'c-normal', product_from: '其他' });
    const official = makeLyricCandidate({
      id: 'c-official',
      product_from: '官方推荐歌词',
      contenttype: 1,
    });
    mockInvoke({
      api_lyric_search: { status: 200, candidates: [normal, official] },
      api_lyric_get: '[00:01.00]first line',
    });

    await store.load(music);

    expect(store.lyric?.id).toBe('c-official');
    expect(store.lyric?.lines).toHaveLength(1);
    expect(store.lyric?.lines[0].words[0].text).toBe('first line');
    expect(store.matchedMap[8]).toStrictEqual({ id: 'c-official', fmt: LyricFormat.Lrc });
  });

  it('显式指定候选歌词时跳过搜索', async () => {
    const store = useLyricStore();
    const music = makePlayingMusic({ id: 9 });
    const candidate = makeLyricCandidate({ id: 'c-direct', contenttype: 1 });
    mockInvoke({
      music_lyric_get: '[00:01.00]line',
    });

    await store.load(music, candidate);

    // contenttype=1 → Lrc，直接走本地获取
    expect(store.lyric?.id).toBe('c-direct');
    expect(store.lyric?.fmt).toBe(LyricFormat.Lrc);
  });

  it('搜索无结果时 lyric 为 null', async () => {
    const store = useLyricStore();
    mockInvoke({ api_lyric_search: { status: 200, candidates: [] } });

    await store.load(makePlayingMusic({ id: 10 }));

    expect(store.lyric).toStrictEqual({ id: '', fmt: LyricFormat.Krc, lines: [] });
    expect(store.matchedMap[10]).toBeUndefined();
  });

  it('后端异常时静默回退且不中断', async () => {
    const store = useLyricStore();
    const music = makePlayingMusic({ id: 11 });
    store.setMatchedLyric(11, 'k1', LyricFormat.Krc);
    mockInvoke({
      music_lyric_get: () => {
        throw new Error('io error');
      },
      api_lyric_search: { status: 500, candidates: [] },
    });

    await store.load(music);
    await flushAsync();

    expect(notifyMock.error).not.toHaveBeenCalled();
    expect(store.lyric).toStrictEqual({ id: '', fmt: LyricFormat.Krc, lines: [] });
    expect(store.isLoading).toBe(false);
  });
});
