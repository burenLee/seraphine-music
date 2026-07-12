<script lang="ts" setup>
interface Porps {
  slideWidth?: number
  options: SlideOption[]
  selection?: SlideOption
}

interface Emits {
  change: [option: SlideOption]
}

const { slideWidth = 72, options, selection } = defineProps<Porps>()
const emits = defineEmits<Emits>()

const currentIndex = computed(() => options.findIndex((item) => item.value === selection?.value))
</script>

<template>
  <div
    class="card relative flex text-xs items-center overflow-hidden p-1"
    :style="{ '--slide-width': `${slideWidth}px`, '--slide-height': '2rem' }">
    <!-- 滑块 -->
    <div
      v-if="options.length > 0"
      class="card-actived absolute left-1 top-1 h-[var(--slide-height)] w-[var(--slide-width)] rounded-lg transition-transform duration-300"
      :style="{ transform: `translateX(${slideWidth * currentIndex}px)` }"></div>
    <!-- 无滑块 -->
    <div v-else class="w-full text-center font-bold leading-[var(--slide-height)]">无滑块选项</div>

    <div
      v-for="(item, index) in options"
      :key="index"
      class="relative w-[var(--slide-width)] h-[var(--slide-height)] shrink-0 cursor-pointer rounded-lg text-center font-bold leading-[var(--slide-height)] transition-colors duration-300"
      :class="index === currentIndex ? 'text-background' : 'card-hover'"
      :data-disabled="item.disabled"
      @click="emits('change', item)">
      {{ item.label }}
    </div>
  </div>
</template>
