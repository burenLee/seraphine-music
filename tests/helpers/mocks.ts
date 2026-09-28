import { invoke } from '@tauri-apps/api/core';
import { emit, emitTo, listen } from '@tauri-apps/api/event';
import { vi, type Mock } from 'vitest';

import { notify } from '@/components/Notification.vue';

type AnyFn = (...args: any[]) => any;

/** 底层 invoke mock（`@/utils/tools` 的 invoke 封装最终也会走到这里），默认 resolve undefined */
export const invokeMock = vi.mocked(invoke) as unknown as Mock<AnyFn>;

/** Tauri 事件 listen mock，可用 triggerListen 模拟事件推送 */
export const listenMock = vi.mocked(listen) as unknown as Mock<AnyFn>;

export const emitMock = vi.mocked(emit) as unknown as Mock<AnyFn>;

export const emitToMock = vi.mocked(emitTo) as unknown as Mock<AnyFn>;

/** 全局通知 mock（`@/components/Notification.vue` 的具名导出） */
export const notifyMock = vi.mocked(notify);

type InvokeHandlers = {
  [C in InvokeCmd]?:
    | InvokeReturn<C>
    | ((args: InvokeArgs<C>) => InvokeReturn<C> | Promise<InvokeReturn<C>>);
};

/**
 * 按命令名配置 invoke 返回值或处理器；未配置的命令默认 resolve undefined。
 *
 * @example
 * mockInvoke({
 *   api_song_url: { status: 1, backupUrl: ['https://example.com/a.mp3'] },
 *   music_player_load_url: () => undefined,
 * })
 */
export const mockInvoke = (handlers: InvokeHandlers) => {
  invokeMock.mockImplementation((cmd: string, args?: unknown) => {
    const handler = handlers[cmd as InvokeCmd];
    if (handler === undefined) return Promise.resolve(undefined);
    return Promise.resolve(
      typeof handler === 'function' ? (handler as (a: unknown) => unknown)(args) : handler,
    );
  });
};

/**
 * 触发指定事件的所有已注册 listen 回调（模拟后端或其他窗口推送事件）。
 */
export const triggerListen = <T>(event: string, payload: T) => {
  for (const call of listenMock.mock.calls) {
    const [listenEvent, handler] = call as [string, (e: { event: string; payload: T }) => void];
    if (listenEvent === event) handler({ event, payload });
  }
};

/**
 * 排空微任务队列，等待 store 水合 watch 等 async 副作用落定。
 * 用宏任务（setTimeout 0）而非固定轮次微任务：await 链不产生宏任务，setTimeout
 * 回调执行时，事件循环已把所有 pending 微任务（含多键 await 循环）完整排空。
 * 注意：fake timers 环境下请改用 `vi.advanceTimersByTimeAsync(0)`。
 */
export const flushAsync = async () => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  await new Promise((resolve) => setTimeout(resolve, 0));
};
