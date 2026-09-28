import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import { useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { Interval, ListType, PlayingMode, PlayingOrigin, PlayingQuality } from '@/utils/params';

import {
  createTestPinia,
  makeListMusic,
  makeMusicList,
  makePlayingMusic,
} from '../helpers/factories';
import { invokeMock, mockInvoke, notifyMock } from '../helpers/mocks';

// 水合 watch（pause/monitor）与 setMusic 链路均为微任务，fake timers 下用 advanceTimersByTimeAsync(0) 排空
const flush = () => vi.advanceTimersByTimeAsync(0);

// 水合 watch 启动 monitorPlay 时会注册播放进度 Channel，需在清空调用记录前缓存
let playChannelCache: { onmessage: ((pg: number) => void) | null } | null = null;

const setupStore = async () => {
  const store = useMusicStore();
  const listStore = useListStore();
  await flush();
  const playCall = invokeMock.mock.calls.find(([cmd]) => cmd === 'music_player_monitor_play');
  playChannelCache = playCall
    ? (playCall[1] as { channel: { onmessage: ((pg: number) => void) | null } }).channel
    : null;
  invokeMock.mockClear();
  return { store, listStore };
};

/** 构造 count 首本地歌曲组成的播放列表，并让 store 播放第 currentIndex 首（isPlaying=true） */
const setupPlaying = async (count = 3, currentIndex = 1) => {
  const { store, listStore } = await setupStore();
  const list = Array.from({ length: count }, (_, i) =>
    makeListMusic({ id: i + 1, hash: null, path: `/music/t${i + 1}.mp3`, title: `T${i + 1}` }),
  );
  listStore.setList(ListType.Play, makeMusicList(list, { id: 'play' }));
  await store.setMusic(list[currentIndex], { origin: PlayingOrigin.Local });
  invokeMock.mockClear();
  return { store, listStore, list };
};

/** 取水合后注册的播放进度 Channel（setupStore 已缓存） */
const getPlayChannel = () => playChannelCache!;

/** 模拟播放进度跨越「距结束 0.3s」阈值，触发 playAutoNext */
const triggerAutoNext = async () => {
  const channel = getPlayChannel();
  channel.onmessage?.(100); // 建立 lastProgress（未达阈值）
  channel.onmessage?.(179.9); // 跨越阈值（duration=180）
  await flush();
};

beforeEach(() => {
  vi.useFakeTimers();
  createTestPinia();
});

afterEach(() => {
  vi.useRealTimers();
});

describe('初始默认值', () => {
  it('状态机各标志与字段均为初始值', async () => {
    const { store } = await setupStore();

    expect(store.isHydrated).toBe(true);
    expect(store.isLoading).toBe(false);
    expect(store.isLoaded).toBe(false);
    expect(store.isPlaying).toBe(false);
    expect(store.isDragging).toBe(false);
    expect(store.music).toBeUndefined();
    expect(store.origin).toBe(PlayingOrigin.Local);
    expect(store.mode).toBe(PlayingMode.OrderPlay);
    expect(store.volume).toBe(100);
    expect(store.lastVolumn).toBe(100);
    expect(store.quality).toBe(PlayingQuality.Bitrate128);
    expect(store.playProgress).toBe(0);
    expect(store.downloadProgress).toBe(0);
  });
});

describe('setMusic 加载流程', () => {
  it('传入 null 时停止播放', async () => {
    const { store } = await setupStore();

    await store.setMusic(undefined);

    expect(invokeMock).toHaveBeenCalledWith('music_player_stop', undefined);
  });

  it('相同 id 且非 loop 时不重复加载', async () => {
    const { store } = await setupStore();
    const music = makePlayingMusic({ path: '/music/a.mp3' });
    await store.setMusic(music);
    invokeMock.mockClear();

    await store.setMusic({ ...music });

    expect(invokeMock).not.toHaveBeenCalledWith('music_player_load_file', expect.anything());
  });

  it('本地音频：停止 → 加载文件 → 自动播放', async () => {
    const { store } = await setupStore();
    const music = makePlayingMusic({ path: '/music/a.mp3', title: 'Song', artist: 'Artist' });

    await store.setMusic(music, { origin: PlayingOrigin.Local });

    expect(invokeMock).toHaveBeenCalledWith('music_player_stop', undefined);
    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: '/music/a.mp3' });
    expect(invokeMock).toHaveBeenCalledWith('music_player_play', undefined);
    expect(store.music?.id).toBe(music.id);
    expect(store.isLoaded).toBe(true);
    expect(store.isPlaying).toBe(true);
    expect(document.title).toBe('Song - Artist');
  });

  it('autoPlay=false 时只加载不播放', async () => {
    const { store } = await setupStore();

    await store.setMusic(makePlayingMusic(), { autoPlay: false });

    expect(invokeMock).not.toHaveBeenCalledWith('music_player_play', undefined);
    expect(store.isLoaded).toBe(true);
    expect(store.isPlaying).toBe(false);
  });

  it('在线音频：先取 URL 再加载，music.path 被写为播放地址', async () => {
    const { store } = await setupStore();
    mockInvoke({
      api_song_url: { status: 1, backupUrl: ['https://cdn.example.com/a.mp3'] },
    });
    const music = makePlayingMusic({ hash: 'hash-abc', path: null });

    await store.setMusic(music, { origin: PlayingOrigin.Online });

    expect(invokeMock).toHaveBeenCalledWith('api_song_url', {
      hash: 'hash-abc',
      quality: PlayingQuality.Bitrate128,
    });
    expect(invokeMock).toHaveBeenCalledWith('music_player_load_url', {
      url: 'https://cdn.example.com/a.mp3',
      hash: 'hash-abc',
    });
    expect(store.music?.path).toBe('https://cdn.example.com/a.mp3');
    expect(store.isLoaded).toBe(true);
  });

  it('在线音频取 URL 失败时提示错误且不加载', async () => {
    const { store } = await setupStore();
    mockInvoke({ api_song_url: { status: 0 } });
    const music = makePlayingMusic({ title: 'BadSong', hash: 'hash-x', path: null });

    await store.setMusic(music, { origin: PlayingOrigin.Online });

    expect(store.isLoaded).toBe(false);
    expect(store.isPlaying).toBe(false);
    expect(notifyMock.warning).toHaveBeenCalledWith('无法播放：《BadSong》，自动切换下一首...');
    expect(invokeMock).not.toHaveBeenCalledWith('music_player_load_url', expect.anything());
  });

  it('本地音频缺 path 时提示错误', async () => {
    const { store } = await setupStore();
    const music = makePlayingMusic({ title: 'NoPath', path: null });

    await store.setMusic(music, { origin: PlayingOrigin.Local });

    expect(store.isLoaded).toBe(false);
    expect(notifyMock.warning).toHaveBeenCalledWith('无法播放：《NoPath》，自动切换下一首...');
  });
});

