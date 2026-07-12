import { useMusicStore } from './music'
import { notify } from '@/components/Notification'
import { AutoStartMode, CloseStatus, DefaultSystemFonts, ShortcutKey } from '@/utils/params'
import {
  ShortcutEvent,
  isRegistered,
  register,
  unregister,
  unregisterAll
} from '@tauri-apps/plugin-global-shortcut'

const Default_Shortcut: Record<ShortcutKey, string> = {
  playOrPause: 'Alt+F5',
  addVolumn: 'Alt+Up',
  subVolumn: 'Alt+Down',
  mute: 'Alt+S',
  prev: 'Alt+Left',
  next: 'Alt+Right',
  backward: 'Ctrl+Alt+Left',
  forward: 'Ctrl+Alt+Right'
}

export const useSettingStore = defineStore(
  'setting',
  () => {
    const isHydrated = ref(false) // store 持久化的水合状态
    const isMaximized = ref(false) // 最大化
    const isFullscreen = ref(false) // 全屏

    const autoLiteVipState = ref(false) // 自动领取概念版会员
    const availableFonts = ref<[FontName, FontValue][]>([]) // 可用字体列表
    const fontFamily = ref<FontValue>('system-ui') // 字体
    const autoStartState = ref(false) // 开机自启状态
    const autoStartMode = ref(AutoStartMode.Foreground) // 开机自启模式
    const closeStatus = ref<CloseStatus>() // 关闭按钮状态
    const globalShortcutState = ref(true) // 全局快捷键状态
    const mediaShortcutState = ref(true) // 媒体快捷键状态
    const shortcutMap = ref({ ...Default_Shortcut }) // 快捷键映射
    const device = ref('') // 设备ID
    const version = ref('0.1.0') // 应用版本

    const musicStore = useMusicStore()

    watch(
      isHydrated,
      async () => {
        await unregisterAll()

        registerAllGlobalShortcut()
        registerMediaShortcut()
      },
      { once: true }
    )

    const toggleAutoLiteVipState = () => (autoLiteVipState.value = !autoLiteVipState.value)
    const toggleMaximizedState = (state: boolean) => (isMaximized.value = state)
    const toggleFullscreenState = (state: boolean) => (isFullscreen.value = state)
    const getAvailableFonts = () => {
      const canvas = document.createElement('canvas')
      const context = canvas.getContext('2d')
      if (!context) return

      const testString = '天地玄黄 AWayonmw01 宇宙洪荒'
      const baseFont = 'monospace'
      const fontSize = 100

      context.font = `${fontSize}px ${baseFont}`
      const baseWidth = context.measureText(testString).width
      const fonts: [FontName, FontValue][] = []

      DefaultSystemFonts.forEach(([name, value]) => {
        context.font = `${fontSize}px "${value}", ${baseFont}`
        const testWidth = context.measureText(testString).width

        if (Math.abs(baseWidth - testWidth) > 0.1) fonts.push([name, value])
      })

      availableFonts.value = fonts
    }
    const setFontFamily = (font: FontValue) => (fontFamily.value = font)
    const toggleAutoStartState = (state: boolean) => (autoStartState.value = state)
    const setAutoStartMode = (mode: AutoStartMode) => (autoStartMode.value = mode)
    const setCloseStatus = (status: CloseStatus) => (closeStatus.value = status)
    const toggleGlobalShortcutState = () => {
      globalShortcutState.value = !globalShortcutState.value

      if (globalShortcutState.value) {
        registerAllGlobalShortcut()
      } else {
        unregisterAllGlobalShortcut()
      }
    }
    const toggleMediaShortcutState = () => {
      mediaShortcutState.value = !mediaShortcutState.value

      if (mediaShortcutState.value) {
        registerMediaShortcut()
      } else {
        unregisterMediaShortcut()
      }
    }
    const setDevice = (id: string) => (device.value = id)

    const setShortcutMap = (key: ShortcutKey, value: string) => (shortcutMap.value[key] = value)
    const resetShortcutMap = () => (shortcutMap.value = { ...Default_Shortcut })

    // 注册全局快捷键
    const registerGlobalShortcut = async (type: ShortcutKey) => {
      const shortcut = shortcutMap.value[type]
      if (!shortcut) return

      const parts = shortcut.split('+').map((p) => p.trim())
      const modifierKeys = ['Ctrl', 'Shift', 'Alt', 'Meta']
      const mainKeys = parts.filter((p) => !modifierKeys.includes(p))

      if (mainKeys.length !== 1) {
        setShortcutMap(type, '')
        return
      }

      if (await isRegistered(shortcut)) return

      let registerFn: (e: ShortcutEvent) => void = () => {}

      switch (type) {
        case 'playOrPause':
          registerFn = (e) => {
            if (e.state === 'Released') return

            if (musicStore.isPlaying) {
              musicStore.pause()
            } else {
              musicStore.play()
            }
          }
          break
        case 'addVolumn':
          registerFn = (e) => {
            if (e.state === 'Released') return
            musicStore.setVolume(musicStore.volume + 5)
          }
          break
        case 'subVolumn':
          registerFn = (e) => {
            if (e.state === 'Released') return
            musicStore.setVolume(musicStore.volume - 5)
          }
          break
        case 'mute':
          registerFn = (e) => {
            if (e.state === 'Released') return

            musicStore.setVolume(musicStore.volume ? 0 : musicStore.lastVolumn)
          }
          break
        case 'prev':
          registerFn = (e) => {
            if (e.state === 'Released') return
            musicStore.playPrevOrNext('prev')
          }
          break
        case 'next':
          registerFn = (e) => {
            if (e.state === 'Released') return
            musicStore.playPrevOrNext('next')
          }
          break
        case 'forward':
          registerFn = (e) => {
            if (e.state === 'Released') return
            musicStore.seek(musicStore.playProgress + 5)
          }
          break
        case 'backward':
          registerFn = (e) => {
            if (e.state === 'Released') return
            musicStore.seek(musicStore.playProgress - 5)
          }
          break
      }

      try {
        await register(shortcut, registerFn)
      } catch (e) {
        console.error(e)

        mediaShortcutState.value = false
        notify.error('注册失败, 可能被占用')
      }
    }

    // 注销全局快捷键
    const unregisterGlobalShortcut = async (type: ShortcutKey) => {
      const shortcut = shortcutMap.value[type]
      if (!shortcut) return

      await unregister(shortcut)
    }

    const registerAllGlobalShortcut = async () => {
      for (const key of Object.keys(shortcutMap.value)) {
        await registerGlobalShortcut(key as ShortcutKey)
      }
    }

    const unregisterAllGlobalShortcut = async () => {
      for (const key of Object.keys(shortcutMap.value)) {
        await unregisterGlobalShortcut(key as ShortcutKey)
      }
    }

    const registerMediaShortcut = async () => {
      if (!mediaShortcutState.value) return

      try {
        await register('MediaPlayPause', (e) => {
          if (e.state === 'Released') return

          if (musicStore.isPlaying) {
            musicStore.pause()
          } else {
            musicStore.play()
          }
        })
        await register('MediaTrackNext', (e) => {
          if (e.state === 'Released') return

          musicStore.playPrevOrNext('next')
        })
        await register('MediaTrackPrevious', (e) => {
          if (e.state === 'Released') return

          musicStore.playPrevOrNext('prev')
        })
      } catch (e) {
        console.error(e)

        mediaShortcutState.value = false
        notify.error('媒体快捷键启用失败, 可能被占用')
      }
    }

    const unregisterMediaShortcut = () => {
      if (mediaShortcutState.value) return

      unregister('MediaPlayPause')
      unregister('MediaTrackNext')
      unregister('MediaTrackPrevious')
    }

    return {
      isHydrated,
      isMaximized,
      isFullscreen,
      autoLiteVipState,
      availableFonts,
      fontFamily,
      autoStartState,
      autoStartMode,
      closeStatus,
      globalShortcutState,
      mediaShortcutState,
      shortcutMap,
      device,
      version,

      toggleAutoLiteVipState,
      toggleMaximizedState,
      toggleFullscreenState,
      getAvailableFonts,
      setFontFamily,
      toggleAutoStartState,
      setAutoStartMode,
      setCloseStatus,
      toggleGlobalShortcutState,
      toggleMediaShortcutState,
      setShortcutMap,
      resetShortcutMap,
      setDevice,
      registerGlobalShortcut,
      unregisterGlobalShortcut,
      registerAllGlobalShortcut,
      unregisterAllGlobalShortcut
    }
  },
  {
    persist: {
      key: 'setting-store',
      pick: [
        'autoLiteVipState',
        'fontFamily',
        'autoStartState',
        'autoStartMode',
        'closeStatus',
        'globalShortcutState',
        'mediaShortcutState',
        'shortcutMap',
        'device',
        'version'
      ],
      afterHydrate: (ctx) => (ctx.store.isHydrated = true)
    }
  }
)
