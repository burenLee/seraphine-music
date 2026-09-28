import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { nextTick } from 'vue';

import { useMiniPlayerBridge } from '@/composables/useMiniPlayerBridge';
import { useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { useSettingStore } from '@/stores/setting';
import { ListType, MiniPlayerEmit, WindowEvent, WindowTarget } from '@/utils/params';

import { createTestPinia, makeListMusic, makeMusicList } from '../helpers/factories';
import { emitToMock, flushAsync, listenMock, triggerListen } from '../helpers/mocks';

describe('useMiniPlayerBridge 迷你播放器事件桥接', () => {
  beforeEach(() => createTestPinia());

  it('start 注册 MiniPlayer 事件监听', async () => {
    const bridge = useMiniPlayerBridge();
    await bridge.start();
    expect(listenMock).toHaveBeenCalledWith(WindowEvent.MiniPlayer, expect.any(Function));
  });

  it('Init 事件同步音频与播放列表', async () => {
    const bridge = useMiniPlayerBridge();
    await bridge.start();
    emitToMock.mockClear();

    triggerListen(WindowEvent.MiniPlayer, { type: MiniPlayerEmit.Init, data: undefined });
    await nextTick();

    expect(emitToMock).toHaveBeenCalledWith(
      WindowTarget.MiniPlayer,
      WindowEvent.MiniPlayer,
      expect.objectContaining({ type: MiniPlayerEmit.Audio }),
    );
    expect(emitToMock).toHaveBeenCalledWith(
      WindowTarget.MiniPlayer,
      WindowEvent.MiniPlayer,
      expect.objectContaining({ type: MiniPlayerEmit.Playlist }),
    );
  });

  it('Play / Pause / Prev / Next 事件分发到播放器 store', async () => {
    const bridge = useMiniPlayerBridge();
    const musicStore = useMusicStore();
    await bridge.start();

    const play = vi.spyOn(musicStore, 'play');
    const pause = vi.spyOn(musicStore, 'pause');
    const playPrevOrNext = vi.spyOn(musicStore, 'playPrevOrNext');

    triggerListen(WindowEvent.MiniPlayer, { type: MiniPlayerEmit.Play, data: undefined });
    triggerListen(WindowEvent.MiniPlayer, { type: MiniPlayerEmit.Pause, data: undefined });
    triggerListen(WindowEvent.MiniPlayer, { type: MiniPlayerEmit.Prev, data: undefined });
    triggerListen(WindowEvent.MiniPlayer, { type: MiniPlayerEmit.Next, data: undefined });

    expect(play).toHaveBeenCalledTimes(1);
    expect(pause).toHaveBeenCalledTimes(1);
    expect(playPrevOrNext).toHaveBeenCalledWith('prev');
    expect(playPrevOrNext).toHaveBeenCalledWith('next');
  });

  it('Set 事件调用 setMusic', async () => {
    const bridge = useMiniPlayerBridge();
    const musicStore = useMusicStore();
    const setMusic = vi.spyOn(musicStore, 'setMusic');
    await bridge.start();

    const music = makeListMusic({ id: 1, hash: null, path: '/music/1.mp3' });
    triggerListen(WindowEvent.MiniPlayer, { type: MiniPlayerEmit.Set, data: music });

    expect(setMusic).toHaveBeenCalledWith(music, expect.objectContaining({ origin: 0 }));
  });

  it('Pos 事件保存迷你播放器坐标', async () => {
    const bridge = useMiniPlayerBridge();
    const settingStore = useSettingStore();
    const setMiniPlayerPosition = vi.spyOn(settingStore, 'setMiniPlayerPosition');
    await bridge.start();

    triggerListen(WindowEvent.MiniPlayer, { type: MiniPlayerEmit.Pos, data: { x: 10, y: 20 } });
    expect(setMiniPlayerPosition).toHaveBeenCalledWith({ x: 10, y: 20 });
  });

  it('Close 事件停止桥接并关闭窗口', async () => {
    const close = vi.fn();
    vi.mocked(WebviewWindow.getByLabel).mockResolvedValueOnce({ close } as unknown as WebviewWindow);

    const bridge = useMiniPlayerBridge();
    await bridge.start();

    triggerListen(WindowEvent.MiniPlayer, { type: MiniPlayerEmit.Close, data: undefined });
    await flushAsync();

    expect(close).toHaveBeenCalledTimes(1);
  });

  it('播放列表变化触发同步', async () => {
    const bridge = useMiniPlayerBridge();
    const listStore = useListStore();
    await bridge.start();
    emitToMock.mockClear();

    listStore.setList(ListType.Play, makeMusicList([makeListMusic()]));
    await nextTick();

    expect(emitToMock).toHaveBeenCalledWith(
      WindowTarget.MiniPlayer,
      WindowEvent.MiniPlayer,
      expect.objectContaining({ type: MiniPlayerEmit.Playlist }),
    );
  });
});
