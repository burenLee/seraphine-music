import { beforeEach, describe, expect, it } from 'vitest';
import { nextTick } from 'vue';

import { useContextMenuStore } from '@/stores/context-menu';

import { createTestPinia, makeContextMenuOption } from '../helpers/factories';

beforeEach(() => {
  createTestPinia();
});

describe('右键菜单', () => {
  it('初始为隐藏状态', () => {
    const store = useContextMenuStore();

    expect(store.visible).toBe(false);
    expect(store.position).toStrictEqual({ x: 0, y: 0 });
    expect(store.options).toStrictEqual([]);
  });

  it('show 设置位置与选项，下一 tick 后可见', async () => {
    const store = useContextMenuStore();
    const options = [
      makeContextMenuOption({ label: '播放' }),
      makeContextMenuOption({ label: '删除' }),
    ];

    store.show({ x: 100, y: 200, options });

    expect(store.position).toStrictEqual({ x: 100, y: 200 });
    expect(store.options).toStrictEqual(options);
    // show 先强制隐藏再于 nextTick 显示，避免重复触发时菜单不刷新
    expect(store.visible).toBe(false);

    await nextTick();
    expect(store.visible).toBe(true);
  });

  it('hide 立即隐藏', async () => {
    const store = useContextMenuStore();
    store.show({ x: 0, y: 0, options: [] });
    await nextTick();

    store.hide();

    expect(store.visible).toBe(false);
  });
});
