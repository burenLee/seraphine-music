<script lang="ts" setup>
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { getCurrentWindow } from '@tauri-apps/api/window';

import ActionButton from '@/components/ActionButton.vue';
import { notify } from '@/components/Notification.vue';
import { useMiniPlayerBridge } from '@/composables/useMiniPlayerBridge';
import { MiniPlayerSize, WindowTarget } from '@/constants';
import { useStateStore } from '@/stores/state';

const stateStore = useStateStore();
const miniBridge = useMiniPlayerBridge();

const handleMiniPlayer = async () => {
  let miniWindow = await WebviewWindow.getByLabel(WindowTarget.MiniPlayer);

  if (!miniWindow) {
    const mainWindow = getCurrentWindow();
    const scaleFactor = await mainWindow.scaleFactor();

    const { x, y } = stateStore.miniPlayerPosition;
    const logicalX =
      x / scaleFactor || Math.round(window.screen.availWidth - MiniPlayerSize.width - 16);
    const logicalY = y / scaleFactor || 48;

    miniWindow = new WebviewWindow(WindowTarget.MiniPlayer, {
      title: 'Seraphine 迷你播放器',
      url: '/mini-player.html',
      width: MiniPlayerSize.width,
      height: MiniPlayerSize.height,
      x: logicalX,
      y: logicalY,
      transparent: true,
      decorations: false,
      shadow: false,
      alwaysOnTop: true,
      skipTaskbar: true,
      resizable: false,
    });

    miniWindow.once('tauri://error', () => {
      notify.error('迷你播放器创建失败');
    });
    miniWindow.once('tauri://created', () => {
      miniBridge.start();
      mainWindow.hide();
    });
    miniWindow.once('tauri://destroyed', () => {
      mainWindow.show();
    });
  } else {
    miniBridge.stop();
  }
};
</script>

<template>
  <ActionButton
    class="size-8"
    mode="text"
    prefix-icon="PIP"
    icon-size="18"
    title="迷你播放器"
    @click="handleMiniPlayer"
  />
</template>
