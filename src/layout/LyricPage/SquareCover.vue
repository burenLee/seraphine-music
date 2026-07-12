<script lang="ts" setup>
import Image from '@/components/Image.vue'
import { useMusicStore } from '@/stores/music'
import { useSettingStore } from '@/stores/setting'
import { PlayingOrigin } from '@/utils/params'
import { getPic } from '@/utils/tools'
import { convertFileSrc } from '@tauri-apps/api/core'

const musicStore = useMusicStore()
const settingStore = useSettingStore()

const cover = computed(() => {
  if (!musicStore.music?.cover) return ''

  return musicStore.origin === PlayingOrigin.Online
    ? getPic(musicStore.music.cover, 'lg')
    : convertFileSrc(musicStore.music.cover)
})
</script>

<template>
  <Image
    class="rounded-3xl"
    :class="settingStore.isFullscreen || settingStore.isMaximized ? 'size-96' : 'size-80'"
    :img="cover"
    :icon-size="96" />
</template>
