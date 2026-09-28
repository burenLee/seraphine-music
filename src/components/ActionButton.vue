<script lang="ts" setup>
import { IconMap, IconName } from '@/utils/icons';
import { cn } from '@/utils/tools';

type BtnMode = 'text' | 'button';
type BtnTheme = 'default' | 'info' | 'success' | 'warning' | 'error';

interface Props {
  mode?: BtnMode;
  theme?: BtnTheme;
  /** 前缀图标 */
  prefixIcon?: IconName;
  /** 后缀图标 */
  suffixIcon?: IconName;
  /** 图标尺寸 */
  size?: number | string;
  disabled?: boolean;
}

interface Emits {
  click: [e: MouseEvent];
}

const {
  mode = 'button',
  theme = 'default',
  prefixIcon,
  suffixIcon,
  size = 16,
  disabled,
} = defineProps<Props>();
const emits = defineEmits<Emits>();

const themes: Record<BtnTheme, string> = {
  default: 'action-default',
  info: 'action-info',
  success: 'action-success',
  warning: 'action-warning',
  error: 'action-error',
};
</script>

<template>
  <button
    :class="
      cn(
        'flex h-8 items-center justify-center rounded-lg border px-3 text-xs font-bold transition-all active:scale-90',
        mode === 'text' && '!border-none !bg-transparent',
        themes[theme],
        $attrs.class,
      )
    "
    :data-disabled="disabled"
    @click="emits('click', $event)"
  >
    <component v-if="prefixIcon" :is="IconMap[prefixIcon]" :height="size" :width="size" />

    <div class="whitespace-nowrap px-1">
      <slot></slot>
    </div>

    <component v-if="suffixIcon" :is="IconMap[suffixIcon]" :height="size" :width="size" />
  </button>
</template>
