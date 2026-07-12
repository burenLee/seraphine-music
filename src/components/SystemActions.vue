<script lang="ts" setup>
import Modal from './Modal.vue'
import SvgIcon from './SvgIcon.vue'
import { useSettingStore } from '@/stores/setting'
import { CloseStatus } from '@/utils/params'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useColorMode } from '@vueuse/core'

const colorMode = useColorMode()
const settingStore = useSettingStore()

const currentWindow = getCurrentWindow()

const closeVisible = ref(false)
const closeStatus = ref(settingStore.closeStatus ?? CloseStatus.Hide)

// 最小化
const handleMinimize = () => {
  currentWindow.minimize()
}

// 最大化
const handleMaximize = async () => {
  await currentWindow.toggleMaximize()

  const isMaximized = await currentWindow.isMaximized()
  settingStore.toggleMaximizedState(isMaximized)
}

// 关闭
const handleClose = () => {
  if (settingStore.closeStatus === undefined) {
    closeVisible.value = true
  } else {
    handleCloseStatus(settingStore.closeStatus)
  }
}

const handleCloseStatus = (closeStatus: CloseStatus) => {
  switch (closeStatus) {
    case CloseStatus.Hide:
      currentWindow.hide()
      break
    case CloseStatus.Exit:
      currentWindow.close()
      break
  }
}

const handleCancel = () => {
  closeVisible.value = false
}

const handleConfirm = () => {
  settingStore.setCloseStatus(closeStatus.value)
  handleCloseStatus(closeStatus.value)
  handleCancel()
}
</script>

<template>
  <SvgIcon
    class="action-icon"
    :name="colorMode === 'dark' ? 'Moon' : 'Sun'"
    title="主题"
    size="18"
    @click="colorMode = colorMode === 'dark' ? 'light' : 'dark'" />

  <SvgIcon class="action-icon" name="PIP" size="18" :disabled="true" />

  <SvgIcon class="action-icon" name="Minimize" @click="handleMinimize" />

  <SvgIcon
    class="action-icon"
    :name="!settingStore.isMaximized ? 'Maximize' : 'Restore'"
    size="14"
    @click="handleMaximize" />

  <SvgIcon class="action-icon hover:text-error" name="Close" size="14" @click="handleClose" />

  <Modal
    v-model="closeVisible"
    class="w-80"
    title="关闭"
    @cancel="handleCancel"
    @confirm="handleConfirm">
    <div class="px-6">
      <label class="flex items-center gap-2">
        <input type="radio" name="closeAction" v-model="closeStatus" :value="CloseStatus.Hide" />
        最小化到托盘
      </label>

      <label class="flex items-center mt-2 gap-2">
        <input type="radio" name="closeAction" v-model="closeStatus" :value="CloseStatus.Exit" />
        退出程序
      </label>
    </div>
  </Modal>
</template>
