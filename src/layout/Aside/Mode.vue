<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { computed, onMounted, ref } from 'vue';

import { notify } from '@/components/Notification.vue';
import SelectModal from '@/components/SelectModal.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { Mode } from '@/constants';
import { useStateStore } from '@/stores/state';
import { invoke } from '@/utils/tools';

const stateStore = useStateStore();

const modeVisible = ref(false);
const modeOptions = ref<SelectOption[]>([]);

const modeSelection = computed<SelectOption | undefined>(
  () =>
    modeOptions.value.find((option) => option.value === stateStore.mode) || modeOptions.value[0],
);

const getModeList = async () => {
  try {
    modeOptions.value = await invoke('app_mode_all');
  } catch {
    notify.error('获取播放器模式列表失败');
  }
};

const getMode = async () => {
  try {
    const mode = await invoke('app_mode_get');
    stateStore.setMode(mode);
  } catch {
    notify.error('获取播放器模式失败');
  }
};

const modeSelect = async (mode: Mode) => {
  if (mode === stateStore.mode) return;

  modeVisible.value = false;

  try {
    await invoke('app_mode_set', { mode });

    // TODO: 改变mode需要一个重置全局变量的方法
    // reload()
  } catch {
    notify.error('设置播放器模式失败');
  }
};

onMounted(async () => {
  await getModeList();
  await getMode();
});
</script>

<template>
  <div class="relative" v-on-click-outside="() => (modeVisible = false)">
    <div class="flex cursor-pointer items-center gap-1" @click="modeVisible = !modeVisible">
      <div class="whitespace-nowrap font-bold">
        {{ modeSelection?.label || '未知模式' }}
      </div>
      <SvgIcon
        class="transition-transform"
        :class="modeVisible ? 'rotate-180' : ''"
        name="Down"
        size="16"
      />
    </div>

    <SelectModal
      class="absolute left-1/2 top-full -translate-x-1/2"
      transition="zoom-top"
      :visible="modeVisible"
      :options="modeOptions"
      :selection="modeSelection"
      @select="modeSelect"
    />
  </div>
</template>
