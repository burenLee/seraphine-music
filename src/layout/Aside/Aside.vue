<script lang="ts" setup>
import Menus from './Menus.vue';
import Mode from './Mode.vue';
import UserPlaylist from './UserPlaylist.vue';
</script>

<template>
  <aside class="card z-10 flex flex-col rounded-none border-0">
    <div data-tauri-drag-region class="px-8 pt-6">
      <div
        class="pointer-events-none bg-gradient-to-r from-info to-success bg-clip-text font-mono text-3xl font-bold text-transparent"
      >
        Seraphine
      </div>

      <div class="mt-2 flex gap-2 text-minor">
        音乐播放器
        <Mode class="flex-1" />
      </div>
    </div>

    <Menus />

    <UserPlaylist class="h-0 flex-1" />
  </aside>
</template>
