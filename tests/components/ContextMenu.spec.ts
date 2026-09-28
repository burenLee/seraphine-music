import { enableAutoUnmount, mount } from '@vue/test-utils';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import ContextMenu from '@/components/ContextMenu.vue';
import { useContextMenuStore } from '@/stores/context-menu';

import { createTestPinia, makeContextMenuOption } from '../helpers/factories';
import { flushAsync } from '../helpers/mocks';

enableAutoUnmount(afterEach);

const findItem = (text: string) =>
  Array.from(document.body.querySelectorAll('li')).find((li) => li.textContent?.includes(text));

describe('ContextMenu 右键菜单', () => {
  let store: ReturnType<typeof useContextMenuStore>;

  beforeEach(() => {
    createTestPinia();
    store = useContextMenuStore();
  });

  it('visible=false 时不渲染菜单', () => {
    mount(ContextMenu);
    expect(document.body.querySelector('ul')).toBeNull();
  });

  it('show 后渲染选项与分割线', async () => {
    mount(ContextMenu);
    store.show({
      x: 10,
      y: 20,
      options: [
        makeContextMenuOption({ label: '播放' }),
        makeContextMenuOption({ divider: true }),
        makeContextMenuOption({ label: '删除' }),
      ],
    });
    await flushAsync();
    const text = document.body.textContent ?? '';
    expect(text).toContain('播放');
    expect(text).toContain('删除');
  });

  it('点击带 onClick 的选项触发回调并隐藏菜单', async () => {
    const onClick = vi.fn();
    mount(ContextMenu);
    store.show({ x: 0, y: 0, options: [makeContextMenuOption({ label: '播放', onClick })] });
    await flushAsync();
    findItem('播放')?.dispatchEvent(new MouseEvent('click'));
    await flushAsync();
    expect(onClick).toHaveBeenCalledTimes(1);
    expect(store.visible).toBe(false);
  });

  it('无 onClick 的选项点击不隐藏', async () => {
    mount(ContextMenu);
    store.show({ x: 0, y: 0, options: [makeContextMenuOption({ label: '占位' })] });
    await flushAsync();
    findItem('占位')?.dispatchEvent(new MouseEvent('click'));
    await flushAsync();
    expect(store.visible).toBe(true);
  });

  it('有 children 的选项 hover 后渲染子菜单', async () => {
    mount(ContextMenu);
    store.show({
      x: 0,
      y: 0,
      options: [
        makeContextMenuOption({
          label: '更多',
          children: [makeContextMenuOption({ label: '子项' })],
        }),
      ],
    });
    await flushAsync();
    findItem('更多')?.dispatchEvent(new MouseEvent('mouseenter'));
    await flushAsync();
    expect(document.body.textContent ?? '').toContain('子项');
  });
});
