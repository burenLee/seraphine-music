import { beforeEach, describe, expect, it, vi } from 'vitest';

import { Mode, YouthVip } from '@/constants';
import { useStateStore } from '@/stores/state';
import { useUserStore } from '@/stores/user';

import { createTestPinia, makePlaylistInfo, makeUserInfo } from '../helpers/factories';
import { flushAsync, invokeMock, mockInvoke, notifyMock } from '../helpers/mocks';

// user store 依赖 useRoute/useRouter，文件级 mock vue-router
const { routeMock, routerReplaceMock } = vi.hoisted(() => ({
  routeMock: { name: undefined as string | undefined },
  routerReplaceMock: vi.fn(),
}));

vi.mock('vue-router', () => ({
  useRoute: () => routeMock,
  useRouter: () => ({ replace: routerReplaceMock }),
}));

beforeEach(() => {
  createTestPinia();
  routeMock.name = undefined;
});

describe('初始默认值', () => {
  it('未登录初始状态', async () => {
    const store = useUserStore();
    await flushAsync();

    expect(store.isHydrated).toBe(true);
    expect(store.userinfo).toBeUndefined();
    expect(store.userPlaylist).toStrictEqual([]);
  });
});

describe('login 登录（核心路径）', () => {
  it('KgLite 模式登录后获取概念版 VIP 等级', async () => {
    const store = useUserStore();
    await flushAsync();
    mockInvoke({
      api_youth_union_vip: {
        status: 1,
        data: { busi_vip: [{ is_vip: 1, product_type: 'svip' }], is_vip: 1 },
      },
    });

    await store.login(makeUserInfo());

    expect(store.userinfo?.youthVip).toBe(YouthVip.Svip);
  });

  it('VIP 接口异常时提示错误，等级为 Not', async () => {
    const store = useUserStore();
    await flushAsync();
    mockInvoke({
      api_youth_union_vip: {
        status: 1,
        data: { busi_vip: [{ is_vip: 0, product_type: 'svip' }], is_vip: 0 },
      },
    });

    await store.login(makeUserInfo());

    expect(store.userinfo?.youthVip).toBe(YouthVip.Not);
  });

  it('KgMobile 模式登录不查询概念版 VIP', async () => {
    const stateStore = useStateStore();
    const store = useUserStore();
    await flushAsync();
    stateStore.setMode(Mode.KgMobile);
    invokeMock.mockClear();

    await store.login(makeUserInfo());

    expect(store.userinfo).toBeDefined();
    expect(invokeMock).not.toHaveBeenCalledWith('api_youth_union_vip', undefined);
  });

  it('getYouthVipState 无登录时直接返回 undefined', async () => {
    const store = useUserStore();
    await flushAsync();

    expect(await store.getYouthVipState()).toBeUndefined();
    expect(invokeMock).not.toHaveBeenCalledWith('api_youth_union_vip', undefined);
  });
});

describe('logout 登出', () => {
  it('调用后端登出并清空用户信息', async () => {
    const store = useUserStore();
    await flushAsync();
    mockInvoke({ api_youth_union_vip: { status: 0, data: { busi_vip: [], is_vip: 0 } } });
    await store.login(makeUserInfo());
    invokeMock.mockClear();

    await store.logout();

    expect(invokeMock).toHaveBeenCalledWith('api_login_out', undefined);
    expect(store.userinfo).toBeUndefined();
  });

  it('在歌单页面登出时重定向到首页', async () => {
    const store = useUserStore();
    await flushAsync();
    mockInvoke({ api_youth_union_vip: { status: 0, data: { busi_vip: [], is_vip: 0 } } });
    await store.login(makeUserInfo());
    routeMock.name = 'UserPlaylistTable';

    await store.logout();

    expect(routerReplaceMock).toHaveBeenCalledWith('/');
  });

  it('未登录时不调用后端', async () => {
    const store = useUserStore();
    await flushAsync();
    invokeMock.mockClear();

    await store.logout();

    expect(invokeMock).not.toHaveBeenCalledWith('api_login_out', undefined);
  });

  it('登出失败时提示错误', async () => {
    const store = useUserStore();
    await flushAsync();
    mockInvoke({ api_youth_union_vip: { status: 0, data: { busi_vip: [], is_vip: 0 } } });
    await store.login(makeUserInfo());
    invokeMock.mockRejectedValueOnce(new Error('network'));

    await store.logout();

    expect(notifyMock.error).toHaveBeenCalledWith('退出登录失败');
  });
});

