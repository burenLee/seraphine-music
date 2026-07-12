<script lang="ts" setup>
import HttpMode from './HttpMode.vue'
import Menus from './Menus.vue'
import UserPlaylist from './UserPlaylist.vue'
</script>

<template>
  <aside class="card flex flex-col z-10 rounded-none">
    <div data-tauri-drag-region class="px-8 py-6">
      <div
        class="pointer-events-none bg-gradient-to-r from-info to-success bg-clip-text font-mono text-2xl font-bold text-transparent">
        Seraphine
      </div>

      <div class="mt-2 flex gap-2 text-minor">
        音乐播放器
        <HttpMode class="flex-1" />
      </div>
    </div>

    <Menus />

    <UserPlaylist class="flex-1 h-0" />
  </aside>
</template>
