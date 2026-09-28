<script lang="ts" setup>
import { LogicalSize } from '@tauri-apps/api/dpi';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { vOnClickOutside } from '@vueuse/components';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';

import Modal from '@/components/Modal.vue';
import { notify } from '@/components/Notification.vue';
import SelectModal from '@/components/SelectModal.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useUpdaterStore } from '@/stores/updater';
import { useUserStore } from '@/stores/user';
import { MenuAction } from '@/utils/params';

const router = useRouter();

const userStore = useUserStore();
const updaterStore = useUpdaterStore();

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

const menuVisible = ref(false);
const updateModalVisible = ref(false);

const handleSelect = (action: MenuAction) => {
  switch (action) {
    case MenuAction.Restore:
      const mainWindow = getCurrentWindow();
      mainWindow.unmaximize();
      mainWindow.setFullscreen(false);
      mainWindow.setSize(new LogicalSize(1152, 768));
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
      getCurrentWindow().close();
      break;
  }

  menuVisible.value = false;
};

const handleCheck = async () => {
  await updaterStore.check();

  if (updaterStore.updateInfo?.hasUpdate) updateModalVisible.value = true;
};

const handleUpdateConfirm = () => {
  updateModalVisible.value = false;
  router.push('/setting');

  updaterStore.download();
};

const handleUpdateCancel = () => {
  updateModalVisible.value = false;
};
</script>

<template>
  <div class="relative" v-on-click-outside="() => (menuVisible = false)">
    <SvgIcon class="action-icon" name="Menu" size="20" @click="menuVisible = !menuVisible" />

    <SelectModal
      class="absolute left-1/2 top-full -translate-x-1/2"
      transition="zoom-top"
      :visible="menuVisible"
      :options="menuOptions"
      @select="handleSelect"
    />
  </div>

  <Modal
    class="w-80"
    v-model="updateModalVisible"
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
        >{{ updaterStore.updateInfo.body }}</pre>
    </div>
  </Modal>
</template>
