import { LyricTransMode, PlayingOrigin, PlayingQuality } from './params'
import { notify } from '@/components/Notification'
import { invoke as tauriInvoke } from '@tauri-apps/api/core'
import { ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

/**
 * 合并 class 样式
 * @param inputs 所有的 class 样式
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

/**
 * 判断字符串是否为英文
 * @param text 待判断的字符串
 */
export function isEnglishText(text: string) {
  return /^[a-zA-Z\s\-_',.!?;:()"\[\]]+$/.test(text.trim())
}

/**
 * 将 `秒` 转为 `分钟：秒` 格式
 * @param s 秒数
 */
export function formatDuration(s: number) {
  if (s <= 0) return '00:00'

  const mins = Math.floor(s / 60).toString()
  const secs = Math.floor(s % 60).toString()

  return `${mins.padStart(2, '0')}:${secs.padStart(2, '0')}`
}

// 文件大小单位 (使用 ReadonlyArray 防止意外修改)
const SIZE_UNITS = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'] as const
const LOG_1024 = Math.log(1024)

/**
 * 格式化文件大小
 * @param bytes 文件大小（字节）
 * @param decimals 保留的小数位数，默认为 2
 */
export function formatFileSize(bytes: number, decimals: number = 2) {
  if (!Number.isFinite(bytes) || bytes <= 0) return '0 B'

  const safeDecimals = Math.max(0, decimals)
  const unitIndex = Math.min(Math.floor(Math.log(bytes) / LOG_1024), SIZE_UNITS.length - 1)
  const size = bytes / Math.pow(1024, unitIndex)

  return `${size.toFixed(safeDecimals)} ${SIZE_UNITS[unitIndex]}`
}

/**
 * 获取 `[0, maxNum]` 内且与 `srcNum` 不同的随机整数
 * @param maxNum 最大值 (包含)
 * @param srcNum 来源值，需要避免的值
 */
export function getRandomNumber(maxNum: number, srcNum: number) {
  if (!Number.isInteger(maxNum) || maxNum < 0) {
    throw new Error('maxNum 必须是非负整数')
  }

  if (maxNum === 0) return 0
  if (maxNum === 1) return srcNum === 0 ? 1 : 0

  let random = Math.floor(Math.random() * (maxNum + 1))

  // 如果随机数等于 srcNum，则循环到下一个值
  while (random === srcNum) random = (random + 1) % (maxNum + 1)

  return random
}

const picSizes = { sm: '64', md: '120', lg: '400' } as const

/**
 * 针对 kg 的图片获取其指定尺寸的 URL
 * @param url 图片 URL
 * @param size 图片大小, 默认 sm
 */
export function getPic(url: string, size: keyof typeof picSizes = 'sm') {
  return url.replace('{size}', picSizes[size])
}

/**
 * 获取音频的 `名称 + 歌手` 信息
 * @param music 音频
 * @param mode 排列模式, "ta": `title - artist`, "at": `artist - title`
 */
export function getFullName(music: PlayingMusic | ListMusic, mode: 'ta' | 'at' = 'ta') {
  return music.artist
    ? mode === 'ta'
      ? `${music.title} - ${music.artist}`
      : `${music.artist} - ${music.title}`
    : music.title
}

/** 获取音频的权限标签 */
export function getPrivilegeTags(privilege: number, payType: number) {
  const tags = []

  if (privilege === 10) {
    if (payType === 2) {
      tags.push('付费')
    } else if (payType === 3) {
      tags.push('VIP')
    }
  }

  return tags
}

/**
 * 获取音频的来源
 * @param music 音频
 */
export function getPlayingOrigin(music: ListMusic) {
  return music.hash ? PlayingOrigin.Online : PlayingOrigin.Local
}

/**
 * 获取音质
 */
export function getQuality({ audio_bitrate, bit_depth, sample_rate }: MusicDetail) {
  if (!audio_bitrate) return PlayingQuality.Bitrate128

  // 无损 FLAC - 比特率 > 500kbps + 位深度 >= 16bit + 采样率 >= 44.1kHz
  if (audio_bitrate > 500) {
    return bit_depth && bit_depth >= 16 && sample_rate && sample_rate >= 44100
      ? PlayingQuality.BitrateFlac
      : PlayingQuality.BitrateHigh
  }

  // 高音质 (high) - 高质量有损压缩
  if (audio_bitrate >= 256) return PlayingQuality.BitrateHigh

  // 320kbps
  if (audio_bitrate >= 192) return PlayingQuality.Bitrate320

  // 128kbps
  return PlayingQuality.Bitrate128
}

/** 解析 KRC 歌词内容 */
export function parseKrcLyric(content: string) {
  if (!content) return []

  // 译制歌词信息
  const translations: Translation = {
    [LyricTransMode.Roman]: [],
    [LyricTransMode.Trans]: []
  }

  let matchIndex = 0 // 歌词匹配索引
  const lines: LyricLine[] = []

  for (const line of content.split('\n')) {
    const languageMatch = line.match(/\[language:([^\]]*)\]/) // 匹配 [language:xx]
    if (languageMatch) {
      // 处理翻译歌词
      try {
        const lyricLanguageList: LyricLanguageList = JSON.parse(atob(languageMatch[1]))

        lyricLanguageList.content.forEach((item) => {
          const transContent = item.lyricContent.map((content) => content.join(''))

          if (item.type === 0) translations[LyricTransMode.Roman] = transContent
          else if (item.type === 1) translations[LyricTransMode.Trans] = transContent
        })
      } catch (error) {
        console.error(error)
      }
    }

    // 处理原文歌词
    const lineMatch = line.match(/\[(\d+),(\d+)\](.*)/) // 匹配 中括号[]
    if (!lineMatch) continue

    const offset = parseInt(lineMatch[1]) // line 偏移量
    const duration = parseInt(lineMatch[2]) // line 时长
    const wordsPart = lineMatch[3] // word 部分

    const words: LyricWord[] = [] // word 列表
    const wordReg = /<(\d+),(\d+),(\d+)>([^<]*)/g // 匹配 尖括号<>
    let wordMatch = null

    while ((wordMatch = wordReg.exec(wordsPart)) !== null) {
      const wordOffset = parseInt(wordMatch[1]) // word 偏移量
      const wordDuration = parseInt(wordMatch[2]) // word 时长
      const wordText = wordMatch[4] //  word 文本

      words.push({ offset: offset + wordOffset, duration: wordDuration, text: wordText })
    }

    lines.push({
      offset,
      duration,
      words,
      translations: {
        [LyricTransMode.Roman]: translations[LyricTransMode.Roman][matchIndex],
        [LyricTransMode.Trans]: translations[LyricTransMode.Trans][matchIndex++]
      }
    })
  }

  return lines
}

/** 解析 lrc 歌词 */
export function parseLrcLyric(content: string) {
  if (!content) return []

  const lines: LyricLine[] = []

  for (const line of content.split('\n')) {
    const match = line.match(/\[(\d+):(\d+)\.(\d+)\](.*)/)
    if (!match) continue

    const offset = parseInt(match[1]) * 60 * 1000 + parseInt(match[2]) * 1000 + parseInt(match[3])
    const text = match[4]
    if (!text) continue

    lines.push({
      offset,
      duration: 0,
      words: [{ offset, duration: 0, text }],
      translations: {
        [LyricTransMode.Roman]: '',
        [LyricTransMode.Trans]: ''
      }
    })
  }

  return lines
}

type InvokeKey = keyof Invoke
type InvokeParams<K extends InvokeKey> = Invoke[K]['params']
type InvokeReturn<K extends InvokeKey> = Invoke[K]['return']

/**
 * 封装 Tauri 的 invoke 函数
 * @param key 函数名
 * @param params 参数
 */
export async function invoke<K extends InvokeKey>(
  key: K,
  params?: InvokeParams<K>
): Promise<InvokeReturn<K> | undefined> {
  try {
    const res = await tauriInvoke<InvokeReturn<K>>(key, params)
    return res
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e)
    notify.error(msg)

    // throw Error(msg)
  }
}