describe('播放控制', () => {
  it('play 守卫：无音频 / 加载中 / 未加载时不调用后端', async () => {
    const { store } = await setupStore();

    await store.play(); // 无 music
    expect(invokeMock).not.toHaveBeenCalledWith('music_player_play', undefined);

    store.isLoading = true;
    store.music = makePlayingMusic();
    await store.play();
    expect(invokeMock).not.toHaveBeenCalledWith('music_player_play', undefined);
  });

  it('play/pause 更新 isPlaying 标志', async () => {
    const { store } = await setupPlaying();

    await store.pause();
    expect(store.isPlaying).toBe(false);
    expect(invokeMock).toHaveBeenCalledWith('music_player_pause', undefined);

    await store.play();
    expect(store.isPlaying).toBe(true);
  });

  it('play/pause 后端失败时提示错误且状态不变', async () => {
    const { store } = await setupPlaying();
    invokeMock.mockRejectedValueOnce(new Error('io'));
    await store.pause();
    expect(store.isPlaying).toBe(true);
    expect(notifyMock.error).toHaveBeenCalledWith('无法暂停音频');
  });

  it('stop 复位播放与下载进度', async () => {
    const { store } = await setupPlaying();
    store.playProgress = 66;
    store.downloadProgress = 0.5;

    await store.stop();

    expect(store.isPlaying).toBe(false);
    expect(store.playProgress).toBe(0);
    expect(store.downloadProgress).toBe(0);
    expect(document.title).toBe('Seraphine');
  });

  it('seek 透传位置参数', async () => {
    const { store } = await setupPlaying();

    await store.seek(42);

    expect(invokeMock).toHaveBeenCalledWith('music_player_seek', { pos: 42 });
  });
});

