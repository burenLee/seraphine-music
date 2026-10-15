import { beforeEach, describe, expect, it } from 'vitest';

import { useStateStore } from '@/stores/state';

import { createTestPinia } from '../helpers/factories';

beforeEach(() => {
  createTestPinia();
});

describe('刷新计数', () => {
  it('初始 refreshId 为 0，refresh 单调递增', () => {
    const store = useStateStore();

    expect(store.refreshId).toBe(0);

    store.refresh();
    store.refresh();

    expect(store.refreshId).toBe(2);
  });
});
