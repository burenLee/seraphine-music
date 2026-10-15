import { defineStore } from 'pinia';
import { ref, watch } from 'vue';

import { DefaultSystemFonts, Mode } from '@/constants';

/** 全局状态 */
export const useStateStore = defineStore(
  'state',
  () => {
    const isHydrated = ref(false);

    const refreshId = ref(0);
    const mode = ref<Mode>(Mode.KgLite); // Mode
    const isMaximized = ref(false); // 最大化
    const isFullscreen = ref(false); // 全屏
    const availableFonts = ref<FontItem[]>([]); // 可用字体列表
    const fontFamily = ref<FontValue>('system-ui'); // 字体
    const miniPlayerPosition = ref({ x: 0, y: 0 }); // mini播放器坐标
    const desktopLyricPosition = ref({ x: 0, y: 0 }); //桌面歌词坐标

    watch(
      isHydrated,
      () => {
        initAvailableFonts();
      },
      { once: true },
    );

    const refresh = () => {
      refreshId.value++;
    };

    const setMode = (newMode: Mode) => {
      mode.value = newMode;
    };

    const toggleMaximized = (newState: boolean) => {
      isMaximized.value = newState;
    };

    const toggleFullscreen = (newState: boolean) => {
      isFullscreen.value = newState;
    };

    /** 初始化可用字体列表 */
    const initAvailableFonts = () => {
      const canvas = document.createElement('canvas');
      const context = canvas.getContext('2d');
      if (!context) return;

      const testString = '天地玄黄 AWayonmw01 宇宙洪荒';
      const baseFont = 'monospace';
      const fontSize = 100;

      context.font = `${fontSize}px ${baseFont}`;
      const baseWidth = context.measureText(testString).width;
      const fonts: FontItem[] = [];

      DefaultSystemFonts.forEach(([name, value]) => {
        context.font = `${fontSize}px "${value}", ${baseFont}`;
        const testWidth = context.measureText(testString).width;

        if (Math.abs(baseWidth - testWidth) > 0.1) fonts.push([name, value]);
      });

      availableFonts.value = fonts;
    };

    const setFontFamily = (font: FontValue) => {
      fontFamily.value = font;
    };

    const setMiniPlayerPosition = (pos: { x: number; y: number }) => {
      miniPlayerPosition.value = pos;
    };

    const setdesktopLyricPosition = (pos: { x: number; y: number }) => {
      desktopLyricPosition.value = pos;
    };

    return {
      isHydrated,
      refreshId,
      mode,
      isMaximized,
      isFullscreen,
      availableFonts,
      fontFamily,
      miniPlayerPosition,
      desktopLyricPosition,

      refresh,
      setMode,
      toggleMaximized,
      toggleFullscreen,
      setFontFamily,
      setMiniPlayerPosition,
      setdesktopLyricPosition,
    };
  },
  {
    persist: {
      key: 'state-store',
      pick: [],
      afterHydrate: (ctx) => (ctx.store.isHydrated = true),
    },
  },
);
