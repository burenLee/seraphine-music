import { describe, expect, it } from 'vitest';

import { YouthVip } from '@/constants';

// 这些枚举值同时是前端 UI、Tauri invoke 参数与窗口事件契约，断言值域防止意外漂移

describe('登录与用户常量', () => {
  it('YouthVip：概念版vip等级按序定义', () => {
    expect(YouthVip.Not).toBe(0);
    expect(YouthVip.Dvip).toBe(1);
    expect(YouthVip.Qvip).toBe(2);
    expect(YouthVip.Tvip).toBe(3);
    expect(YouthVip.Svip).toBe(4);
  });
});
