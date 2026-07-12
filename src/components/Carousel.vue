<script lang="ts" setup>
import { useEventListener } from '@vueuse/core'

interface Props {
  content: string
  /**
   * 速度
   * @default 30
   */
  speed?: number
  /**
   * 间隔
   * @default 64
   */
  padding?: number
}

const { content, speed = 30, padding = 64 } = defineProps<Props>()

const carouselRef = useTemplateRef('carouselRef') // 容器元素
const contentRef = useTemplateRef('contentRef') // 内容元素

const contentWidth = ref(0) // 内容宽度
const shouldAnimate = ref(false) // 是否需要动画

const setWidths = () => {
  shouldAnimate.value = false

  nextTick(() => {
    if (!carouselRef.value || !contentRef.value) return

    contentWidth.value = contentRef.value.scrollWidth
    shouldAnimate.value = contentWidth.value > carouselRef.value.clientWidth + padding
  })
}

watch(() => content, setWidths, { immediate: true })

useEventListener('resize', setWidths)
</script>

<template>
  <div ref="carouselRef" class="overflow-hidden whitespace-nowrap">
    <div
      ref="contentRef"
      class="inline-block"
      :class="{ 'animate-carousel': shouldAnimate }"
      :style="{
        animationDelay: '0.5s',
        animationDuration: `${contentWidth / speed}s`,
        '--padding': `${padding}px`
      }">
      <span class="pr-[var(--padding)]">{{ content }}</span>
      <span v-if="shouldAnimate" class="pr-[var(--padding)]">{{ content }}</span>
    </div>
  </div>
</template>
