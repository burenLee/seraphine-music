<script lang="ts" setup>
import SelectModal from '@/components/SelectModal.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useLyricStore } from '@/stores/lyric'
import { LyricPageMode } from '@/utils/params'
import { vOnClickOutside } from '@vueuse/components'

const lyricStore = useLyricStore()

const pageModeOptions: SelectOption[] = [
  { label: '方形封面', value: LyricPageMode.Cover, prefixIcon: 'Picture' },
  { label: '炫胶唱片', value: LyricPageMode.Record, prefixIcon: 'Album' },
  { label: '歌手写真', value: LyricPageMode.Photo, prefixIcon: 'User', disabled: true }
]

const pageVisible = ref(false)

const pageModeSelection = computed(
  () => pageModeOptions.find((option) => option.value === lyricStore.pageMode) || pageModeOptions[0]
)

const pageModeSelect = (mode: LyricPageMode) => {
  lyricStore.setPageMode(mode)
  pageVisible.value = false
}
</script>

<template>
  <div class="relative" v-on-click-outside="() => (pageVisible = false)">
    <div
      class="w-26 flex cursor-pointer items-center gap-1 p-1"
      @click="pageVisible = !pageVisible">
      <SvgIcon :name="pageModeSelection.prefixIcon || 'Picture'" />
      {{ pageModeSelection.label }}
      <SvgIcon class="transition-transform" :class="{ 'rotate-180': pageVisible }" name="Down" />
    </div>

    <SelectModal
      class="absolute left-1/2 top-full -translate-x-1/2"
      :visible="pageVisible"
      :options="pageModeOptions"
      :selection="pageModeSelection"
      @select="pageModeSelect" />
  </div>
</template>
