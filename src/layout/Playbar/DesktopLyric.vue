<script lang="ts" setup>
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ref } from 'vue';

import { notify } from '@/components/Notification.vue';
import { useDesktopLyricBridge } from '@/composables/useDesktopLyricBridge';
import { WindowTarget, DesktopLyricSize } from '@/constants';
import { useMusicStore } from '@/stores/music';
import { useStateStore } from '@/stores/state';

const stateStore = useStateStore();
const musicStore = useMusicStore();

const lyricBridge = useDesktopLyricBridge();

const mainWindow = getCurrentWindow();

const visible = ref(false);

const handleDesktopLyric = async () => {
  let lyricWindow = await WebviewWindow.getByLabel(WindowTarget.DesktopLyric);

  if (!lyricWindow) {
    const scaleFactor = await mainWindow.scaleFactor();

    const { width, height } = DesktopLyricSize;
    const { x, y } = stateStore.desktopLyricPosition;
    const logicalX = x / scaleFactor || Math.round((window.screen.availWidth - width) / 2);
    const logicalY = y / scaleFactor || Math.round(window.screen.availHeight - height);

    lyricWindow = new WebviewWindow(WindowTarget.DesktopLyric, {
      title: '桌面歌词',
      url: '/desktop-lyric.html',
      width,
      height,
      x: logicalX,
      y: logicalY,
      transparent: true,
      decorations: false,
      alwaysOnTop: true,
      shadow: false,
      skipTaskbar: true,
      resizable: false,
    });

    lyricWindow.once('tauri://error', () => {
      notify.error('桌面歌词创建失败');
    });
    lyricWindow.once('tauri://created', () => {
      lyricBridge.start();
      visible.value = true;
    });
    lyricWindow.once('tauri://destroyed', () => {
      visible.value = false;
    });
  } else {
    lyricBridge.stop();
  }
};
</script>

<template>
  <div
    class="action-icon size-8 text-center font-bold leading-8"
    :class="visible ? 'text-info' : ''"
    title="桌面歌词"
    :data-disabled="!musicStore.music"
    @click="handleDesktopLyric"
  >
    词
  </div>
</template>
