import { getVersion } from '@tauri-apps/api/app';
import { relaunch } from '@tauri-apps/plugin-process';
import { Update, check as tauriCheck } from '@tauri-apps/plugin-updater';
import { defineStore } from 'pinia';
import { ref, watch } from 'vue';

import { notify } from '@/components/Notification.vue';

interface UpdateInfo {
  hasUpdate: boolean;
  currentVersion: string;
  latestVersion: string;
  body?: string;
  date?: string;
}

interface DownloadInfo {
  downloaded: number;
  total: number;
  speed: number;
}

/** 更新配置 */
export const useUpdaterStore = defineStore(
  'updater',
  () => {
    const isHydrated = ref(false); // store 持久化的水合状态

    const isChecking = ref(false);
    const isDownloading = ref(false);
    const isDownloaded = ref(false);
    const updateInfo = ref<UpdateInfo>();
    const downloadInfo = ref<DownloadInfo>();

    let updater: Update | undefined = undefined;

    watch(
      isHydrated,
      async () => {
        await check();
      },
      { once: true },
    );

    const check = async () => {
      if (isChecking.value || isDownloading.value) return;

      isChecking.value = true;

      try {
        await updater?.close();
        const update = await tauriCheck();

        if (!update) {
          updater = undefined;

          const currentVersion = await getVersion();
          updateInfo.value = {
            hasUpdate: false,
            currentVersion: `v${currentVersion}`,
            latestVersion: `v${currentVersion}`,
          };

          notify.success('已是最新版本');
        } else {
          updater = update;

          updateInfo.value = {
            hasUpdate: true,
            currentVersion: `v${update.currentVersion}`,
            latestVersion: `v${update.version}`,
            body: update.body,
            date: update.date,
          };

          notify.success(`发现新版本 ${update.version}`);
        }
      } catch {
        updater = undefined;
        notify.error('无法获取新版本');

        const currentVersion = await getVersion();
        updateInfo.value = {
          hasUpdate: false,
          currentVersion: `v${currentVersion}`,
          latestVersion: `v${currentVersion}`,
        };
      } finally {
        isChecking.value = false;
      }
    };

    const download = async () => {
      if (isDownloading.value || !updater) return;

      isDownloading.value = true;
      isDownloaded.value = false;
      downloadInfo.value = undefined;

      try {
        let start = performance.now();
        // 记录上一次进度回调的时间
        let lastProgressTime = start;

        await updater.download((e) => {
          if (!downloadInfo.value) downloadInfo.value = { downloaded: 0, total: 0, speed: 0 };

          switch (e.event) {
            case 'Started':
              start = performance.now();
              lastProgressTime = start;
              downloadInfo.value = { total: e.data.contentLength ?? 0, downloaded: 0, speed: 0 };
              break;
            case 'Progress': {
              const now = performance.now();
              const chunkBytes = e.data.chunkLength;
              downloadInfo.value.downloaded += chunkBytes;

              const deltaSec = (now - lastProgressTime) / 1000;
              if (deltaSec > 0) downloadInfo.value.speed = chunkBytes / deltaSec;

              lastProgressTime = now;
              break;
            }
            case 'Finished':
              isDownloaded.value = true;
              downloadInfo.value.speed = 0;
              break;
          }
        });
      } catch {
        notify.error('无法下载新版本');
      } finally {
        isDownloading.value = false;
      }
    };

    const install = async () => {
      if (!isDownloaded.value || !updater) return;

      try {
        await updater.install();
        await relaunch();
      } catch {
        notify.error('无法安装并重启应用');
      }
    };

    const reset = () => {
      isDownloading.value = false;
      isDownloaded.value = false;
      downloadInfo.value = undefined;
    };

    return {
      isHydrated,
      isChecking,
      isDownloading,
      isDownloaded,
      updateInfo,
      downloadInfo,

      check,
      download,
      install,
      reset,
    };
  },
  {
    persist: {
      key: 'updater-store',
      pick: [],
      afterHydrate: (ctx) => (ctx.store.isHydrated = true),
    },
  },
);
