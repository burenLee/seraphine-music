import {
  isRegistered,
  register,
  unregister,
  unregisterAll,
} from '@tauri-apps/plugin-global-shortcut';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { useSettingStore } from '@/stores/setting';
import { AutoStartMode, CloseStatus, Mode, ShortcutKey } from '@/utils/params';

import { createTestPinia } from '../helpers/factories';
import { flushAsync } from '../helpers/mocks';

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
    expect(store.isMaximized).toBe(false);
    expect(store.isFullscreen).toBe(false);
    expect(store.mode).toBe(Mode.KgLite);
    expect(store.autoLiteVipState).toBe(false);
    expect(store.fontFamily).toBe('system-ui');
    expect(store.autoStartState).toBe(false);
    expect(store.autoStartMode).toBe(AutoStartMode.Foreground);
    expect(store.closeStatus).toBeUndefined();
    expect(store.globalShortcutState).toBe(true);
    expect(store.mediaShortcutState).toBe(true);
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
    expect(store.miniPlayerPosition).toStrictEqual({ x: 0, y: 0 });
    expect(store.desktopLyricPosition).toStrictEqual({ x: 0, y: 0 });
  });
});

describe('基础设置', () => {
  it('各 setter/toggle 直接更新状态', async () => {
    const store = useSettingStore();
    await flushAsync();

    store.setMode(Mode.KgMobile);
    store.toggleAutoLiteVipState();
    store.toggleMaximizedState(true);
    store.toggleFullscreenState(true);
    store.setFontFamily('Arial');
    store.toggleAutoStartState(true);
    store.setAutoStartMode(AutoStartMode.Background);
    store.setCloseStatus(CloseStatus.Hide);
    store.setDevice('device-1');
    store.setMiniPlayerPosition({ x: 10, y: 20 });
    store.setdesktopLyricPosition({ x: 30, y: 40 });

    expect(store.mode).toBe(Mode.KgMobile);
    expect(store.autoLiteVipState).toBe(true);
    expect(store.isMaximized).toBe(true);
    expect(store.isFullscreen).toBe(true);
    expect(store.fontFamily).toBe('Arial');
    expect(store.autoStartState).toBe(true);
    expect(store.autoStartMode).toBe(AutoStartMode.Background);
    expect(store.closeStatus).toBe(CloseStatus.Hide);
    expect(store.device).toBe('device-1');
    expect(store.miniPlayerPosition).toStrictEqual({ x: 10, y: 20 });
    expect(store.desktopLyricPosition).toStrictEqual({ x: 30, y: 40 });
  });

  it('setShortcutMap / resetShortcutMap', async () => {
    const store = useSettingStore();
    await flushAsync();

    store.setShortcutMap(ShortcutKey.PlayOrPause, 'Ctrl+P');
    expect(store.shortcutMap.playOrPause).toBe('Ctrl+P');

    store.resetShortcutMap();
    expect(store.shortcutMap.playOrPause).toBe('Alt+F5');
  });

  it('getAvailableFonts 在 canvas 不可用时安全跳过', async () => {
    const store = useSettingStore();
    await flushAsync();

    expect(() => store.getAvailableFonts()).not.toThrow();
  });
});

describe('全局快捷键注册', () => {
  it('水合后注册 8 个全局快捷键与 3 个媒体快捷键', async () => {
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
    for (const s of ['MediaPlayPause', 'MediaTrackNext', 'MediaTrackPrevious']) {
      expect(shortcuts).toContain(s);
    }
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

  it('toggleGlobalShortcutState 关闭时注销全部', async () => {
    const store = useSettingStore();
    await flushAsync();
    unregisterMock.mockClear();

    store.toggleGlobalShortcutState();
    await flushAsync();

    expect(store.globalShortcutState).toBe(false);
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

  it('toggleMediaShortcutState 关闭后重新开启可重新注册', async () => {
    const store = useSettingStore();
    await flushAsync();
    registerMock.mockClear();

    store.toggleMediaShortcutState();
    expect(store.mediaShortcutState).toBe(false);

    store.toggleMediaShortcutState();
    await flushAsync();
    expect(store.mediaShortcutState).toBe(true);

    const shortcuts = registerMock.mock.calls.map(([shortcut]) => shortcut);
    expect(shortcuts).toContain('MediaPlayPause');
  });
});

describe('持久化水合', () => {
  it('恢复持久化字段', async () => {
    localStorage.setItem(
      'setting-store',
      JSON.stringify({
        autoLiteVipState: true,
        fontFamily: 'Consolas',
        autoStartState: true,
        autoStartMode: AutoStartMode.Background,
        closeStatus: CloseStatus.Exit,
        globalShortcutState: false,
        mediaShortcutState: false,
        shortcutMap: { playOrPause: 'Ctrl+Space' },
        device: 'dev-9',
        miniPlayerPosition: { x: 5, y: 6 },
        desktopLyricPosition: { x: 7, y: 8 },
      }),
    );

    const store = useSettingStore();
    await flushAsync();

    expect(store.autoLiteVipState).toBe(true);
    expect(store.fontFamily).toBe('Consolas');
    expect(store.autoStartMode).toBe(AutoStartMode.Background);
    expect(store.closeStatus).toBe(CloseStatus.Exit);
    expect(store.device).toBe('dev-9');
    expect(store.miniPlayerPosition).toStrictEqual({ x: 5, y: 6 });
    // 快捷键被整体水合替换，未持久化的键不存在
    expect(store.shortcutMap.playOrPause).toBe('Ctrl+Space');
  });
});
