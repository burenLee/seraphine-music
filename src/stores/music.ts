import { useListStore } from './list'
import { useLyricStore } from './lyric'
import { notify } from '@/components/Notification'
import { Interval, PlayingMode, PlayingOrigin, PlayingQuality } from '@/utils/params'
import { getFullName, getPlayingOrigin, getRandomNumber, invoke } from '@/utils/tools'
import { Channel } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'

export const useMusicStore = defineStore(
  'music',
  () => {
    const isHydrated = ref(false) // store 持久化的水合状态

    const isLoading = ref(false) // 音频加载中
    const isLoaded = ref(false) // 音频已加载, 用于判断音频是否加载成功
    const isPlaying = ref(false) // 音频播放中
    const isDragging = ref(false) // 进度条拖动中

    const music = ref<PlayingMusic | null>(null)
    const origin = ref(PlayingOrigin.Local) // 来源
    const mode = ref(PlayingMode.OrderPlay) // 播放模式
    const volume = ref(100) // 音量
    const lastVolumn = ref(volume.value || 1) // 最后修改音量,用于静音恢复
    const quality = ref(PlayingQuality.Bitrate128) // 音质
    const playProgress = ref(0) // 播放进度 (s)
    const downloadProgress = ref(0) // 下载进度 (%)
    const retryCount = ref(0) // 重试次数

    const MAX_RETRY_COUNT = 3 // 最大重试次数

    let waitNextTimer: number | null = null
    let waitDownloadTimer: number | null = null
    let playChannel: Channel<number> | null = null
    let downloadChannel: Channel<number> | null = null

    const listStore = useListStore()
    const { load: loadLyric } = useLyricStore()

    // 水合完成时恢复播放状态
    watch(
      isHydrated,
      async () => {
        // 测试环境前端允许全局刷新, 但是后端不会刷新, 所以需要执行暂停
        if (import.meta.env.DEV) await pause()

        monitorDevice()
        monitorPlay()
        monitorDownload()

        if (volume.value !== 100) await setVolume(volume.value)
        if (music.value !== null) await load(music.value, origin.value)
        if (playProgress.value !== 0) await seek(playProgress.value)
      },
      { once: true }
    )

    // 设置播放音频
    const setMusic = async (
      newMusic: PlayingMusic | null,
      options?: { origin?: PlayingOrigin; loop?: boolean; autoPlay?: boolean }
    ) => {
      if (!newMusic) return stop()

      const { origin = PlayingOrigin.Local, loop = false, autoPlay = true } = options || {}
      if (newMusic.id === music.value?.id && !loop) return

      try {
        await stop()
        await load(newMusic, origin)
        if (autoPlay) await play()
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e)
        notify.error(msg)
      }
    }

    // 设置音量
    const setVolume = async (newVolume: number) => {
      if (newVolume < 0) newVolume = 0
      if (newVolume > 100) newVolume = 100

      await invoke('music_player_set_volume', { volume: newVolume })
      volume.value = newVolume

      if (newVolume > 0) lastVolumn.value = newVolume
    }

    // 设置播放模式
    const setMode = (newMode: PlayingMode) => {
      mode.value = newMode
    }

    // 设置播放音质
    const setQuality = (newQuality: PlayingQuality) => {
      quality.value = newQuality
    }

    // 加载音频
    const load = async (newMusic: PlayingMusic, newOrigin: PlayingOrigin) => {
      isLoading.value = true
      isLoaded.value = false

      music.value = newMusic
      origin.value = newOrigin

      switch (newOrigin) {
        case PlayingOrigin.Local:
          if (!newMusic.path) {
            isLoading.value = false
            startWaitNext()
            throw new Error(`无法播放: "${newMusic.title}"`)
          }

          await invoke('music_player_load_file', { path: newMusic.path }).then(
            () => (isLoaded.value = true)
          )
          break

        case PlayingOrigin.Online:
          if (!newMusic.hash) {
            isLoading.value = false
            startWaitNext()
            throw new Error(`无法播放: "${newMusic.title}", 即将切换下一首`)
          }

          const api_song_url = await invoke('api_song_url', {
            hash: newMusic.hash,
            quality: quality.value
          })
          if (api_song_url?.status !== 1 || !api_song_url.backupUrl?.length) {
            isLoading.value = false
            startWaitNext()
            throw new Error(`无法播放: "${newMusic.title}", 即将切换下一首`)
          }

          music.value.path = api_song_url.backupUrl[0]
          await invoke('music_player_load_url', {
            path: music.value.path,
            hash: newMusic.hash
          }).then(() => (isLoaded.value = true))
          break
      }

      const title = getFullName(newMusic)
      document.title = title
      getCurrentWindow().setTitle(title)

      loadLyric(music.value)
      retryCount.value = 0
      isLoading.value = false
    }

    // 播放音频
    const play = async () => {
      if (!music.value || isLoading.value || !isLoaded.value) return

      await invoke('music_player_play')
      isPlaying.value = true
    }

    // 暂停音频
    const pause = async () => {
      await invoke('music_player_pause')
      isPlaying.value = false
    }

    // 停止音频
    const stop = async () => {
      stopWaitNext()
      stopWaitDownload()

      await invoke('music_player_stop')
      isPlaying.value = false
      playProgress.value = 0
      downloadProgress.value = 0

      const title = 'Seraphine'
      document.title = title
      getCurrentWindow().setTitle(title)
    }

    // 跳转
    const seek = async (pos: number) => {
      await invoke('music_player_seek', { pos })
    }

    // 自动播放下一首
    const playAutoNext = () => {
      const lastIndex = listStore.play.list.length - 1
      // 处理播放列表为空的情况
      if (lastIndex === -1) {
        setMusic(music.value, { origin: origin.value, loop: true })
        return
      }

      // 把 music 不存在和不在当前列表中的情况统一处理
      let index = listStore.play.list.findIndex((m) => m.id === music.value?.id)
      let loop = false
      let autoPlay = true

      switch (mode.value) {
        case PlayingMode.OrderPlay:
          if (index === lastIndex) {
            loop = true
            autoPlay = false
          } else {
            index = index === -1 ? 0 : index + 1
          }
          break
        case PlayingMode.SinglePlay:
          loop = true
          autoPlay = false
          break
        case PlayingMode.OrderLoop:
          index = index === lastIndex || index === -1 ? 0 : index + 1
          break
        case PlayingMode.SingleLoop:
          loop = true
          break
        case PlayingMode.RandomPlay:
          index = getRandomNumber(lastIndex, index)
          break
      }

      const newMusic = listStore.play.list[index]
      setMusic(newMusic, { origin: getPlayingOrigin(newMusic), loop, autoPlay })
    }

    // 播放上/下一首
    const playPrevOrNext = (type: 'prev' | 'next') => {
      const lastIndex = listStore.play.list.length - 1
      if (lastIndex === -1) {
        setMusic(music.value, { origin: origin.value, loop: true })
        return
      }

      let index = 0
      // 如果是随机播放则随机选取, 其他则按顺序选取
      if (mode.value === PlayingMode.RandomPlay) {
        index = getRandomNumber(lastIndex, index)
      } else {
        index = listStore.play.list.findIndex((m) => m.id === music.value?.id)

        if (index === -1) {
          // 如果不存在则从列表开头播放
          index = 0
        } else {
          if (type === 'prev') {
            // 上一首
            index = index === 0 ? lastIndex : index - 1
          } else {
            // 下一首
            index = index === lastIndex ? 0 : index + 1
          }
        }
      }

      const newMusic = listStore.play.list[index]
      setMusic(newMusic, { origin: getPlayingOrigin(newMusic), loop: true })
    }

    const startWaitNext = () => {
      if (waitNextTimer !== null) clearTimeout(waitNextTimer)

      waitNextTimer = setTimeout(() => {
        if (++retryCount.value >= MAX_RETRY_COUNT) {
          notify.error('重试次数过多, 停止重试')
          retryCount.value = 0
          return
        }

        playPrevOrNext('next')
      }, Interval.PoN)
    }

    const stopWaitNext = () => {
      if (waitNextTimer === null) return

      clearTimeout(waitNextTimer)
      waitNextTimer = null
    }

    // 开始修改进度
    const startChangeProgress = () => {
      if (!music.value || isDragging.value) return

      stopWaitDownload()
      isDragging.value = true
    }

    // 结束修改进度
    const stopChangeProgress = async (pos: number) => {
      if (!music.value || !isDragging.value) return

      if (origin.value === PlayingOrigin.Online) {
        if (downloadProgress.value >= pos / music.value.duration) {
          await seek(pos)
          await play()
        } else {
          await startWaitDownload(pos)
        }
      } else {
        await seek(pos)
        await play()
      }

      isDragging.value = false
    }

    // 等待下载进度
    const startWaitDownload = async (pos: number) => {
      if (!music.value) return

      isLoading.value = true
      await pause()

      const targetProgress = pos / music.value.duration

      if (waitDownloadTimer !== null) clearInterval(waitDownloadTimer)
      waitDownloadTimer = setInterval(async () => {
        if (!music.value) {
          isLoading.value = false
          stopWaitDownload()
          return
        }
        if (downloadProgress.value < targetProgress) return

        isLoading.value = false
        stopWaitDownload()
        await seek(pos)
        await play()
      }, Interval.Long)
    }

    // 取消等待下载进度
    const stopWaitDownload = () => {
      if (waitDownloadTimer === null) return

      clearInterval(waitDownloadTimer)
      waitDownloadTimer = null
    }

    const monitorDevice = () => {
      listen('reload_device', async () => {
        const pg = playProgress.value
        await setMusic(music.value, { origin: origin.value, loop: true, autoPlay: isPlaying.value })
        await seek(pg)
      })
    }

    // 监听下载进度
    const monitorDownload = () => {
      if (downloadChannel !== null) return

      downloadChannel = new Channel<number>()
      downloadChannel.onmessage = (pg) => {
        if (!music.value) return

        downloadProgress.value = pg
      }

      return invoke('music_player_monitor_download', { channel: downloadChannel })
    }

    // 监听播放进度
    const monitorPlay = () => {
      if (playChannel !== null) return

      let last_pg = 0 // 用于检测是否越过阈值（防止重复触发）

      playChannel = new Channel<number>()
      playChannel.onmessage = (pg) => {
        if (!music.value || !isPlaying.value || isLoading.value) {
          last_pg = pg
          return
        }

        // 仅在从未到达阈值的位置“跨越”到阈值或更后的位置时触发一次
        if (pg + 0.5 >= music.value.duration && last_pg + 0.5 < music.value.duration) {
          playAutoNext()
        }

        playProgress.value = pg
        last_pg = pg
      }

      return invoke('music_player_monitor_play', { channel: playChannel })
    }

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
      stopChangeProgress
    }
  },
  {
    persist: {
      key: 'music-store',
      pick: ['music', 'origin', 'volume', 'mode', 'quality', 'playProgress'],
      afterHydrate: (ctx) => (ctx.store.isHydrated = true)
    }
  }
)
