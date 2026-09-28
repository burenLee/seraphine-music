<script lang="ts" setup>
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { getAllWindows, getCurrentWindow } from '@tauri-apps/api/window';
import { vOnClickOutside } from '@vueuse/components';
import { useColorMode } from '@vueuse/core';
import { computed, ref } from 'vue';

import Modal from '@/components/Modal.vue';
import { notify } from '@/components/Notification.vue';
import SelectModal from '@/components/SelectModal.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useMiniPlayerBridge } from '@/composables/useMiniPlayerBridge';
import { useSettingStore } from '@/stores/setting';
import { IconName } from '@/utils/icons';
import { CloseStatus, ThemeMode, WindowTarget, miniPlayerSize } from '@/utils/params';

const mainWindow = getCurrentWindow();

const colorMode = useColorMode({ emitAuto: true });
const settingStore = useSettingStore();
const miniBridge = useMiniPlayerBridge();

const closeVisible = ref(false);
const themeVisible = ref(false);
const closeStatus = ref(settingStore.closeStatus ?? CloseStatus.Hide);

const themeOptions: Array<SelectOption<ThemeMode>> = [
  { label: '浅色', value: ThemeMode.Light, prefixIcon: 'Sun' },
  { label: '深色', value: ThemeMode.Dark, prefixIcon: 'Moon' },
  { label: '跟随系统', value: ThemeMode.Auto, prefixIcon: 'Laptop' },
];

const themeSelection = computed<SelectOption<ThemeMode>>(
  () => themeOptions.find((item) => item.value === colorMode.value) || themeOptions[2],
);

const themeIcon = computed<IconName>(() => {
  switch (colorMode.value) {
    case ThemeMode.Light:
      return 'Sun';
    case ThemeMode.Dark:
      return 'Moon';
    case ThemeMode.Auto:
      return 'Laptop';
    default:
      return 'Sun';
  }
});
const maxIcon = computed<IconName>(() => (settingStore.isMaximized ? 'Restore' : 'Square'));

const handleTheme = (mode: ThemeMode) => {
  colorMode.value = mode;

  themeVisible.value = false;
};

const handleMiniPlayer = async () => {
  let miniWindow = await WebviewWindow.getByLabel(WindowTarget.MiniPlayer);

  if (!miniWindow) {
    const scaleFactor = await mainWindow.scaleFactor();

    const { width, height } = miniPlayerSize;
    const { x, y } = settingStore.miniPlayerPosition;
    const logicalX = x / scaleFactor || Math.round(window.screen.availWidth - width - 16);
    const logicalY = y / scaleFactor || 48;

    miniWindow = new WebviewWindow(WindowTarget.MiniPlayer, {
      title: 'Seraphine 迷你播放器',
      url: '/mini-player.html',
      width,
      height,
      x: logicalX,
      y: logicalY,
      transparent: true,
      decorations: false,
      shadow: false,
      alwaysOnTop: true,
      skipTaskbar: true,
      resizable: false,
    });

    miniWindow.once('tauri://error', () => {
      notify.error('迷你播放器创建失败');
    });
    miniWindow.once('tauri://created', () => {
      mainWindow.hide();
      miniBridge.start();
    });
    miniWindow.once('tauri://destroyed', () => {
      mainWindow.show();
    });
  } else {
    miniBridge.stop();
  }
};

// 最小化
const handleMinimize = () => {
  mainWindow.minimize();
};

// 最大化
const handleMaximize = async () => {
  await mainWindow.toggleMaximize();

  const isMaximized = await mainWindow.isMaximized();
  settingStore.toggleMaximizedState(isMaximized);
};

const handleCloseStatus = async (closeStatus: CloseStatus) => {
  switch (closeStatus) {
    case CloseStatus.Hide:
      mainWindow.hide();
      break;
    case CloseStatus.Exit:
      const lyricWindow = (await getAllWindows()).find(
        (win) => win.label === WindowTarget.DesktopLyric,
      );
      if (lyricWindow) lyricWindow.close();

      mainWindow.close();
      break;
  }
};

const handleClose = () => {
  if (settingStore.closeStatus === undefined) {
    closeVisible.value = true;
  } else {
    handleCloseStatus(settingStore.closeStatus);
  }
};

const handleCancel = () => {
  closeVisible.value = false;
};

const handleConfirm = () => {
  settingStore.setCloseStatus(closeStatus.value);
  handleCloseStatus(closeStatus.value);
  handleCancel();
};
</script>

<template>
  <div class="relative" v-on-click-outside="() => (themeVisible = false)">
    <SvgIcon
      class="action-icon"
      :name="themeIcon"
      title="主题"
      size="18"
      @click="themeVisible = !themeVisible"
    />

    <SelectModal
      class="absolute left-1/2 top-full -translate-x-1/2"
      transition="zoom-top"
      :visible="themeVisible"
      :options="themeOptions"
      :selection="themeSelection"
      @select="handleTheme"
    />
  </div>
  <SvgIcon class="action-icon" name="PIP" title="迷你播放器" size="18" @click="handleMiniPlayer" />
  <SvgIcon class="action-icon" name="Remove" title="最小化" size="20" @click="handleMinimize" />
  <SvgIcon class="action-icon" :name="maxIcon" title="最大化" size="18" @click="handleMaximize" />
  <SvgIcon class="action-icon hover:text-error" name="Close" size="20" @click="handleClose" />

  <Modal
    v-model="closeVisible"
    class="w-80"
    title="关闭窗口"
    @cancel="handleCancel"
    @confirm="handleConfirm"
  >
    <div class="px-6">
      <label class="flex items-center gap-2">
        <input type="radio" name="closeAction" v-model="closeStatus" :value="CloseStatus.Hide" />
        最小化到托盘
      </label>

      <label class="mt-2 flex items-center gap-2">
        <input type="radio" name="closeAction" v-model="closeStatus" :value="CloseStatus.Exit" />
        退出程序
      </label>
    </div>
  </Modal>
</template>
