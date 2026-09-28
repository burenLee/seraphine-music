<script lang="ts" setup>
import { useEventListener } from '@vueuse/core';
import { nextTick, ref, useTemplateRef, watch } from 'vue';

interface Props {
  content: string;
  /**
   * 速度
   * @default 30
   */
  speed?: number;
  /**
   * 间隔
   * @default 64
   */
  padding?: number;
  /**
   * 展示鼠标悬停动画
   * @default false
   */
  hover?: boolean;
}

const { content, speed = 30, padding = 64, hover = false } = defineProps<Props>();

const carouselRef = useTemplateRef('carouselRef'); // 容器元素
const contentRef = useTemplateRef('contentRef'); // 内容元素

const contentWidth = ref(0); // 内容宽度
const shouldAnimate = ref(false); // 是否需要动画

const setWidths = () => {
  shouldAnimate.value = false;

  nextTick(() => {
    if (!carouselRef.value || !contentRef.value) return;

    contentWidth.value = contentRef.value.scrollWidth;
    shouldAnimate.value = contentWidth.value > carouselRef.value.clientWidth;
  });
};

watch(() => content, setWidths, { immediate: true });

useEventListener('resize', setWidths);
</script>

<template>
  <div ref="carouselRef" class="overflow-hidden whitespace-nowrap">
    <div
      ref="contentRef"
      class="inline-flex items-center"
      :class="{ 'animate-carousel': shouldAnimate }"
      :style="{
        animationDelay: '0.5s',
        animationDuration: `${contentWidth / speed}s`,
        '--padding': `${padding}px`,
      }"
    >
      <div :class="hover ? 'animate-underline' : ''">{{ content }}</div>

      <template v-if="shouldAnimate">
        <div class="w-[var(--padding)]"></div>
        <div :class="hover ? 'animate-underline' : ''">{{ content }}</div>
        <div class="w-[var(--padding)]"></div>
      </template>
    </div>
  </div>
</template>
