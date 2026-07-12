<script lang="ts" setup>
import DesktopLyric from './DesktopLyric.vue'
import PlayList from './PlayList.vue'
import PlayMode from './PlayMode.vue'
import Quality from './Quality.vue'
import Volume from './Volume.vue'
import Carousel from '@/components/Carousel.vue'
import Image from '@/components/Image.vue'
import ProgressRange from '@/components/ProgressRange.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useLyricStore } from '@/stores/lyric'
import { useMusicStore } from '@/stores/music'
import { Interval, PlayingOrigin } from '@/utils/params'
import { formatDuration, getPic } from '@/utils/tools'
import { convertFileSrc } from '@tauri-apps/api/core'
import { watchThrottled } from '@vueuse/core'

const musicStore = useMusicStore()
const lyricStore = useLyricStore()

const playProgress = ref(musicStore.playProgress)

const cover = computed(() => {
  if (!musicStore.music?.cover) return ''

  return musicStore.origin === PlayingOrigin.Online
    ? getPic(musicStore.music.cover)
    : convertFileSrc(musicStore.music.cover)
})

// 节流监听播放进度
watchThrottled(
  () => musicStore.playProgress,
  (pg) => !musicStore.isDragging && (playProgress.value = pg),
  { throttle: Interval.Long }
)
</script>

<template>
  <!-- todo: 未来实现自定义背景就需要把playbar的背景设置为带有透明度的颜色 -->
  <div class="relative flex items-center z-20 bg-background px-4">
    <ProgressRange
      class="absolute -top-2 left-0"
      show-mode="hover"
      v-model="playProgress"
      :loading-progress="musicStore.downloadProgress"
      :min="0"
      :max="musicStore.music?.duration ?? 0"
      :step="0.1"
      :disabled="!musicStore.music || !musicStore.isLoaded"
      @start-change="musicStore.startChangeProgress"
      @stop-change="musicStore.stopChangeProgress" />

    <div class="flex flex-1 gap-3">
      <div class="group/cover card size-12 cursor-pointer" @click="lyricStore.togglePageVisible">
        <Transition name="zoom-fade" mode="out-in">
          <SvgIcon
            v-if="lyricStore.pageVisible"
            class="size-full text-minor"
            name="DoubleDown"
            size="42" />
          <div class="relative size-full" v-else>
            <Image class="size-full" :img="cover" />

            <SvgIcon
              class="absolute left-0 top-0 size-full rounded-lg bg-actived text-background opacity-0 transition-opacity group-hover/cover:opacity-100"
              name="DoubleUp"
              size="42" />
          </div>
        </Transition>
      </div>

      <div class="w-0 flex-1">
        <Carousel class="music-title" :content="musicStore.music?.title || 'Seraphine'" />
        <div class="music-artist">{{ musicStore.music?.artist }}</div>
      </div>
    </div>

    <SvgIcon
      class="action-icon ml-16"
      name="PreviousBold"
      size="24"
      @click="musicStore.playPrevOrNext('prev')" />

    <SvgIcon
      v-if="!musicStore.isLoading"
      class="action-icon mx-8 size-10"
      :name="musicStore.isPlaying ? 'PauseBold' : 'PlayBold'"
      size="32"
      @click="musicStore.isPlaying ? musicStore.pause() : musicStore.play()" />
    <SvgIcon v-else class="action-icon pointer-events-none mx-8 size-10" name="Ring" size="32" />

    <SvgIcon
      class="action-icon mr-16"
      name="NextBold"
      size="24"
      @click="musicStore.playPrevOrNext('next')" />

    <div class="flex flex-1 items-center justify-end gap-1">
      <div class="mx-2">
        {{ formatDuration(playProgress) }} / {{ formatDuration(musicStore.music?.duration ?? 0) }}
      </div>
      <PlayMode />
      <Volume />
      <Quality />
      <DesktopLyric />
      <PlayList />
    </div>
  </div>
</template>