describe('音量与模式设置', () => {
  it('setVolume 夹在 [0,100] 并同步后端', async () => {
    const { store } = await setupStore();

    await store.setVolume(150);
    expect(store.volume).toBe(100);
    expect(invokeMock).toHaveBeenCalledWith('music_player_set_volume', { volume: 100 });

    await store.setVolume(-10);
    expect(store.volume).toBe(0);
  });

  it('lastVolumn 记录最后的非零音量', async () => {
    const { store } = await setupStore();

    await store.setVolume(30);
    expect(store.lastVolumn).toBe(30);

    await store.setVolume(0);
    expect(store.lastVolumn).toBe(30);
  });

  it('setVolume 失败时提示错误', async () => {
    const { store } = await setupStore();
    invokeMock.mockRejectedValueOnce(new Error('io'));

    await store.setVolume(50);

    expect(store.volume).toBe(100);
    expect(notifyMock.error).toHaveBeenCalledWith('无法设置音量');
  });

  it('setMode / setQuality 直接更新状态', async () => {
    const { store } = await setupStore();

    store.setMode(PlayingMode.RandomPlay);
    store.setQuality(PlayingQuality.BitrateFlac);

    expect(store.mode).toBe(PlayingMode.RandomPlay);
    expect(store.quality).toBe(PlayingQuality.BitrateFlac);
  });
});

describe('playPrevOrNext 手动切歌', () => {
  it('列表为空时循环重载当前音频', async () => {
    const { store } = await setupStore();
    const music = makePlayingMusic({ path: '/music/solo.mp3' });
    await store.setMusic(music);
    invokeMock.mockClear();

    store.playPrevOrNext('next');
    await flush();

    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: '/music/solo.mp3' });
  });

  it('顺序模式：next 到下一首，末位环绕到开头', async () => {
    const { store, list } = await setupPlaying(3, 1);

    store.playPrevOrNext('next');
    await flush();
    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: list[2].path });

    invokeMock.mockClear();
    store.playPrevOrNext('next');
    await flush();
    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: list[0].path });
  });

  it('顺序模式：prev 到上一首，开头环绕到末位', async () => {
    const { store, list } = await setupPlaying(3, 0);

    store.playPrevOrNext('prev');
    await flush();

    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: list[2].path });
  });

  it('当前音频不在列表时从开头播放', async () => {
    const { store, listStore, list } = await setupPlaying(3, 1);
    await store.setMusic(makePlayingMusic({ id: 999, path: '/music/outside.mp3' }));
    invokeMock.mockClear();

    store.playPrevOrNext('next');
    await flush();

    expect(listStore.play.list).toHaveLength(3);
    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: list[0].path });
  });

  it('随机模式：切到的不是当前索引 0 之外的确定性行为（结果 ∈ 列表且不恒等于原曲）', async () => {
    const { store, list } = await setupPlaying(3, 1);
    store.setMode(PlayingMode.RandomPlay);

    const paths = new Set<string>();
    for (let i = 0; i < 20; i += 1) {
      store.playPrevOrNext('next');
      await flush();
      const calls = invokeMock.mock.calls.filter(([cmd]) => cmd === 'music_player_load_file');
      const call = calls[calls.length - 1];
      if (call) paths.add((call[1] as { path: string }).path);
      invokeMock.mockClear();
    }

    // 随机源 srcNum=0，因此不会切到列表第 0 首；多次采样应命中过其他曲目
    expect(paths.has(list[0].path!)).toBe(false);
    expect(paths.size).toBeGreaterThan(0);
  });
});

