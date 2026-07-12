<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue'
import SlideBar from '@/components/SlideBar.vue'
import { useUserStore } from '@/stores/user'
import { QrcodeStatus, QrcodeType } from '@/utils/params'
import { invoke } from '@/utils/tools'
import Qrcode from 'qrcode.vue'

interface Emits {
  close: []
}

const emits = defineEmits<Emits>()

const userStore = useUserStore()

const qrcodeOptions: Array<SlideOption<QrcodeType>> = [
  { label: '酷狗', value: QrcodeType.KG },
  { label: 'QQ', value: QrcodeType.QQ, disabled: true },
  { label: '微信', value: QrcodeType.WX }
]

const POLL_INTERVAL = 2 * 1000 // 轮询间隔时间 2s
const POLL_TIMEOUT = 60 * 1000 // 轮询超时时间 60s

let qrcodeKey = '' // 二维码 key
let pollInterval: number | null = null // 轮询定时器
let pollTimeout: number | null = null // 轮询超时定时器

const qrcodeSelection = ref(qrcodeOptions[0])
const qrcodeLoading = ref(false) // 二维码加载中
const qrcodeStatus = ref<QrcodeStatus>(QrcodeStatus.Ready) // 二维码状态
const qrcodeUrl = ref('') // 二维码 url

const loadQrcode = (optison: SlideOption<QrcodeType>) => {
  qrcodeLoading.value = false
  qrcodeStatus.value = QrcodeStatus.Ready
  qrcodeUrl.value = ''
  qrcodeKey = ''
  cleatTimer()

  switch (optison.value) {
    case QrcodeType.KG:
      loadKgQrcode()
      break
    case QrcodeType.QQ:
      qrcodeUrl.value = ''
      qrcodeKey = ''
      break
    case QrcodeType.WX:
      loadWxQrcode()
      break
  }
}

// 加载 kg 二维码
const loadKgQrcode = async () => {
  if (qrcodeLoading.value) return

  qrcodeLoading.value = true

  const api_login_qr_key = await invoke('api_login_qr_key')
  if (api_login_qr_key?.status !== 1) {
    qrcodeLoading.value = false
    qrcodeStatus.value = QrcodeStatus.Fail
    return
  }

  const api_login_qr_create = await invoke('api_login_qr_create', {
    key: api_login_qr_key.data.qrcode
  })
  if (!api_login_qr_create) {
    qrcodeLoading.value = false
    qrcodeStatus.value = QrcodeStatus.Fail
    return
  }

  qrcodeUrl.value = api_login_qr_create
  qrcodeKey = api_login_qr_key.data.qrcode
  qrcodeLoading.value = false
  checkKgStatus()
}

// 检查 kg 二维码状态
const checkKgStatus = async () => {
  if (pollInterval !== null) clearInterval(pollInterval)

  pollInterval = setInterval(async () => {
    if (!qrcodeKey) {
      qrcodeStatus.value = QrcodeStatus.Fail
      cleatTimer()
      return
    }

    const api_login_qr_check = await invoke('api_login_qr_check', { key: qrcodeKey })
    if (api_login_qr_check?.status !== 1) {
      qrcodeStatus.value = QrcodeStatus.Fail
      cleatTimer()
      return
    }

    if (api_login_qr_check.data.status === 0) {
      // 过期
      qrcodeStatus.value = QrcodeStatus.Timeout
      cleatTimer()
    } else if (api_login_qr_check.data.status === 1) {
      // 等待扫码
      qrcodeStatus.value = QrcodeStatus.Ready
    } else if (api_login_qr_check.data.status === 2) {
      // 待确认
      qrcodeStatus.value = QrcodeStatus.Scan
    } else if (api_login_qr_check.data.status === 4) {
      // 登录成功
      qrcodeStatus.value = QrcodeStatus.Confirm

      userStore.login(api_login_qr_check.data)
      cleatTimer()

      emits('close')
    } else {
      // 未知状态
      qrcodeStatus.value = QrcodeStatus.Fail
      cleatTimer()
    }
  }, POLL_INTERVAL)

  if (pollTimeout !== null) clearTimeout(pollTimeout)

  pollTimeout = setTimeout(() => {
    qrcodeStatus.value = QrcodeStatus.Timeout
    cleatTimer()
  }, POLL_TIMEOUT)
}

