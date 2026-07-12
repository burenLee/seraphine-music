<script lang="ts" setup>
import { IconMap, IconName } from '@/utils/icons'
import { cn } from '@/utils/tools'

type BtnMode = 'text' | 'button'
type BtnTheme = 'default' | 'info' | 'success' | 'warning' | 'error'

interface Props {
  mode?: BtnMode
  theme?: BtnTheme
  /** 前缀图标 */
  prefixIcon?: IconName
  /** 后缀图标 */
  suffixIcon?: IconName
  /** 图标尺寸 */
  size?: number | string
  disabled?: boolean
}

interface Emits {
  click: [e: MouseEvent]
}

const {
  mode = 'button',
  theme = 'default',
  prefixIcon,
  suffixIcon,
  size = 16,
  disabled
} = defineProps<Props>()
const emits = defineEmits<Emits>()

const themes: Record<BtnTheme, string> = {
  default: 'border-border bg-card hover:bg-hover',
  info: ' border-info bg-info-bg text-info hover:bg-info hover:text-neutral-50',
  success: 'border-success bg-success-bg text-success hover:bg-success hover:text-neutral-50',
  warning: 'border-warning bg-warning-bg text-warning hover:bg-warning hover:text-neutral-50',
  error: 'border-error bg-error-bg text-error hover:bg-error hover:text-neutral-50'
}
</script>

<template>
  <button
    :class="
      cn(
        'flex items-center justify-center px-3 border h-8 rounded-lg text-xs font-bold transition-all active:scale-90',
        mode === 'text' && '!bg-transparent !border-none',
        themes[theme],
        $attrs.class
      )
    "
    :data-disabled="disabled"
    @click="emits('click', $event)">
    <component v-if="prefixIcon" :is="IconMap[prefixIcon]" :height="size" :width="size" />

    <div class="px-1 whitespace-nowrap">
      <slot></slot>
    </div>

    <component v-if="suffixIcon" :is="IconMap[suffixIcon]" :height="size" :width="size" />
  </button>
</template>
