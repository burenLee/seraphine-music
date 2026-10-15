<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue';
import { LyricFontSize } from '@/constants';
import { useLyricStore } from '@/stores/lyric';

const lyricStore = useLyricStore();
</script>

<template>
  <div class="card cursor-pointer overflow-hidden">
    <action-button
      class="h-8 w-full"
      mode="text"
      prefix-icon="ZoomIn"
      title="增大歌词字体"
      :disabled="lyricStore.fontSize >= LyricFontSize.Max"
      @click="lyricStore.setFontSize('add')"
    />

    <div class="h-px w-full bg-border"></div>

    <action-button
      class="h-8 w-full"
      mode="text"
      prefix-icon="Restart"
      title="重置歌词字体大小"
      @click="lyricStore.setFontSize('restart')"
    />

    <div class="h-px w-full bg-border"></div>

    <action-button
      class="h-8 w-full"
      mode="text"
      prefix-icon="ZoomOut"
      title="减小歌词字体"
      :disabled="lyricStore.fontSize <= LyricFontSize.Min"
      @click="lyricStore.setFontSize('sub')"
    />
  </div>
</template>
