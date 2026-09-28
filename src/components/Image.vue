<script lang="ts" setup>
import { ref, watch } from 'vue';

import SvgIcon from '@/components/SvgIcon.vue';
import { IconName } from '@/utils/icons';
import { cn } from '@/utils/tools';

interface Props {
  src: string;
  icon?: IconName;
  iconSize?: number;
}

const props = defineProps<Props>();

const isLoading = ref(false);
const isReady = ref(false);

watch(
  () => props.src,
  (src) => {
    // 空字符串不发起加载，直接回落到占位图标
    if (!src) {
      isLoading.value = false;
      isReady.value = false;
      return;
    }

    isLoading.value = true;
    isReady.value = false;

    const img = new Image();
    img.onload = () => {
      isLoading.value = false;
      isReady.value = true;
    };
    img.onerror = () => {
      isLoading.value = false;
      isReady.value = false;
    };

    img.src = src;
  },
  { immediate: true },
);
</script>

<template>
  <SvgIcon
    v-if="!props.src || isLoading || !isReady"
    :class="cn('card', $attrs.class)"
    :name="props.icon || 'Music'"
    :size="props.iconSize || 20"
  />
  <img
    v-else
    :class="cn('card', $attrs.class)"
    :src="props.src"
    alt=""
    decoding="async"
    :draggable="false"
  />
</template>
