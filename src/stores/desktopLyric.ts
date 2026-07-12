import { LyricTextColor } from '@/utils/params'

export const useDesktopLyricStore = defineStore(
  'desktopLyric',
  () => {
    const isLocked = ref(false) // 是否锁定
    const fontFamily = ref<FontValue>('system-ui') // 字体类型
    const fontSize = ref(32) // 字体大小
    const textColor = ref<LyricTextColor | string>(LyricTextColor.Blue) // 文本颜色
    const offsetMap = ref<Record<ID, number>>({}) // 进度偏移量列表(s)

    const toggleLock = (newIsLocked: boolean) => (isLocked.value = newIsLocked)
    const setFontFamily = (newFontFamily: FontValue) => (fontFamily.value = newFontFamily)
    const setFontSize = (newFontSize: number) => (fontSize.value = newFontSize)
    const setTextColor = (newTextColor: string) => (textColor.value = newTextColor)
    const setOffsetMap = (id: ID, step: number) => (offsetMap.value[id] = step)

    return {
      isLocked,
      fontSize,
      fontFamily,
      textColor,
      offsetMap,

      toggleLock,
      setFontSize,
      setFontFamily,
      setTextColor,
      setOffsetMap
    }
  },
  {
    persist: {
      key: 'desktop-lyric-store',
      pick: ['isLocked', 'fontSize', 'fontFamily', 'textColor', 'offsetMap']
    }
  }
)
