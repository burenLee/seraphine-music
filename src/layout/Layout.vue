<script lang="ts" setup>
import ContextMenu from '@/components/ContextMenu.vue'
import { NotificationContainer } from '@/components/Notification'
import Aside from '@/layout/Aside/Aside.vue'
import Header from '@/layout/Header/Header.vue'
import LyricPage from '@/layout/LyricPage/LyricPage.vue'
import Main from '@/layout/Main/Main.vue'
import Playbar from '@/layout/Playbar/Playbar.vue'
import { useSettingStore } from '@/stores/setting'

const settingStore = useSettingStore()
</script>

<template>
  <div
    class="grid h-screen w-screen bg-background"
    :style="{
      fontFamily: settingStore.fontFamily,
      gridTemplateColumns: 'var(--aside-width) 1fr',
      gridTemplateRows: 'var(--header-height) 1fr var(--playbar-height)',
      '--aside-width': '14rem',
      '--header-height': '3.5rem',
      '--playbar-height': '4.5rem'
    }">
    <!-- 侧边栏 -->
    <Aside class="row-span-2" />
    <!-- 头部操作栏 -->
    <Header />
    <!-- 主体内容 -->
    <Main />
    <!-- 播放栏 -->
    <Playbar v-if="!settingStore.isFullscreen" class="col-span-2" />
    <!-- 歌词页 -->
    <LyricPage />

    <!-- 右键菜单 -->
    <ContextMenu />
    <!-- 提示框容器 -->
    <NotificationContainer />
  </div>
</template>
