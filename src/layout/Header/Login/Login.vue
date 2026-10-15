<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { computed, ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Image from '@/components/Image.vue';
import Modal from '@/components/Modal.vue';
import { notify } from '@/components/Notification.vue';
import SelectModal from '@/components/SelectModal.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { InvokeApiStatus, LoginMode, UserAction, YouthVip } from '@/constants';
import { useUserStore } from '@/stores/user';
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
          youth_day_vip.status !== InvokeApiStatus.Success &&
          userStore.userinfo.youthVip === YouthVip.Not
        ) {
          notify.error('领取会员失败');
          return;
        }

        const youth_day_upgrade = await invoke('api_youth_day_upgrade');
        if (youth_day_upgrade.status !== InvokeApiStatus.Success) {
          notify.error('领取会员失败');
          return;
        }

        notify.success('已领取会员');
        userStore.setYouthVipState();
      } catch {
        notify.error('领取会员失败');
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
    class="animate-underline mx-2 cursor-pointer whitespace-nowrap font-bold leading-8"
    @click="modalVisible = true"
  >
    点击登录
  </div>

  <div v-else class="relative" v-on-click-outside="() => (userVisible = false)">
    <div class="flex cursor-pointer items-center gap-1 px-2" @click="userVisible = !userVisible">
      <Image class="size-8 rounded-full" :src="userStore.userinfo.pic" icon="User" />
      <div class="max-w-24 truncate font-bold" :title="userStore.userinfo.nickname">
        {{ userStore.userinfo.nickname }}
      </div>
      <SvgIcon class="transition-transform" :class="userVisible ? 'rotate-180' : ''" name="Down" />

      <div
        v-if="isSvip"
        class="absolute bottom-0 left-7 flex size-4 scale-75 items-center justify-center rounded-full bg-info text-xs font-bold text-neutral-100"
      >
        v
      </div>
    </div>

    <SelectModal
      class="absolute left-1/2 top-full -translate-x-1/2"
      transition="zoom-top"
      :visible="userVisible"
      :options="userOptions"
      @select="handleSelect"
    />
  </div>

  <Modal
    :visible="modalVisible"
    class="relative h-96 w-[40rem]"
    hideHeader
    hideFooter
    @close="modalVisible = false"
  >
    <Sidebar class="w-56" v-model="mode" />

    <Transition name="slide-left-login">
      <QRCode
        v-if="mode === LoginMode.Code"
        class="absolute right-0 top-0 h-full w-[26rem]"
        @close="modalVisible = false"
      />
    </Transition>

    <Transition name="slide-right-login">
      <Form
        v-if="mode === LoginMode.Form"
        class="absolute left-0 top-0 h-full w-[26rem]"
        @close="modalVisible = false"
      />
    </Transition>

    <ActionButton
      class="absolute bottom-8 left-0 z-10 transition-transform duration-500"
      :class="mode === LoginMode.Code ? 'translate-x-[12.5rem]' : 'translate-x-[22rem]'"
      prefixIcon="Left"
      suffixIcon="Right"
      @click="mode = mode === LoginMode.Code ? LoginMode.Form : LoginMode.Code"
    >
      {{ mode === LoginMode.Code ? '账号' : '扫码' }}
    </ActionButton>

    <ActionButton
      class="absolute right-4 top-4 z-50"
      mode="text"
      prefixIcon="Close"
      iconSize="24"
      @click="modalVisible = false"
    />
  </Modal>
</template>
