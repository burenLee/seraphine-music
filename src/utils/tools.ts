import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { openPath, revealItemInDir } from '@tauri-apps/plugin-opener';
import { ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

import { notify } from '@/components/Notification.vue';

import { SizeUnits } from './params';

/**
 * 封装 Tauri 的 invoke 函数
 * @description 统一参数和返回类型
 * @param cmd 指令名称
 * @param args 要传递给指令的参数
 */
export async function invoke<C extends InvokeCmd>(cmd: C, args?: InvokeArgs<C>) {
  try {
    return await tauriInvoke<InvokeReturn<C>>(cmd, args);
  } catch (error) {
    // 不参与 UI 处理，直接抛出错误在使用时捕获
    const msg = error instanceof Error ? error.message : String(error);
    throw Error(msg);
  }
}

/**
 * 合并 class 样式
 * @param inputs 所有的 class 样式
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 * 判断字符串是否为英文
 * @param text 待判断的字符串
 */
export function isEnglishText(text: string) {
  return /^[a-zA-Z\s\-_',.!?;:()"[\]]+$/.test(text.trim());
}

/**
 * 将 `秒` 转为 `分钟：秒` 格式
 * @param s 秒数
 */
export function formatDuration(s: number) {
  if (s <= 0) return '00:00';

  const mins = Math.floor(s / 60).toString();
  const secs = Math.floor(s % 60).toString();

  return `${mins.padStart(2, '0')}:${secs.padStart(2, '0')}`;
}

const LOG_1024 = Math.log(1024);

/**
 * 格式化文件大小
 * @param bytes 文件大小（字节）
 * @param decimals 保留的小数位数，默认为 2
 */
export function formatFileSize(bytes: number, decimals: number = 2) {
  if (!Number.isFinite(bytes) || bytes <= 0) return '0 B';

  const unitIndex = Math.min(Math.floor(Math.log(bytes) / LOG_1024), SizeUnits.length - 1);
  const size = bytes / Math.pow(1024, unitIndex);
  const safeDecimals = Math.max(0, decimals);
  return `${size.toFixed(safeDecimals)} ${SizeUnits[unitIndex]}`;
}

/**
 * 格式化数字，达到万级加「万」、达到亿级加「亿」
 * @param num 待格式化的数字
 */
export function formatCount(num: number) {
  if (num >= 1e8) return `${(num / 1e8).toFixed(1).replace(/\.0$/, '')}亿`;
  if (num >= 1e4) return `${(num / 1e4).toFixed(1).replace(/\.0$/, '')}万`;
  return num.toString();
}

/** 设置应用标题 */
export async function setAppTitle(title: string) {
  document.title = title;
  await getCurrentWindow().setTitle(title);
}

/**
 * 获取 `[0, maxNum]` 内且与 `srcNum` 不同的随机整数
 * @param maxNum 最大值 (包含)
 * @param srcNum 来源值，需要避免的值，默认 -1（不避免任何值）
 */
export function genRandomNum(maxNum: number, srcNum: number = -1) {
  if (!Number.isInteger(maxNum) || maxNum < 0) {
    throw new Error('maxNum 必须是非负整数');
  }

  if (maxNum === 0) return 0;
  if (maxNum === 1) return srcNum === 0 ? 1 : 0;

  let random = Math.floor(Math.random() * (maxNum + 1));

  // 如果随机数等于 srcNum，则循环到下一个值
  while (random === srcNum) random = (random + 1) % (maxNum + 1);

  return random;
}

/** 拦截浏览器快捷键 */
export function interdictHotkeys(enabled: boolean = true) {
  if (import.meta.env.DEV || !enabled) return;

  window.addEventListener('contextmenu', (e) => e.preventDefault(), true);
  window.addEventListener(
    'keydown',
    (e) => {
      // 中文输入法组合过程，直接放行，防止打字故障
      if (e.isComposing) return;

      const target = e.target as HTMLElement;
      // 如果焦点在输入框/可编辑区域，放行所有按键
      const isEditable = ['INPUT', 'TEXTAREA'].includes(target.tagName) || target.isContentEditable;
      if (isEditable) return;

      const { code, ctrlKey, metaKey, shiftKey } = e;
      const ctrlMeta = ctrlKey || metaKey;

      // 黑名单：所有需要拦截的快捷键
      const needBlock =
        // Tab焦点切换
        code === 'Tab' ||
        // F功能键
        ['F1', 'F2', 'F3', 'F4', 'F5', 'F6', 'F7', 'F8', 'F9', 'F10', 'F11', 'F12'].includes(
          code,
        ) ||
        // Ctrl/Command组合浏览器快捷键
        (ctrlMeta &&
          ['KeyG', 'KeyJ', 'KeyW', 'KeyT', 'KeyP', 'KeyR', 'KeyS', 'KeyF'].includes(code)) ||
        // Ctrl+Shift组合
        (ctrlMeta && shiftKey && ['KeyR', 'KeyI', 'KeyJ', 'KeyC'].includes(code));

      if (needBlock) {
        e.preventDefault();
        e.stopPropagation();
      }
    },
    true,
  );
}

/** 在资源管理器中打开路径所在位置 */
export async function revealPath(path: string) {
  if (!path) return;

  try {
    await revealItemInDir(path);
  } catch {
    notify.error('打开路径所在位置失败');
  }
}

/** 在资源管理器中打开文件夹 */
export async function openDir(path: string) {
  if (!path) return;

  try {
    await openPath(path);
  } catch {
    notify.error('打开文件夹失败');
  }
}
