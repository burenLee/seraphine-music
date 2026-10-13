<script lang="ts" setup>
import Logo from '@/assets/imgs/logo.png';

import Menus from './Menus.vue';
import Mode from './Mode.vue';
import UserPlaylist from './UserPlaylist/Playlist.vue';
</script>

<template>
  <aside class="flex flex-col bg-card">
    <div data-tauri-drag-region class="flex items-center gap-3 px-6 pt-5">
      <img class="pointer-events-none mt-1 size-10" :src="Logo" alt="" draggable="false" />
      <div class="pointer-events-none">
        <div
          class="bg-gradient-to-r from-info to-success bg-clip-text font-mono text-2xl font-bold leading-7 text-transparent"
        >
          Seraphine
        </div>

        <Mode class="pointer-events-auto w-fit text-xs leading-4" />
      </div>
    </div>

    <Menus />

    <UserPlaylist class="min-h-0 flex-1" />
  </aside>
</template>