describe('receiveYouthVip 自动领取会员', () => {
  it('未登录时提示且不调用后端', async () => {
    const store = useUserStore();
    await flushAsync();
    invokeMock.mockClear();

    await store.receiveYouthVip();

    expect(invokeMock).not.toHaveBeenCalledWith('api_youth_day_vip', {});
    expect(notifyMock.warning).toHaveBeenCalledWith('领取 酷狗概念版 会员需先登录');
  });

  it('领取成功后刷新会员等级', async () => {
    const store = useUserStore();
    await flushAsync();
    mockInvoke({
      api_youth_union_vip: { status: 0, data: { busi_vip: [], is_vip: 0 } },
      api_youth_day_vip: { status: 1, data: {} },
      api_youth_day_upgrade: { status: 1, data: {} },
    });
    await store.login(makeUserInfo());
    invokeMock.mockClear();

    await store.receiveYouthVip();

    expect(invokeMock).toHaveBeenCalledWith('api_youth_day_vip', {});
    expect(invokeMock).toHaveBeenCalledWith('api_youth_day_upgrade', undefined);
    expect(notifyMock.success).toHaveBeenCalledWith('领取 酷狗概念版 会员成功');
  });

  it('已持有会员时提示已领取且不重复请求', async () => {
    const store = useUserStore();
    await flushAsync();
    mockInvoke({
      api_youth_union_vip: {
        status: 1,
        data: { busi_vip: [{ is_vip: 1, product_type: 'svip' }], is_vip: 1 },
      },
    });
    await store.login(makeUserInfo());
    invokeMock.mockClear();

    await store.receiveYouthVip();

    expect(invokeMock).not.toHaveBeenCalledWith('api_youth_day_vip', {});
    expect(notifyMock.info).toHaveBeenCalledWith('已领取 酷狗概念版 会员');
  });

  it('接口状态非成功且未持有会员时提示失败', async () => {
    const store = useUserStore();
    await flushAsync();
    mockInvoke({
      api_youth_union_vip: { status: 0, data: { busi_vip: [], is_vip: 0 } },
      api_youth_day_vip: { status: 0, data: {} },
    });
    await store.login(makeUserInfo());
    invokeMock.mockClear();

    await store.receiveYouthVip();

    expect(notifyMock.error).toHaveBeenCalledWith('领取 酷狗概念版 会员失败');
  });

  it('升级接口失败时提示错误', async () => {
    const store = useUserStore();
    await flushAsync();
    mockInvoke({
      api_youth_union_vip: { status: 0, data: { busi_vip: [], is_vip: 0 } },
      api_youth_day_vip: { status: 1, data: {} },
      api_youth_day_upgrade: { status: 0, data: {} },
    });
    await store.login(makeUserInfo());
    invokeMock.mockClear();

    await store.receiveYouthVip();

    expect(notifyMock.error).toHaveBeenCalledWith('领取 酷狗概念版 会员失败');
  });

  it('请求异常时提示错误', async () => {
    const store = useUserStore();
    await flushAsync();
    mockInvoke({
      api_youth_union_vip: { status: 0, data: { busi_vip: [], is_vip: 0 } },
    });
    await store.login(makeUserInfo());
    invokeMock.mockRejectedValueOnce(new Error('network'));

    await store.receiveYouthVip();

    expect(notifyMock.error).toHaveBeenCalledWith('领取 酷狗概念版 会员失败');
  });

  it('开启自动领取后水合时自动领取', async () => {
    localStorage.setItem('setting-store', JSON.stringify({ autoYouthVip: true }));
    localStorage.setItem('user-store', JSON.stringify({ userinfo: makeUserInfo() }));
    mockInvoke({
      api_login_online: true,
      api_youth_union_vip: { status: 0, data: { busi_vip: [], is_vip: 0 } },
      api_youth_day_vip: { status: 1, data: {} },
    });

    useUserStore();
    await flushAsync();

    expect(invokeMock).toHaveBeenCalledWith('api_youth_day_vip', {});
  });

  it('未开启自动领取时水合不领取', async () => {
    localStorage.setItem('setting-store', JSON.stringify({ autoYouthVip: false }));
    localStorage.setItem('user-store', JSON.stringify({ userinfo: makeUserInfo() }));
    mockInvoke({
      api_login_online: true,
      api_youth_union_vip: { status: 0, data: { busi_vip: [], is_vip: 0 } },
    });

    useUserStore();
    await flushAsync();

    expect(invokeMock).not.toHaveBeenCalledWith('api_youth_day_vip', {});
  });
});

describe('setUserPlaylist', () => {
  it('替换用户歌单', async () => {
    const store = useUserStore();
    await flushAsync();
    const playlists = [makePlaylistInfo(), makePlaylistInfo()];

    store.setUserPlaylist(playlists);

    expect(store.userPlaylist).toStrictEqual(playlists);
  });
});

describe('持久化水合', () => {
  it('后端在线则保留登录态', async () => {
    localStorage.setItem('user-store', JSON.stringify({ userinfo: makeUserInfo({ userid: 42 }) }));
    mockInvoke({
      api_login_online: true,
      api_youth_union_vip: { status: 0, data: { busi_vip: [], is_vip: 0 } },
    });

    const store = useUserStore();
    await flushAsync();

    expect(store.userinfo?.userid).toBe(42);
  });

  it('后端离线则清除登录态', async () => {
    localStorage.setItem('user-store', JSON.stringify({ userinfo: makeUserInfo({ userid: 42 }) }));
    mockInvoke({
      api_login_online: false,
      api_youth_union_vip: { status: 0, data: { busi_vip: [], is_vip: 0 } },
    });

    const store = useUserStore();
    await flushAsync();

    expect(store.userinfo).toBeUndefined();
  });
});
