<script lang="ts" setup>
import { useSettingStore } from '@/stores/setting'
import { CloseStatus } from '@/utils/params'

const settingStore = useSettingStore()
</script>

<template>
  <div class="flex items-center text-base">
    <div class="font-bold w-40">关闭主窗口:</div>

    <div class="flex gap-6">
      <label class="flex items-center gap-2 cursor-pointer">
        <input
          type="radio"
          name="closeAction"
          :checked="settingStore.closeStatus === CloseStatus.Hide"
          @change="settingStore.setCloseStatus(CloseStatus.Hide)" />
        最小化到托盘
      </label>

      <label class="flex items-center gap-2 cursor-pointer">
        <input
          type="radio"
          name="closeAction"
          :checked="settingStore.closeStatus === CloseStatus.Exit"
          @change="settingStore.setCloseStatus(CloseStatus.Exit)" />
        退出程序
      </label>
    </div>
  </div>
</template>
