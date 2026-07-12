<script setup lang="ts">
import Login from './Login/Login.vue'
import Menu from './Menu/Menu.vue'
import Search from './Search.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import SystemActions from '@/components/SystemActions.vue'
import { useRefreshStore } from '@/stores/refresh'

const router = useRouter()
const refreshStore = useRefreshStore()
</script>

<template>
  <header data-tauri-drag-region class="flex z-10 justify-between px-6 pt-6">
    <!-- 左侧 -->
    <div class="flex items-center gap-1">
      <SvgIcon class="action-icon" name="Left" size="18" @click="router.back" />
      <SvgIcon class="action-icon" name="Refresh" size="18" @click="refreshStore.refresh" />
      <Search />
    </div>

    <!-- 右侧 -->
    <div class="flex items-center gap-1">
      <Login />
      <Menu />
      <div class="mx-1 h-4 w-px bg-minor" />
      <SystemActions />
    </div>
  </header>
</template>
