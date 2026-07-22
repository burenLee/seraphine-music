<script lang="ts" setup>
import Form from './Form.vue'
import QRCode from './QRCode.vue'
import Sidebar from './Sidebar.vue'
import ActionButton from '@/components/ActionButton.vue'
import Image from '@/components/Image.vue'
import Modal from '@/components/Modal.vue'
import { notify } from '@/components/Notification.tsx'
import SelectModal from '@/components/SelectModal.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useUserStore } from '@/stores/user'
import { LoginMode, UserAction } from '@/utils/params.ts'
import { invoke } from '@/utils/tools.ts'
import { vOnClickOutside } from '@vueuse/components'

const route = useRoute()
const router = useRouter()

const userStore = useUserStore()

const userOptions = computed<Array<SelectOption<UserAction>>>(() => [
  {
    label: userStore.isVip ? '今日已领' : '领取会员',
    value: UserAction.Vip,
    prefixIcon: 'Verified',
    disabled: userStore.isVip
  },
  { label: '个人资料', value: UserAction.Info, prefixIcon: 'User', disabled: true },
  { label: '退出登录', value: UserAction.Logout, prefixIcon: 'Logout' }
])

const mode = ref(LoginMode.Code)
const modalVisible = ref(false)
const userVisible = ref(false)

const handleSelect = async (value: UserAction) => {
  switch (value) {
    case UserAction.Vip:
      await userStore.getVipState()
      if (userStore.isVip) {
        notify.success('已领取会员')
        return
      }

      const youth_day_vip = await invoke('api_youth_day_vip')
      if (youth_day_vip?.status !== 1) {
        notify.error('领取畅听VIP失败')
        return
      }

      const youth_day_upgrade = await invoke('api_youth_day_upgrade')
      if (youth_day_upgrade?.status !== 1) {
        notify.error('升级VIP失败')
        return
      }

      notify.success('升级VIP成功')
      userStore.isVip = true
      break
    case UserAction.Info:
      break
    case UserAction.Logout:
      userStore.logout()
      if (route.path === '/user-playlist-table') router.replace('/')
      break
  }
}
</script>

<template>
  <div
    v-if="!userStore.userinfo"
    class="animate-underline mx-2 cursor-pointer font-bold leading-6"
    @click="modalVisible = true">
    点击登录
  </div>

  <div
    v-else
    class="relative mx-2 flex cursor-pointer items-center"
    v-on-click-outside="() => (userVisible = false)"
    @click="userVisible = !userVisible">
    <Image class="size-6 rounded-full" :img="userStore.userinfo.pic" icon="User" :icon-size="16" />
    <div class="max-w-24 truncate pl-1 font-bold" :title="userStore.userinfo.nickname">
      {{ userStore.userinfo.nickname }}
    </div>
    <SvgIcon class="transition-transform" :class="{ 'rotate-180': userVisible }" name="Down" />

    <div
      v-if="userStore.isVip"
      class="absolute -bottom-1 left-3 flex items-center border border-border pl-1 font-bold size-4 rounded-full bg-info scale-[70%] text-xs text-neutral-100">
      v
    </div>

    <SelectModal
      class="absolute left-1/2 -translate-x-1/2 top-full"
      transition="zoom-top"
      :visible="userVisible"
      :options="userOptions"
      @select="handleSelect" />
  </div>

  <Modal
    class="relative h-96 w-[40rem]"
    v-model="modalVisible"
    hideHeader
    hideFooter
    @close="modalVisible = false">
    <SvgIcon
      class="action-icon absolute right-4 top-4 z-50 cursor-pointer"
      name="Close"
      size="12"
      @click="modalVisible = false" />

    <Sidebar class="w-56" v-model="mode" />
    <ActionButton
      class="absolute bottom-8 left-0 transition-transform z-10 duration-500"
      :class="mode === LoginMode.Code ? 'translate-x-[12.5rem]' : 'translate-x-[22rem]'"
      prefixIcon="Left"
      suffixIcon="Right"
      @click="mode = mode === LoginMode.Code ? LoginMode.Form : LoginMode.Code">
      {{ mode === LoginMode.Code ? '账号' : '扫码' }}
    </ActionButton>

    <Transition name="slide-login-left">
      <QRCode
        v-if="mode === LoginMode.Code"
        class="absolute right-0 top-0 h-full w-[26rem]"
        @close="modalVisible = false" />
    </Transition>

    <Transition name="slide-login-right">
      <Form
        v-if="mode === LoginMode.Form"
        class="absolute left-0 top-0 h-full w-[26rem]"
        @close="modalVisible = false" />
    </Transition>
  </Modal>
</template>
