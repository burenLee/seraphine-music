<script lang="ts" setup>
import { ref, watchEffect } from 'vue';

import SvgIcon from '@/components/SvgIcon.vue';
import { IconName } from '@/utils/icons';
import { cn } from '@/utils/tools';

interface Props {
  src: string | null | undefined;
  icon?: IconName;
  iconSize?: string | number;
}

const props = defineProps<Props>();

const isLoading = ref(false);
const isReady = ref(false);

// 复用同一个 Image 实例，避免每次 src 变化都重新创建
const img = new Image();
img.onload = () => {
  isLoading.value = false;
  isReady.value = true;
};
img.onerror = () => {
  isLoading.value = false;
  isReady.value = false;
};

watchEffect(() => {
  if (!props.src) {
    // 空字符串不发起加载，直接回落到占位图标
    isLoading.value = false;
    isReady.value = false;
  } else {
    isLoading.value = true;
    isReady.value = false;

    img.src = props.src;
  }
});
</script>

<template>
  <div v-if="isLoading" :class="cn('card', $attrs.class)"></div>
  <div
    v-else-if="!props.src || !isReady"
    :class="cn('card flex items-center justify-center', $attrs.class)"
  >
    <SvgIcon class="pointer-events-none" :name="props.icon || 'Music'" :size="props.iconSize" />
  </div>

  <img
    v-else
    :class="cn('card', $attrs.class)"
    :src="props.src"
    alt=""
    decoding="async"
    loading="lazy"
    :draggable="false"
  />
</template>
