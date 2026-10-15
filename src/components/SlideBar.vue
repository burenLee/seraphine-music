<script lang="ts" setup>
import { computed } from 'vue';

interface Props {
  slideWidth?: number;
  options: SlideOption[];
  selection?: SlideOption;
}

interface Emits {
  change: [option: SlideOption];
}

const { slideWidth = 72, options, selection } = defineProps<Props>();
const emits = defineEmits<Emits>();

const currentIndex = computed(() => options.findIndex((item) => item.value === selection?.value));
</script>

<template>
  <div
    class="card relative flex items-center overflow-hidden text-xs"
    :style="{ '--slide-width': `${slideWidth}px` }"
  >
    <div
      v-if="options.length > 0"
      class="card-actived absolute left-0 top-0 h-8 w-[var(--slide-width)] rounded-lg transition-transform duration-300"
      :style="{ transform: `translateX(${slideWidth * currentIndex}px)` }"
    ></div>
    <div v-else class="w-full text-center font-bold leading-8">无滑块选项</div>

    <div
      v-for="(item, index) in options"
      :key="index"
      class="relative w-[var(--slide-width)] shrink-0 cursor-pointer rounded-lg text-center font-bold leading-8 transition-colors duration-300"
      :class="index === currentIndex ? 'text-background' : 'card-hover'"
      :data-disabled="item.disabled"
      @click="emits('change', item)"
    >
      {{ item.label }}
    </div>
  </div>
</template>
