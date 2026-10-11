<script lang="ts">
const ActionBtnTheme: Record<BtnTheme, string> = {
  default: 'border-border bg-card hover:bg-hover',
  info: 'border-info bg-info-bg text-info hover:bg-info hover:text-neutral-50',
  success: 'border-success bg-success-bg text-success hover:bg-success hover:text-neutral-50',
  warning: 'border-warning bg-warning-bg text-warning hover:bg-warning hover:text-neutral-50',
  error: 'border-error bg-error-bg text-error hover:bg-error hover:text-neutral-50',
} as const;

const ActionBtnSize: Record<BtnSize, string> = {
  small: 'h-6 px-2',
  middle: 'h-8 px-3',
  large: 'h-10 px-4',
} as const;
</script>

<script lang="ts" setup>
import { IconMap, IconName } from '@/utils/icons';
import { cn } from '@/utils/tools';

interface Props {
  mode?: BtnMode;
  theme?: BtnTheme;
  size?: BtnSize;
  /** 前缀图标 */
  prefixIcon?: IconName;
  /** 后缀图标 */
  suffixIcon?: IconName;
  /** 图标尺寸 */
  iconSize?: number | string;
  disabled?: boolean;
}

interface Emits {
  click: [e: MouseEvent];
}

const {
  mode = 'button',
  theme = 'default',
  size = 'middle',
  prefixIcon,
  suffixIcon,
  iconSize = 16,
  disabled,
} = defineProps<Props>();
const emits = defineEmits<Emits>();
</script>

<template>
  <button
    :class="
      cn(
        'flex items-center justify-center rounded-lg border transition-all active:scale-90',
        ActionBtnTheme[theme],
        ActionBtnSize[size],
        mode === 'text' && '!border-none !bg-transparent',
        $attrs.class,
      )
    "
    :data-disabled="disabled"
    @click="emits('click', $event)"
  >
    <component v-if="prefixIcon" :is="IconMap[prefixIcon]" :height="iconSize" :width="iconSize" />

    <div class="whitespace-nowrap px-1 text-xs font-bold">
      <slot></slot>
    </div>

    <component v-if="suffixIcon" :is="IconMap[suffixIcon]" :height="iconSize" :width="iconSize" />
  </button>
</template>
