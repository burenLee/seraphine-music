import { emitTo, listen } from '@tauri-apps/api/event';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { PhysicalPosition } from '@tauri-apps/api/window';
import { computed, watch } from 'vue';

import { MiniPlayerEmit, WindowEvent, WindowTarget } from '@/constants';
import { useListStore } from '@/stores/list';
import { useLyricStore } from '@/stores/lyric';
import { useMusicStore } from '@/stores/music';
import { useStateStore } from '@/stores/state';
import { getOrigin } from '@/utils/music';

export function useMiniPlayerBridge() {
  const stateStore = useStateStore();
  const listStore = useListStore();
  const musicStore = useMusicStore();
  const lyricStore = useLyricStore();

  const stopFns: (() => void)[] = [];

  // 当前高亮歌词行索引
  const currentIndex = computed(() => {
    const { id, lines } = lyricStore.lyric;
    if (!id || !lines.length) return -1;

    const progress =
      (musicStore.playProgress + (lyricStore.offsetMap[lyricStore.lyric.id] || 0)) * 1000 + 100;

    let index = -1;
    let low = 0;
    let high = lines.length - 1;
    while (low <= high) {
      const mid = (low + high) >>> 1;
      if (lines[mid].offset <= progress) {
        index = mid;
        low = mid + 1;
      } else {
        high = mid - 1;
      }
    }

    return index;
  });

  const syncAudio = () => {
    emitTo(WindowTarget.MiniPlayer, WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Audio,
      data: {
        isLoading: musicStore.isLoading,
        isPlaying: musicStore.isPlaying,
        music: musicStore.music,
      },
    });
  };

  const syncPlaylist = () => {
    emitTo(WindowTarget.MiniPlayer, WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Playlist,
      data: listStore.play.list,
    });
  };

  const syncLyric = () => {
    const { id, lines } = lyricStore.lyric;
    if (!id) return;

    emitTo(WindowTarget.MiniPlayer, WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Lyric,
      data: lines[currentIndex.value].words.map((item) => item.text).join(''),
    });
  };

  /** 启动桥接 */
  const start = async () => {
    const unwatchAudio = watch(
      [() => musicStore.isLoading, () => musicStore.isPlaying, () => musicStore.music],
      syncAudio,
    );
    const unwatchPlaylist = watch(() => listStore.play.list, syncPlaylist);
    const unwatchLyric = watch(currentIndex, syncLyric);

    const unlistenAction = await listen<{ type: MiniPlayerEmit; data: unknown }>(
      WindowEvent.MiniPlayer,
      async (e) => {
        switch (e.payload.type) {
          case MiniPlayerEmit.Init:
            syncAudio();
            syncPlaylist();
            syncLyric();
            break;
          case MiniPlayerEmit.Pos:
            const { x, y } = e.payload.data as PhysicalPosition;
            stateStore.setMiniPlayerPosition({ x, y });
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
