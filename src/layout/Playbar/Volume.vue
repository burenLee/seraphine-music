<script lang="ts" setup>
import ProgressRange from '@/components/ProgressRange.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useMusicStore } from '@/stores/music'
import { IconName } from '@/utils/icons'

const musicStore = useMusicStore()

const volumnIcon = computed<IconName>(() =>
  musicStore.volume > 50 ? 'VolumeLoud' : musicStore.volume > 0 ? 'VolumeSmall' : 'VolumeOff'
)
</script>

<template>
  <div class="group/volumn relative">
    <SvgIcon
      class="action-icon"
      :name="volumnIcon"
      size="18"
      @click="musicStore.setVolume(musicStore.volume ? 0 : musicStore.lastVolumn)" />

    <div
      class="pointer-events-none absolute bottom-full left-1/2 z-10 -translate-x-1/2 pb-2 opacity-0 transition-opacity group-hover/volumn:pointer-events-auto group-hover/volumn:opacity-100">
      <div
        class="flex items-center justify-between card border border-border w-14 flex-col bg-background pb-3 pt-4 shadow-md shadow-shadow">
        <ProgressRange
          class="h-32"
          direction="vertical"
          :model-value="musicStore.volume"
          @update:model-value="musicStore.setVolume($event)" />

        <div class="mt-2 text-center">{{ musicStore.volume }}%</div>
      </div>
    </div>
  </div>
</template>
