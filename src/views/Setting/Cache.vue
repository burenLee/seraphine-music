<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue'
import Modal from '@/components/Modal.vue'
import { notify } from '@/components/Notification'
import { invoke } from '@/utils/tools'

const cacheOptions = ref([
  { label: '音频', value: '' },
  { label: '歌词', value: '' },
  { label: '本地封面', value: '' }
])
const clearVisible = ref(false)
const clearOption = ref<{ label: string; value: string }>()

const getPaths = async () => {
  const path_all = await invoke('system_path_all')
  if (!path_all) return

  cacheOptions.value.forEach((option) => {
    if (option.label === '音频') {
      option.value = path_all.temp
    } else if (option.label === '歌词') {
      option.value = path_all.lyric
    } else if (option.label === '本地封面') {
      option.value = path_all.cover
    }
  })
}

const handlePathSet = () => {}

const handlePathOpen = (option: { label: string; value: string }) => {
  invoke('music_dir_open', { path: option.value })
}

const handlePathClear = (option: { label: string; value: string }) => {
  clearOption.value = option
  clearVisible.value = true
}

const handleClearConfirm = async () => {
  if (!clearOption.value) return
  await invoke('music_file_clear', { dirPath: clearOption.value.value })
  notify.success('清理成功')

  handleClearCancel()
}

const handleClearCancel = () => {
  clearOption.value = undefined
  clearVisible.value = false
}

onMounted(() => {
  getPaths()
})
</script>

<template>
  <div class="flex text-base">
    <div class="font-bold w-40">缓存:</div>

    <div class="space-y-3 flex-1">
      <div v-for="(option, index) in cacheOptions" :key="index" class="flex items-center gap-3">
        <div class="w-20">{{ option.label }}</div>
        <input class="card flex-1 px-2 py-1" :value="option.value" readonly />
        <ActionButton theme="success" @click="handlePathSet" disabled>设置</ActionButton>
        <ActionButton theme="success" @click="handlePathOpen(option)">打开</ActionButton>
        <ActionButton theme="error" @click="handlePathClear(option)">清理</ActionButton>
      </div>
    </div>
  </div>

  <Modal
    v-model="clearVisible"
    class="w-80"
    title="删除"
    @confirm="handleClearConfirm"
    @cancel="handleClearCancel">
    <div class="px-4">
      <span>确认清空</span>
      <span class="font-bold px-1">{{ clearOption?.label || '' }}</span>
      <span>缓存?</span>
    </div>
    <div v-if="clearOption?.label === '本地封面'" class="px-4 mt-2 text-warning">
      tips: 建议不要清除本地封面缓存
    </div>
  </Modal>
</template>
