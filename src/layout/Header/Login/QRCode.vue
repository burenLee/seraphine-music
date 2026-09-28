<script lang="ts" setup>
import Qrcode from 'qrcode.vue';
import { onUnmounted, ref, watch } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import SlideBar from '@/components/SlideBar.vue';
import { useUserStore } from '@/stores/user';
import { ApiInvokeStatus, QrcodeStatus, QrcodeType } from '@/utils/params';
import { invoke } from '@/utils/tools';

interface Emits {
  close: [];
}

const emits = defineEmits<Emits>();

const userStore = useUserStore();

const qrcodeOptions: Array<SlideOption<QrcodeType>> = [
  { label: '酷狗', value: QrcodeType.KG },
  { label: 'QQ', value: QrcodeType.QQ, disabled: true },
  { label: '微信', value: QrcodeType.WX },
];

const POLL_INTERVAL = 1 * 1000; // 轮询间隔时间 1s
const POLL_TIMEOUT = 60 * 1000; // 轮询超时时间 60s

let qrcodeKey = ''; // 二维码 key
let pollInterval: ReturnType<typeof setTimeout> | null = null; // 轮询定时器
let pollTimeout: ReturnType<typeof setTimeout> | null = null; // 轮询超时定时器

const qrcodeSelection = ref(qrcodeOptions[0]);
const qrcodeLoading = ref(false); // 二维码加载中
const qrcodeStatus = ref<QrcodeStatus>(QrcodeStatus.Ready); // 二维码状态
const qrcodeUrl = ref(''); // 二维码 url

const loadQrcode = (optison: SlideOption<QrcodeType>) => {
  qrcodeLoading.value = false;
  qrcodeStatus.value = QrcodeStatus.Ready;
  qrcodeUrl.value = '';
  qrcodeKey = '';
  cleatTimer();

  switch (optison.value) {
    case QrcodeType.KG:
      loadKgQrcode();
      break;
    case QrcodeType.QQ:
      qrcodeUrl.value = '';
      qrcodeKey = '';
      break;
    case QrcodeType.WX:
      loadWxQrcode();
      break;
  }
};

// 加载 酷狗 二维码
const loadKgQrcode = async () => {
  if (qrcodeLoading.value) return;

  qrcodeLoading.value = true;

  try {
    const login_qr_key = await invoke('api_login_qr_key');
    if (login_qr_key.status !== ApiInvokeStatus.Success) {
      qrcodeStatus.value = QrcodeStatus.Fail;
      qrcodeLoading.value = false;
      return;
    }

    const login_qr_create = await invoke('api_login_qr_create', { key: login_qr_key.data.qrcode });
    if (!login_qr_create) {
      qrcodeStatus.value = QrcodeStatus.Fail;
      qrcodeLoading.value = false;
      return;
    }

    qrcodeKey = login_qr_key.data.qrcode;
    qrcodeUrl.value = login_qr_create;
    checkKgStatus();
  } catch {
    qrcodeStatus.value = QrcodeStatus.Fail;
  } finally {
    qrcodeLoading.value = false;
  }
};

// 检查 酷狗 二维码状态
const checkKgStatus = async () => {
  if (pollInterval !== null) clearInterval(pollInterval);

  pollInterval = setInterval(async () => {
    // 存在用户信息为已经登录
    if (userStore.userinfo) return;

    if (!qrcodeKey) {
      qrcodeStatus.value = QrcodeStatus.Fail;
      cleatTimer();
      return;
    }

    try {
      const { status, data } = await invoke('api_login_qr_check', { key: qrcodeKey });
      if (status !== ApiInvokeStatus.Success) {
        qrcodeStatus.value = QrcodeStatus.Fail;
        cleatTimer();
        return;
      }

      switch (data.status) {
        case 0: // 过期
          qrcodeStatus.value = QrcodeStatus.Timeout;
          cleatTimer();
          break;
        case 1: // 等待扫码
          qrcodeStatus.value = QrcodeStatus.Ready;
          break;
        case 2: // 待确认
          qrcodeStatus.value = QrcodeStatus.Scan;
          break;
        case 4: // 登录成功
          qrcodeStatus.value = QrcodeStatus.Confirm;
          cleatTimer();

          userStore.login(data);
          emits('close');
          break;
        default: // 未知状态
          qrcodeStatus.value = QrcodeStatus.Fail;
          cleatTimer();
          break;
      }
    } catch {
      qrcodeStatus.value = QrcodeStatus.Fail;
      cleatTimer();
    }
  }, POLL_INTERVAL);

  if (pollTimeout !== null) clearTimeout(pollTimeout);
  pollTimeout = setTimeout(() => {
    qrcodeStatus.value = QrcodeStatus.Timeout;
    cleatTimer();
  }, POLL_TIMEOUT);
};

