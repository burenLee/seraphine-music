<script lang="ts" setup>
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ref } from 'vue';

import { notify } from '@/components/Notification.vue';
import { useDesktopLyricBridge } from '@/composables/useDesktopLyricBridge';
import { useMusicStore } from '@/stores/music';
import { useSettingStore } from '@/stores/setting';
import { WindowTarget, desktopLyricSize } from '@/utils/params';

const mainWindow = getCurrentWindow();

const musicStore = useMusicStore();
const settingStore = useSettingStore();

const lyricBridge = useDesktopLyricBridge();

const visible = ref(false);

const handleDesktopLyric = async () => {
  let lyricWindow = await WebviewWindow.getByLabel(WindowTarget.DesktopLyric);

  if (!lyricWindow) {
    const scaleFactor = await mainWindow.scaleFactor();

    const { width, height } = desktopLyricSize;
    const { x, y } = settingStore.desktopLyricPosition;
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
    visible.value = false;
  }
};
</script>

<template>
  <div
    class="action-icon text-center font-bold leading-8"
    :class="visible ? 'text-info' : ''"
    title="桌面歌词"
    :data-disabled="!musicStore.music"
    @click="handleDesktopLyric"
  >
    词
  </div>
</template>
