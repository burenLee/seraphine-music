<script lang="ts" setup>
import { convertFileSrc } from '@tauri-apps/api/core';
import { computed, nextTick, ref, watch } from 'vue';

import ChangZhen from '@/assets/imgs/chang_zhen.webp';
import Image from '@/components/Image.vue';
import { useMusicStore } from '@/stores/music';
import { useSettingStore } from '@/stores/setting';
import { getPic } from '@/utils/music';
import { PicSize, PlayingOrigin } from '@/utils/params';

const musicStore = useMusicStore();
const settingStore = useSettingStore();

const shouldAnimate = ref(true);

const cover = computed(() => {
  if (!musicStore.music?.cover) return '';

  return musicStore.origin === PlayingOrigin.Online
    ? getPic(musicStore.music.cover, PicSize.Lg)
    : convertFileSrc(musicStore.music.cover);
});

watch(
  () => musicStore.music,
  () => {
    shouldAnimate.value = false;

    nextTick(() => (shouldAnimate.value = true));
  },
);
</script>

<template>
  <div class="relative">
    <div
      class="rounded-full bg-neutral-950 p-16"
      :class="[
        shouldAnimate ? 'animate-spin-slow' : '',
        settingStore.isFullscreen || settingStore.isMaximized ? 'size-96' : 'size-80',
      ]"
      :style="{
        boxShadow: '0 0 0.5rem black',
        animationDelay: '300ms',
        animationPlayState: musicStore.isPlaying ? 'running' : 'paused',
      }"
    >
      <div class="size-full rounded-full bg-minor p-2">
        <Image class="size-full rounded-full" :src="cover" :icon-size="80" />
      </div>
    </div>

    <img
      class="absolute transition-transform duration-300"
      :class="[
        musicStore.isPlaying ? '-rotate-[36deg]' : '-rotate-[55deg]',
        settingStore.isFullscreen || settingStore.isMaximized
          ? '-top-28 left-36 w-20 origin-[50px_48px]'
          : '-top-24 left-32 w-16 origin-[42px_40px]',
      ]"
      :src="ChangZhen"
      alt=""
    />
  </div>
</template>
