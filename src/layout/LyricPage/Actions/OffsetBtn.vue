<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue';
import { useLyricStore } from '@/stores/lyric';

const { setOffsetMap } = useLyricStore();
</script>

<template>
  <div class="card cursor-pointer overflow-hidden">
    <ActionButton
      class="h-8 w-full"
      mode="text"
      prefix-icon="ForwardRight"
      title="歌词进度 + 0.2 秒"
      @click="setOffsetMap('add')"
    />

    <div class="h-px w-full bg-border"></div>

    <ActionButton
      class="h-8 w-full"
      mode="text"
      prefix-icon="Restart"
      title="重置歌词进度"
      @click="setOffsetMap('restart')"
    />

    <div class="h-px w-full bg-border"></div>

    <ActionButton
      class="h-8 w-full"
      mode="text"
      prefix-icon="ForwardLeft"
      title="歌词进度 - 0.2 秒"
      @click="setOffsetMap('sub')"
    />
  </div>
</template>
