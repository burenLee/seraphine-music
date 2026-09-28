import { notify } from '@/components/Notification.vue';

import { LyricTransMode, PicSize, PlayingOrigin, PlayingQuality } from './params';

/**
 * 针对 酷狗 的图片获取其指定尺寸的 URL
 * @param url 图片 URL
 * @param size 图片大小, 默认 sm
 */
export function getPic(url: string, size = PicSize.Sm) {
  return url.replace('{size}', size);
}

/**
 * 获取音频标题
 * @param music 音频
 * @param mode 排列模式, "ta": `title - artist`, "at": `artist - title`
 */
export function getFullName(music: PlayingMusicInfo | MusicInfo, mode: 'ta' | 'at' = 'ta') {
  return music.artist
    ? mode === 'ta'
      ? `${music.title} - ${music.artist}`
      : `${music.artist} - ${music.title}`
    : music.title;
}

/** 获取音频权限标签 */
export function getPrivilegeTags(privilege: number, payType: number) {
  const tags = [];

  if (privilege === 10) {
    if (payType === 2) {
      tags.push('付费');
    } else if (payType === 3) {
      tags.push('VIP');
    }
  }

  return tags;
}

/** 获取音频来源 */
export function getOrigin(music: MusicInfo) {
  return music.hash ? PlayingOrigin.Online : PlayingOrigin.Local;
}

/** 获取音频音质 */
export function getQuality({ audio_bitrate, bit_depth, sample_rate }: MusicDetail) {
  if (!audio_bitrate) return PlayingQuality.Bitrate128;

  // 无损 FLAC - 比特率 > 500kbps + 位深度 >= 16bit + 采样率 >= 44.1kHz
  if (audio_bitrate > 500) {
    return bit_depth && bit_depth >= 16 && sample_rate && sample_rate >= 44100
      ? PlayingQuality.BitrateFlac
      : PlayingQuality.BitrateHigh;
  }

  // 高音质 (high) - 高质量有损压缩
  if (audio_bitrate >= 256) return PlayingQuality.BitrateHigh;

  // 320kbps
  if (audio_bitrate >= 192) return PlayingQuality.Bitrate320;

  // 128kbps
  return PlayingQuality.Bitrate128;
}

/** 解析 KRC 歌词内容 */
export function parseKrcLyric(content: string) {
  if (!content) return [];

  // 译制歌词信息
  const translations: Translation = {
    [LyricTransMode.Roman]: [],
    [LyricTransMode.Trans]: [],
  };

  let matchIndex = 0; // 歌词匹配索引
  const lines: LyricLine[] = [];

  for (const line of content.split('\n')) {
    const languageMatch = line.match(/\[language:([^\]]*)\]/); // 匹配 [language:xx]
    if (languageMatch) {
      // 处理翻译歌词
      try {
        const lyricLanguageList: LyricLanguageList = JSON.parse(atob(languageMatch[1]));

        lyricLanguageList.content.forEach((item) => {
          const transContent = item.lyricContent.map((content) => content.join(''));

          switch (item.type) {
            case LyricTransMode.Roman:
              translations[LyricTransMode.Roman] = transContent;
              break;
            case LyricTransMode.Trans:
              translations[LyricTransMode.Trans] = transContent;
              break;
          }
        });
      } catch {
        notify.error('解析 KRC 翻译歌词失败');
      }
    }

    // 处理原文歌词
    const lineMatch = line.match(/\[(\d+),(\d+)\](.*)/); // 匹配 中括号[]
    if (!lineMatch) continue;

    const offset = parseInt(lineMatch[1]); // line 偏移量
    const duration = parseInt(lineMatch[2]); // line 时长
    const wordsPart = lineMatch[3]; // word 部分

    const words: LyricWord[] = []; // word 列表
    const wordReg = /<(\d+),(\d+),(\d+)>([^<]*)/g; // 匹配 尖括号<>
    let wordMatch = null;

    while ((wordMatch = wordReg.exec(wordsPart)) !== null) {
      const wordOffset = parseInt(wordMatch[1]); // word 偏移量
      const wordDuration = parseInt(wordMatch[2]); // word 时长
      const wordText = wordMatch[4]; //  word 文本

      words.push({ offset: offset + wordOffset, duration: wordDuration, text: wordText });
    }

    lines.push({
      offset,
      duration,
      words,
      translations: {
        [LyricTransMode.Roman]: translations[LyricTransMode.Roman][matchIndex],
        [LyricTransMode.Trans]: translations[LyricTransMode.Trans][matchIndex++],
      },
    });
  }

  return lines;
}

/** 解析 lrc 歌词 */
export function parseLrcLyric(content: string) {
  if (!content) return [];

  // 译制歌词信息
  const translations: Translation = {
    [LyricTransMode.Roman]: [],
    [LyricTransMode.Trans]: [],
  };

  let matchIndex = 0; // 歌词匹配索引
  const lines: LyricLine[] = [];

  for (const line of content.split('\n')) {
    const languageMatch = line.match(/\[language:([^\]]*)\]/); // 匹配 [language:xx]
    if (languageMatch) {
      // 处理翻译歌词
      try {
        const lyricLanguageList: LyricLanguageList = JSON.parse(atob(languageMatch[1]));

        lyricLanguageList.content.forEach((item) => {
          const transContent = item.lyricContent.map((content) => content.join(''));

          switch (item.type) {
            case LyricTransMode.Roman:
              translations[LyricTransMode.Roman] = transContent;
              break;
            case LyricTransMode.Trans:
              translations[LyricTransMode.Trans] = transContent;
              break;
          }
        });
      } catch {
        notify.error('解析 LRC 翻译歌词失败');
      }
    }

    const match = line.match(/\[(\d+):(\d+)\.(\d+)\](.*)/);
    if (!match) continue;

    const offset = parseInt(match[1]) * 60 * 1000 + parseInt(match[2]) * 1000 + parseInt(match[3]);
    const text = match[4];
    if (!text) continue;

    lines.push({
      offset,
      duration: 0,
      words: [{ offset, duration: 0, text }],
      translations: {
        [LyricTransMode.Roman]: translations[LyricTransMode.Roman][matchIndex],
        [LyricTransMode.Trans]: translations[LyricTransMode.Trans][matchIndex++],
      },
    });
  }

  return lines;
}
