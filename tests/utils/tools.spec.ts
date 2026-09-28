import { getCurrentWindow } from '@tauri-apps/api/window';
import { openPath, revealItemInDir } from '@tauri-apps/plugin-opener';
import { describe, expect, it, vi } from 'vitest';

import {
  cn,
  formatDuration,
  formatFileSize,
  genRandomNum,
  interdictHotkeys,
  invoke,
  isEnglishText,
  openDir,
  revealPath,
  setAppTitle,
} from '@/utils/tools';

import { invokeMock, notifyMock } from '../helpers/mocks';

const getCurrentWindowMock = vi.mocked(getCurrentWindow);
const revealItemInDirMock = vi.mocked(revealItemInDir);
const openPathMock = vi.mocked(openPath);

describe('invoke 封装', () => {
  it('成功时透传后端返回值', async () => {
    invokeMock.mockResolvedValueOnce({ status: 1 });

    await expect(invoke('api_song_url', { hash: 'abc' })).resolves.toStrictEqual({ status: 1 });
    expect(invokeMock).toHaveBeenCalledWith('api_song_url', { hash: 'abc' });
  });

  it('失败时将错误统一包装为 Error 抛出', async () => {
    invokeMock.mockRejectedValueOnce(new Error('backend boom'));
    await expect(invoke('music_player_play')).rejects.toThrow('backend boom');

    invokeMock.mockRejectedValueOnce('raw string error');
    await expect(invoke('music_player_play')).rejects.toThrow('raw string error');
  });
});

describe('cn', () => {
  it('合并多个 class 并忽略假值', () => {
    expect(cn('a', false, undefined, 'b')).toBe('a b');
  });

  it('tailwind 冲突类后者覆盖前者', () => {
    expect(cn('p-2', 'p-4')).toBe('p-4');
    expect(cn('text-left text-red-500', 'text-center')).toBe('text-red-500 text-center');
  });
});

describe('isEnglishText', () => {
  it('纯英文（含常见标点）返回 true', () => {
    expect(isEnglishText('Hello, World!')).toBe(true);
    expect(isEnglishText("It's a test-case.")).toBe(true);
  });

  it('包含中文等非英文字符返回 false', () => {
    expect(isEnglishText('你好')).toBe(false);
    expect(isEnglishText('Hello 你好')).toBe(false);
  });

  it('空字符串返回 false', () => {
    expect(isEnglishText('')).toBe(false);
    expect(isEnglishText('   ')).toBe(false);
  });
});

describe('formatDuration', () => {
  it('0 与负数返回 00:00', () => {
    expect(formatDuration(0)).toBe('00:00');
    expect(formatDuration(-5)).toBe('00:00');
  });

  it('秒数转为 mm:ss 并补零', () => {
    expect(formatDuration(59)).toBe('00:59');
    expect(formatDuration(61)).toBe('01:01');
    expect(formatDuration(600)).toBe('10:00');
  });

  it('小数秒向下取整', () => {
    expect(formatDuration(59.9)).toBe('00:59');
  });
});

describe('formatFileSize', () => {
  it('非法输入返回 0 B', () => {
    expect(formatFileSize(0)).toBe('0 B');
    expect(formatFileSize(-1)).toBe('0 B');
    expect(formatFileSize(NaN)).toBe('0 B');
    expect(formatFileSize(Infinity)).toBe('0 B');
  });

  it('按 1024 换算单位', () => {
    expect(formatFileSize(512)).toBe('512.00 B');
    expect(formatFileSize(1024)).toBe('1.00 KB');
    expect(formatFileSize(1536)).toBe('1.50 KB');
    expect(formatFileSize(1024 ** 2)).toBe('1.00 MB');
    expect(formatFileSize(1024 ** 3)).toBe('1.00 GB');
  });

  it('支持自定义小数位', () => {
    expect(formatFileSize(1024, 0)).toBe('1 KB');
    expect(formatFileSize(1536, 1)).toBe('1.5 KB');
  });

  it('超出最大单位时封顶为 YB', () => {
    expect(formatFileSize(1024 ** 10)).toContain('YB');
  });
});

describe('setAppTitle', () => {
  it('同时设置 document.title 与窗口标题', () => {
    setAppTitle('Seraphine');

    expect(document.title).toBe('Seraphine');
    const results = getCurrentWindowMock.mock.results;
    const windowMock = results[results.length - 1]?.value;
    expect(windowMock.setTitle).toHaveBeenCalledWith('Seraphine');
  });
});

describe('getRandomNumber', () => {
  it('maxNum 非非负整数时抛错', () => {
    expect(() => genRandomNum(-1, 0)).toThrow('maxNum 必须是非负整数');
    expect(() => genRandomNum(1.5, 0)).toThrow('maxNum 必须是非负整数');
  });

  it('maxNum=0 时恒返回 0', () => {
    expect(genRandomNum(0, 0)).toBe(0);
  });

  it('maxNum=1 时返回与 srcNum 相反的值', () => {
    expect(genRandomNum(1, 0)).toBe(1);
    expect(genRandomNum(1, 1)).toBe(0);
  });

  it('结果在 [0, maxNum] 内且不等于 srcNum', () => {
    for (let i = 0; i < 100; i += 1) {
      const result = genRandomNum(5, 2);
      expect(result).toBeGreaterThanOrEqual(0);
      expect(result).toBeLessThanOrEqual(5);
      expect(result).not.toBe(2);
      expect(Number.isInteger(result)).toBe(true);
    }
  });
});

describe('interdictHotkeys', () => {
  it('测试环境（DEV）下为 no-op，不抛错', () => {
    expect(() => interdictHotkeys()).not.toThrow();
    expect(() => interdictHotkeys(false)).not.toThrow();
  });
});

describe('revealPath / openDir', () => {
  it('空路径直接返回，不调用插件', async () => {
    await revealPath('');
    await openDir('');
    expect(revealItemInDirMock).not.toHaveBeenCalled();
    expect(openPathMock).not.toHaveBeenCalled();
  });

  it('正常路径调用对应插件', async () => {
    await revealPath('/music/a.mp3');
    expect(revealItemInDirMock).toHaveBeenCalledWith('/music/a.mp3');

    await openDir('/music');
    expect(openPathMock).toHaveBeenCalledWith('/music');
  });

  it('插件失败时 notify.error，不向外抛错', async () => {
    revealItemInDirMock.mockRejectedValueOnce(new Error('io error'));
    await expect(revealPath('/music/a.mp3')).resolves.toBeUndefined();
    expect(notifyMock.error).toHaveBeenCalledWith('打开路径所在位置失败');

    openPathMock.mockRejectedValueOnce(new Error('io error'));
    await expect(openDir('/music')).resolves.toBeUndefined();
    expect(notifyMock.error).toHaveBeenCalledWith('打开文件夹失败');
  });
});
