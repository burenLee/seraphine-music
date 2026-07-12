import {
  LyricFontSize,
  LyricFormat,
  LyricOffset,
  LyricPageMode,
  LyricTextAlign,
  LyricTextColor,
  LyricTransMode
} from '@/utils/params'
import { getFullName, invoke, parseKrcLyric, parseLrcLyric } from '@/utils/tools'

type MatchedMap = Record<ID, { id: string; fmt: LyricFormat }>
type OffsetMap = Record<ID, number>

export const useLyricStore = defineStore(
  'lyric',
  () => {
    const pageVisible = ref(false) // 歌词页可见性
    const pageMode = ref(LyricPageMode.Cover) // 歌词页背景模式

    const isLoading = ref(false)
    const lyric = ref<LyricInfo>()
    const setting = ref<LyricSetting>({
      fontFamily: 'system-ui',
      fontSize: LyricFontSize.Default,
      textColor: LyricTextColor.Blue,
      textAlign: LyricTextAlign.Left,
      transMode: LyricTransMode.Off
    })
    const matchedMap = ref<MatchedMap>({}) // 歌词匹配列表, 记录歌曲使用的歌词id
    const offsetMap = ref<OffsetMap>({}) // 歌词偏移量列表

    // 当前歌词的偏移量
    const offset = computed(() => (lyric.value ? offsetMap.value[lyric.value.id] || 0 : 0))

    const togglePageVisible = () => (pageVisible.value = !pageVisible.value)
    const setPageMode = (newMode: LyricPageMode) => (pageMode.value = newMode)
    const setLyric = (newLyric: LyricInfo | undefined) => (lyric.value = newLyric)
    const setFontFamily = (newFontFamily: FontValue) => (setting.value.fontFamily = newFontFamily)
    const setFontSize = (mode: 'add' | 'sub' | 'restart') => {
      let fontSize = setting.value.fontSize

      switch (mode) {
        case 'add':
          fontSize += LyricFontSize.Step
          break
        case 'sub':
          fontSize -= LyricFontSize.Step
          break
        case 'restart':
          fontSize = LyricFontSize.Default
          break
      }

      setting.value.fontSize = fontSize
    }
    const setTextColor = (newColor: string) => (setting.value.textColor = newColor)
    const setTextAlign = (newTextAlign: LyricTextAlign) => (setting.value.textAlign = newTextAlign)
    const setTransMode = (newTransMode: LyricTransMode) => (setting.value.transMode = newTransMode)
    const setMatchedLyric = (musicId: ID, lyricId: string, fmt: LyricFormat) =>
      (matchedMap.value[musicId] = { id: lyricId, fmt })
    const setOffsetMap = (mode: 'add' | 'sub' | 'restart') => {
      if (!lyric.value) return

      let offset = offsetMap.value[lyric.value.id] || LyricOffset.Default

      switch (mode) {
        case 'add':
          offset += LyricOffset.Step
          break
        case 'sub':
          offset -= LyricOffset.Step
          break
        case 'restart':
          offset = LyricOffset.Default
          break
      }

      offsetMap.value[lyric.value.id] = offset
    }

    const load = async (music: PlayingMusic, lyric?: LyricCandidate) => {
      isLoading.value = true

      let lyricGet = await getLocalLyric(music, lyric)
      if (!lyricGet) lyricGet = await getOnlineLyric(music, lyric)

      if (lyricGet) {
        let lyricLines: LyricLine[] = []

        switch (lyricGet.fmt) {
          case LyricFormat.Krc:
            lyricLines = parseKrcLyric(lyricGet.content)
            break
          case LyricFormat.Lrc:
            lyricLines = parseLrcLyric(lyricGet.content)
            break
        }

        setLyric({ id: lyricGet.id, fmt: lyricGet.fmt, lines: lyricLines })
        setMatchedLyric(music.id, lyricGet.id, lyricGet.fmt)
      } else {
        setLyric(undefined)
      }

      isLoading.value = false
    }

    // 获取本地歌词
    const getLocalLyric = (music: PlayingMusic, lyric?: LyricCandidate) => {
      let id = ''
      let fmt = LyricFormat.Krc

      if (lyric) {
        id = lyric.id
      } else {
        const matchedLyric = matchedMap.value[music.id]
        if (!matchedLyric) return

        id = matchedLyric.id
        fmt = matchedLyric.fmt
      }

      return invoke('music_lyric_get', { name: getFullName(music), id, fmt })
    }

    // 获取网络歌词
    const getOnlineLyric = async (music: PlayingMusic, lyric?: LyricCandidate) => {
      if (!lyric) {
        // 搜索歌词列表
        const lyric_search = await invoke('api_lyric_search', {
          keyword: getFullName(music, 'at'),
          hash: music.hash
        })
        if (lyric_search?.status !== 200) return

        // 默认选择官方推荐, 其次评分最高的(即第一个)
        lyric =
          lyric_search.candidates.find((item) => item.product_from === '官方推荐歌词') ||
          lyric_search.candidates[0]

        if (!lyric) return
      }

      return invoke('api_lyric_get', {
        name: getFullName(music),
        id: lyric.id,
        accesskey: lyric.accesskey
      })
    }

    return {
      pageVisible,
      pageMode,
      isLoading,
      lyric,
      setting,
      matchedMap,
      offsetMap,
      offset,

      togglePageVisible,
      setPageMode,
      setLyric,
      setFontFamily,
      setFontSize,
      setTextColor,
      setTextAlign,
      setTransMode,
      setMatchedLyric,
      setOffsetMap,
      load
    }
  },
  {
    persist: {
      key: 'lyric-store',
      pick: ['pageMode', 'setting', 'matchedMap', 'offsetMap']
    }
  }
)
