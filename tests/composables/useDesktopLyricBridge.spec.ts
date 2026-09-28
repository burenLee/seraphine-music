import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { useDesktopLyricBridge } from '@/composables/useDesktopLyricBridge';
import { useMusicStore } from '@/stores/music';
import { useSettingStore } from '@/stores/setting';
import { DesktopLyricEmit, WindowEvent, WindowTarget } from '@/utils/params';

import { createTestPinia } from '../helpers/factories';
import { emitToMock, flushAsync, listenMock, triggerListen } from '../helpers/mocks';

describe('useDesktopLyricBridge 桌面歌词事件桥接', () => {
  beforeEach(() => createTestPinia());

  it('start 注册 DesktopLyric 事件监听', async () => {
    const bridge = useDesktopLyricBridge();
    await bridge.start();
    expect(listenMock).toHaveBeenCalledWith(WindowEvent.DesktopLyric, expect.any(Function));
  });

  it('Init 事件同步音频 / 歌词 / 字体 / 进度', async () => {
    const bridge = useDesktopLyricBridge();
    await bridge.start();
    emitToMock.mockClear();

    triggerListen(WindowEvent.DesktopLyric, { type: DesktopLyricEmit.Init, data: undefined });

    for (const type of [
      DesktopLyricEmit.Audio,
      DesktopLyricEmit.Lyric,
      DesktopLyricEmit.Fonts,
      DesktopLyricEmit.Progress,
    ]) {
      expect(emitToMock).toHaveBeenCalledWith(
        WindowTarget.DesktopLyric,
        WindowEvent.DesktopLyric,
        expect.objectContaining({ type }),
      );
    }
  });

  it('Main 事件唤起主窗口', async () => {
    const bridge = useDesktopLyricBridge();
    await bridge.start();

    const currentWindow = getCurrentWindow();
    vi.mocked(getCurrentWindow).mockReturnValue(currentWindow);
    const show = vi.spyOn(currentWindow, 'show');

    triggerListen(WindowEvent.DesktopLyric, { type: DesktopLyricEmit.Main, data: undefined });
    expect(show).toHaveBeenCalledTimes(1);
  });

  it('Play / Pause / Prev / Next 事件分发到播放器 store', async () => {
    const bridge = useDesktopLyricBridge();
    const musicStore = useMusicStore();
    await bridge.start();

    const play = vi.spyOn(musicStore, 'play');
    const pause = vi.spyOn(musicStore, 'pause');
    const playPrevOrNext = vi.spyOn(musicStore, 'playPrevOrNext');

    triggerListen(WindowEvent.DesktopLyric, { type: DesktopLyricEmit.Play, data: undefined });
    triggerListen(WindowEvent.DesktopLyric, { type: DesktopLyricEmit.Pause, data: undefined });
    triggerListen(WindowEvent.DesktopLyric, { type: DesktopLyricEmit.Prev, data: undefined });
    triggerListen(WindowEvent.DesktopLyric, { type: DesktopLyricEmit.Next, data: undefined });

    expect(play).toHaveBeenCalledTimes(1);
    expect(pause).toHaveBeenCalledTimes(1);
    expect(playPrevOrNext).toHaveBeenCalledWith('prev');
    expect(playPrevOrNext).toHaveBeenCalledWith('next');
  });

  it('Pos 事件保存桌面歌词坐标', async () => {
    const bridge = useDesktopLyricBridge();
    const settingStore = useSettingStore();
    const setPosition = vi.spyOn(settingStore, 'setdesktopLyricPosition');
    await bridge.start();

    triggerListen(WindowEvent.DesktopLyric, { type: DesktopLyricEmit.Pos, data: { x: 5, y: 6 } });
    expect(setPosition).toHaveBeenCalledWith({ x: 5, y: 6 });
  });

  it('Close 事件停止桥接并关闭窗口', async () => {
    const close = vi.fn();
    vi.mocked(WebviewWindow.getByLabel).mockResolvedValueOnce({
      close,
    } as unknown as WebviewWindow);

    const bridge = useDesktopLyricBridge();
    await bridge.start();

    triggerListen(WindowEvent.DesktopLyric, { type: DesktopLyricEmit.Close, data: undefined });
    await flushAsync();

    expect(close).toHaveBeenCalledTimes(1);
  });
});