// 获取 wx 二维码
const loadWxQrcode = async () => {
  if (qrcodeLoading.value) return;

  qrcodeLoading.value = true;

  try {
    const { errcode, uuid, qrcode } = await invoke('api_login_wx_create');
    if (errcode === 0) {
      qrcodeKey = uuid;
      qrcodeUrl.value = qrcode.qrcodeurl;
      checkWxStatus();
    } else {
      qrcodeStatus.value = QrcodeStatus.Fail;
    }
  } catch {
    qrcodeStatus.value = QrcodeStatus.Fail;
  } finally {
    qrcodeLoading.value = false;
  }
};

// 检查 wx 二维码状态
const checkWxStatus = async () => {
  if (pollInterval !== null) clearInterval(pollInterval);

  pollInterval = setInterval(async () => {
    // 存在用户信息为已经登录
    if (userStore.userinfo) return;

    if (!qrcodeUrl.value) {
      qrcodeStatus.value = QrcodeStatus.Fail;
      cleatTimer();
      return;
    }

    try {
      const { wx_code, wx_errcode } = await invoke('api_login_wx_check', { uuid: qrcodeKey });

      switch (wx_errcode) {
        case 402: // 过期
          qrcodeStatus.value = QrcodeStatus.Timeout;
          break;
        case 403: // 拒绝登录
          qrcodeStatus.value = QrcodeStatus.Fail;
          cleatTimer();
          break;
        case 404: // 已经扫描
          qrcodeStatus.value = QrcodeStatus.Scan;
          break;
        case 405: // 登录成功
          qrcodeStatus.value = QrcodeStatus.Confirm;
          cleatTimer();

          const { status, data } = await invoke('api_login_openplat', { code: wx_code });
          if (status !== ApiInvokeStatus.Success) {
            qrcodeStatus.value = QrcodeStatus.Fail;
          } else {
            userStore.login(data);
            emits('close');
          }
          break;
        case 408: // 等待扫描
          qrcodeStatus.value = QrcodeStatus.Ready;
          break;
        default:
          qrcodeStatus.value = QrcodeStatus.Fail;
          cleatTimer();
          break;
      }
    } catch {
      qrcodeStatus.value = QrcodeStatus.Fail;
      cleatTimer();
    }
  }, POLL_INTERVAL);

  if (pollTimeout !== null) clearTimeout(pollTimeout);
  pollTimeout = setTimeout(() => {
    qrcodeStatus.value = QrcodeStatus.Timeout;
    cleatTimer();
  }, POLL_TIMEOUT);
};

// 清理定时器
const cleatTimer = () => {
  if (pollInterval !== null) {
    clearInterval(pollInterval);
    pollInterval = null;
  }

  if (pollTimeout !== null) {
    clearTimeout(pollTimeout);
    pollTimeout = null;
  }
};

// 监听 slidebar 选项变化
watch(qrcodeSelection, loadQrcode, { immediate: true });

onUnmounted(cleatTimer);
</script>

<template>
  <div class="flex flex-col items-center px-16 py-8">
    <SlideBar
      :options="qrcodeOptions"
      :selection="qrcodeSelection"
      @change="qrcodeSelection = $event"
    />

    <div class="card relative mt-8 size-40 border border-border">
      <div v-if="qrcodeLoading" class="flex size-full items-center justify-center font-bold">
        获取中...
      </div>

      <Qrcode v-else-if="qrcodeUrl" class="absolute inset-2" :value="qrcodeUrl" :size="144" />

      <div class="absolute inset-0 flex items-center justify-center">
        <div
          v-if="qrcodeStatus === QrcodeStatus.Scan"
          class="rounded-lg bg-background px-2 py-1 font-bold"
        >
          等待确认
        </div>

        <div
          v-else-if="qrcodeStatus === QrcodeStatus.Confirm"
          class="rounded-lg bg-background px-2 py-1 font-bold"
        >
          已登录
        </div>

        <div
          v-else-if="qrcodeStatus === QrcodeStatus.Timeout"
          class="flex items-center rounded-lg bg-background px-2 py-1 font-bold"
        >
          已超时
          <ActionButton
            class="h-6 px-1 text-sm hover:text-info"
            mode="text"
            theme="info"
            suffix-icon="Refresh"
            @click="loadQrcode(qrcodeSelection)"
          >
            刷新
          </ActionButton>
        </div>

        <div
          v-else-if="qrcodeStatus === QrcodeStatus.Fail"
          class="flex items-center rounded-lg bg-background px-2 py-1 font-bold"
        >
          失败
          <ActionButton
            class="h-6 px-1 text-sm hover:text-info"
            mode="text"
            theme="info"
            suffix-icon="Refresh"
            @click="loadQrcode(qrcodeSelection)"
          >
            刷新
          </ActionButton>
        </div>
      </div>
    </div>

    <div class="mt-8 font-bold">请使用 {{ qrcodeSelection.label }} 扫码登录</div>
  </div>
</template>
