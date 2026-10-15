<script lang="ts" setup>
import { getAllWindows, getCurrentWindow } from '@tauri-apps/api/window';
import { ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Modal from '@/components/Modal.vue';
import { CloseMode } from '@/constants';
import { useSettingStore } from '@/stores/setting';

const settingStore = useSettingStore();

const closeVisible = ref(false);
const closeMode = ref(settingStore.closeMode ?? CloseMode.Hide);

const handleCloseStatus = async (closeMode: CloseMode) => {
  switch (closeMode) {
    case CloseMode.Hide:
      getCurrentWindow().hide();
      break;
    case CloseMode.Exit:
      let allWindows = await getAllWindows();
      allWindows.forEach((window) => window.close());
      break;
  }
};

const handleClose = () => {
  if (settingStore.closeMode === undefined) {
    closeVisible.value = true;
  } else {
    handleCloseStatus(settingStore.closeMode);
  }
};

const handleCancel = () => {
  closeVisible.value = false;
};

const handleConfirm = () => {
  handleCloseStatus(closeMode.value);
  settingStore.setCloseMode(closeMode.value);

  handleCancel();
};
</script>

<template>
  <ActionButton
    class="size-8 hover:text-error"
    mode="text"
    prefix-icon="Close"
    icon-size="20"
    @click="handleClose"
  />

  <Modal
    :visible="closeVisible"
    class="w-80"
    title="关闭窗口"
    @cancel="handleCancel"
    @confirm="handleConfirm"
  >
    <div class="flex items-center gap-3 px-6">
      <label class="flex cursor-pointer items-center gap-1">
        <input type="radio" name="closeAction" v-model="closeMode" :value="CloseMode.Hide" />
        最小化到托盘
      </label>

      <label class="flex cursor-pointer items-center gap-1">
        <input type="radio" name="closeAction" v-model="closeMode" :value="CloseMode.Exit" />
        退出程序
      </label>
    </div>
  </Modal>
</template>
