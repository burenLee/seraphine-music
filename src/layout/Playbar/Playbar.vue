<script lang="ts">
const excludeNodeType = [
  'text',
  'password',
  'search',
  'tel',
  'url',
  'email',
  'number',
  'checkbox',
  'radio',
  'color',
];
</script>

<script lang="ts" setup>
import { useEventListener, watchThrottled } from '@vueuse/core';
import { ref, watch } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Carousel from '@/components/Carousel.vue';
import Image from '@/components/Image.vue';
import ProgressRange from '@/components/ProgressRange.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { usePlayingCover } from '@/composables/usePlayingCover.ts';
import { Interval, PicSize } from '@/constants';
import { useLyricStore } from '@/stores/lyric';
import { useMusicStore } from '@/stores/music';
import { formatDuration } from '@/utils/tools.ts';

import DesktopLyric from './DesktopLyric.vue';
import Like from './Like.vue';
import PlayList from './PlayList.vue';
import PlayMode from './PlayMode.vue';
import Quality from './Quality.vue';
import Volume from './Volume.vue';

const musicStore = useMusicStore();
const lyricStore = useLyricStore();

const cover = usePlayingCover(PicSize.Md);

const playProgress = ref(musicStore.playProgress);

// 节流监听播放进度
watchThrottled(
  () => musicStore.playProgress,
  (pg) => !musicStore.isDragging && (playProgress.value = pg),
  { throttle: Interval.Sec },
);

// 切歌时立即归零进度, 避免节流同步滞后导致进度条在新 duration 下跳变
watch(
  () => musicStore.music?.id,
  () => (playProgress.value = 0),
);

// 监听空格来暂停/播放
useEventListener(document, 'keyup', (e: KeyboardEvent) => {
  if (e.code !== 'Space' || e.repeat) return;

  // 如果光标在输入框/文本域，不拦截
  const type = (e.target as HTMLInputElement).type;
  if (type && excludeNodeType.includes(type)) return;

  if (musicStore.isPlaying) {
    musicStore.pause();
  } else {
    musicStore.play();
  }
});
</script>

<template>
  <!-- todo: 未来实现自定义背景就需要把playbar的背景设置为带有透明度的颜色 -->
  <div class="relative flex h-full items-center gap-8 px-4">
    <ProgressRange
      class="absolute bottom-[calc(var(--playbar-height)-0.5rem)] left-0"
      show-mode="hover"
      v-model="playProgress"
      :loading-progress="musicStore.downloadProgress"
      :max="musicStore.music?.duration"
      :step="0.1"
      :disabled="!musicStore.music || !musicStore.isLoaded"
      @start-change="musicStore.startChangeProgress"
      @stop-change="musicStore.stopChangeProgress"
    />

    <div class="flex flex-1 items-center gap-3">
      <div class="group/cover card size-12 cursor-pointer" @click="lyricStore.togglePageVisible">
        <SvgIcon v-if="lyricStore.pageVisible" class="size-full text-minor" name="DoubleDown" />

        <div v-else class="relative size-full">
          <Image class="size-full" :src="cover" icon-size="24" />

          <SvgIcon
            class="absolute left-0 top-0 size-full rounded-lg bg-actived text-background opacity-0 transition-opacity group-hover/cover:opacity-100"
            name="DoubleUp"
          />
        </div>
      </div>

      <div class="w-0 flex-1">
        <Carousel class="music-title" :content="musicStore.music?.title || 'Seraphine'" />
        <div class="music-artist">{{ musicStore.music?.artist }}</div>
      </div>
    </div>

    <div class="flex items-center gap-8">
      <ActionButton
        mode="text"
        prefix-icon="PreviousBold"
        icon-size="24"
        @click="musicStore.playPrevOrNext('prev')"
      />
      <ActionButton
        v-if="!musicStore.isLoading"
        mode="text"
        :prefix-icon="musicStore.isPlaying ? 'PauseBold' : 'PlayBold'"
        icon-size="32"
        @click="musicStore.isPlaying ? musicStore.pause() : musicStore.play()"
      />
      <ActionButton v-else mode="text" prefix-icon="Ring" icon-size="32" />
      <ActionButton
        mode="text"
        prefix-icon="NextBold"
        icon-size="24"
        @click="musicStore.playPrevOrNext('next')"
      />
    </div>

    <div class="flex flex-1 items-center justify-end gap-1">
      <div class="px-2">
        {{ formatDuration(playProgress) }} /
        {{ formatDuration(musicStore.music?.duration ?? 0) }}
      </div>
      <Like />
      <PlayMode />
      <Volume />
      <Quality />
      <DesktopLyric />
      <PlayList />
    </div>
  </div>
</template>