// 获取 wx 二维码
const loadWxQrcode = async () => {
  if (qrcodeLoading.value) return

  qrcodeLoading.value = true

  const api_login_wx_create = await invoke('api_login_wx_create')
  if (api_login_wx_create?.errcode !== 0) {
    qrcodeLoading.value = false
    qrcodeStatus.value = QrcodeStatus.Fail
    return
  }

  qrcodeUrl.value = api_login_wx_create.qrcode.qrcodeurl
  qrcodeKey = api_login_wx_create.uuid
  qrcodeLoading.value = false
  checkWxStatus()
}

// 检查 wx 二维码状态
const checkWxStatus = async () => {
  if (pollInterval !== null) clearInterval(pollInterval)

  pollInterval = setInterval(async () => {
    if (!qrcodeUrl.value) {
      qrcodeStatus.value = QrcodeStatus.Fail
      return cleatTimer()
    }

    const api_login_wx_check = await invoke('api_login_wx_check', { uuid: qrcodeKey })
    if (!api_login_wx_check) {
      qrcodeStatus.value = QrcodeStatus.Fail
      return cleatTimer()
    }

    if (api_login_wx_check.wx_errcode === 402) {
      // 过期
      qrcodeStatus.value = QrcodeStatus.Timeout
    } else if (api_login_wx_check.wx_errcode === 403) {
      // 拒绝登录
      qrcodeStatus.value = QrcodeStatus.Fail
      cleatTimer()
    } else if (api_login_wx_check.wx_errcode === 404) {
      // 已经扫描
      qrcodeStatus.value = QrcodeStatus.Scan
    } else if (api_login_wx_check.wx_errcode === 405) {
      // 登录成功
      qrcodeStatus.value = QrcodeStatus.Confirm

      const api_login_openplat = await invoke('api_login_openplat', {
        code: api_login_wx_check.wx_code
      })

      if (!api_login_openplat) {
        qrcodeStatus.value = QrcodeStatus.Fail
        cleatTimer()
        return
      }

      userStore.login(api_login_openplat.data)
      cleatTimer()

      emits('close')
    } else if (api_login_wx_check.wx_errcode === 408) {
      // 等待扫描
      qrcodeStatus.value = QrcodeStatus.Ready
    }
  }, POLL_INTERVAL)

  if (pollTimeout !== null) clearTimeout(pollTimeout)

  pollTimeout = setTimeout(() => {
    qrcodeStatus.value = QrcodeStatus.Timeout
    cleatTimer()
  }, POLL_TIMEOUT)
}

// 清理定时器
const cleatTimer = () => {
  if (pollInterval !== null) {
    clearInterval(pollInterval)
    pollInterval = null
  }

  if (pollTimeout !== null) {
    clearTimeout(pollTimeout)
    pollTimeout = null
  }
}

// 监听 slidebar 选项变化
watch(qrcodeSelection, loadQrcode, { immediate: true })

onUnmounted(cleatTimer)
</script>

<template>
  <div class="flex flex-col items-center px-16 py-8">
    <SlideBar
      :options="qrcodeOptions"
      :selection="qrcodeSelection"
      @change="qrcodeSelection = $event" />

    <div class="card relative border-border border mt-8 size-40">
      <div v-if="qrcodeLoading" class="flex items-center justify-center size-full font-bold">
        获取中...
      </div>

      <Qrcode v-else-if="qrcodeUrl" class="absolute inset-2" :value="qrcodeUrl" :size="144" />

      <div class="absolute inset-0 flex justify-center items-center">
        <div
          v-if="qrcodeStatus === QrcodeStatus.Scan"
          class="p-1 bg-background font-bold rounded-lg">
          等待确认
        </div>

        <div
          v-else-if="qrcodeStatus === QrcodeStatus.Confirm"
          class="p-1 bg-background font-bold rounded-lg">
          已登录
        </div>

        <div
          v-else-if="qrcodeStatus === QrcodeStatus.Timeout"
          class="p-1 bg-background flex items-center font-bold rounded-lg">
          已超时
          <ActionButton
            class="px-1 text-sm h-6 hover:text-info"
            mode="text"
            theme="info"
            suffix-icon="Refresh"
            @click="loadQrcode(qrcodeSelection)">
            刷新
          </ActionButton>
        </div>

        <div
          v-else-if="qrcodeStatus === QrcodeStatus.Fail"
          class="p-1 bg-background flex items-center font-bold rounded-lg">
          失败
          <ActionButton
            class="px-1 text-sm h-6 hover:text-info"
            mode="text"
            theme="info"
            suffix-icon="Refresh"
            @click="loadQrcode(qrcodeSelection)">
            刷新
          </ActionButton>
        </div>
      </div>
    </div>

    <div class="mt-8 font-bold">请使用 {{ qrcodeSelection.label }} 扫码登录</div>
  </div>
</template>