describe('playAutoNext 自动切歌（5 种播放模式）', () => {
  it('OrderPlay：非末位自动切下一首', async () => {
    const { list } = await setupPlaying(3, 1);

    await triggerAutoNext();

    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: list[2].path });
  });

  it('OrderPlay：末位循环重载但不自动播放', async () => {
    const { list } = await setupPlaying(3, 2);

    await triggerAutoNext();

    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: list[2].path });
    expect(invokeMock).not.toHaveBeenCalledWith('music_player_play', undefined);
  });

  it('SinglePlay：重载当前曲且不自动播放', async () => {
    const { store, list } = await setupPlaying(3, 1);
    store.setMode(PlayingMode.SinglePlay);

    await triggerAutoNext();

    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: list[1].path });
    expect(invokeMock).not.toHaveBeenCalledWith('music_player_play', undefined);
  });

  it('OrderLoop：末位回到开头并继续播放', async () => {
    const { store, list } = await setupPlaying(3, 2);
    store.setMode(PlayingMode.OrderLoop);

    await triggerAutoNext();

    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: list[0].path });
    expect(invokeMock).toHaveBeenCalledWith('music_player_play', undefined);
  });

  it('SingleLoop：重载当前曲并继续播放', async () => {
    const { store, list } = await setupPlaying(3, 1);
    store.setMode(PlayingMode.SingleLoop);

    await triggerAutoNext();

    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: list[1].path });
    expect(invokeMock).toHaveBeenCalledWith('music_player_play', undefined);
  });

  it('RandomPlay：切到非当前索引的曲目', async () => {
    const { store, list } = await setupPlaying(3, 1);
    store.setMode(PlayingMode.RandomPlay);

    await triggerAutoNext();

    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', expect.anything());
    expect(invokeMock).not.toHaveBeenCalledWith('music_player_load_file', { path: list[1].path });
  });

  it('未播放状态下进度推进不触发自动切歌', async () => {
    const { store } = await setupPlaying(3, 1);
    await store.pause();
    invokeMock.mockClear();

    const channel = getPlayChannel();
    channel.onmessage?.(100);
    channel.onmessage?.(179.9);
    await flush();

    expect(invokeMock).not.toHaveBeenCalledWith('music_player_load_file', expect.anything());
  });
});

describe('进度拖拽', () => {
  it('startChangeProgress 守卫与标志', async () => {
    const { store } = await setupPlaying();

    store.startChangeProgress();
    expect(store.isDragging).toBe(true);

    store.startChangeProgress(); // 重复调用保持
    expect(store.isDragging).toBe(true);
  });

  it('无音频时不进入拖拽状态', async () => {
    const { store } = await setupStore();

    store.startChangeProgress();

    expect(store.isDragging).toBe(false);
  });

  it('本地音频：松手即 seek 并恢复播放', async () => {
    const { store } = await setupPlaying();

    store.startChangeProgress();
    await store.stopChangeProgress(60);

    expect(invokeMock).toHaveBeenCalledWith('music_player_seek', { pos: 60 });
    expect(invokeMock).toHaveBeenCalledWith('music_player_play', undefined);
    expect(store.isDragging).toBe(false);
  });

  it('在线音频下载进度充足：直接 seek 播放', async () => {
    const { store } = await setupStore();
    mockInvoke({ api_song_url: { status: 1, backupUrl: ['https://cdn.example.com/a.mp3'] } });
    await store.setMusic(makePlayingMusic({ hash: 'h1', path: null, duration: 180 }), {
      origin: PlayingOrigin.Online,
    });
    invokeMock.mockClear();

    store.downloadProgress = 0.8;
    store.startChangeProgress();
    await store.stopChangeProgress(90); // 90/180 = 0.5 <= 0.8

    expect(invokeMock).toHaveBeenCalledWith('music_player_seek', { pos: 90 });
    expect(store.isDragging).toBe(false);
  });

  it('在线音频下载进度不足：暂停等待，下载达标后再 seek 播放', async () => {
    const { store } = await setupStore();
    mockInvoke({ api_song_url: { status: 1, backupUrl: ['https://cdn.example.com/a.mp3'] } });
    await store.setMusic(makePlayingMusic({ hash: 'h1', path: null, duration: 180 }), {
      origin: PlayingOrigin.Online,
    });
    invokeMock.mockClear();

    store.downloadProgress = 0.1;
    store.startChangeProgress();
    const pending = store.stopChangeProgress(90); // 90/180 = 0.5 > 0.1 → 进入等待
    await flush();

    expect(store.isLoading).toBe(true);
    expect(invokeMock).toHaveBeenCalledWith('music_player_pause', undefined);
    expect(invokeMock).not.toHaveBeenCalledWith('music_player_seek', { pos: 90 });

    // 模拟后端下载推进
    store.downloadProgress = 0.9;
    await vi.advanceTimersByTimeAsync(Interval.Long);
    await pending;

    expect(invokeMock).toHaveBeenCalledWith('music_player_seek', { pos: 90 });
    expect(invokeMock).toHaveBeenCalledWith('music_player_play', undefined);
    expect(store.isLoading).toBe(false);
  });
});

