import { enableAutoUnmount, mount } from '@vue/test-utils';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import ContextMenu from '@/components/ContextMenu/Container.vue';
import { useContextMenuStore } from '@/stores/context-menu';

import { createTestPinia, makeContextMenuOption } from '../helpers/factories';
import { flushAsync } from '../helpers/mocks';

enableAutoUnmount(afterEach);

// 通过选项内 label 文本（.truncate div）精确匹配选项 li，
// 避免父项 textContent 包含子菜单文本导致误匹配
const findItem = (text: string) =>
  Array.from(document.body.querySelectorAll('li')).find(
    (li) => li.querySelector(':scope > .truncate')?.textContent?.trim() === text,
  );

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
      top: 10,
      left: 20,
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
    store.show({ top: 0, left: 0, options: [makeContextMenuOption({ label: '播放', onClick })] });
    await flushAsync();
    findItem('播放')?.dispatchEvent(new MouseEvent('click'));
    await flushAsync();
    expect(onClick).toHaveBeenCalledTimes(1);
    expect(store.visible).toBe(false);
  });

  it('无 onClick 的选项点击后同样隐藏菜单', async () => {
    mount(ContextMenu);
    store.show({ top: 0, left: 0, options: [makeContextMenuOption({ label: '占位' })] });
    await flushAsync();
    findItem('占位')?.dispatchEvent(new MouseEvent('click'));
    await flushAsync();
    expect(store.visible).toBe(false);
  });

  it('有 children 的选项 hover 后渲染子菜单', async () => {
    mount(ContextMenu);
    store.show({
      top: 0,
      left: 0,
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

  it('支持三层嵌套：逐级 hover 渲染孙菜单', async () => {
    mount(ContextMenu);
    store.show({
      top: 0,
      left: 0,
      options: [
        makeContextMenuOption({
          label: '更多',
          children: [
            makeContextMenuOption({ label: '子项1' }),
            makeContextMenuOption({
              label: '子项2',
              children: [makeContextMenuOption({ label: '孙项' })],
            }),
          ],
        }),
      ],
    });
    await flushAsync();

    // hover 第一层 → 渲染第二层
    findItem('更多')?.dispatchEvent(new MouseEvent('mouseenter'));
    await flushAsync();
    expect(document.body.textContent ?? '').toContain('子项2');

    // hover 第二层 → 渲染第三层
    findItem('子项2')?.dispatchEvent(new MouseEvent('mouseenter'));
    await flushAsync();
    expect(document.body.textContent ?? '').toContain('孙项');
  });

  it('三层嵌套：点击最深层选项触发回调并关闭', async () => {
    const onClick = vi.fn();
    mount(ContextMenu);
    store.show({
      top: 0,
      left: 0,
      options: [
        makeContextMenuOption({
          label: '更多',
          children: [
            makeContextMenuOption({
              label: '子项',
              children: [makeContextMenuOption({ label: '孙项', onClick })],
            }),
          ],
        }),
      ],
    });
    await flushAsync();

    findItem('更多')?.dispatchEvent(new MouseEvent('mouseenter'));
    await flushAsync();
    findItem('子项')?.dispatchEvent(new MouseEvent('mouseenter'));
    await flushAsync();
    findItem('孙项')?.dispatchEvent(new MouseEvent('click'));
    await flushAsync();

    expect(onClick).toHaveBeenCalledTimes(1);
    expect(store.visible).toBe(false);
  });

  it('点击子菜单项（第二层）触发回调并关闭', async () => {
    const onClick = vi.fn();
    mount(ContextMenu);
    store.show({
      top: 0,
      left: 0,
      options: [
        makeContextMenuOption({
          label: '更多',
          children: [makeContextMenuOption({ label: '子项', onClick })],
        }),
      ],
    });
    await flushAsync();
    findItem('更多')?.dispatchEvent(new MouseEvent('mouseenter'));
    await flushAsync();
    findItem('子项')?.dispatchEvent(new MouseEvent('click'));
    await flushAsync();
    expect(onClick).toHaveBeenCalledTimes(1);
    expect(store.visible).toBe(false);
  });
});
