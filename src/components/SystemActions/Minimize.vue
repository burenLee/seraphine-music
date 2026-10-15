<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window';

import ActionButton from '@/components/ActionButton.vue';

const handleMinimize = () => {
  getCurrentWindow().minimize();
};
</script>

<template>
  <ActionButton
    class="size-8"
    mode="text"
    prefix-icon="Remove"
    icon-size="18"
    @click="handleMinimize"
  />
</template>
