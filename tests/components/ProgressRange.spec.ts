import { mount } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';

import ProgressRange from '@/components/ProgressRange.vue';

describe('ProgressRange 进度条', () => {
  it('默认 max=100、step=1、min=0', () => {
    const wrapper = mount(ProgressRange, { props: { modelValue: 50 } });
    const input = wrapper.get('input[type="range"]');
    expect(input.attributes('min')).toBe('0');
    expect(input.attributes('max')).toBe('100');
    expect(input.attributes('step')).toBe('1');
  });

  it('自定义 max / step / direction 透传到 range', () => {
    const wrapper = mount(ProgressRange, {
      props: { modelValue: 20, max: 200, step: 5, direction: 'vertical' },
    });
    const input = wrapper.get('input');
    expect(input.attributes('max')).toBe('200');
    expect(input.attributes('step')).toBe('5');
  });

  it('v-model 双向绑定（.number 转数字）', async () => {
    const wrapper = mount(ProgressRange, { props: { modelValue: 30 } });
    await wrapper.get('input').setValue('70');
    expect(wrapper.emitted('update:modelValue')?.[0]?.[0]).toBe(70);
  });

  it('mousedown / mouseup 分别触发 startChange / stopChange', async () => {
    const wrapper = mount(ProgressRange, { props: { modelValue: 30 } });
    const input = wrapper.get('input');
    await input.trigger('mousedown');
    await input.trigger('mouseup');
    expect(wrapper.emitted('startChange')).toHaveLength(1);
    expect(wrapper.emitted('stopChange')).toHaveLength(1);
  });

  it('进度超出最大值时钳制在 100%', () => {
    const wrapper = mount(ProgressRange, { props: { modelValue: 150, max: 100 } });
    const current = wrapper.findAll('div')[3];
    expect(current.attributes('style')).toContain('--size: 100%');
  });
});
