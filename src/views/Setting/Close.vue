<script lang="ts" setup>
import { CloseMode } from '@/constants';
import { useSettingStore } from '@/stores/setting';

const settingStore = useSettingStore();
</script>

<template>
  <div class="flex items-center text-base">
    <div class="w-40 font-bold">关闭主窗口:</div>

    <div class="flex gap-6">
      <label class="flex cursor-pointer items-center gap-1">
        <input
          type="radio"
          name="closeAction"
          :checked="settingStore.closeMode === CloseMode.Hide"
          @change="settingStore.setCloseMode(CloseMode.Hide)"
        />
        最小化到托盘
      </label>

      <label class="flex cursor-pointer items-center gap-1">
        <input
          type="radio"
          name="closeAction"
          :checked="settingStore.closeMode === CloseMode.Exit"
          @change="settingStore.setCloseMode(CloseMode.Exit)"
        />
        退出程序
      </label>
    </div>
  </div>
</template>
