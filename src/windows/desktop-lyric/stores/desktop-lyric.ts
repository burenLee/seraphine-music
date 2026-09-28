import { defineStore } from 'pinia';
import { ref } from 'vue';

import {
  LyricAccentColor,
  LyricBaseColor,
  LyricFontSize,
  LyricOffset,
  LyricTransMode,
} from '@/utils/params';

export const useDesktopLyricStore = defineStore(
  'desktop-lyric',
  () => {
    const fontFamily = ref<FontValue>('system-ui'); // 字体类型
    const fontSize = ref(LyricFontSize.Default); // 字体大小
    const textBaseColor = ref(LyricBaseColor.Blue); // 文本基色
    const textAccentColor = ref(LyricAccentColor.Blue); // 文本高亮色
    const transMode = ref(LyricTransMode.Off); // 翻译文本
    const offsetMap = ref<Record<ID, number>>({}); // 进度偏移量列表(s)

    const setFontFamily = (newFontFamily: FontValue) => {
      fontFamily.value = newFontFamily;
    };
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
    const setTextColors = ([newBaseColor, newAccentColor]: readonly [
      LyricBaseColor,
      LyricAccentColor,
    ]) => {
      textBaseColor.value = newBaseColor;
      textAccentColor.value = newAccentColor;
    };
    const setTransMode = (newTransMode: LyricTransMode) => {
      transMode.value = newTransMode;
    };
    const setOffsetMap = (mode: 'add' | 'sub' | 'restart', id: ID) => {
      if (!id) return;

      let offset = offsetMap.value[id] || LyricOffset.Default;

      switch (mode) {
        case 'add':
          offset += LyricOffset.Step;
          break;
        case 'sub':
          offset -= LyricOffset.Step;
          break;
        case 'restart':
          offset = LyricOffset.Default;
          break;
      }

      offsetMap.value[id] = offset;
    };

    return {
      fontSize,
      fontFamily,
      textBaseColor,
      textAccentColor,
      transMode,
      offsetMap,

      setFontSize,
      setFontFamily,
      setTextColors,
      setTransMode,
      setOffsetMap,
    };
  },
  {
    persist: {
      key: 'desktop-lyric-store',
      pick: [
        'fontSize',
        'fontFamily',
        'textBaseColor',
        'textAccentColor',
        'transMode',
        'offsetMap',
      ],
    },
  },
);
