import { mount } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';

import SelectModal from '@/components/SelectModal.vue';

const options: SelectOption[] = [
  { label: '选项 A', value: 'a' },
  { label: '选项 B', value: 'b' },
];

describe('SelectModal 选择弹窗', () => {
  it('visible=false 时不渲染选项', () => {
    const wrapper = mount(SelectModal, { props: { visible: false, options } });
    expect(wrapper.find('li').exists()).toBe(false);
  });

  it('visible=true 时渲染全部选项', () => {
    const wrapper = mount(SelectModal, { props: { visible: true, options } });
    const items = wrapper.findAll('li');
    expect(items).toHaveLength(2);
    expect(items[0].text()).toContain('选项 A');
    expect(items[1].text()).toContain('选项 B');
  });

  it('selection 匹配项高亮为 card-actived', () => {
    const wrapper = mount(SelectModal, {
      props: { visible: true, options, selection: { label: '选项 B', value: 'b' } },
    });
    expect(wrapper.findAll('li')[0].classes()).toContain('card-hover');
    expect(wrapper.findAll('li')[1].classes()).toContain('card-actived');
  });

  it('点击选项触发 select(value, option)', async () => {
    const wrapper = mount(SelectModal, { props: { visible: true, options } });
    await wrapper.findAll('li')[1].trigger('click');
    expect(wrapper.emitted('select')?.[0]).toEqual(['b', options[1]]);
  });

  it('空选项时显示「暂无数据」', () => {
    const wrapper = mount(SelectModal, { props: { visible: true, options: [] } });
    expect(wrapper.text()).toContain('暂无数据');
  });
});
