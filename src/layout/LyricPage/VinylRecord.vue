<script lang="ts" setup>
import ChangZhen from '@/assets/changzhen.webp'
import Image from '@/components/Image.vue'
import { useMusicStore } from '@/stores/music'
import { useSettingStore } from '@/stores/setting'
import { PlayingOrigin } from '@/utils/params'
import { getPic } from '@/utils/tools'
import { convertFileSrc } from '@tauri-apps/api/core'

const musicStore = useMusicStore()
const settingStore = useSettingStore()

const shouldAnimate = ref(true)

const cover = computed(() => {
  if (!musicStore.music?.cover) return ''

  return musicStore.origin === PlayingOrigin.Online
    ? getPic(musicStore.music.cover, 'lg')
    : convertFileSrc(musicStore.music.cover)
})

watch(
  () => musicStore.music,
  () => {
    shouldAnimate.value = false

    nextTick(() => (shouldAnimate.value = true))
  }
)
</script>

<template>
  <div class="relative">
    <div
      class="rounded-full bg-neutral-950 p-16"
      :class="[
        shouldAnimate ? 'animate-spin-slow' : '',
        settingStore.isFullscreen || settingStore.isMaximized ? 'size-96' : 'size-80'
      ]"
      :style="{
        boxShadow: '0 0 0.5rem black',
        animationDelay: '300ms',
        animationPlayState: musicStore.isPlaying ? 'running' : 'paused'
      }">
      <div class="p-2 bg-minor size-full rounded-full">
        <Image class="rounded-full size-full" :img="cover" :icon-size="80" />
      </div>
    </div>

    <img
      class="absolute transition-transform duration-300"
      :class="[
        musicStore.isPlaying ? '-rotate-[36deg]' : '-rotate-[55deg]',
        settingStore.isFullscreen || settingStore.isMaximized
          ? 'w-20 origin-[50px_48px] -top-28 left-36'
          : 'w-16 origin-[42px_40px] -top-24 left-32'
      ]"
      :src="ChangZhen"
      alt="" />
  </div>
</template>
