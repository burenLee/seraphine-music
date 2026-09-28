<script lang="ts" setup>
import { onMounted, ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Modal from '@/components/Modal.vue';
import { notify } from '@/components/Notification.vue';
import { invoke, openDir } from '@/utils/tools';

interface CacheOption {
  label: string;
  value: string;
}

const cacheOptions = ref<CacheOption[]>([
  { label: '音频', value: '' },
  { label: '歌词', value: '' },
  { label: '封面(本地音频)', value: '' },
  { label: '日志', value: '' },
]);
const clearOption = ref<CacheOption>();
const clearVisible = ref(false);

const getPaths = async () => {
  try {
    const path_all = await invoke('app_dirs_all');

    cacheOptions.value.forEach((option) => {
      switch (option.label) {
        case '音频':
          option.value = path_all.audio;
          break;
        case '歌词':
          option.value = path_all.lyric;
          break;
        case '封面(本地音频)':
          option.value = path_all.cover;
          break;
        case '日志':
          option.value = path_all.app_log;
          break;
      }
    });
  } catch {
    notify.error('获取缓存路径失败');
  }
};

const setPath = () => {
  // TODO: 设置缓存路径
};

const clearCache = (option: CacheOption) => {
  clearOption.value = option;
  clearVisible.value = true;
};

const handleClearConfirm = async () => {
  if (!clearOption.value) return;

  try {
    await invoke('app_dirs_clear', { path: clearOption.value.value });

    notify.success('清理成功');
    handleClearCancel();
  } catch {
    notify.error('清理失败');
  }
};

const handleClearCancel = () => {
  clearOption.value = undefined;
  clearVisible.value = false;
};

onMounted(() => {
  getPaths();
});
</script>

<template>
  <div class="flex text-base">
    <div class="w-40 font-bold">缓存:</div>

    <div class="flex-1 space-y-3">
      <div v-for="(option, index) in cacheOptions" :key="index" class="flex items-center gap-3">
        <div class="w-32">{{ option.label }}</div>
        <input class="card min-w-0 flex-1 truncate px-2 py-1" :value="option.value" readonly />
        <ActionButton theme="success" @click="setPath()" disabled>设置</ActionButton>
        <ActionButton theme="success" @click="openDir(option.value)">打开</ActionButton>
        <ActionButton theme="error" @click="clearCache(option)">清理</ActionButton>
      </div>
    </div>
  </div>

  <Modal
    v-model="clearVisible"
    class="w-80"
    title="清空缓存"
    @confirm="handleClearConfirm"
    @cancel="handleClearCancel"
  >
    <div class="px-6">
      <span>确认清空</span>
      <span class="px-1 font-bold">{{ clearOption?.label || '' }}</span>
      <span>缓存?</span>
    </div>
  </Modal>
</template>
