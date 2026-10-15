<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window';

import ActionButton from '@/components/ActionButton.vue';
import { useStateStore } from '@/stores/state';

const stateStore = useStateStore();

const handleMaximize = async () => {
  const mainWindow = getCurrentWindow();

  await mainWindow.toggleMaximize();
  stateStore.toggleMaximized(await mainWindow.isMaximized());
};
</script>

<template>
  <ActionButton
    class="size-8"
    mode="text"
    :prefix-icon="stateStore.isMaximized ? 'Restore' : 'Square'"
    icon-size="18"
    @click="handleMaximize"
  />
</template>
