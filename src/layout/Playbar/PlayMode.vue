<script lang="ts" setup>
import SelectModal from '@/components/SelectModal.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useMusicStore } from '@/stores/music'
import { PlayingMode } from '@/utils/params'
import { vOnClickOutside } from '@vueuse/components'

const musicStore = useMusicStore()

const modeOptions: SelectOption[] = [
  { label: '顺序播放', value: PlayingMode.OrderPlay, prefixIcon: 'OrderPlay' },
  { label: '单曲播放', value: PlayingMode.SinglePlay, prefixIcon: 'SinglePlay' },
  { label: '列表循环', value: PlayingMode.OrderLoop, prefixIcon: 'RepeatAll' },
  { label: '单曲循环', value: PlayingMode.SingleLoop, prefixIcon: 'RepeatOne' },
  { label: '随机播放', value: PlayingMode.RandomPlay, prefixIcon: 'RandomPlay' }
]

const modeVisible = ref(false)

const modeSelection = computed(
  () => modeOptions.find((option) => option.value === musicStore.mode) || modeOptions[0]
)

const modeSelect = (mode: PlayingMode) => {
  musicStore.setMode(mode)
  modeVisible.value = false
}
</script>

<template>
  <div class="relative" v-on-click-outside="() => (modeVisible = false)">
    <SvgIcon
      class="action-icon"
      :name="modeSelection.prefixIcon || 'OrderPlay'"
      size="18"
      @click="modeVisible = !modeVisible" />

    <SelectModal
      class="absolute bottom-full left-1/2 mb-2 -translate-x-1/2"
      transition="zoom-bottom"
      :visible="modeVisible"
      :options="modeOptions"
      :selection="modeSelection"
      @select="modeSelect" />
  </div>
</template>
