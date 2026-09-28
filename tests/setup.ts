import { beforeEach, vi } from 'vitest';

import { invokeMock, listenMock } from './helpers/mocks';

/**
 * 全局测试环境：
 * - mock 全部 @tauri-apps/* 模块（invoke / event / window / dpi / webviewWindow / 各插件）
 * - mock `@/components/Notification.vue` 的 `notify`（被多个 store 与 utils 依赖）
 * - polyfill happy-dom 缺失的浏览器 API（IntersectionObserver 等）
 * - 每个测试前重置 invoke/listen 默认实现与 localStorage
 */

// ---------- @tauri-apps/api ----------

vi.mock('@tauri-apps/api/core', () => {
  class Channel<T = unknown> {
    public onmessage: ((response: T) => void) | null = null;
  }

  return {
    invoke: vi.fn(() => Promise.resolve(undefined)),
    convertFileSrc: vi.fn((path: string) => `asset://localhost/${encodeURIComponent(path)}`),
    Channel,
  };
});

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
  emit: vi.fn(() => Promise.resolve()),
  emitTo: vi.fn(() => Promise.resolve()),
}));

const createWindowMock = () => ({
  setTitle: vi.fn(() => Promise.resolve()),
  show: vi.fn(() => Promise.resolve()),
  hide: vi.fn(() => Promise.resolve()),
  close: vi.fn(() => Promise.resolve()),
  minimize: vi.fn(() => Promise.resolve()),
  toggleMaximize: vi.fn(() => Promise.resolve()),
  isMaximized: vi.fn(() => Promise.resolve(false)),
  setFullscreen: vi.fn(() => Promise.resolve()),
  isFullscreen: vi.fn(() => Promise.resolve(false)),
  setFocus: vi.fn(() => Promise.resolve()),
  center: vi.fn(() => Promise.resolve()),
  setPosition: vi.fn(() => Promise.resolve()),
  setSize: vi.fn(() => Promise.resolve()),
  innerSize: vi.fn(() => Promise.resolve({ width: 1280, height: 800 })),
  innerPosition: vi.fn(() => Promise.resolve({ x: 0, y: 0 })),
  outerPosition: vi.fn(() => Promise.resolve({ x: 0, y: 0 })),
  setAlwaysOnTop: vi.fn(() => Promise.resolve()),
  setIgnoreCursorEvents: vi.fn(() => Promise.resolve()),
  startDragging: vi.fn(() => Promise.resolve()),
  listen: vi.fn(() => Promise.resolve(() => {})),
  onMoved: vi.fn(() => Promise.resolve(() => {})),
  onResized: vi.fn(() => Promise.resolve(() => {})),
});

vi.mock('@tauri-apps/api/window', () => {
  class PhysicalPosition {
    constructor(
      public x: number,
      public y: number,
    ) {}
  }

  class PhysicalSize {
    constructor(
      public width: number,
      public height: number,
    ) {}
  }

  class Window {
    static getByLabel = vi.fn(() => Promise.resolve(null));
  }

  return {
    getCurrentWindow: vi.fn(() => createWindowMock()),
    getAllWindows: vi.fn(() => []),
    PhysicalPosition,
    PhysicalSize,
    Window,
  };
});

vi.mock('@tauri-apps/api/dpi', () => {
  class LogicalSize {
    constructor(
      public width: number,
      public height: number,
    ) {}
  }

  class LogicalPosition {
    constructor(
      public x: number,
      public y: number,
    ) {}
  }

  return { LogicalSize, LogicalPosition };
});

vi.mock('@tauri-apps/api/webviewWindow', () => ({
  WebviewWindow: class {
    static getByLabel = vi.fn(() => Promise.resolve(null));
  },
}));

vi.mock('@tauri-apps/api/app', () => ({
  getVersion: vi.fn(() => Promise.resolve('0.2.0')),
}));

// ---------- @tauri-apps/plugin-* ----------

