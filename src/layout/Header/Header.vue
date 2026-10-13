<script setup lang="ts">
import { useRouter } from 'vue-router';

import ActionButton from '@/components/ActionButton.vue';
import SystemActions from '@/components/SystemActions.vue';
import { useRefreshStore } from '@/stores/refresh';

import Login from './Login/Login.vue';
import Menu from './Menu.vue';
import Search from './Search.vue';

const router = useRouter();
const refreshStore = useRefreshStore();
</script>

<template>
  <header data-tauri-drag-region class="flex items-center gap-1 px-5 pt-4">
    <ActionButton
      class="size-8"
      mode="text"
      prefix-icon="Left"
      icon-size="18"
      @click="router.back"
    />
    <ActionButton
      class="size-8"
      mode="text"
      prefix-icon="Refresh"
      icon-size="18"
      @click="refreshStore.refresh"
    />
    <Search class="mr-auto" />

    <Login />
    <Menu />
    <SystemActions />
  </header>
</template>
