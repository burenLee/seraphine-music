import { enableAutoUnmount, mount } from '@vue/test-utils';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { nextTick } from 'vue';

import Modal from '@/components/Modal.vue';

import { createTestPinia } from '../helpers/factories';

enableAutoUnmount(afterEach);

const mask = () => document.body.querySelector('.fixed') as HTMLElement | null;

describe('Modal 模态框', () => {
  beforeEach(() => createTestPinia());

  it('visible=false 时不渲染', () => {
    mount(Modal, { props: { modelValue: false } });
    expect(mask()).toBeNull();
  });

  it('visible=true 时渲染标题与默认按钮', () => {
    mount(Modal, { props: { modelValue: true, title: '标题' } });
    const text = document.body.textContent ?? '';
    expect(text).toContain('标题');
    expect(text).toContain('确认');
    expect(text).toContain('取消');
  });

  it('点击遮罩触发 cancel（maskClosed 默认开启）', async () => {
    const wrapper = mount(Modal, { props: { modelValue: true } });
    mask()?.dispatchEvent(new MouseEvent('click'));
    await nextTick();
    expect(wrapper.emitted('cancel')).toBeTruthy();
  });

  it('maskClosed=false 时点击遮罩不触发 cancel', async () => {
    const wrapper = mount(Modal, { props: { modelValue: true, maskClosed: false } });
    mask()?.dispatchEvent(new MouseEvent('click'));
    await nextTick();
    expect(wrapper.emitted('cancel')).toBeUndefined();
  });

  it('点击确认按钮触发 confirm', async () => {
    const wrapper = mount(Modal, { props: { modelValue: true } });
    const btns = Array.from(document.body.querySelectorAll('button'));
    const confirm = btns.find((b) => b.textContent?.includes('确认'));
    confirm?.dispatchEvent(new MouseEvent('click'));
    await nextTick();
    expect(wrapper.emitted('confirm')).toBeTruthy();
  });

  it('Escape 键触发 cancel', async () => {
    const wrapper = mount(Modal, { props: { modelValue: false } });
    await wrapper.setProps({ modelValue: true });
    await nextTick();
    window.dispatchEvent(new KeyboardEvent('keyup', { key: 'Escape' }));
    await nextTick();
    expect(wrapper.emitted('cancel')).toBeTruthy();
  });

  it('hideFooter / hideConfirm / hideCancel 控制操作区显隐', () => {
    mount(Modal, {
      props: { modelValue: true, hideFooter: false, hideConfirm: true, hideCancel: true },
    });
    const text = document.body.textContent ?? '';
    expect(text).not.toContain('确认');
    expect(text).not.toContain('取消');
  });

  it('hideHeader 隐藏标题栏', () => {
    mount(Modal, { props: { modelValue: true, title: '标题', hideHeader: true } });
    expect(document.body.textContent ?? '').not.toContain('标题');
  });
});
