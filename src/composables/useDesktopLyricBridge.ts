import { useMainLyricStore } from '@/stores/lyric-main'
import { useMusicStore } from '@/stores/music'
import {
  Interval,
  LyricEmitType,
  LyricPushPayload,
  WindowEvent,
  WindowName
} from '@/utils/params'
import { getFullName } from '@/utils/tools'
import { emitTo, listen } from '@tauri-apps/api/event'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { watchThrottled } from '@vueuse/core'

const FORWARD_DURATION = 100

export function useDesktopLyricBridge(onClose: () => void) {
  const musicStore = useMusicStore()
  const lyricStore = useMainLyricStore()

  const stopFns: (() => void)[] = []
  let unlistenAction: (() => void) | undefined
  const availableFonts = ref<[FontName, FontValue][]>([])

  /** 桌面歌词窗口是否打开 */
  const isLyricOpen = () =>
    WebviewWindow.getByLabel(WindowName.DesktopLyric) !== null

  /** 构建推送载荷 */
  const buildPushPayload = (): LyricPushPayload => {
    const lines = lyricStore.lyric?.lines || []
    const progress = (musicStore.playProgress + lyricStore.offset) * 1000 + FORWARD_DURATION

    let currentIndex = -1
    for (let i = 0; i < lines.length; i++) {
      const start = lines[i].offset
      const end = lines[i + 1]?.offset || Infinity
      if (progress >= start && progress < end) {
        currentIndex = i
        break
      }
    }

    return {
      lines,
      currentIndex,
      isPlaying: musicStore.isPlaying,
      songTitle: musicStore.music ? getFullName(musicStore.music) : 'Seraphine',
      transMode: lyricStore.setting.transMode,
      availableFonts: availableFonts.value
    }
  }

  /** 推送当前歌词状态 */
  const pushState = () => {
    if (!isLyricOpen()) return
    emitTo(WindowName.DesktopLyric, WindowEvent.DesktopLyric, {
      type: LyricEmitType.PushState,
      data: buildPushPayload()
    })
  }

  /** 响应子窗口请求 */
  const sendLyric = () => {
    if (!isLyricOpen()) return
    emitTo(WindowName.DesktopLyric, WindowEvent.DesktopLyric, {
      type: LyricEmitType.SendLyric,
      data: buildPushPayload()
    })
  }

  /** 启动桥接 */
  const start = async () => {
    // 监听音频/歌词变化（节流）
    stopFns.push(
      watchThrottled(
        [() => musicStore.playProgress, () => lyricStore.lyric, () => lyricStore.offset],
        pushState,
        { throttle: Interval.Long }
      )
    )

    // 监听播放状态变化
    stopFns.push(watch(() => musicStore.isPlaying, pushState))

    // 监听翻译模式变化
    stopFns.push(watch(() => lyricStore.setting.transMode, pushState))

    // 监听桌面歌词窗口操作
    unlistenAction = await listen<{
      type: LyricEmitType
      fontFamily?: string
      textColor?: string
    }>(WindowEvent.DesktopLyric, (e) => {
      switch (e.payload.type) {
        case LyricEmitType.GetLyric:
          sendLyric()
          break

        case LyricEmitType.Play:
          musicStore.play()
          break
        case LyricEmitType.Pause:
          musicStore.pause()
          break
        case LyricEmitType.Prev:
          musicStore.playPrevOrNext('prev')
          break
        case LyricEmitType.Next:
          musicStore.playPrevOrNext('next')
          break

        case LyricEmitType.OffsetForward:
          lyricStore.setOffsetMap('add')
          break
        case LyricEmitType.OffsetBackward:
          lyricStore.setOffsetMap('sub')
          break

        case LyricEmitType.Main:
          getCurrentWindow().show()
          break

        case LyricEmitType.Close:
          stop()
          onClose()
          break
      }
    })
  }

  /** 停止桥接 */
  const stop = () => {
    stopFns.forEach((fn) => fn())
    stopFns.length = 0
    unlistenAction?.()
    unlistenAction = undefined
  }

  /** 立即推送当前状态（窗口创建后的初始推送） */
  const pushNow = () => pushState()

  return { start, stop, pushNow }
}
