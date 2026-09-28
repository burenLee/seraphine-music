import { describe, expect, it } from 'vitest';

import {
  getFullName,
  getOrigin,
  getPic,
  getPrivilegeTags,
  getQuality,
  parseKrcLyric,
  parseLrcLyric,
} from '@/utils/music';
import { LyricTransMode, PicSize, PlayingOrigin, PlayingQuality } from '@/utils/params';

import { makeListMusic, makeMusicDetail, makePlayingMusic } from '../helpers/factories';
import { notifyMock } from '../helpers/mocks';

describe('getPic', () => {
  it('默认替换为 Sm(64) 尺寸', () => {
    expect(getPic('https://img.com/{size}/a.jpg')).toBe('https://img.com/64/a.jpg');
  });

  it('替换为指定尺寸', () => {
    expect(getPic('https://img.com/{size}/a.jpg', PicSize.Lg)).toBe('https://img.com/400/a.jpg');
  });

  it('URL 无占位符时原样返回', () => {
    expect(getPic('https://img.com/a.jpg')).toBe('https://img.com/a.jpg');
  });
});

describe('getFullName', () => {
  it('ta 模式：title - artist', () => {
    const music = makePlayingMusic({ title: 'Song', artist: 'Artist' });
    expect(getFullName(music)).toBe('Song - Artist');
  });

  it('at 模式：artist - title', () => {
    const music = makePlayingMusic({ title: 'Song', artist: 'Artist' });
    expect(getFullName(music, 'at')).toBe('Artist - Song');
  });

  it('artist 为空时只返回 title', () => {
    const music = makePlayingMusic({ title: 'Song', artist: null });
    expect(getFullName(music)).toBe('Song');
    expect(getFullName(music, 'at')).toBe('Song');
  });
});

describe('getPrivilegeTags', () => {
  it('privilege=10 且 payType=2 → 付费', () => {
    expect(getPrivilegeTags(10, 2)).toStrictEqual(['付费']);
  });

  it('privilege=10 且 payType=3 → VIP', () => {
    expect(getPrivilegeTags(10, 3)).toStrictEqual(['VIP']);
  });

  it('其他组合无标签', () => {
    expect(getPrivilegeTags(10, 0)).toStrictEqual([]);
    expect(getPrivilegeTags(0, 2)).toStrictEqual([]);
    expect(getPrivilegeTags(8, 3)).toStrictEqual([]);
  });
});

describe('getOrigin', () => {
  it('有 hash → Online', () => {
    expect(getOrigin(makeListMusic({ hash: 'abc' }))).toBe(PlayingOrigin.Online);
  });

  it('hash 为 null → Local', () => {
    expect(getOrigin(makeListMusic({ hash: null }))).toBe(PlayingOrigin.Local);
  });
});

describe('getQuality', () => {
  it('audio_bitrate 为空 → Bitrate128', () => {
    expect(getQuality(makeMusicDetail({ audio_bitrate: null }))).toBe(PlayingQuality.Bitrate128);
  });

  it('>500kbps + 16bit + 44.1kHz → Flac', () => {
    const detail = makeMusicDetail({ audio_bitrate: 900, bit_depth: 16, sample_rate: 44100 });
    expect(getQuality(detail)).toBe(PlayingQuality.BitrateFlac);
  });

  it('>500kbps 但位深/采样率不足 → BitrateHigh', () => {
    expect(getQuality(makeMusicDetail({ audio_bitrate: 900, bit_depth: 8 }))).toBe(
      PlayingQuality.BitrateHigh,
    );
    expect(
      getQuality(makeMusicDetail({ audio_bitrate: 900, bit_depth: 16, sample_rate: 22050 })),
    ).toBe(PlayingQuality.BitrateHigh);
  });

  it('256-500kbps → BitrateHigh', () => {
    expect(getQuality(makeMusicDetail({ audio_bitrate: 256 }))).toBe(PlayingQuality.BitrateHigh);
    expect(getQuality(makeMusicDetail({ audio_bitrate: 500 }))).toBe(PlayingQuality.BitrateHigh);
  });

  it('192-255kbps → Bitrate320', () => {
    expect(getQuality(makeMusicDetail({ audio_bitrate: 192 }))).toBe(PlayingQuality.Bitrate320);
    expect(getQuality(makeMusicDetail({ audio_bitrate: 255 }))).toBe(PlayingQuality.Bitrate320);
  });

  it('<192kbps → Bitrate128', () => {
    expect(getQuality(makeMusicDetail({ audio_bitrate: 128 }))).toBe(PlayingQuality.Bitrate128);
  });
});

/** 构造 KRC/LRC 的 [language:base64] 翻译行（type 0=Roman, 1=Trans） */
const makeLanguageLine = (roman: string[], trans: string[]) => {
  const list: LyricLanguageList = {
    content: [
      { type: LyricTransMode.Roman, language: 0, lyricContent: roman.map((t) => [t]) },
      { type: LyricTransMode.Trans, language: 0, lyricContent: trans.map((t) => [t]) },
    ],
    version: 1,
  };
  return `[language:${btoa(JSON.stringify(list))}]`;
};

