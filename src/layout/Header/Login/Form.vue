<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue'
import { notify } from '@/components/Notification'
import SlideBar from '@/components/SlideBar.vue'
import { useUserStore } from '@/stores/user'
import { invoke } from '@/utils/tools'
import { useCountdown } from '@vueuse/core'

interface Emits {
  close: []
}

const emits = defineEmits<Emits>()

const userStore = useUserStore()

const formOptions: SlideOption[] = [
  { label: '手机号', value: 'mobile' },
  { label: '账号', value: 'user', disabled: true }
]

const formSelection = ref(formOptions[0])
const currentIndex = ref(0)
const codeLoading = ref(false)
const phoneForm = ref({ mobile: '', code: '' })

const { remaining, start, reset } = useCountdown(60, {
  onComplete: () => {
    codeLoading.value = false
    reset()
  }
})

const getCode = async () => {
  if (codeLoading.value) return
  if (!phoneForm.value.mobile) return notify.error('请填写手机号')
  if (!/^1[3-9]\d{9}$/.test(phoneForm.value.mobile)) return notify.error('请输入正确的手机号')

  codeLoading.value = true

  const api_login_captcha = await invoke('api_login_captcha', { mobile: phoneForm.value.mobile })
  if (api_login_captcha?.status !== 1) {
    notify.error('获取验证码失败')
    codeLoading.value = false
  } else {
    start()
    notify.success('验证码已发送')
  }
}

const handleConfirm = async () => {
  if (!phoneForm.value.mobile || !phoneForm.value.code) return notify.error('请填写完整信息')
  if (!/^1[3-9]\d{9}$/.test(phoneForm.value.mobile)) return notify.error('请输入正确的手机号')

  const api_login_cellphone = await invoke('api_login_cellphone', phoneForm.value)
  if (api_login_cellphone?.status !== 1) {
    notify.error('登录失败')
    return
  }

  userStore.login(api_login_cellphone.data)
  emits('close')
}
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
          v-model="phoneForm.code" />

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
