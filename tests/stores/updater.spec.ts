import { relaunch } from '@tauri-apps/plugin-process';
import { check as tauriCheck } from '@tauri-apps/plugin-updater';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { useUpdaterStore } from '@/stores/updater';

import { createTestPinia } from '../helpers/factories';
import { flushAsync, notifyMock } from '../helpers/mocks';

const tauriCheckMock = vi.mocked(tauriCheck);
const relaunchMock = vi.mocked(relaunch);

type DownloadCallback = (e: {
  event: 'Started' | 'Progress' | 'Finished';
  data: { contentLength?: number; chunkLength?: number };
}) => void;

/** 构造一个假的 Update 对象，download 会同步回放完整下载事件序列 */
const makeFakeUpdate = () => ({
  currentVersion: '0.2.0',
  version: '0.3.0',
  body: 'release notes',
  date: '2026-09-01',
  close: vi.fn(() => Promise.resolve()),
  download: vi.fn(async (cb: DownloadCallback) => {
    cb({ event: 'Started', data: { contentLength: 1000 } });
    cb({ event: 'Progress', data: { chunkLength: 400 } });
    cb({ event: 'Progress', data: { chunkLength: 600 } });
    cb({ event: 'Finished', data: {} });
  }),
  install: vi.fn(() => Promise.resolve()),
});

beforeEach(() => {
  createTestPinia();
});

describe('check 检查更新', () => {
  it('水合后自动检查：无更新时提示已是最新', async () => {
    const store = useUpdaterStore();
    await flushAsync();

    expect(store.isHydrated).toBe(true);
    expect(store.updateInfo).toStrictEqual({
      hasUpdate: false,
      currentVersion: 'v0.2.0',
      latestVersion: 'v0.2.0',
    });
    expect(store.isChecking).toBe(false);
  });

  it('发现新版本时填充更新信息', async () => {
    const store = useUpdaterStore();
    await flushAsync();
    tauriCheckMock.mockResolvedValueOnce(makeFakeUpdate() as never);

    await store.check();

    expect(store.updateInfo).toStrictEqual({
      hasUpdate: true,
      currentVersion: 'v0.2.0',
      latestVersion: 'v0.3.0',
      body: 'release notes',
      date: '2026-09-01',
    });
    expect(notifyMock.success).toHaveBeenCalledWith('发现新版本 0.3.0');
  });

  it('检查失败时提示错误', async () => {
    const store = useUpdaterStore();
    await flushAsync();
    tauriCheckMock.mockRejectedValueOnce(new Error('network'));

    await store.check();

    expect(notifyMock.error).toHaveBeenCalledWith('无法获取新版本');
    expect(store.updateInfo).toStrictEqual({
      hasUpdate: false,
      currentVersion: 'v0.2.0',
      latestVersion: 'v0.2.0',
    });
  });
});

describe('download 下载更新', () => {
  it('无可用的更新对象时不执行', async () => {
    const store = useUpdaterStore();
    await flushAsync();

    await store.download();

    expect(store.isDownloading).toBe(false);
    expect(store.downloadInfo).toBeUndefined();
  });

  it('下载事件序列推进下载状态', async () => {
    const store = useUpdaterStore();
    await flushAsync();
    tauriCheckMock.mockResolvedValueOnce(makeFakeUpdate() as never);
    await store.check();

    await store.download();

    expect(store.isDownloading).toBe(false);
    expect(store.isDownloaded).toBe(true);
    expect(store.downloadInfo).toStrictEqual({ downloaded: 1000, total: 1000, speed: 0 });
  });
});

describe('install 安装重启', () => {
  it('未下载完成时不执行', async () => {
    const store = useUpdaterStore();
    await flushAsync();

    await store.install();

    expect(relaunchMock).not.toHaveBeenCalled();
  });

  it('下载完成后安装并重启', async () => {
    const store = useUpdaterStore();
    await flushAsync();
    const fakeUpdate = makeFakeUpdate();
    tauriCheckMock.mockResolvedValueOnce(fakeUpdate as never);
    await store.check();
    await store.download();

    await store.install();

    expect(fakeUpdate.install).toHaveBeenCalled();
    expect(relaunchMock).toHaveBeenCalled();
  });
});

describe('reset 重置', () => {
  it('复位下载相关状态', async () => {
    const store = useUpdaterStore();
    await flushAsync();
    tauriCheckMock.mockResolvedValueOnce(makeFakeUpdate() as never);
    await store.check();
    await store.download();

    store.reset();

    expect(store.isDownloading).toBe(false);
    expect(store.isDownloaded).toBe(false);
    expect(store.downloadInfo).toBeUndefined();
  });
});