describe('parseKrcLyric', () => {
  it('空内容返回空数组', () => {
    expect(parseKrcLyric('')).toStrictEqual([]);
  });

  it('解析行偏移/时长与逐字时间轴', () => {
    const lines = parseKrcLyric('[1000,2000]<0,500,0>Hello<500,300,0>World');

    expect(lines).toHaveLength(1);
    expect(lines[0].offset).toBe(1000);
    expect(lines[0].duration).toBe(2000);
    // word 的绝对偏移 = 行偏移 + word 相对偏移
    expect(lines[0].words).toStrictEqual([
      { offset: 1000, duration: 500, text: 'Hello' },
      { offset: 1500, duration: 300, text: 'World' },
    ]);
  });

  it('不匹配行格式的内容被跳过', () => {
    expect(parseKrcLyric('[ti:Title]\n[ar:Artist]')).toStrictEqual([]);
  });

  it('行内无逐字信息时 words 为空数组', () => {
    const lines = parseKrcLyric('[1000,2000]plain text');
    expect(lines).toHaveLength(1);
    expect(lines[0].words).toStrictEqual([]);
  });

  it('解析 [language:] 翻译并按行配对', () => {
    const content = [
      makeLanguageLine(['roman1', 'roman2'], ['trans1', 'trans2']),
      '[0,1000]<0,500,0>line1',
      '[1000,1000]<0,500,0>line2',
    ].join('\n');

    const lines = parseKrcLyric(content);

    expect(lines).toHaveLength(2);
    expect(lines[0].translations[LyricTransMode.Roman]).toBe('roman1');
    expect(lines[0].translations[LyricTransMode.Trans]).toBe('trans1');
    expect(lines[1].translations[LyricTransMode.Roman]).toBe('roman2');
    expect(lines[1].translations[LyricTransMode.Trans]).toBe('trans2');
  });

  it('翻译行数不足时对应行为 undefined', () => {
    const content = [
      makeLanguageLine(['roman1'], ['trans1']),
      '[0,1000]<0,500,0>line1',
      '[1000,1000]<0,500,0>line2',
    ].join('\n');

    const lines = parseKrcLyric(content);

    expect(lines[1].translations[LyricTransMode.Roman]).toBeUndefined();
    expect(lines[1].translations[LyricTransMode.Trans]).toBeUndefined();
  });

  it('非法翻译内容触发 notify.error 但不中断解析', () => {
    const content = ['[language:!!!not-base64!!!]', '[0,1000]<0,500,0>line1'].join('\n');

    const lines = parseKrcLyric(content);

    expect(notifyMock.error).toHaveBeenCalledWith('解析 KRC 翻译歌词失败');
    expect(lines).toHaveLength(1);
  });
});

describe('parseLrcLyric', () => {
  it('空内容返回空数组', () => {
    expect(parseLrcLyric('')).toStrictEqual([]);
  });

  it('解析 [mm:ss.xx] 时间戳为毫秒偏移', () => {
    const lines = parseLrcLyric('[01:23.45]Hello world');

    expect(lines).toHaveLength(1);
    expect(lines[0].offset).toBe(83045);
    expect(lines[0].duration).toBe(0);
    expect(lines[0].words).toStrictEqual([{ offset: 83045, duration: 0, text: 'Hello world' }]);
  });

  it('元数据行与空文本行被跳过', () => {
    const content = ['[ti:Title]', '[00:01.00]', '[00:02.00]text'].join('\n');

    const lines = parseLrcLyric(content);

    expect(lines).toHaveLength(1);
    expect(lines[0].offset).toBe(2000);
  });

  it('解析多行并保持顺序', () => {
    const content = '[00:01.00]first\n[00:02.00]second\n[00:03.00]third';
    const lines = parseLrcLyric(content);

    expect(lines.map((l) => l.offset)).toStrictEqual([1000, 2000, 3000]);
    expect(lines.map((l) => l.words[0].text)).toStrictEqual(['first', 'second', 'third']);
  });

  it('解析 [language:] 翻译并按行配对', () => {
    const content = [makeLanguageLine(['roman1'], ['trans1']), '[00:01.00]line1'].join('\n');

    const lines = parseLrcLyric(content);

    expect(lines[0].translations[LyricTransMode.Roman]).toBe('roman1');
    expect(lines[0].translations[LyricTransMode.Trans]).toBe('trans1');
  });

  it('非法翻译内容触发 notify.error 但不中断解析', () => {
    const content = ['[language:!!!not-base64!!!]', '[00:01.00]line1'].join('\n');

    const lines = parseLrcLyric(content);

    expect(notifyMock.error).toHaveBeenCalledWith('解析 LRC 翻译歌词失败');
    expect(lines).toHaveLength(1);
  });
});
