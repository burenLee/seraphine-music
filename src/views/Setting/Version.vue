<script lang="ts" setup>
import { computed } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import ProgressRange from '@/components/ProgressRange.vue';
import { useUpdaterStore } from '@/stores/updater';
import { formatFileSize } from '@/utils/tools';

const updaterStore = useUpdaterStore();

const progressPercent = computed(() =>
  updaterStore.downloadInfo
    ? Math.ceil((updaterStore.downloadInfo.downloaded / updaterStore.downloadInfo.total) * 100)
    : 0,
);
</script>

<template>
  <div class="flex text-base">
    <div class="w-40 shrink-0 font-bold">关于 Seraphine:</div>

    <div class="space-y-3">
      <!-- 当前版本 + 检查更新按钮 -->
      <div class="flex items-center gap-3">
        <div>
          当前版本
          <span class="font-bold">{{ updaterStore.updateInfo?.currentVersion }}</span>

          <span v-if="updaterStore.updateInfo?.hasUpdate" class="text-info">
            [新版本
            <span class="font-bold">{{ updaterStore.updateInfo.latestVersion }}</span>
            ]
          </span>
        </div>

        <ActionButton
          theme="success"
          :disabled="updaterStore.isChecking || updaterStore.isDownloading"
          @click="updaterStore.check"
        >
          {{ updaterStore.isChecking ? '检查中...' : '检查更新' }}
        </ActionButton>

        <ActionButton
          v-if="updaterStore.updateInfo?.hasUpdate"
          theme="success"
          :disabled="updaterStore.isDownloading"
          @click="updaterStore.download"
        >
          下载更新
        </ActionButton>
      </div>

      <div v-if="!updaterStore.updateInfo?.hasUpdate" class="font-bold text-minor">
        已是最新版本
      </div>
      <div v-else-if="!updaterStore.isDownloaded" class="space-y-3">
        <!-- Changelog（latest.json 的 notes 字段，GitHub 自动生成的 Release Notes） -->
        <pre v-if="updaterStore.updateInfo?.body" class="card max-h-60 overflow-auto p-3 text-sm">{{
          updaterStore.updateInfo.body
        }}</pre>

        <!-- 下载进度 -->
        <div v-if="updaterStore.isDownloading && updaterStore.downloadInfo" class="space-y-2">
          <div class="flex items-center justify-between text-sm">
            <span>正在下载...</span>
            <span>{{ progressPercent }} %</span>
          </div>

          <ProgressRange
            v-model="updaterStore.downloadInfo.downloaded"
            :max="updaterStore.downloadInfo.total"
            showMode="hover"
            :disabled="true"
          />

          <div class="flex items-center justify-between text-xs">
            <span>
              {{ formatFileSize(updaterStore.downloadInfo.downloaded ?? 0) }}
              /
              {{ formatFileSize(updaterStore.downloadInfo.total ?? 0) }}
            </span>

            <span> {{ formatFileSize(updaterStore.downloadInfo.speed) }}/s </span>
          </div>
        </div>
      </div>

      <!-- 下载完成 -->
      <div v-if="updaterStore.isDownloaded" class="space-y-2">
        <div class="text-success">下载完成，安装后将自动重启应用</div>
        <ActionButton theme="success" @click="updaterStore.install">安装更新</ActionButton>
      </div>
    </div>
  </div>
</template>
