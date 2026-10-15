import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import { notify } from '@/components/Notification.vue';
import { InvokeApiStatus, Mode, ModeTitle, YouthVip } from '@/constants';
import { invoke } from '@/utils/tools';

import { useSettingStore } from './setting';
import { useStateStore } from './state';

/** 用户配置 */
export const useUserStore = defineStore(
  'user',
  () => {
    const isHydrated = ref(false); // store 持久化的水合状态

    const userinfo = ref<UserInfo>(); // 用户信息
    const userPlaylist = ref<PlaylistInfo[]>([]); // 用户歌单列表

    const route = useRoute();
    const router = useRouter();

    const stateStore = useStateStore();
    const settingStore = useSettingStore();

    watch(
      isHydrated,
      async () => {
        await checkLogin();

        switch (stateStore.mode) {
          case Mode.KgLite:
            await setYouthVipState();

            if (settingStore.autoYouthVip) receiveYouthVip();
            break;
          default:
            break;
        }
      },
      { once: true },
    );

    const setUserPlaylist = (newUserPlaylist: PlaylistInfo[]) => {
      userPlaylist.value = newUserPlaylist;
    };

    // 获取登录状态, 用以同步后端登录状态
    const checkLogin = async () => {
      if (!userinfo.value) return;

      const login_online = await invoke('api_login_online');
      // 后端在线, 保留登录态
      if (login_online) return;

      // 后端离线或异常, 清除登录态
      userinfo.value = undefined;
    };

    /** 获取概念版会员状态 */
    const getYouthVipState = async () => {
      if (!userinfo.value) return;

      try {
        const { status, data } = await invoke('api_youth_union_vip');
        if (status !== InvokeApiStatus.Success) return YouthVip.Not;

        for (const vip of data.busi_vip) {
          if (vip.is_vip !== 1) continue;

          // 按优先级返回
          if (vip.product_type === 'svip') return YouthVip.Svip;
          if (vip.product_type === 'tvip') return YouthVip.Tvip;
          if (vip.product_type === 'qvip') return YouthVip.Qvip;
          if (vip.product_type === 'dvip') return YouthVip.Dvip;
        }
      } catch {
        notify.error(`获取 ${ModeTitle.KgLite} 会员状态失败`);
      }

      return YouthVip.Not;
    };

    /** 设置概念版会员状态 */
    const setYouthVipState = async () => {
      if (!userinfo.value) {
        notify.warning(`领取 ${ModeTitle.KgLite} 会员需先登录`);
        return;
      }

      userinfo.value.youthVip = await getYouthVipState();
    };

    /** 领取概念版会员 */
    const receiveYouthVip = async () => {
      if (!userinfo.value) {
        notify.warning(`领取 ${ModeTitle.KgLite} 会员需先登录`);
        return;
      }
      if (userinfo.value.youthVip === YouthVip.Svip) {
        notify.info(`已领取 ${ModeTitle.KgLite} 会员`);
        return;
      }

      try {
        const youth_day_vip = await invoke('api_youth_day_vip', {});
        // 存在vip等级时无法领取
        if (
          youth_day_vip.status !== InvokeApiStatus.Success &&
          userinfo.value.youthVip === YouthVip.Not
        ) {
          notify.error(`领取 ${ModeTitle.KgLite} 会员失败`);
          return;
        }

        const youth_day_upgrade = await invoke('api_youth_day_upgrade');
        if (youth_day_upgrade.status !== InvokeApiStatus.Success) {
          notify.error(`领取 ${ModeTitle.KgLite} 会员失败`);
          return;
        }

        notify.success(`领取 ${ModeTitle.KgLite} 会员成功`);
        await setYouthVipState();
      } catch {
        notify.error(`领取 ${ModeTitle.KgLite} 会员失败`);
      }
    };

    const login = async (newUserinfo: UserInfo) => {
      userinfo.value = newUserinfo;

      switch (stateStore.mode) {
        case Mode.KgLite:
          userinfo.value.youthVip = await getYouthVipState();
          break;
        case Mode.KgMobile:
          break;
      }
    };

    const logout = async () => {
      if (!userinfo.value) return;

      try {
        await invoke('api_login_out');

        userinfo.value = undefined;

        // 如果在用户歌单页面,重定向到首页
        if (route.name === 'UserPlaylistTable') await router.replace('/');
      } catch {
        notify.error('退出登录失败');
      }
    };

    return {
      isHydrated,
      userinfo,
      userPlaylist,

      getYouthVipState,
      setYouthVipState,
      receiveYouthVip,
      setUserPlaylist,
      login,
      logout,
    };
  },
  {
    persist: {
      key: 'user-store',
      pick: ['userinfo'],
      afterHydrate: (ctx) => (ctx.store.isHydrated = true),
    },
  },
);
