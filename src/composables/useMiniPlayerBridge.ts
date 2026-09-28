import { emitTo, listen } from '@tauri-apps/api/event';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { PhysicalPosition } from '@tauri-apps/api/window';
import { watchThrottled } from '@vueuse/core';
import { computed, watch } from 'vue';

import { useListStore } from '@/stores/list';
import { useLyricStore } from '@/stores/lyric';
import { useMusicStore } from '@/stores/music';
import { useSettingStore } from '@/stores/setting';
import { getOrigin } from '@/utils/music';
import { Interval, MiniPlayerEmit, WindowEvent, WindowTarget } from '@/utils/params';

const FORWARD_DURATION = 150; // 歌词提前滚动时间 (ms)

export function useMiniPlayerBridge() {
  const listStore = useListStore();
  const musicStore = useMusicStore();
  const lyricStore = useLyricStore();
  const settingStore = useSettingStore();

  const stopFns: (() => void)[] = [];

  // 当前歌词的偏移量
  const lyricOffset = computed(() =>
    lyricStore.lyric ? lyricStore.offsetMap[lyricStore.lyric.id] || 0 : 0,
  );

  const syncAudio = async () => {
    const data: MiniPlayerAudio = {
      isLoading: musicStore.isLoading,
      isPlaying: musicStore.isPlaying,
      music: musicStore.music,
      origin: musicStore.origin,
    };
    await emitTo(WindowTarget.MiniPlayer, WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Audio,
      data,
    });
  };

  const syncPlaylist = async () => {
    await emitTo(WindowTarget.MiniPlayer, WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Playlist,
      data: listStore.play.list,
    });
  };

  const syncLyric = () => {
    if (!lyricStore.lyric) return;

    const lines = lyricStore.lyric.lines;
    const pg = (musicStore.playProgress + lyricOffset.value) * 1000 + FORWARD_DURATION;

    let text = '';
    for (let i = 0; i < lines.length; i++) {
      if (pg < lines[i].offset || pg > (lines[i + 1]?.offset || Infinity)) continue;

      text = lines[i].words.map((word) => word.text).join('');
      break;
    }

    emitTo(WindowTarget.MiniPlayer, WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Lyric,
      data: text,
    });
  };

  /** 启动桥接 */
  const start = async () => {
    const unwatchAudio = watch(
      [() => musicStore.isLoading, () => musicStore.isPlaying, () => musicStore.music],
      syncAudio,
    );
    const unwatchPlaylist = watch(() => listStore.play.list, syncPlaylist);
    const unwatchLyric = watchThrottled(
      [() => musicStore.playProgress, () => lyricStore.lyric, lyricOffset],
      syncLyric,
      { throttle: Interval.Long },
    );
    const unlistenAction = await listen<{ type: MiniPlayerEmit; data: unknown }>(
      WindowEvent.MiniPlayer,
      async (e) => {
        switch (e.payload.type) {
          case MiniPlayerEmit.Init:
            syncAudio();
            syncPlaylist();
            syncLyric();
            break;
          case MiniPlayerEmit.Play:
            musicStore.play();
            break;
          case MiniPlayerEmit.Pause:
            musicStore.pause();
            break;
          case MiniPlayerEmit.Prev:
            musicStore.playPrevOrNext('prev');
            break;
          case MiniPlayerEmit.Next:
            musicStore.playPrevOrNext('next');
            break;
          case MiniPlayerEmit.Set: {
            const music = e.payload.data as MusicInfo;
            musicStore.setMusic(music, { origin: getOrigin(music) });
            break;
          }
          case MiniPlayerEmit.Close:
            stop();
            break;
          case MiniPlayerEmit.Pos:
            const { x, y } = e.payload.data as PhysicalPosition;
            settingStore.setMiniPlayerPosition({ x, y });
            break;
        }
      },
    );

    stopFns.push(unwatchAudio, unwatchLyric, unwatchPlaylist, unlistenAction);
  };

  /** 停止桥接 */
  const stop = async () => {
    stopFns.forEach((fn) => fn());
    stopFns.length = 0;

    const miniWindow = await WebviewWindow.getByLabel(WindowTarget.MiniPlayer);
    if (miniWindow) miniWindow.close();
  };

  return { start, stop };
}
