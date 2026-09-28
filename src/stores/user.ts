import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import { notify } from '@/components/Notification.vue';
import { ApiInvokeStatus, Mode, YouthVip } from '@/utils/params';
import { invoke } from '@/utils/tools';

import { useSettingStore } from './setting';

/** 用户配置 */
export const useUserStore = defineStore(
  'user',
  () => {
    const isHydrated = ref(false); // store 持久化的水合状态

    const userinfo = ref<UserInfo>();
    const userPlaylist = ref<ApiPlaylist[]>([]);

    const route = useRoute();
    const router = useRouter();

    const settingStore = useSettingStore();

    watch(
      isHydrated,
      async () => {
        await getLoginState();
        await setYouthVip();
      },
      { once: true },
    );

    // 获取登录状态, 用以同步后端登录状态
    const getLoginState = async () => {
      if (!userinfo.value) return;

      try {
        const login_online = await invoke('api_login_online');
        if (login_online) return;

        userinfo.value = undefined;
      } catch {
        notify.error('获取登录信息失败');
        userinfo.value = undefined;
      }
    };

    const getYouthVip = async () => {
      if (!userinfo.value) return;

      try {
        const { status, data } = await invoke('api_youth_union_vip');
        if (status !== ApiInvokeStatus.Success) return YouthVip.Not;

        for (const vip of data.busi_vip) {
          if (vip.is_vip !== 1) continue;

          if (vip.product_type === 'svip') return YouthVip.Svip;
          if (vip.product_type === 'tvip') return YouthVip.Tvip;
          if (vip.product_type === 'qvip') return YouthVip.Qvip;
          if (vip.product_type === 'dvip') return YouthVip.Dvip;
        }
      } catch {
        notify.error('获取会员信息失败');
      }

      return YouthVip.Not;
    };

    const setYouthVip = async () => {
      if (!userinfo.value) return;

      userinfo.value.youthVip = await getYouthVip();
    };

    const setUserPlaylist = (newUserPlaylist: ApiPlaylist[]) => {
      userPlaylist.value = newUserPlaylist;
    };

    const login = async (newUserinfo: UserInfo) => {
      userinfo.value = newUserinfo;

      switch (settingStore.mode) {
        case Mode.KgLite:
          userinfo.value.youthVip = await getYouthVip();
          break;
        case Mode.KgMobile:
          break;
        default:
          break;
      }
    };

    const logout = async () => {
      if (!userinfo.value) return;

      try {
        await invoke('api_login_out');
        await invoke('api_register_dev');

        userinfo.value = undefined;
        // 如果在歌单页面,重定向到首页
        if (route.name === 'UserPlaylistTable') await router.replace('/');
      } catch {
        notify.error('退出登录失败');
      }
    };

    return {
      isHydrated,
      userinfo,
      userPlaylist,

      getYouthVip,
      setYouthVip,
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
