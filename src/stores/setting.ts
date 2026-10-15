import { listen } from '@tauri-apps/api/event';
import {
  isRegistered,
  register,
  unregister,
  unregisterAll,
} from '@tauri-apps/plugin-global-shortcut';
import { defineStore } from 'pinia';
import { ref, watch } from 'vue';

import { notify } from '@/components/Notification.vue';
import { AutoStartMode, CloseMode, ShortcutKey } from '@/constants';

import { useMusicStore } from './music';

const DefaultShortcut: Record<ShortcutKey, string> = {
  playOrPause: 'Alt+F5',
  addVolumn: 'Alt+Up',
  subVolumn: 'Alt+Down',
  mute: 'Alt+S',
  prev: 'Alt+Left',
  next: 'Alt+Right',
  backward: 'Ctrl+Alt+Left',
  forward: 'Ctrl+Alt+Right',
};

/** 全局设置 */
export const useSettingStore = defineStore(
  'setting',
  () => {
    const isHydrated = ref(false); // store 持久化的水合状态

    const autoYouthVip = ref(true); // 自动领取概念版会员
    const autoStart = ref(false); // 开机自启状态
    const autoStartMode = ref(AutoStartMode.Foreground); // 开机自启模式
    const closeMode = ref<CloseMode>(); // 关闭按钮状态
    const globalShortcut = ref(true); // 全局快捷键状态
    const mediaShortcut = ref(true); // 媒体快捷键状态
    const shortcutMap = ref({ ...DefaultShortcut }); // 快捷键映射
    const device = ref(''); // 设备ID

    const musicStore = useMusicStore();

    watch(
      isHydrated,
      async () => {
        await unregisterAll();

        if (globalShortcut.value) registerAllGlobalShortcut();
        if (mediaShortcut.value) registerMediaCommandListener();
      },
      { once: true },
    );

    const toggleAutoYouthVip = () => {
      autoYouthVip.value = !autoYouthVip.value;
    };

    const toggleAutoStart = (state: boolean) => {
      autoStart.value = state;
    };

    const setAutoStartMode = (mode: AutoStartMode) => {
      autoStartMode.value = mode;
    };

    const setCloseMode = (status: CloseMode) => {
      closeMode.value = status;
    };

    const toggleGlobalShortcut = () => {
      globalShortcut.value = !globalShortcut.value;

      if (globalShortcut.value) {
        registerAllGlobalShortcut();
      } else {
        unregisterAllGlobalShortcut();
      }
    };

    const toggleMediaShortcut = () => {
      mediaShortcut.value = !mediaShortcut.value;

      // 媒体键由系统原生通道接管（SMTC / MPRIS / MPRemoteCommandCenter），
      // 仅需在开启时监听 media:command 事件，关闭时监听器内部的守卫自动忽略。
      if (mediaShortcut.value) registerMediaCommandListener();
    };

    const setDevice = (id: string) => {
      device.value = id;
    };

    const setShortcutMap = (key: ShortcutKey, value: string) => {
      shortcutMap.value[key] = value;
    };

    const resetShortcutMap = () => {
      shortcutMap.value = { ...DefaultShortcut };
    };

    // 注册全局快捷键
    const registerGlobalShortcut = async (type: ShortcutKey) => {
      const shortcut = shortcutMap.value[type];
      if (!shortcut) return;

      const parts = shortcut.split('+').map((p) => p.trim());
      const modifierKeys = ['Ctrl', 'Shift', 'Alt', 'Meta'];
      const mainKeys = parts.filter((p) => !modifierKeys.includes(p));

      if (mainKeys.length !== 1) {
        setShortcutMap(type, '');
        return;
      }

      if (await isRegistered(shortcut)) return;

      try {
        switch (type) {
          case 'playOrPause':
            await register(shortcut, async (e) => {
              if (e.state === 'Released') return;

              if (musicStore.isPlaying) {
                await musicStore.pause();
              } else {
                await musicStore.play();
              }
            });
            break;
          case 'addVolumn':
            await register(shortcut, async (e) => {
              if (e.state === 'Released') return;
              await musicStore.setVolume(musicStore.volume + 5);
            });
            break;
          case 'subVolumn':
            await register(shortcut, async (e) => {
              if (e.state === 'Released') return;
              await musicStore.setVolume(musicStore.volume - 5);
            });
            break;
          case 'mute':
            await register(shortcut, async (e) => {
              if (e.state === 'Released') return;

              await musicStore.setVolume(musicStore.volume ? 0 : musicStore.lastVolumn);
            });
            break;
          case 'prev':
            await register(shortcut, (e) => {
              if (e.state === 'Released') return;
              musicStore.playPrevOrNext('prev');
            });
            break;
          case 'next':
            await register(shortcut, (e) => {
              if (e.state === 'Released') return;
              musicStore.playPrevOrNext('next');
            });
            break;
          case 'forward':
            await register(shortcut, async (e) => {
              if (e.state === 'Released') return;
              await musicStore.seek(musicStore.playProgress + 5);
            });
            break;
          case 'backward':
            await register(shortcut, async (e) => {
              if (e.state === 'Released') return;
              await musicStore.seek(musicStore.playProgress - 5);
            });
            break;
        }
      } catch {
        mediaShortcut.value = false;
        notify.error('全局快捷键注册失败, 可能被占用');
      }
    };

    const registerAllGlobalShortcut = async () => {
      for (const key of Object.keys(shortcutMap.value)) {
        await registerGlobalShortcut(key as ShortcutKey);
      }
    };

    // 注销全局快捷键
    const unregisterGlobalShortcut = async (type: ShortcutKey) => {
      const shortcut = shortcutMap.value[type];
      if (!shortcut) return;

      await unregister(shortcut);
    };

    const unregisterAllGlobalShortcut = async () => {
      for (const key of Object.keys(shortcutMap.value)) {
        await unregisterGlobalShortcut(key as ShortcutKey);
      }
    };

    // 监听系统媒体键（SMTC / MPRIS / MPRemoteCommandCenter）转发来的命令
    let mediaUnlisten: (() => void) | undefined = undefined;

    const registerMediaCommandListener = async () => {
      // 防止重复注册叠加监听器
      mediaUnlisten?.();
      mediaUnlisten = undefined;

      mediaUnlisten = await listen<string>('media:command', (event) => {
        switch (event.payload) {
          case 'Play':
            musicStore.play();
            break;
          case 'Pause':
            musicStore.pause();
            break;
          case 'Next':
            musicStore.playPrevOrNext('next');
            break;
          case 'Prev':
            musicStore.playPrevOrNext('prev');
            break;
          default:
            break;
        }
      });
    };

    return {
      isHydrated,
      autoYouthVip,
      autoStart,
      autoStartMode,
      closeMode,
      globalShortcut,
      mediaShortcut,
      shortcutMap,
      device,

      toggleAutoYouthVip,
      toggleAutoStart,
      setAutoStartMode,
      setCloseMode,
      toggleGlobalShortcut,
      toggleMediaShortcut,
      setShortcutMap,
      resetShortcutMap,
      setDevice,
      registerGlobalShortcut,
      unregisterGlobalShortcut,
      registerAllGlobalShortcut,
      unregisterAllGlobalShortcut,
    };
  },
  {
    persist: {
      key: 'setting-store',
      pick: [
        'autoYouthVip',
        'fontFamily',
        'autoStart',
        'autoStartMode',
        'closeMode',
        'globalShortcut',
        'mediaShortcut',
        'shortcutMap',
        'device',
        'miniPlayerPosition',
        'desktopLyricPosition',
      ],
      afterHydrate: (ctx) => (ctx.store.isHydrated = true),
    },
  },
);
