<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue'
import { useSettingStore } from '@/stores/setting'

const settingStore = useSettingStore()
</script>

<template>
  <div class="flex text-base">
    <div class="font-bold w-40">关于 Seraphine:</div>

    <div>
      <div class="flex items-center gap-3">
        <div>当前版本 v{{ settingStore.version }}</div>

        <ActionButton theme="success" disabled>检查更新</ActionButton>
      </div>

      <div class="flex gap-6 mt-3" :data-disabled="true">
        <label class="flex items-center gap-2 cursor-pointer">
          <input type="radio" name="closeAction" />
          自动更新
        </label>

        <label class="flex items-center gap-2 cursor-pointer">
          <input type="radio" name="closeAction" />
          有新版本提醒我
        </label>
      </div>
    </div>
  </div>
</template>
