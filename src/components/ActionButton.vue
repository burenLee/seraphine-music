<script lang="ts" setup>
import { IconMap, IconName } from '@/utils/icons';
import { ActionBtnSize, ActionBtnTheme } from '@/utils/params';
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
        mode === 'text' && '!border-none !bg-transparent',
        ActionBtnTheme[theme],
        ActionBtnSize[size],
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
