<script setup lang="ts">
import { useRouter } from 'vue-router';

import ActionButton from '@/components/ActionButton.vue';
import Close from '@/components/SystemActions/Close.vue';
import Maximize from '@/components/SystemActions/Maximize.vue';
import Minimize from '@/components/SystemActions/Minimize.vue';
import MiniPlayer from '@/components/SystemActions/MiniPlayer.vue';
import Theme from '@/components/SystemActions/Theme.vue';
import { useStateStore } from '@/stores/state.ts';

import Login from './Login/Login.vue';
import Menu from './Menu.vue';
import Search from './Search.vue';

const router = useRouter();
const stateStore = useStateStore();
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
      @click="stateStore.refresh"
    />
    <Search class="mr-auto" />

    <Login />
    <Menu />

    <div class="mx-1 h-4 w-px bg-minor" />

    <Theme />
    <MiniPlayer />
    <Minimize />
    <Maximize />
    <Close />
  </header>
</template>
