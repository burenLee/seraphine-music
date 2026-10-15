import {
  isRegistered,
  register,
  unregister,
  unregisterAll,
} from '@tauri-apps/plugin-global-shortcut';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { AutoStartMode, CloseMode, ShortcutKey } from '@/constants';
import { useSettingStore } from '@/stores/setting';

import { createTestPinia } from '../helpers/factories';
import { flushAsync, listenMock } from '../helpers/mocks';

const registerMock = vi.mocked(register);
const unregisterMock = vi.mocked(unregister);
const unregisterAllMock = vi.mocked(unregisterAll);
const isRegisteredMock = vi.mocked(isRegistered);

beforeEach(() => {
  createTestPinia();
});

describe('初始默认值', () => {
  it('设置字段与默认快捷键映射', async () => {
    const store = useSettingStore();
    await flushAsync();

    expect(store.isHydrated).toBe(true);
    expect(store.autoYouthVip).toBe(true);
    expect(store.autoStart).toBe(false);
    expect(store.autoStartMode).toBe(AutoStartMode.Foreground);
    expect(store.closeMode).toBeUndefined();
    expect(store.globalShortcut).toBe(true);
    expect(store.mediaShortcut).toBe(true);
    expect(store.shortcutMap).toStrictEqual({
      playOrPause: 'Alt+F5',
      addVolumn: 'Alt+Up',
      subVolumn: 'Alt+Down',
      mute: 'Alt+S',
      prev: 'Alt+Left',
      next: 'Alt+Right',
      backward: 'Ctrl+Alt+Left',
      forward: 'Ctrl+Alt+Right',
    });
    expect(store.device).toBe('');
  });
});

describe('基础设置', () => {
  it('各 setter/toggle 直接更新状态', async () => {
    const store = useSettingStore();
    await flushAsync();

    store.toggleAutoYouthVip();
    store.toggleAutoStart(true);
    store.setAutoStartMode(AutoStartMode.Background);
    store.setCloseMode(CloseMode.Hide);
    store.setDevice('device-1');

    expect(store.autoYouthVip).toBe(false);
    expect(store.autoStart).toBe(true);
    expect(store.autoStartMode).toBe(AutoStartMode.Background);
    expect(store.closeMode).toBe(CloseMode.Hide);
    expect(store.device).toBe('device-1');
  });

  it('setShortcutMap / resetShortcutMap', async () => {
    const store = useSettingStore();
    await flushAsync();

    store.setShortcutMap(ShortcutKey.PlayOrPause, 'Ctrl+P');
    expect(store.shortcutMap.playOrPause).toBe('Ctrl+P');

    store.resetShortcutMap();
    expect(store.shortcutMap.playOrPause).toBe('Alt+F5');
  });
});

describe('全局快捷键注册', () => {
  it('水合后注册 8 个全局快捷键，并监听 media:command', async () => {
    useSettingStore();
    await flushAsync();

    expect(unregisterAllMock).toHaveBeenCalled();

    const shortcuts = registerMock.mock.calls.map(([shortcut]) => shortcut);
    for (const s of [
      'Alt+F5',
      'Alt+Up',
      'Alt+Down',
      'Alt+S',
      'Alt+Left',
      'Alt+Right',
      'Ctrl+Alt+Left',
      'Ctrl+Alt+Right',
    ]) {
      expect(shortcuts).toContain(s);
    }
    // 媒体键改走系统原生通道，不再注册 MPRIS/SMTC 类 global-shortcut
    expect(shortcuts).not.toContain('MediaPlayPause');
    expect(shortcuts).not.toContain('MediaTrackNext');
    expect(shortcuts).not.toContain('MediaTrackPrevious');
    expect(listenMock).toHaveBeenCalledWith('media:command', expect.any(Function));
  });

  it('已注册的快捷键跳过重复注册', async () => {
    const store = useSettingStore();
    await flushAsync();
    registerMock.mockClear();
    isRegisteredMock.mockResolvedValueOnce(true);

    await store.registerGlobalShortcut(ShortcutKey.PlayOrPause);

    expect(registerMock).not.toHaveBeenCalled();
  });

  it('非法快捷键（多个主键）被清空且不注册', async () => {
    const store = useSettingStore();
    await flushAsync();
    registerMock.mockClear();
    store.setShortcutMap(ShortcutKey.PlayOrPause, 'Alt+A+B');

    await store.registerGlobalShortcut(ShortcutKey.PlayOrPause);

    expect(store.shortcutMap.playOrPause).toBe('');
    expect(registerMock).not.toHaveBeenCalled();
  });

  it('toggleGlobalShortcut 关闭时注销全部', async () => {
    const store = useSettingStore();
    await flushAsync();
    unregisterMock.mockClear();

    store.toggleGlobalShortcut();
    await flushAsync();

    expect(store.globalShortcut).toBe(false);
    const unregistered = unregisterMock.mock.calls.map(([shortcut]) => shortcut);
    for (const s of [
      'Alt+F5',
      'Alt+Up',
      'Alt+Down',
      'Alt+S',
      'Alt+Left',
      'Alt+Right',
      'Ctrl+Alt+Left',
      'Ctrl+Alt+Right',
    ]) {
      expect(unregistered).toContain(s);
    }
  });

  it('toggleMediaShortcut 关闭后重新开启会重新监听 media:command', async () => {
    const store = useSettingStore();
    await flushAsync();
    listenMock.mockClear();

    store.toggleMediaShortcut();
    expect(store.mediaShortcut).toBe(false);

    store.toggleMediaShortcut();
    await flushAsync();
    expect(store.mediaShortcut).toBe(true);

    expect(listenMock).toHaveBeenCalledWith('media:command', expect.any(Function));
  });
});

describe('持久化水合', () => {
  it('恢复持久化字段', async () => {
    localStorage.setItem(
      'setting-store',
      JSON.stringify({
        autoYouthVip: false,
        autoStart: true,
        autoStartMode: AutoStartMode.Background,
        closeMode: CloseMode.Exit,
        globalShortcut: false,
        mediaShortcut: false,
        shortcutMap: { playOrPause: 'Ctrl+Space' },
        device: 'dev-9',
      }),
    );

    const store = useSettingStore();
    await flushAsync();

    expect(store.autoYouthVip).toBe(false);
    expect(store.autoStartMode).toBe(AutoStartMode.Background);
    expect(store.closeMode).toBe(CloseMode.Exit);
    expect(store.device).toBe('dev-9');
    // 快捷键被整体水合替换，未持久化的键不存在
    expect(store.shortcutMap.playOrPause).toBe('Ctrl+Space');
  });
});
