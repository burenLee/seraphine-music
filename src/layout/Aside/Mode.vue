<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { computed, onMounted, ref } from 'vue';

import { notify } from '@/components/Notification.vue';
import SelectModal from '@/components/SelectModal.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useSettingStore } from '@/stores/setting';
import { Mode } from '@/utils/params';
import { invoke } from '@/utils/tools';

const settingStore = useSettingStore();

const modeVisible = ref(false);
const modeOptions = ref<SelectOption[]>([]);

const modeSelection = computed<SelectOption | undefined>(
  () =>
    modeOptions.value.find((option) => option.value === settingStore.mode) || modeOptions.value[0],
);

const getModeList = async () => {
  try {
    modeOptions.value = await invoke('app_mode_all');
  } catch {
    notify.error('无法获取播放器模式列表');
  }
};

const getMode = async () => {
  try {
    const mode = await invoke('app_mode_get');
    settingStore.setMode(mode);
  } catch {
    notify.error('无法获取播放器模式');
  }
};

const modeSelect = async (mode: Mode) => {
  if (mode === settingStore.mode) return;

  try {
    await invoke('app_mode_set', { mode });
    modeVisible.value = false;

    // TODO: 改变mode需要一个重置全局变量的方法
    // reload()
  } catch {
    notify.error('无法设置播放器模式');
  }
};

onMounted(async () => {
  await getModeList();
  await getMode();
});
</script>

<template>
  <div class="relative" v-on-click-outside="() => (modeVisible = false)">
    <div
      class="flex cursor-pointer items-center whitespace-nowrap font-bold text-minor"
      @click="modeVisible = !modeVisible"
    >
      <div class="w-0 flex-1 truncate">{{ modeSelection?.label || '未选择模式' }}</div>
      <SvgIcon class="transition-transform" :class="modeVisible ? 'rotate-180' : ''" name="Down" />
    </div>

    <SelectModal
      class="absolute left-1/2 top-full -translate-x-1/2"
      :visible="modeVisible"
      :options="modeOptions"
      :selection="modeSelection"
      @select="modeSelect"
    />
  </div>
</template>
