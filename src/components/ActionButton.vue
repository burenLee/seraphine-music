<script lang="ts">
const ActionBtnTheme: Record<BtnTheme, string> = {
  default:
    'border-border bg-card text-foreground hover:bg-hover disabled:text-minor disabled:border-card',
  info: 'border-info bg-info-bg text-info hover:bg-info hover:text-neutral-50 disabled:text-minor disabled:border-info-bg',
  success:
    'border-success bg-success-bg text-success hover:bg-success hover:text-neutral-50 disabled:text-minor disabled:border-success-bg',
  warning:
    'border-warning bg-warning-bg text-warning hover:bg-warning hover:text-neutral-50 disabled:text-minor disabled:border-warning-bg',
  error:
    'border-error bg-error-bg text-error hover:bg-error hover:text-neutral-50 disabled:text-minor disabled:border-error-bg',
} as const;

const ActionBtnSize: Record<BtnSize, string> = {
  small: 'h-6 px-2',
  middle: 'h-8 px-3',
  large: 'h-10 px-4',
} as const;
</script>

<script lang="ts" setup>
import SvgIcon from '@/components/SvgIcon.vue';
import { IconName } from '@/utils/icons';
import { cn } from '@/utils/tools';

interface Props {
  mode?: BtnMode;
  theme?: BtnTheme;
  size?: BtnSize;
  /** 前缀图标 */
  prefixIcon?: IconName;
  /** 后缀图标 */
  suffixIcon?: IconName;
  iconSize?: string | number;
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
        'flex items-center justify-center rounded-lg border text-xs transition-all active:scale-90 disabled:pointer-events-none',
        ActionBtnTheme[theme],
        ActionBtnSize[size],
        mode === 'text' &&
          'h-fit border-none bg-transparent px-0 hover:bg-transparent hover:text-minor',
        $attrs.class,
      )
    "
    tabindex="-1"
    :disabled="disabled"
    @click="emits('click', $event)"
  >
    <SvgIcon v-if="prefixIcon" :name="prefixIcon" :size="iconSize" />

    <div v-if="$slots.default" class="whitespace-nowrap px-1 font-bold">
      <slot></slot>
    </div>

    <SvgIcon v-if="suffixIcon" :name="suffixIcon" :size="iconSize" />
  </button>
</template>
