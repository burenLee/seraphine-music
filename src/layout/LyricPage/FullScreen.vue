<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useEventListener } from '@vueuse/core';

import ActionButton from '@/components/ActionButton.vue';
import { useStateStore } from '@/stores/state';

const stateStore = useStateStore();

const currentWindow = getCurrentWindow();

const toggleFullScreen = async () => {
  const isFullscreen = await currentWindow.isFullscreen();

  await currentWindow.setFullscreen(!isFullscreen);
  stateStore.toggleFullscreen(!isFullscreen);
};

useEventListener('keyup', async (e) => {
  if (e.key !== 'Escape' || stateStore.isFullscreen) return;

  await currentWindow.setFullscreen(false);
  stateStore.toggleFullscreen(false);
});
</script>

<template>
  <ActionButton
    class="size-8"
    mode="text"
    :prefix-icon="stateStore.isFullscreen ? 'QuitFullScreen' : 'FullScreen'"
    @click="toggleFullScreen"
  />
</template>