vi.mock('@tauri-apps/plugin-global-shortcut', () => ({
  isRegistered: vi.fn(() => Promise.resolve(false)),
  register: vi.fn(() => Promise.resolve()),
  unregister: vi.fn(() => Promise.resolve()),
  unregisterAll: vi.fn(() => Promise.resolve()),
}));

vi.mock('@tauri-apps/plugin-updater', () => ({
  check: vi.fn(() => Promise.resolve(null)),
  Update: class {
    download = vi.fn(() => Promise.resolve());

    install = vi.fn(() => Promise.resolve());

    downloadAndInstall = vi.fn(() => Promise.resolve());
  },
}));

vi.mock('@tauri-apps/plugin-process', () => ({
  relaunch: vi.fn(() => Promise.resolve()),
}));

vi.mock('@tauri-apps/plugin-autostart', () => ({
  enable: vi.fn(() => Promise.resolve()),
  disable: vi.fn(() => Promise.resolve()),
  isEnabled: vi.fn(() => Promise.resolve(false)),
}));

vi.mock('@tauri-apps/plugin-clipboard-manager', () => ({
  writeText: vi.fn(() => Promise.resolve()),
  readText: vi.fn(() => Promise.resolve('')),
}));

vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(() => Promise.resolve(null)),
  save: vi.fn(() => Promise.resolve(null)),
}));

vi.mock('@tauri-apps/plugin-opener', () => ({
  openPath: vi.fn(() => Promise.resolve()),
  revealItemInDir: vi.fn(() => Promise.resolve()),
}));

vi.mock('@tauri-apps/plugin-log', () => ({
  error: vi.fn(() => Promise.resolve()),
  warn: vi.fn(() => Promise.resolve()),
  info: vi.fn(() => Promise.resolve()),
  debug: vi.fn(() => Promise.resolve()),
  trace: vi.fn(() => Promise.resolve()),
}));

// ---------- 项目内模块 ----------

vi.mock('@/components/Notification.vue', () => ({
  notify: {
    success: vi.fn(),
    info: vi.fn(),
    warning: vi.fn(),
    error: vi.fn(),
  },
  default: { name: 'Notification', render: () => null },
}));

// ---------- 浏览器 API polyfill ----------

class IntersectionObserverMock implements IntersectionObserver {
  readonly root = null;

  readonly rootMargin = '0px';

  readonly thresholds = [0];

  scrollMargin!: string;

  constructor(
    private readonly callback: IntersectionObserverCallback,
    _options?: IntersectionObserverInit,
  ) {}

  observe(target: Element) {
    // 微任务异步触发「可见」回调：业务代码通常是先 observe() 再注册回调（如 utils/hooks.ts
    // 的 ObserverManager），同步触发会丢回调；microtask 保证注册完成后才派发
    queueMicrotask(() => {
      this.callback([{ isIntersecting: true, target } as IntersectionObserverEntry], this);
    });
  }

  unobserve() {}

  disconnect() {}

  takeRecords(): IntersectionObserverEntry[] {
    return [];
  }
}

vi.stubGlobal('IntersectionObserver', IntersectionObserverMock);

if (!('matchMedia' in window)) {
  vi.stubGlobal('matchMedia', (query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addEventListener: () => {},
    removeEventListener: () => {},
    addListener: () => {},
    removeListener: () => {},
    dispatchEvent: () => false,
  }));
}

// ---------- 每个测试前重置 ----------

beforeEach(() => {
  // vi.clearAllMocks 只清调用记录、保留实现；invoke/listen 的实现可能被上个测试用
  // mockInvoke/mockImplementation 覆盖过，这里显式恢复默认实现，保证测试间互不影响
  vi.clearAllMocks();
  invokeMock.mockImplementation(() => Promise.resolve(undefined));
  listenMock.mockImplementation(() => Promise.resolve(() => {}));
  localStorage.clear();
});
