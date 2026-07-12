<script lang="ts" setup>
import { useLyricStore } from '@/stores/lyric'
import { LyricTransMode } from '@/utils/params'

const lyricStore = useLyricStore()

const handleClick = (mode: LyricTransMode) => {
  lyricStore.setTransMode(lyricStore.setting.transMode === mode ? LyricTransMode.Off : mode)
}
</script>

<template>
  <div class="card cursor-pointer overflow-hidden">
    <div
      class="h-8 w-full text-center leading-8 transition-colors"
      :class="lyricStore.setting.transMode === LyricTransMode.Trans ? 'card-actived' : 'card-hover'"
      title="翻译"
      @click="handleClick(LyricTransMode.Trans)">
      译
    </div>

    <div class="h-px w-full bg-border"></div>

    <div
      class="h-8 w-full text-center leading-8 transition-colors"
      :class="lyricStore.setting.transMode === LyricTransMode.Roman ? 'card-actived' : 'card-hover'"
      title="音译"
      @click="handleClick(LyricTransMode.Roman)">
      音
    </div>
  </div>
</template>
