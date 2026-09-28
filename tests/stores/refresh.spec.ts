import { beforeEach, describe, expect, it } from 'vitest';

import { useRefreshStore } from '@/stores/refresh';

import { createTestPinia } from '../helpers/factories';

beforeEach(() => {
  createTestPinia();
});

describe('刷新计数', () => {
  it('初始 key 为 0，refresh 单调递增', () => {
    const store = useRefreshStore();

    expect(store.key).toBe(0);

    store.refresh();
    store.refresh();

    expect(store.key).toBe(2);
  });
});
