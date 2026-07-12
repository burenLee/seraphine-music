<script lang="ts" setup>
import SelectModal from '@/components/SelectModal.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { invoke } from '@/utils/tools'
import { vOnClickOutside } from '@vueuse/components'

const modeVisible = ref(false)
const modeOptions = ref<SelectOption[]>([])
const modeSelection = ref<SelectOption>()

const getModeList = async () => {
  const http_mode_list = await invoke('http_mode_list')
  if (http_mode_list) modeOptions.value = http_mode_list
}

const getMode = async () => {
  const http_mode_get = await invoke('http_mode_get')
  modeSelection.value = modeOptions.value.find((option) => option.value === http_mode_get)
}

const modeSelect = async (mode: string) => {
  const existMode = modeOptions.value.find((option) => option.value === mode)
  if (!existMode) return

  await invoke('http_mode_set', { mode })
  modeSelection.value = existMode
  modeVisible.value = false
  window.location.reload()
}

onMounted(async () => {
  await getModeList()
  await getMode()
})
</script>

<template>
  <div class="relative" v-on-click-outside="() => (modeVisible = false)">
    <div
      class="flex cursor-pointer items-center whitespace-nowrap font-bold text-minor"
      @click="modeVisible = !modeVisible">
      <div class="flex-1 w-0 truncate">{{ modeSelection?.label || '未选择模式' }}</div>
      <SvgIcon class="transition-transform" :class="modeVisible ? 'rotate-180' : ''" name="Down" />
    </div>

    <SelectModal
      class="absolute left-1/2 top-full -translate-x-1/2"
      :visible="modeVisible"
      :options="modeOptions"
      :selection="modeSelection"
      @select="modeSelect" />
  </div>
</template>
