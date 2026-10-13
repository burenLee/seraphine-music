import { mount } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';

import SvgIcon from '@/components/SvgIcon.vue';
import { IconMap } from '@/utils/icons';

describe('SvgIcon 图标组件', () => {
  it('渲染指定图标组件', () => {
    const wrapper = mount(SvgIcon, { props: { name: 'Play' } });
    expect(wrapper.findComponent(IconMap.Play).exists()).toBe(true);
  });

  it('默认尺寸为 16, 且装饰性图标对辅助技术隐藏', () => {
    const wrapper = mount(SvgIcon, { props: { name: 'Play' } });
    const svg = wrapper.find('svg');
    expect(svg.attributes('width')).toBe('16');
    expect(svg.attributes('height')).toBe('16');
    expect(svg.attributes('aria-hidden')).toBe('true');
  });

  it('自定义尺寸与 disabled 状态', () => {
    const wrapper = mount(SvgIcon, { props: { name: 'Pause', size: 32, disabled: true } });
    const svg = wrapper.find('svg');
    expect(svg.attributes('data-disabled')).toBe('true');
    expect(svg.attributes('width')).toBe('32');
  });

  it('disabled 为 false 时不渲染 data-disabled 属性', () => {
    const wrapper = mount(SvgIcon, { props: { name: 'Play' } });
    expect(wrapper.attributes('data-disabled')).toBeUndefined();
  });
});
