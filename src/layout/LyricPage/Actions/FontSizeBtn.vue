<script lang="ts" setup>
import SvgIcon from '@/components/SvgIcon.vue'
import { useLyricStore } from '@/stores/lyric'
import { LyricFontSize } from '@/utils/params'

const lyricStore = useLyricStore()
</script>

<template>
  <div class="card cursor-pointer overflow-hidden">
    <SvgIcon
      class="action-icon"
      name="ZoomIn"
      title="增大歌词字体"
      :disabled="lyricStore.setting.fontSize >= LyricFontSize.Max"
      @click="lyricStore.setFontSize('add')" />

    <div class="h-px w-full bg-border"></div>

    <SvgIcon
      class="action-icon"
      name="Restart"
      title="重置歌词字体大小"
      @click="lyricStore.setFontSize('restart')" />

    <div class="h-px w-full bg-border"></div>

    <SvgIcon
      class="action-icon"
      name="ZoomOut"
      title="减小歌词字体"
      :disabled="lyricStore.setting.fontSize <= LyricFontSize.Min"
      @click="lyricStore.setFontSize('sub')" />
  </div>
</template>
