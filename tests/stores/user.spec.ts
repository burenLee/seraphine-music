import { beforeEach, describe, expect, it, vi } from 'vitest';

import { useSettingStore } from '@/stores/setting';
import { useUserStore } from '@/stores/user';
import { Mode, YouthVip } from '@/utils/params';

import { createTestPinia, makePlaylist, makeUserInfo } from '../helpers/factories';
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
    const settingStore = useSettingStore();
    const store = useUserStore();
    await flushAsync();
    settingStore.setMode(Mode.KgMobile);
    invokeMock.mockClear();

    await store.login(makeUserInfo());

    expect(store.userinfo).toBeDefined();
    expect(invokeMock).not.toHaveBeenCalledWith('api_youth_union_vip', undefined);
  });

  it('getYouthVip 无登录时直接返回 undefined', async () => {
    const store = useUserStore();
    await flushAsync();

    expect(await store.getYouthVip()).toBeUndefined();
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
    expect(invokeMock).toHaveBeenCalledWith('api_register_dev', undefined);
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

describe('setUserPlaylist', () => {
  it('替换用户歌单', async () => {
    const store = useUserStore();
    await flushAsync();
    const playlists = [makePlaylist(), makePlaylist()];

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
