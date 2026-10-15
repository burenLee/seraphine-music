<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { computed, ref } from 'vue';

import SelectModal from '@/components/SelectModal.vue';
import { PlayingOrigin, PlayingQuality } from '@/constants';
import { useMusicStore } from '@/stores/music';

const musicStore = useMusicStore();

const qualityOptions: Array<SelectOption<PlayingQuality>> = [
  { label: 'Hi_Res音质', value: PlayingQuality.BitrateHigh },
  { label: '无损音质', value: PlayingQuality.BitrateFlac },
  { label: '高品音质', value: PlayingQuality.Bitrate320 },
  { label: '标准音质', value: PlayingQuality.Bitrate128 },
];

const qualityVisible = ref(false);

const qualitySelection = computed(
  () =>
    qualityOptions.find((option) => option.value === musicStore.quality) ||
    qualityOptions[qualityOptions.length - 1],
);

const modeSelect = async (quality: PlayingQuality) => {
  const wasPlaying = musicStore.isPlaying;

  musicStore.setQuality(quality);
  await musicStore.setMusic(musicStore.music, {
    origin: musicStore.origin,
    loop: true,
    autoPlay: false,
  });

  if (wasPlaying) await musicStore.play();

  qualityVisible.value = false;
};
</script>

<template>
  <div class="relative px-2" v-on-click-outside="() => (qualityVisible = false)">
    <div
      class="action-icon size-8 text-center font-bold leading-8"
      :data-disabled="musicStore.origin === PlayingOrigin.Local"
      @click="qualityVisible = !qualityVisible"
    >
      {{ qualitySelection.label.slice(0, -2) }}
    </div>

    <SelectModal
      class="absolute bottom-full left-1/2 -translate-x-1/2"
      transition="zoom-bottom"
      :visible="qualityVisible"
      :options="qualityOptions"
      :selection="qualitySelection"
      @select="modeSelect"
    />
  </div>
</template>