describe('加载失败自动重试（MAX_RETRY_COUNT）', () => {
  it('连续失败重试切歌，达到上限后停止并提示', async () => {
    const { store, listStore } = await setupStore();
    const bad1 = makeListMusic({ id: 'bad-1', title: 'Bad1', hash: null, path: null });
    const bad2 = makeListMusic({ id: 'bad-2', title: 'Bad2', hash: null, path: null });
    listStore.setList(ListType.Play, makeMusicList([bad1, bad2], { id: 'play' }));

    await store.setMusic(bad1, { origin: PlayingOrigin.Local });
    expect(notifyMock.warning).toHaveBeenCalledWith('无法播放：《Bad1》，自动切换下一首...');

    // 第 1 次重试：切到 bad2 → 同样失败
    await vi.advanceTimersByTimeAsync(Interval.PoN);
    expect(notifyMock.warning).toHaveBeenCalledWith('无法播放：《Bad2》，自动切换下一首...');
    expect(notifyMock.error).not.toHaveBeenCalledWith('重试次数过多, 停止重试');

    // 第 2 次重试：切回 bad1 → 失败
    await vi.advanceTimersByTimeAsync(Interval.PoN);
    expect(notifyMock.error).not.toHaveBeenCalledWith('重试次数过多, 停止重试');

    // 第 3 次：达到 MAX_RETRY_COUNT，停止重试
    notifyMock.error.mockClear();
    await vi.advanceTimersByTimeAsync(Interval.PoN);
    expect(notifyMock.error).toHaveBeenCalledWith('重试次数过多, 停止重试');

    // 不再发起新的切歌
    const stopCalls = invokeMock.mock.calls.filter(([cmd]) => cmd === 'music_player_stop').length;
    await vi.advanceTimersByTimeAsync(Interval.PoN * 5);
    expect(invokeMock.mock.calls.filter(([cmd]) => cmd === 'music_player_stop').length).toBe(
      stopCalls,
    );
  });
});

describe('持久化水合', () => {
  it('恢复持久化字段并重新加载音频/音量/进度', async () => {
    const persistedMusic = makePlayingMusic({ id: 'old', path: '/music/old.mp3' });
    localStorage.setItem(
      'music-store',
      JSON.stringify({
        music: persistedMusic,
        origin: PlayingOrigin.Local,
        volume: 50,
        mode: PlayingMode.OrderLoop,
        quality: PlayingQuality.Bitrate320,
        playProgress: 30,
      }),
    );

    const store = useMusicStore();
    await flush();

    expect(store.music?.id).toBe('old');
    expect(store.volume).toBe(50);
    expect(store.mode).toBe(PlayingMode.OrderLoop);
    expect(store.quality).toBe(PlayingQuality.Bitrate320);
    // 水合 watch：音量非默认 → 同步后端；有音频 → 重载；有进度 → seek
    expect(invokeMock).toHaveBeenCalledWith('music_player_set_volume', { volume: 50 });
    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: '/music/old.mp3' });
    expect(invokeMock).toHaveBeenCalledWith('music_player_seek', { pos: 30 });
  });
});
