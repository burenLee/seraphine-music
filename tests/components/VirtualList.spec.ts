import { enableAutoUnmount, mount } from '@vue/test-utils';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import VirtualList from '@/components/VirtualList.vue';

enableAutoUnmount(afterEach);

interface Row {
  id: number;
  title: string;
}

const columns: TableColumn[] = [{ key: 'title' }];
const makeRows = (n: number): Row[] =>
  Array.from({ length: n }, (_, i) => ({ id: i + 1, title: `行 ${i + 1}` }));

describe('VirtualList 虚拟列表', () => {
  beforeEach(() => {
    vi.stubGlobal('scrollTo', vi.fn());
    HTMLElement.prototype.scrollTo = vi.fn() as never;
  });

  it('空列表显示「列表为空」', () => {
    const wrapper = mount(VirtualList, { props: { list: [], columns } });
    expect(wrapper.text()).toContain('列表为空');
  });

  it('loading 时渲染骨架屏', () => {
    const wrapper = mount(VirtualList, { props: { list: [], columns, loading: true } });
    expect(wrapper.findAll('.card')).toHaveLength(5);
  });

  it('渲染可见窗口行（clientHeight=0 时窗口为 10 行）', () => {
    const wrapper = mount(VirtualList, { props: { list: makeRows(20), columns } });
    expect(wrapper.findAll('li')).toHaveLength(10);
    expect(wrapper.text()).toContain('行 1');
  });

  it('非勾选模式下行点击触发 lineClick', async () => {
    const rows = makeRows(3);
    const wrapper = mount(VirtualList, { props: { list: rows, columns } });
    await wrapper.findAll('li')[1].trigger('click');
    expect(wrapper.emitted('lineClick')?.[0]).toEqual([rows[1]]);
  });

  it('行双击触发 lineDblClick', async () => {
    const rows = makeRows(3);
    const wrapper = mount(VirtualList, { props: { list: rows, columns } });
    await wrapper.findAll('li')[0].trigger('dblclick');
    expect(wrapper.emitted('lineDblClick')?.[0]).toEqual([rows[0]]);
  });

  it('勾选模式下行点击触发 check 并渲染复选框', async () => {
    const rows = makeRows(3);
    const wrapper = mount(VirtualList, { props: { list: rows, columns, checking: true } });
    expect(wrapper.find('input[type="checkbox"]').exists()).toBe(true);
    await wrapper.findAll('li')[0].trigger('click');
    expect(wrapper.emitted('check')?.[0]).toEqual([rows[0].id]);
  });

  it('右键触发 contextmenu(事件, 行数据)', async () => {
    const rows = makeRows(3);
    const wrapper = mount(VirtualList, { props: { list: rows, columns } });
    await wrapper.findAll('li')[0].trigger('contextmenu');
    expect(wrapper.emitted('contextmenu')?.[0]?.[1]).toEqual(rows[0]);
  });

  it('滚动到底部触发 infinite', async () => {
    const wrapper = mount(VirtualList, { props: { list: makeRows(20), columns } });
    const container = wrapper.get('.relative');
    Object.defineProperty(container.element, 'clientHeight', { value: 640, configurable: true });
    Object.defineProperty(container.element, 'scrollHeight', { value: 640, configurable: true });
    Object.defineProperty(container.element, 'scrollTop', {
      value: 0,
      writable: true,
      configurable: true,
    });

    await container.trigger('scroll');
    expect(wrapper.emitted('scroll')).toHaveLength(1);
  });

  it('scrollToIndex 越界时不滚动', () => {
    const wrapper = mount(VirtualList, { props: { list: makeRows(3), columns } });
    (wrapper.vm as unknown as { scrollToIndex: (i: number) => void }).scrollToIndex(-1);
    (wrapper.vm as unknown as { scrollToIndex: (i: number) => void }).scrollToIndex(99);
    expect(HTMLElement.prototype.scrollTo).not.toHaveBeenCalled();
  });

  it('scrollToTarget 定位到指定行', () => {
    const wrapper = mount(VirtualList, { props: { list: makeRows(3), columns } });
    (wrapper.vm as unknown as { scrollToTarget: (t: number) => void }).scrollToTarget(2);
    expect(HTMLElement.prototype.scrollTo).toHaveBeenCalled();
  });
});
