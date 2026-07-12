<script lang="ts" setup>
import AddModal from './AddModal.vue'
import { notify } from '@/components/Notification.tsx'
import SelectModal from '@/components/SelectModal.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useListContext } from '@/utils/hooks'
import { AddMusicType } from '@/utils/params'
import { invoke } from '@/utils/tools'
import { open } from '@tauri-apps/plugin-dialog'
import { vOnClickOutside } from '@vueuse/components'

const { listStore, listType, list } = useListContext()

const scanTypes = ref<string[]>([])
const addVisible = ref(false)
const addModalVisible = ref(false)

const addOptions: Array<SelectOption<AddMusicType>> = [
  { label: '添加歌曲', value: AddMusicType.Add, prefixIcon: 'AddMusic' },
  { label: '扫描歌曲', value: AddMusicType.Scan, prefixIcon: 'AddFolder' }
]

const handleAddSelect = async (addType: AddMusicType) => {
  addVisible.value = false

  switch (addType) {
    case AddMusicType.Add:
      addList()
      break
    case AddMusicType.Scan:
      addModalVisible.value = true
      break
  }
}

const addList = async () => {
  const filePaths = await open({
    multiple: true,
    title: '添加歌曲',
    filters: [{ name: '', extensions: scanTypes.value }]
  })
  if (!filePaths) return

  const music_scan_file = await invoke('music_scan_file', {
    filePaths,
    startIndex: list.value.list.length
  })
  if (!music_scan_file) return

  const addLen = listStore.addList(listType, music_scan_file)
  notify.success(`成功添加 ${addLen} 首歌曲`)
}

// 获取可扫描文件类型
const getScanTypes = async () => {
  const music_type = await invoke('music_scan_type')
  if (!music_type) return

  scanTypes.value = music_type
}

onMounted(getScanTypes)
</script>

<template>
  <div class="relative" v-on-click-outside="() => (addVisible = false)">
    <SvgIcon
      class="action-icon card-hover transition-colors rounded-lg hover:text-foreground"
      name="Plus"
      @click="addVisible = !addVisible" />

    <SelectModal
      class="absolute left-1/2 top-full -translate-x-1/2"
      transition="zoom-top"
      :visible="addVisible"
      :options="addOptions"
      @select="handleAddSelect" />

    <AddModal v-model="addModalVisible" :scan-types="scanTypes" @close="addModalVisible = false" />
  </div>
</template>
