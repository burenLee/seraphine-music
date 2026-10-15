<script lang="ts" setup>
import { LogicalSize } from '@tauri-apps/api/dpi';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { vOnClickOutside } from '@vueuse/components';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';

import ActionButton from '@/components/ActionButton.vue';
import Modal from '@/components/Modal.vue';
import { notify } from '@/components/Notification.vue';
import SelectModal from '@/components/SelectModal.vue';
import { MenuAction } from '@/constants';
import { useUpdaterStore } from '@/stores/updater';
import { useUserStore } from '@/stores/user';

const router = useRouter();

const userStore = useUserStore();
const updaterStore = useUpdaterStore();

const mainWindow = getCurrentWindow();

const menuVisible = ref(false);
const updateModalVisible = ref(false);

const menuOptions = computed<Array<SelectOption<MenuAction>>>(() => [
  { label: '恢复默认窗口', value: MenuAction.Restore, prefixIcon: 'Restart' },
  { label: '检查更新', value: MenuAction.Update, prefixIcon: 'Refresh' },
  { label: '设置', value: MenuAction.Setting, prefixIcon: 'Setting' },
  {
    label: '退出登录',
    value: MenuAction.Logout,
    prefixIcon: 'Logout',
    disabled: !userStore.userinfo,
  },
  { label: '退出播放器', value: MenuAction.Exit, prefixIcon: 'Exit' },
]);

const handleSelect = (action: MenuAction) => {
  menuVisible.value = false;

  switch (action) {
    case MenuAction.Restore:
      mainWindow.unmaximize();
      mainWindow.setFullscreen(false);
      // size 硬编码在 tauri.conf.json
      mainWindow.setSize(new LogicalSize(1080, 720));
      mainWindow.center();
      break;
    case MenuAction.Update:
      notify.info('检查更新中...');
      handleCheck();
      break;
    case MenuAction.Setting:
      router.push('/setting');
      break;
    case MenuAction.Logout:
      userStore.logout();
      break;
    case MenuAction.Exit:
      mainWindow.close();
      break;
  }
};

const handleCheck = async () => {
  await updaterStore.check();

  if (updaterStore.updateInfo?.hasUpdate) updateModalVisible.value = true;
};

const handleUpdateConfirm = async () => {
  updateModalVisible.value = false;

  await router.push('/setting');
  await updaterStore.download();
};

const handleUpdateCancel = () => {
  updateModalVisible.value = false;
};
</script>

<template>
  <div class="relative" v-on-click-outside="() => (menuVisible = false)">
    <ActionButton
      class="size-8"
      mode="text"
      prefix-icon="Menu"
      icon-size="18"
      @click="menuVisible = !menuVisible"
    />

    <SelectModal
      class="absolute left-1/2 top-full -translate-x-1/2"
      transition="zoom-top"
      :visible="menuVisible"
      :options="menuOptions"
      @select="handleSelect"
    />
  </div>

  <Modal
    :visible="updateModalVisible"
    class="w-80"
    title="版本更新"
    confirm-label="下载更新"
    @confirm="handleUpdateConfirm"
    @cancel="handleUpdateCancel"
  >
    <div class="space-y-3 px-6">
      <div>
        发现新版本
        <span class="font-bold text-info">
          {{ updaterStore.updateInfo?.latestVersion }}
        </span>
      </div>

      <!-- 更新说明（latest.json 的 notes 字段） -->
      <pre
        v-if="updaterStore.updateInfo?.body"
        class="max-h-60 overflow-auto whitespace-pre-wrap text-sm text-minor"
      >
        {{ updaterStore.updateInfo.body }}
      </pre>
    </div>
  </Modal>
</template>
