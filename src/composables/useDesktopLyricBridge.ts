import { emitTo, listen } from '@tauri-apps/api/event';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { PhysicalPosition, getCurrentWindow } from '@tauri-apps/api/window';
import { watchThrottled } from '@vueuse/core';
import { watch } from 'vue';

import { useLyricStore } from '@/stores/lyric';
import { useMusicStore } from '@/stores/music';
import { useSettingStore } from '@/stores/setting';
import { DesktopLyricEmit, Interval, WindowEvent, WindowTarget } from '@/utils/params';

export function useDesktopLyricBridge() {
  const musicStore = useMusicStore();
  const lyricStore = useLyricStore();
  const settingStore = useSettingStore();

  const stopFns: (() => void)[] = [];

  const syncAudio = () => {
    const data: DesktopLyricAudio = {
      isLoading: musicStore.isLoading,
      isPlaying: musicStore.isPlaying,
      music: musicStore.music,
    };

    emitTo(WindowTarget.DesktopLyric, WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Audio,
      data,
    });
  };

  const syncLyric = () => {
    emitTo(WindowTarget.DesktopLyric, WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Lyric,
      data: lyricStore.lyric,
    });
  };

  const syncFonts = () => {
    if (settingStore.availableFonts.length === 0) settingStore.getAvailableFonts();

    emitTo(WindowTarget.DesktopLyric, WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Fonts,
      data: settingStore.availableFonts,
    });
  };

  const syncProgress = () => {
    emitTo(WindowTarget.DesktopLyric, WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Progress,
      data: musicStore.playProgress,
    });
  };

  /** 启动桥接 */
  const start = async () => {
    const unwatchAudio = watch(
      [() => musicStore.isLoading, () => musicStore.isPlaying, () => musicStore.music],
      syncAudio,
    );
    const unwatchLyric = watch(() => lyricStore.lyric, syncLyric);
    const unwatchFonts = watch(() => settingStore.availableFonts, syncFonts);
    const unwatchProgress = watchThrottled(() => musicStore.playProgress, syncProgress, {
      throttle: Interval.Medium,
    });
    const unlistenAction = await listen<{ type: DesktopLyricEmit; data: unknown }>(
      WindowEvent.DesktopLyric,
      async (e) => {
        switch (e.payload.type) {
          case DesktopLyricEmit.Init:
            syncAudio();
            syncLyric();
            syncFonts();
            syncProgress();
            break;
          case DesktopLyricEmit.Main:
            getCurrentWindow().show();
            break;
          case DesktopLyricEmit.Play:
            musicStore.play();
            break;
          case DesktopLyricEmit.Pause:
            musicStore.pause();
            break;
          case DesktopLyricEmit.Prev:
            musicStore.playPrevOrNext('prev');
            break;
          case DesktopLyricEmit.Next:
            musicStore.playPrevOrNext('next');
            break;
          case DesktopLyricEmit.Close:
            stop();
            break;
          case DesktopLyricEmit.Pos:
            const { x, y } = e.payload.data as PhysicalPosition;
            settingStore.setdesktopLyricPosition({ x, y });
            break;
        }
      },
    );

    stopFns.push(unwatchAudio, unwatchLyric, unwatchFonts, unwatchProgress, unlistenAction);
  };

  /** 停止桥接 */
  const stop = async () => {
    stopFns.forEach((fn) => fn());
    stopFns.length = 0;

    const lyricWindow = await WebviewWindow.getByLabel(WindowTarget.DesktopLyric);
    if (lyricWindow) lyricWindow.close();
  };

  return { start, stop };
}
