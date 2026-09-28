import { mount } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';

import SvgIcon from '@/components/SvgIcon.vue';
import { IconMap } from '@/utils/icons';

describe('SvgIcon 图标组件', () => {
  it('渲染指定图标组件', () => {
    const wrapper = mount(SvgIcon, { props: { name: 'Play' } });
    expect(wrapper.findComponent(IconMap.Play).exists()).toBe(true);
  });

  it('默认尺寸为 16', () => {
    const wrapper = mount(SvgIcon, { props: { name: 'Play' } });
    const svg = wrapper.find('svg');
    expect(svg.attributes('width')).toBe('16');
    expect(svg.attributes('height')).toBe('16');
  });

  it('自定义尺寸与 disabled 状态', () => {
    const wrapper = mount(SvgIcon, { props: { name: 'Pause', size: 32, disabled: true } });
    expect(wrapper.attributes('data-disabled')).toBe('true');
    expect(wrapper.find('svg').attributes('width')).toBe('32');
  });
});
