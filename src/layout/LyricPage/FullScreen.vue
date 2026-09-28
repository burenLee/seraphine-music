<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useEventListener } from '@vueuse/core';

import SvgIcon from '@/components/SvgIcon.vue';
import { useSettingStore } from '@/stores/setting';

const settingStore = useSettingStore();

const currentWindow = getCurrentWindow();

const toggleFullScreen = async () => {
  const isFullscreen = await currentWindow.isFullscreen();

  await currentWindow.setFullscreen(!isFullscreen);
  settingStore.toggleFullscreenState(!isFullscreen);
};

useEventListener('keyup', async (e) => {
  if (e.key !== 'Escape') return;

  await currentWindow.setFullscreen(false);
  settingStore.toggleFullscreenState(false);
});
</script>

<template>
  <SvgIcon
    class="action-icon"
    :name="settingStore.isFullscreen ? 'QuitFullScreen' : 'FullScreen'"
    @click="toggleFullScreen"
  />
</template>
