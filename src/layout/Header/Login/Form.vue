<script lang="ts" setup>
import { useCountdown } from '@vueuse/core';
import { ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import { notify } from '@/components/Notification.vue';
import SlideBar from '@/components/SlideBar.vue';
import { useUserStore } from '@/stores/user';
import { ApiInvokeStatus } from '@/utils/params';
import { invoke } from '@/utils/tools';

interface Emits {
  close: [];
}

const emits = defineEmits<Emits>();

const userStore = useUserStore();

const formOptions: SlideOption[] = [
  { label: '手机号', value: 'mobile' },
  { label: '账号', value: 'user', disabled: true },
];

const formSelection = ref(formOptions[0]);
const currentIndex = ref(0);
const codeLoading = ref(false);
const phoneForm = ref({ mobile: '', code: '' });

const { remaining, start, reset } = useCountdown(60, {
  onComplete: () => {
    codeLoading.value = false;
    reset();
  },
});

const getCode = async () => {
  if (codeLoading.value) return;
  if (!phoneForm.value.mobile) {
    notify.error('请填写手机号');
    return;
  }
  if (!/^1[3-9]\d{9}$/.test(phoneForm.value.mobile)) {
    notify.error('请输入正确的手机号');
    return;
  }

  codeLoading.value = true;

  try {
    const { status } = await invoke('api_login_captcha', { mobile: phoneForm.value.mobile });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('无法获取验证码');
      codeLoading.value = false;
    } else {
      notify.success('验证码已发送');
      start();
    }
  } catch {
    notify.error('无法获取验证码');
    codeLoading.value = false;
  }
};

const handleConfirm = async () => {
  if (!phoneForm.value.mobile || !phoneForm.value.code) {
    notify.error('请填写完整信息');
    return;
  }
  if (!/^1[3-9]\d{9}$/.test(phoneForm.value.mobile)) {
    notify.error('请输入正确的手机号');
    return;
  }

  try {
    const { status, data } = await invoke('api_login_cellphone', phoneForm.value);
    if (status !== ApiInvokeStatus.Success) {
      notify.error('登录失败');
      return;
    }

    userStore.login(data);
    emits('close');
  } catch {
    notify.error('登录失败');
  }
};
</script>

<template>
  <div class="flex flex-col items-center px-16 py-8">
    <SlideBar :options="formOptions" :selection="formSelection" @change="formSelection = $event" />

    <form v-if="currentIndex === 0" class="mt-8 w-56" @submit.prevent="handleConfirm">
      <input class="card w-full px-3 leading-8" placeholder="手机号" v-model="phoneForm.mobile" />

      <div class="mt-6 flex w-full gap-3">
        <input
          class="card w-0 flex-1 px-3 leading-8"
          placeholder="验证码"
          v-model="phoneForm.code"
        />

        <ActionButton v-if="!codeLoading" class="w-24" type="button" theme="info" @click="getCode">
          获取验证码
        </ActionButton>
        <ActionButton v-else class="w-24" type="button" :disabled="true">
          {{ remaining }}
        </ActionButton>
      </div>

      <ActionButton class="mt-8 w-full" type="submit" theme="success">登录</ActionButton>
    </form>
  </div>
</template>
