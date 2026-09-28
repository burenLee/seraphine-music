import { register } from '@tauri-apps/plugin-global-shortcut';
import { check as tauriCheck } from '@tauri-apps/plugin-updater';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import { useListStore } from '@/stores/list';
import { useLyricStore } from '@/stores/lyric';
import { useMusicStore } from '@/stores/music';
import { useSettingStore } from '@/stores/setting';
import { useUpdaterStore } from '@/stores/updater';
import { ListType, PlayingOrigin } from '@/utils/params';

import { createTestPinia, makeListMusic, makeMusicList } from '../helpers/factories';
import { invokeMock, mockInvoke, notifyMock } from '../helpers/mocks';

const registerMock = vi.mocked(register);
const tauriCheckMock = vi.mocked(tauriCheck);

const flush = () => vi.advanceTimersByTimeAsync(0);

beforeEach(() => {
  vi.useFakeTimers();
  createTestPinia();
});

afterEach(() => {
  vi.useRealTimers();
});

describe('N2 快捷键↔切歌↔歌词↔更新通知 联动', () => {
  it('快捷键 next → 切歌 → 歌词联动', async () => {
    useSettingStore();
    const listStore = useListStore();
    const musicStore = useMusicStore();
    const lyricStore = useLyricStore();
    await flush(); // 水合：注册全局/媒体快捷键

    const list = [
      makeListMusic({ id: 1, hash: null, path: '/music/1.mp3', title: 'T1', artist: 'A1' }),
      makeListMusic({ id: 2, hash: null, path: '/music/2.mp3', title: 'T2', artist: 'A2' }),
    ];
    listStore.setList(ListType.Play, makeMusicList(list, { id: 'play' }));

    mockInvoke({ api_lyric_search: { status: 200, candidates: [] } });
    await musicStore.setMusic(list[0], { origin: PlayingOrigin.Local });
    await flush();
    invokeMock.mockClear();

    // 找到 next 快捷键（Alt+Right）的回调并模拟按下
    const nextCall = registerMock.mock.calls.find(([shortcut]) => shortcut === 'Alt+Right');
    expect(nextCall).toBeDefined();
    nextCall![1]({ state: 'Pressed', shortcut: 'Alt+Right', id: 0 });
    await flush();

    // 切歌：加载第 2 首并播放
    expect(invokeMock).toHaveBeenCalledWith('music_player_load_file', { path: '/music/2.mp3' });
    expect(musicStore.music?.id).toBe(2);

    // 歌词联动：切歌触发歌词重新加载（重新搜索）
    expect(invokeMock).toHaveBeenCalledWith('api_lyric_search', expect.anything());
    expect(lyricStore.matchedMap).not.toHaveProperty('2');
  });

  it('更新检查 → 通知链（已是最新版本）', async () => {
    const updaterStore = useUpdaterStore();
    await flush(); // 水合触发 check

    expect(updaterStore.updateInfo?.hasUpdate).toBe(false);
    expect(updaterStore.updateInfo?.latestVersion).toBe('v0.2.0');
  });

  it('发现新版本 → 通知并记录更新信息', async () => {
    tauriCheckMock.mockResolvedValueOnce({
      currentVersion: '0.1.0',
      version: '0.2.0',
      body: 'release notes',
      date: '2026-01-01',
    } as never);

    const updaterStore = useUpdaterStore();
    await flush();

    expect(notifyMock.success).toHaveBeenCalledWith('发现新版本 0.2.0');
    expect(updaterStore.updateInfo?.hasUpdate).toBe(true);
    expect(updaterStore.updateInfo?.latestVersion).toBe('v0.2.0');
    expect(updaterStore.updateInfo?.body).toBe('release notes');
  });
});
