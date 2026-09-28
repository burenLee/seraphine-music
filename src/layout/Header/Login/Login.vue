<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { computed, ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Image from '@/components/Image.vue';
import Modal from '@/components/Modal.vue';
import { notify } from '@/components/Notification.vue';
import SelectModal from '@/components/SelectModal.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useUserStore } from '@/stores/user';
import { ApiInvokeStatus, LoginMode, UserAction, YouthVip } from '@/utils/params';
import { invoke } from '@/utils/tools';

import Form from './Form.vue';
import QRCode from './QRCode.vue';
import Sidebar from './Sidebar.vue';

const userStore = useUserStore();

const isSvip = computed(() => userStore.userinfo?.youthVip === YouthVip.Svip);
const userOptions = computed<Array<SelectOption<UserAction>>>(() => [
  {
    label: isSvip.value ? '已领取会员' : '领取会员',
    value: UserAction.Vip,
    prefixIcon: 'Verified',
    disabled: isSvip.value,
  },
  { label: '个人资料', value: UserAction.Info, prefixIcon: 'User', disabled: true },
  { label: '退出登录', value: UserAction.Logout, prefixIcon: 'Logout' },
]);

const mode = ref(LoginMode.Code);
const modalVisible = ref(false);
const userVisible = ref(false);

const handleSelect = async (value: UserAction) => {
  switch (value) {
    case UserAction.Vip:
      if (!userStore.userinfo) return;
      if (isSvip.value) {
        notify.info('已领取会员');
        return;
      }

      try {
        const youth_day_vip = await invoke('api_youth_day_vip');
        // 存在vip等级时无法领取
        if (
          youth_day_vip.status !== ApiInvokeStatus.Success &&
          userStore.userinfo.youthVip === YouthVip.Not
        ) {
          notify.error('领取畅听VIP失败');
          return;
        }

        const youth_day_upgrade = await invoke('api_youth_day_upgrade');
        if (youth_day_upgrade.status !== ApiInvokeStatus.Success) {
          notify.error('领取VIP失败');
          return;
        }

        notify.success('已领取会员');
        userStore.setYouthVip();
      } catch {
        notify.error('无法领取会员');
      }
      break;
    case UserAction.Info:
      // TODO: 个人资料
      break;
    case UserAction.Logout:
      userStore.logout();
      break;
  }
};
</script>

<template>
  <div
    v-if="!userStore.userinfo"
    class="animate-underline mx-2 cursor-pointer whitespace-nowrap font-bold leading-6"
    @click="modalVisible = true"
  >
    点击登录
  </div>

  <div
    v-else
    class="relative"
    v-on-click-outside="() => (userVisible = false)"
    @click="userVisible = !userVisible"
  >
    <div class="flex cursor-pointer items-center px-2">
      <Image
        class="size-6 rounded-full"
        :src="userStore.userinfo.pic"
        icon="User"
        :icon-size="16"
      />
      <div class="max-w-24 truncate pl-2 pr-1 font-bold" :title="userStore.userinfo.nickname">
        {{ userStore.userinfo.nickname }}
      </div>
      <SvgIcon class="transition-transform" :class="{ 'rotate-180': userVisible }" name="Down" />

      <div
        v-if="isSvip"
        class="absolute -bottom-1 left-5 flex size-4 scale-[70%] items-center rounded-full border border-border bg-info pl-1 text-xs font-bold text-neutral-100"
      >
        v
      </div>
    </div>

    <SelectModal
      class="pointer-events-auto absolute left-1/2 top-full -translate-x-1/2"
      transition="zoom-top"
      :visible="userVisible"
      :options="userOptions"
      @select="handleSelect"
    />
  </div>

  <Modal
    class="relative h-96 w-[40rem]"
    v-model="modalVisible"
    hideHeader
    hideFooter
    @close="modalVisible = false"
  >
    <SvgIcon
      class="action-icon absolute right-4 top-4 z-50 cursor-pointer"
      name="Close"
      size="20"
      @click="modalVisible = false"
    />

    <Sidebar class="w-56" v-model="mode" />
    <ActionButton
      class="absolute bottom-8 left-0 z-10 transition-transform duration-500"
      :class="mode === LoginMode.Code ? 'translate-x-[12.5rem]' : 'translate-x-[22rem]'"
      prefixIcon="Left"
      suffixIcon="Right"
      @click="mode = mode === LoginMode.Code ? LoginMode.Form : LoginMode.Code"
    >
      {{ mode === LoginMode.Code ? '账号' : '扫码' }}
    </ActionButton>

    <Transition name="slide-login-left">
      <QRCode
        v-if="mode === LoginMode.Code"
        class="absolute right-0 top-0 h-full w-[26rem]"
        @close="modalVisible = false"
      />
    </Transition>

    <Transition name="slide-login-right">
      <Form
        v-if="mode === LoginMode.Form"
        class="absolute left-0 top-0 h-full w-[26rem]"
        @close="modalVisible = false"
      />
    </Transition>
  </Modal>
</template>
