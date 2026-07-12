<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue'
import Modal from '@/components/Modal.vue'
import { notify } from '@/components/Notification'
import SvgIcon from '@/components/SvgIcon.vue'
import VirtualList from '@/components/VirtualList.vue'
import { useListContext } from '@/utils/hooks'
import { ScanStatus } from '@/utils/params'
import { getFullName, invoke } from '@/utils/tools'
import { OpenDialogOptions, open } from '@tauri-apps/plugin-dialog'

interface Props {
  scanTypes: string[]
}

interface Emits {
  close: []
}

const visible = defineModel({ required: true, default: false })
const { scanTypes: musicTypes } = defineProps<Props>()
const emits = defineEmits<Emits>()

const { listStore, listType, list } = useListContext()

const scanTypes = ref<string[]>([]) // 扫描类型
const scanPaths = ref<string[]>([]) // 扫描路径
const scanStatus = ref(ScanStatus.Ready) // 扫描状态
const scanList = ref<ListMusic[]>([]) // 扫描结果

// 扫描状态文本
const scanStatusText = computed(() => {
  switch (scanStatus.value) {
    case ScanStatus.Ready:
      return '准备扫描'
    case ScanStatus.Loading:
      return '正在扫描...'
    case ScanStatus.Error:
      return '扫描失败'
    case ScanStatus.Success:
      return `扫描完成, 找到 ${scanList.value.length} 首`
    default:
      return ''
  }
})

const dialogOptions: OpenDialogOptions = { directory: true, multiple: true, title: '选择扫描目录' }
const tableColumns = [
  { key: 'index', slot: true, width: '2.5rem', padding: 0 },
  { key: 'info', slot: true, width: 'auto' },
  { key: 'action', slot: true, width: '2.5rem', padding: 0 }
]

const selectAllType = () => {
  scanTypes.value = scanTypes.value.includes('all') ? [] : ['all', ...musicTypes]
}

const selectType = (type: string) => {
  scanTypes.value = scanTypes.value.includes(type)
    ? scanTypes.value.filter((item) => item !== type)
    : scanTypes.value.concat(type)
}

const addPath = async () => {
  const dirPaths = (await open(dialogOptions)) as string[] | null
  if (!dirPaths) return

  scanPaths.value.push(...dirPaths)
}

const removePath = (index: number) => {
  scanPaths.value.splice(index, 1)

  // 重置扫描结果
  scanList.value = []
  scanStatus.value = ScanStatus.Ready
}

const scanMusic = async () => {
  if (scanStatus.value === ScanStatus.Loading) return
  if (!scanPaths.value.length) {
    notify.error('请选择扫描目录')
    return
  }

  scanStatus.value = ScanStatus.Loading

  const music_scan_dir = await invoke('music_scan_dir', {
    dirPaths: scanPaths.value,
    scanTypes: scanTypes.value.filter((type) => type !== 'all'),
    startIndex: list.value.list.length
  })
  // 中断返回null
  if (music_scan_dir === null) {
    scanStatus.value = ScanStatus.Ready
    return
  }
  if (music_scan_dir === undefined) {
    scanStatus.value = ScanStatus.Error
    return
  }

  scanList.value = music_scan_dir
  scanStatus.value = ScanStatus.Success
}

const addMusic = () => {
  if (!scanList.value.length) {
    notify.error('没有可添加歌曲')
    return
  }

  const addLen = listStore.addList(listType, scanList.value)
  notify.success(`成功添加 ${addLen} 首歌曲`)

  handleCancel()
}

const removeMusic = (index: number) => {
  scanList.value.splice(index, 1)
}

const handleReset = () => {
  scanPaths.value = []
  scanList.value = []
  scanStatus.value = ScanStatus.Ready
}

const handleCancel = async () => {
  // 如果正在扫描中,取消扫描
  if (scanStatus.value === ScanStatus.Loading) await invoke('music_scan_cancel')

  handleReset()
  emits('close')
}

watch(
  () => musicTypes,
  (types) => (scanTypes.value = ['all', ...types])
)
</script>

<template>
  <Modal
    class="w-[40rem]"
    v-model="visible"
    title="扫描歌曲"
    confirmLabel="添加"
    @cancel="handleCancel"
    @confirm="addMusic">
    <div class="flex gap-3 px-4">
      <div class="w-1/2">
        <ActionButton theme="success" @click="addPath">添加文件夹</ActionButton>
        <ul class="card mt-3 h-56 overflow-auto p-2">
          <div
            v-if="!scanPaths.length"
            class="flex items-center justify-center size-full flex-col text-minor">
            <SvgIcon name="LinesRemove" size="56" />
            <div class="text-xl font-bold">列表为空</div>
          </div>

          <template v-else>
            <li
              v-for="(path, index) in scanPaths"
              :key="index"
              class="group/path flex items-center justify-between card-hover gap-2 rounded-lg leading-10">
              <div class="truncate text-minor w-10 text-center">
                {{ index + 1 }}
              </div>

              <div class="w-0 flex-1 truncate">{{ path }}</div>

              <SvgIcon
                class="action-icon w-10 hover:text-error"
                name="Close"
                size="10"
                @click="removePath(index)" />
            </li>
          </template>
        </ul>

        <!-- 扫描规则 -->
        <div class="mt-3">
          <div class="font-bold">扫描规则:</div>
          <div class="mt-2 flex gap-2">
            <div class="shrink-0">文件格式:</div>
            <div class="flex flex-wrap gap-2">
              <label class="flex items-center gap-1">
                <input
                  type="checkbox"
                  name="all"
                  :checked="scanTypes.includes('all')"
                  @change="selectAllType" />
                全部
              </label>

              <label
                v-for="(type, index) in musicTypes"
                :key="index"
                class="flex items-center gap-1">
                <input
                  type="checkbox"
                  :name="type"
                  :checked="scanTypes.includes(type)"
                  @change="selectType(type)" />
                {{ type }}
              </label>
            </div>
          </div>
        </div>
      </div>

      <div class="w-1/2">
        <div class="flex items-center justify-between">
          <ActionButton theme="success" @click="scanMusic">开始扫描</ActionButton>
          <div>{{ scanStatusText }}</div>
        </div>

        <VirtualList
          class="card mt-3 h-56 overflow-auto border border-border p-2"
          :line-height="40"
          :columns="tableColumns"
          :list="scanList">
          <template #index="row">
            <div class="truncate text-center text-minor">
              {{ row.index + 1 }}
            </div>
          </template>

          <template #info="row">
            {{ getFullName(row) }}
          </template>

          <template #action="row">
            <SvgIcon
              class="action-icon hover:text-error"
              name="Close"
              size="10"
              @click="removeMusic(row.index)" />
          </template>
        </VirtualList>
      </div>
    </div>

    <template #actions>
      <ActionButton @click="handleReset">重置</ActionButton>
    </template>
  </Modal>
</template>
