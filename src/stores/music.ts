import { Channel } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { defineStore } from 'pinia';
import { ref, watch } from 'vue';

import { notify } from '@/components/Notification.vue';
import { getFullName, getOrigin } from '@/utils/music';
import {
  ApiInvokeStatus,
  Interval,
  PlayingMode,
  PlayingOrigin,
  PlayingQuality,
} from '@/utils/params';
import { genRandomNum, invoke, setAppTitle } from '@/utils/tools';

import { useListStore } from './list';
import { useLyricStore } from './lyric';

/** 播放音频配置 */
export const useMusicStore = defineStore(
  'music',
  () => {
    const isHydrated = ref(false); // store 持久化的水合状态
    const isLoading = ref(false); // 音频加载中
    const isLoaded = ref(false); // 音频已加载, 用于判断音频是否加载成功
    const isPlaying = ref(false); // 音频播放中
    const isDragging = ref(false); // 进度条拖动中
    const music = ref<PlayingMusicInfo>();
    const origin = ref(PlayingOrigin.Local); // 来源
    const mode = ref(PlayingMode.OrderPlay); // 播放模式
    const volume = ref(100); // 音量
    const lastVolumn = ref(volume.value || 1); // 最后修改音量,用于静音恢复
    const quality = ref(PlayingQuality.Bitrate128); // 音质
    const playProgress = ref(0); // 播放进度 (s)
    const downloadProgress = ref(0); // 下载进度 (%)
    const retryCount = ref(0); // 重试次数

    const MAX_RETRY_COUNT = 3; // 最大重试次数

    let tryNextTimer: ReturnType<typeof setTimeout> | undefined = undefined;
    let waitDownloadTimer: ReturnType<typeof setTimeout> | undefined = undefined;
    let playChannel: Channel<number> | undefined = undefined;
    let downloadChannel: Channel<number> | undefined = undefined;

    const listStore = useListStore();
    const { load: loadLyric } = useLyricStore();

    // 水合完成时恢复播放状态
    watch(
      isHydrated,
      async () => {
        // 测试环境前端允许全局刷新, 但是后端不会刷新, 所以需要执行暂停
        if (import.meta.env.DEV) await pause();

        await monitorDevice();
        await monitorPlay();
        await monitorDownload();

        if (volume.value !== 100) await setVolume(volume.value);
        if (music.value !== undefined) await load(music.value, origin.value);
        if (playProgress.value !== 0) await seek(playProgress.value);
      },
      { once: true },
    );

    /** 设置播放音频 */
    const setMusic = async (
      newMusic: PlayingMusicInfo | undefined,
      options?: { origin?: PlayingOrigin; loop?: boolean; autoPlay?: boolean },
    ) => {
      if (!newMusic) {
        music.value = undefined;
        stop();
        return;
      }

      const { origin = PlayingOrigin.Local, loop = false, autoPlay = true } = options || {};
      if (newMusic.id === music.value?.id && !loop) return;

      try {
        await stop();
        await load(newMusic, origin);
        if (autoPlay) await play();
      } catch {}
    };

    /** 设置音量 */
    const setVolume = async (newVolume: number) => {
      newVolume = Math.max(0, Math.min(100, newVolume));

      try {
        await invoke('music_player_set_volume', { volume: newVolume });
        volume.value = newVolume;
        if (newVolume > 0) lastVolumn.value = newVolume;
      } catch {
        notify.error('无法设置音量');
      }
    };

    /** 设置播放模式 */
    const setMode = (newMode: PlayingMode) => (mode.value = newMode);

    /** 设置播放音质 */
    const setQuality = (newQuality: PlayingQuality) => (quality.value = newQuality);

    /** 加载音频 */
    const load = async (newMusic: PlayingMusicInfo, newOrigin: PlayingOrigin) => {
      isLoading.value = true;
      isLoaded.value = false;

      music.value = newMusic;
      origin.value = newOrigin;

      try {
        const PLAY_ERROR = new Error();

        switch (origin.value) {
          case PlayingOrigin.Local:
            if (!music.value.path) throw PLAY_ERROR;

            await invoke('music_player_load_file', { path: music.value.path });
            break;
          case PlayingOrigin.Online:
            if (!music.value.hash) throw PLAY_ERROR;

            const { status, backupUrl } = await invoke('api_song_url', {
              hash: music.value.hash,
              quality: quality.value,
            });
            if (status !== ApiInvokeStatus.Success || !backupUrl?.length) {
              throw PLAY_ERROR;
            } else {
              music.value.path = backupUrl[0];
              await invoke('music_player_load_url', {
                url: music.value.path,
                hash: music.value.hash,
              });
            }

            break;
        }

        isLoaded.value = true;
        retryCount.value = 0;
      } catch {
        notify.warning(`无法播放：《${newMusic.title}》，自动切换下一首...`);

        startTryNext();
      } finally {
        isLoading.value = false;
        await setAppTitle(getFullName(newMusic));
        await loadLyric(music.value);
      }
    };

    /** 播放音频 */
    const play = async () => {
      if (!music.value || isLoading.value || !isLoaded.value) return;

      try {
        await invoke('music_player_play');
        isPlaying.value = true;
      } catch {
        notify.error('无法播放音频');
      }
    };

    /** 暂停音频 */
    const pause = async () => {
      try {
        await invoke('music_player_pause');
        isPlaying.value = false;
      } catch {
        notify.error('无法暂停音频');
      }
    };

    /** 停止音频 */
    const stop = async () => {
      stopTryNext();
      stopWaitDownload();

      try {
        await invoke('music_player_stop');
        isPlaying.value = false;
        playProgress.value = 0;
        downloadProgress.value = 0;

        setAppTitle('Seraphine');
      } catch {
        notify.error('无法停止音频');
      }
    };

    /** 跳转 */
    const seek = async (pos: number) => {
      try {
        await invoke('music_player_seek', { pos });
      } catch {
        notify.error('无法跳转');
      }
    };

    /** 自动播放下一首 */
    const playAutoNext = () => {
      const lastIndex = listStore.play.list.length - 1;
      // 处理播放列表为空的情况
      if (lastIndex === -1) {
        setMusic(music.value, { origin: origin.value, loop: true });
        return;
      }

      // 把 music 不存在和不在当前列表中的情况统一处理
      let index = listStore.play.list.findIndex((m) => m.id === music.value?.id);
      let loop = false;
      let autoPlay = true;

      switch (mode.value) {
        case PlayingMode.OrderPlay:
          if (index === lastIndex) {
            loop = true;
            autoPlay = false;
          } else {
            index = index === -1 ? 0 : index + 1;
          }
          break;
        case PlayingMode.SinglePlay:
          loop = true;
          autoPlay = false;
          break;
        case PlayingMode.OrderLoop:
          index = index === lastIndex || index === -1 ? 0 : index + 1;
          break;
        case PlayingMode.SingleLoop:
          loop = true;
          break;
        case PlayingMode.RandomPlay:
          index = genRandomNum(lastIndex, index);
          break;
      }

      const newMusic = listStore.play.list[index];
      setMusic(newMusic, { origin: getOrigin(newMusic), loop, autoPlay });
    };

    /** 播放上/下一首 */
    const playPrevOrNext = (type: 'prev' | 'next') => {
      const lastIndex = listStore.play.list.length - 1;
      if (lastIndex === -1) {
        setMusic(music.value, { origin: origin.value, loop: true });
        return;
      }

      let index = 0;
      // 如果是随机播放则随机选取, 其他则按顺序选取
      if (mode.value === PlayingMode.RandomPlay) {
        index = genRandomNum(lastIndex, index);
      } else {
        index = listStore.play.list.findIndex((m) => m.id === music.value?.id);

        if (index === -1) {
          // 如果不存在则从列表开头播放
          index = 0;
        } else {
          if (type === 'prev') {
            // 上一首
            index = index === 0 ? lastIndex : index - 1;
          } else {
            // 下一首
            index = index === lastIndex ? 0 : index + 1;
          }
        }
      }

      const newMusic = listStore.play.list[index];
      setMusic(newMusic, { origin: getOrigin(newMusic), loop: true });
    };

    /** 尝试加载下一首 */
    const startTryNext = () => {
      if (tryNextTimer !== undefined) clearTimeout(tryNextTimer);

      tryNextTimer = setTimeout(() => {
        if (++retryCount.value >= MAX_RETRY_COUNT) {
          notify.error('重试次数过多, 停止重试');
          retryCount.value = 0;
          return;
        }

        playPrevOrNext('next');
      }, Interval.PoN);
    };

    /** 停止尝试加载下一首 */
    const stopTryNext = () => {
      if (tryNextTimer === undefined) return;

      clearTimeout(tryNextTimer);
      tryNextTimer = undefined;
    };

    /** 开始修改进度 */
    const startChangeProgress = () => {
      if (isDragging.value || !music.value) return;

      isDragging.value = true;
      stopWaitDownload();
    };

    /** 结束修改进度 */
    const stopChangeProgress = async (pos: number) => {
      if (!isDragging.value || !music.value) return;

      if (origin.value === PlayingOrigin.Online) {
        if (downloadProgress.value >= pos / music.value.duration) {
          await seek(pos);
          await play();
        } else {
          await startWaitDownload(pos);
        }
      } else {
        await seek(pos);
        await play();
      }

      isDragging.value = false;
    };

    /** 等待下载 */
    const startWaitDownload = async (pos: number) => {
      if (!music.value) return;

      isLoading.value = true;
      await pause();

      const targetProgress = pos / music.value.duration;

      if (waitDownloadTimer !== undefined) clearInterval(waitDownloadTimer);
      waitDownloadTimer = setInterval(async () => {
        if (!music.value) {
          isLoading.value = false;
          stopWaitDownload();
          return;
        }
        if (downloadProgress.value < targetProgress) return;

        isLoading.value = false;
        stopWaitDownload();
        await seek(pos);
        await play();
      }, Interval.Long);
    };

    /** 取消等待下载 */
    const stopWaitDownload = () => {
      if (waitDownloadTimer === undefined) return;

      clearInterval(waitDownloadTimer);
      waitDownloadTimer = undefined;
    };

    const monitorDevice = async () => {
      try {
        await listen('music:reload_device', async () => {
          const lastProgress = playProgress.value;
          await setMusic(music.value, {
            origin: origin.value,
            loop: true,
            autoPlay: isPlaying.value,
          });
          await seek(lastProgress);
        });
      } catch {}
    };

    /** 监听下载进度 */
    const monitorDownload = async () => {
      if (downloadChannel !== undefined) return;

      try {
        downloadChannel = new Channel<number>();
        downloadChannel.onmessage = (pg) => {
          if (!music.value) return;

          downloadProgress.value = pg;
        };

        await invoke('music_player_monitor_download', { channel: downloadChannel });
      } catch {
        notify.error('无法获取下载进度');
      }
    };

    /** 监听播放进度 */
    const monitorPlay = async () => {
      if (playChannel !== undefined) return;

      try {
        let lastProgress = 0; // 用于检测是否越过阈值（防止重复触发）

        playChannel = new Channel<number>();
        playChannel.onmessage = (pg) => {
          if (!music.value || !isPlaying.value || isLoading.value) {
            lastProgress = pg;
            return;
          }

          // 仅在从未到达阈值的位置“跨越”到阈值或更后的位置时触发一次
          if (pg + 0.3 >= music.value.duration && lastProgress + 0.3 < music.value.duration) {
            playAutoNext();
          }

          playProgress.value = pg;
          lastProgress = pg;
        };

        await invoke('music_player_monitor_play', { channel: playChannel });
      } catch {
        notify.error('无法获取播放进度');
      }
    };

    return {
      isHydrated,

      isLoading,
      isLoaded,
      isPlaying,
      music,
      origin,
      volume,
      lastVolumn,
      mode,
      quality,
      isDragging,
      playProgress,
      downloadProgress,

      setMusic,
      setVolume,
      setMode,
      setQuality,
      play,
      pause,
      stop,
      seek,
      playPrevOrNext,
      startChangeProgress,
      stopChangeProgress,
    };
  },
  {
    persist: {
      key: 'music-store',
      pick: ['music', 'origin', 'volume', 'mode', 'quality', 'playProgress'],
      afterHydrate: (ctx) => (ctx.store.isHydrated = true),
    },
  },
);
