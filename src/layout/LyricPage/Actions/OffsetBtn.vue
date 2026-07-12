<script lang="ts" setup>
import SvgIcon from '@/components/SvgIcon.vue'
import { useLyricStore } from '@/stores/lyric'

const { setOffsetMap } = useLyricStore()
</script>

<template>
  <div class="card cursor-pointer overflow-hidden">
    <SvgIcon
      class="action-icon"
      name="ForwardRight"
      title="歌词进度 -0.2 秒"
      @click="setOffsetMap('sub')" />

    <div class="h-px w-full bg-border"></div>

    <SvgIcon
      class="action-icon"
      name="Restart"
      title="重置歌词进度"
      @click="setOffsetMap('restart')" />

    <div class="h-px w-full bg-border"></div>

    <SvgIcon
      class="action-icon"
      name="ForwardLeft"
      title="歌词进度 +0.2 秒"
      @click="setOffsetMap('add')" />
  </div>
</template>
