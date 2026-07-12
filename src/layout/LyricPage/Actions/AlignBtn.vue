<script lang="ts" setup>
import SvgIcon from '@/components/SvgIcon.vue'
import { useLyricStore } from '@/stores/lyric'
import { IconName } from '@/utils/icons'
import { LyricTextAlign } from '@/utils/params'

const lyricStore = useLyricStore()

const Align_Titles = {
  [LyricTextAlign.Left]: '左对齐',
  [LyricTextAlign.Center]: '居中对齐',
  [LyricTextAlign.Right]: '右对齐'
}

const iconName = ref<IconName>('AlignLeft')

const handleClick = () => {
  let mode = LyricTextAlign.Center

  switch (lyricStore.setting.textAlign) {
    case LyricTextAlign.Left:
      mode = LyricTextAlign.Center
      iconName.value = 'AlignCenter'
      break
    case LyricTextAlign.Center:
      mode = LyricTextAlign.Right
      iconName.value = 'AlignRight'
      break
    case LyricTextAlign.Right:
      mode = LyricTextAlign.Left
      iconName.value = 'AlignLeft'
      break
  }

  lyricStore.setTextAlign(mode)
}
</script>

<template>
  <SvgIcon
    class="action-icon card"
    :name="iconName"
    :title="Align_Titles[lyricStore.setting.textAlign]"
    @click="handleClick" />
</template>
