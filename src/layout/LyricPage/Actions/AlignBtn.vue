<script lang="ts" setup>
import { ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import { AlignTitles, LyricTextAlign } from '@/constants';
import { useLyricStore } from '@/stores/lyric';
import { IconName } from '@/utils/icons';

const lyricStore = useLyricStore();

const iconName = ref<IconName>('AlignLeft');

const handleClick = () => {
  let textAlign = LyricTextAlign.Center;

  switch (lyricStore.textAlign) {
    case LyricTextAlign.Left:
      textAlign = LyricTextAlign.Center;
      iconName.value = 'AlignCenter';
      break;
    case LyricTextAlign.Center:
      textAlign = LyricTextAlign.Right;
      iconName.value = 'AlignRight';
      break;
    case LyricTextAlign.Right:
      textAlign = LyricTextAlign.Left;
      iconName.value = 'AlignLeft';
      break;
  }

  lyricStore.setTextAlign(textAlign);
};
</script>

<template>
  <ActionButton
    class="h-8 w-full px-0"
    :prefix-icon="iconName"
    :title="AlignTitles[lyricStore.textAlign]"
    @click="handleClick"
  />
</template>
