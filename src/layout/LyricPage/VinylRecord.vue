<script lang="ts" setup>
import { nextTick, ref, watch } from 'vue';

import ChangZhen from '@/assets/imgs/chang_zhen.webp';
import Image from '@/components/Image.vue';
import { useMusicStore } from '@/stores/music';
import { useStateStore } from '@/stores/state';

interface Props {
  src: string | undefined;
}

const { src } = defineProps<Props>();

const stateStore = useStateStore();
const musicStore = useMusicStore();

const shouldAnimate = ref(true);

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
        stateStore.isFullscreen || stateStore.isMaximized ? 'size-96' : 'size-80',
      ]"
      :style="{
        boxShadow: '0 0 0.5rem black',
        animationDelay: '300ms',
        animationPlayState: musicStore.isPlaying ? 'running' : 'paused',
      }"
    >
      <div class="size-full rounded-full bg-minor p-2">
        <Image class="size-full rounded-full" :src="src" :icon-size="80" />
      </div>
    </div>

    <img
      class="absolute transition-transform duration-300"
      :class="[
        musicStore.isPlaying ? '-rotate-[36deg]' : '-rotate-[55deg]',
        stateStore.isFullscreen || stateStore.isMaximized
          ? '-top-28 left-36 w-20 origin-[50px_48px]'
          : '-top-24 left-32 w-16 origin-[42px_40px]',
      ]"
      :src="ChangZhen"
      alt=""
      draggable="false"
    />
  </div>
</template>
