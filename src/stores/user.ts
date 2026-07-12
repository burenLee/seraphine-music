import { notify } from '@/components/Notification'
import { invoke } from '@/utils/tools'

export const useUserStore = defineStore(
  'user',
  () => {
    const isHydrated = ref(false) // store 持久化的水合状态

    const isVip = ref(false)
    const userinfo = ref<UserInfo>()
    const userPlaylist = ref<Playlist[]>([])

    watch(
      isHydrated,
      () => {
        if (!userinfo.value) return

        getVipState()
      },
      { once: true }
    )

    const getVipState = async () => {
      const youth_union_vip = await invoke('api_youth_union_vip')
      if (youth_union_vip?.status !== 1) return

      isVip.value = youth_union_vip.data.busi_vip.every((item) => item.is_vip === 1)
    }

    const login = async (newUserinfo: UserInfo) => {
      await getVipState()

      userinfo.value = newUserinfo
      notify.success('登录成功')
    }

    const logout = async () => {
      await invoke('api_login_out')
      await invoke('api_register_dev')
      userinfo.value = undefined
      notify.success('已退出登录')
    }

    const setUserPlaylist = (newUserPlaylist: Playlist[]) => {
      userPlaylist.value = newUserPlaylist
    }

    return {
      isHydrated,
      isVip,
      userinfo,
      userPlaylist,

      getVipState,
      login,
      logout,
      setUserPlaylist
    }
  },
  {
    persist: {
      key: 'user-store',
      pick: ['userinfo'],
      afterHydrate: (ctx) => (ctx.store.isHydrated = true)
    }
  }
)
