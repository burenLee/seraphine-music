<script lang="ts" setup>
import { ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import { useLyricStore } from '@/stores/lyric';
import { IconName } from '@/utils/icons';
import { LyricTextAlign } from '@/utils/params';

const lyricStore = useLyricStore();

const Align_Titles: Record<LyricTextAlign, string> = {
  [LyricTextAlign.Left]: '左对齐',
  [LyricTextAlign.Center]: '居中对齐',
  [LyricTextAlign.Right]: '右对齐',
};

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
    :title="Align_Titles[lyricStore.textAlign]"
    @click="handleClick"
  />
</template>
