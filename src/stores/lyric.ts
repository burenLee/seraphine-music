import { defineStore } from 'pinia';
import { ref } from 'vue';

import { notify } from '@/components/Notification.vue';
import { getFullName, parseKrcLyric, parseLrcLyric } from '@/utils/music';
import {
  LyricBaseColor,
  LyricFontSize,
  LyricFormat,
  LyricOffset,
  LyricPageMode,
  LyricTextAlign,
  LyricTransMode,
} from '@/utils/params';
import { invoke } from '@/utils/tools';

type MatchedMap = Record<ID, { id: string; fmt: LyricFormat }>;
type OffsetMap = Record<ID, number>;

/** 歌词配置 */
export const useLyricStore = defineStore(
  'lyric',
  () => {
    const pageVisible = ref(false); // 歌词页可见性
    const pageMode = ref(LyricPageMode.Cover); // 歌词页背景模式
    const isLoading = ref(false);
    const lyric = ref<LyricInfo>(); // 歌词属性
    const fontFamily = ref<FontValue>('system-ui'); // 歌词字体
    const fontSize = ref<number>(LyricFontSize.Default); // 歌词字体大小
    const textColor = ref<LyricBaseColor | string>(LyricBaseColor.Blue); // 歌词字体颜色
    const textAlign = ref(LyricTextAlign.Left); // 歌词对齐方式
    const transMode = ref(LyricTransMode.Off); // 歌词翻译模式
    const matchedMap = ref<MatchedMap>({}); // 歌词匹配列表, 记录歌曲使用的歌词id
    const offsetMap = ref<OffsetMap>({}); // 歌词偏移量列表

    const togglePageVisible = () => (pageVisible.value = !pageVisible.value);
    const setPageMode = (newMode: LyricPageMode) => (pageMode.value = newMode);
    const setLyric = (newLyric: LyricInfo | undefined) => (lyric.value = newLyric);
    const setFontFamily = (newFontFamily: FontValue) => (fontFamily.value = newFontFamily);
    const setFontSize = (mode: 'add' | 'sub' | 'restart') => {
      switch (mode) {
        case 'add':
          fontSize.value += LyricFontSize.Step;
          break;
        case 'sub':
          fontSize.value -= LyricFontSize.Step;
          break;
        case 'restart':
          fontSize.value = LyricFontSize.Default;
          break;
      }
    };
    const setTextColor = (newColor: LyricBaseColor | string) => (textColor.value = newColor);
    const setTextAlign = (newTextAlign: LyricTextAlign) => (textAlign.value = newTextAlign);
    const setTransMode = (newTransMode: LyricTransMode) => (transMode.value = newTransMode);
    const setMatchedLyric = (musicId: ID, lyricInfo: { id: string; fmt: LyricFormat }) => {
      matchedMap.value[musicId] = lyricInfo;
    };
    const setOffsetMap = (mode: 'add' | 'sub' | 'restart') => {
      if (!lyric.value) return;

      switch (mode) {
        case 'add':
          offsetMap.value[lyric.value.id] += LyricOffset.Step;
          break;
        case 'sub':
          offsetMap.value[lyric.value.id] -= LyricOffset.Step;
          break;
        case 'restart':
          offsetMap.value[lyric.value.id] = LyricOffset.Default;
          break;
      }
    };

    /** 加载歌词 */
    const load = async (music: PlayingMusicInfo, lyric?: ApiLyricCandidate) => {
      isLoading.value = true;
      setLyric(undefined);

      let lyricInfo: LyricInfo | undefined = undefined;

      const lyricGet = (await getLocalLyric(music, lyric)) || (await getOnlineLyric(music, lyric));
      if (lyricGet) {
        let lyricLines: LyricLine[] = [];

        switch (lyricGet.fmt) {
          case LyricFormat.Krc:
            lyricLines = parseKrcLyric(lyricGet.content);
            break;
          case LyricFormat.Lrc:
            lyricLines = parseLrcLyric(lyricGet.content);
            break;
        }

        lyricInfo = { id: lyricGet.id, fmt: lyricGet.fmt, lines: lyricLines };
      }

      if (lyricInfo) {
        setLyric(lyricInfo);
        setMatchedLyric(music.id, { id: lyricInfo.id, fmt: lyricInfo.fmt });
      }

      isLoading.value = false;
    };

    /** 获取本地歌词 */
    const getLocalLyric = async (music: PlayingMusicInfo, lyric?: ApiLyricCandidate) => {
      try {
        let id = lyric?.id;
        let fmt = lyric?.contenttype === 0 ? LyricFormat.Krc : LyricFormat.Lrc;

        if (!lyric) {
          const matchedLyric = matchedMap.value[music.id];
          if (!matchedLyric) return;

          id = matchedLyric.id;
          fmt = matchedLyric.fmt;
        }
        if (!id) return;

        return await invoke('music_lyric_get', { id, name: getFullName(music), fmt });
      } catch {
        notify.error('获取本地歌词失败');
      }
    };

    /** 获取在线歌词 */
    const getOnlineLyric = async (music: PlayingMusicInfo, lyric?: ApiLyricCandidate) => {
      try {
        if (!lyric) {
          // 搜索歌词列表
          const { status, candidates } = await invoke('api_lyric_search', {
            keyword: getFullName(music, 'at'),
            hash: music.hash,
          });
          if (status !== 200 || candidates.length === 0) return;

          // 默认选择官方推荐, 其次评分最高的(即第一个)
          lyric = candidates.find((item) => item.product_from === '官方推荐歌词') || candidates[0];
        }

        return await invoke('api_lyric_get', { id: lyric.id, accesskey: lyric.accesskey });
      } catch {
        notify.error('获取在线歌词失败');
      }
    };

    return {
      pageVisible,
      pageMode,
      isLoading,
      lyric,
      fontFamily,
      fontSize,
      textColor,
      textAlign,
      transMode,
      matchedMap,
      offsetMap,

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
      load,
    };
  },
  {
    persist: {
      key: 'lyric-store',
      pick: [
        'pageMode',
        'fontFamily',
        'fontSize',
        'textColor',
        'textAlign',
        'transMode',
        'matchedMap',
        'offsetMap',
      ],
    },
  },
);
