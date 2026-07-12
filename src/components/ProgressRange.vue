<script lang="ts" setup>
import { cn } from '@/utils/tools'

interface Props {
  loadingProgress?: number // 加载进度
  max?: number // 最大值
  step?: number // 步进
  /**
   * 显示方向
   * @param horizontal 水平
   * @param vertical 垂直
   * @default "horizontal"
   */
  direction?: 'horizontal' | 'vertical'
  /**
   * 显示模式
   * @param always 一直显示
   * @param hover 鼠标悬停显示
   * @default "always"
   */
  showMode?: 'always' | 'hover'
  disabled?: boolean
}

interface IEmits {
  startChange: [value: number, e: MouseEvent]
  stopChange: [value: number, e: MouseEvent]
}

const progress = defineModel<number>({ required: true, default: 0 })
const {
  loadingProgress = 0,
  max = 100,
  step = 1,
  direction = 'horizontal',
  showMode = 'always',
  disabled
} = defineProps<Props>()
const emit = defineEmits<IEmits>()

const loadingRatio = computed(() => Math.min(loadingProgress * 100, 100))
const ratio = computed(() => Math.min(Math.max(0, (progress.value / max) * 100), 100))
</script>

<template>
  <div
    :class="
      cn('group relative leading-[0]', direction === 'vertical' ? 'h-full' : 'w-full', $attrs.class)
    "
    :data-disabled="disabled">
    <!-- 总进度条 -->
    <div
      class="pointer-events-none absolute rounded bg-black/10 dark:bg-white/10"
      :class="
        direction === 'vertical'
          ? 'bottom-0 left-1/2 h-full w-0.5 -translate-x-1/2 group-hover:w-1'
          : 'left-0 top-1/2 h-0.5 w-full -translate-y-1/2 group-hover:h-1'
      "></div>

    <!-- 加载进度条 -->
    <div
      class="pointer-events-none absolute rounded bg-black/15 dark:bg-white/15"
      :class="
        direction === 'vertical'
          ? 'bottom-0 left-1/2 h-[var(--size)] w-0.5 -translate-x-1/2 group-hover:w-1'
          : 'left-0 top-1/2 h-0.5 w-[var(--size)] -translate-y-1/2 group-hover:h-1'
      "
      :style="{ '--size': `${loadingRatio}%` }"></div>

    <!-- 当前进度条 -->
    <div
      class="pointer-events-none absolute rounded bg-black/50 dark:bg-white/50"
      :class="
        direction === 'vertical'
          ? 'bottom-0 left-1/2 h-[var(--size)] w-0.5 -translate-x-1/2 group-hover:w-1'
          : 'left-0 top-1/2 h-0.5 w-[var(--size)] -translate-y-1/2 group-hover:h-1'
      "
      :style="{ '--size': `${ratio}%` }"></div>

    <input
      class="relative cursor-pointer appearance-none bg-transparent"
      :class="[
        direction === 'vertical' ? 'h-full [direction:rtl] [writing-mode:vertical-lr]' : 'w-full',
        showMode === 'hover' ? 'opacity-0 group-hover:opacity-100' : ''
      ]"
      type="range"
      :min="0"
      :max="max"
      :step="step"
      v-model.number="progress"
      @mousedown="emit('startChange', ($event.target as HTMLInputElement).valueAsNumber, $event)"
      @mouseup="emit('stopChange', ($event.target as HTMLInputElement).valueAsNumber, $event)" />
  </div>
</template>
