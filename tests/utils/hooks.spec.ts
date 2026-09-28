import { flushPromises, mount } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { defineComponent, shallowRef } from 'vue';

import { useObserver } from '@/utils/hooks';

// 与 utils/hooks.ts 内部的 IntersectionObserverCallback 签名一致（单 entry，非 DOM 标准签名）
type ObserverCallback = (entry: IntersectionObserverEntry, observer: IntersectionObserver) => void;

/** 挂载一个使用 useObserver 的测试组件 */
const mountWithObserver = (callback: ObserverCallback) => {
  const TestComp = defineComponent({
    setup() {
      const el = shallowRef<HTMLDivElement | null>(null);
      const { unobserve } = useObserver(el, callback);
      return { el, unobserve };
    },
    template: '<div ref="el" />',
  });
  return mount(TestComp);
};

describe('useObserver', () => {
  it('元素挂载后被观察，可见性回调被触发', async () => {
    const callback = vi.fn();
    const wrapper = mountWithObserver(callback);

    await flushPromises();
    // setup.ts 的 IntersectionObserver mock 在 observe 后以微任务触发「可见」
    await new Promise((resolve) => setTimeout(resolve, 0));

    expect(callback).toHaveBeenCalledTimes(1);
    const [entry] = callback.mock.calls[0];
    expect(entry.isIntersecting).toBe(true);
    expect(entry.target).toBe(wrapper.element);

    wrapper.unmount();
  });

  it('返回的 unobserve 可调用且不抛错', async () => {
    const callback = vi.fn();
    const wrapper = mountWithObserver(callback);

    await flushPromises();

    expect(() => wrapper.vm.unobserve()).not.toThrow();
    wrapper.unmount();
  });

  it('组件卸载时不抛错', async () => {
    const callback = vi.fn();
    const wrapper = mountWithObserver(callback);

    await flushPromises();

    expect(() => wrapper.unmount()).not.toThrow();
  });
});
