<script lang="ts" setup>
import { useWindowSize } from '@vueuse/core';
import { computed, ref } from 'vue';

import SvgIcon from '@/components/SvgIcon.vue';
import {
  CONTAINER_PADDING,
  CONTAINER_WIDTH,
  DIVIDER_LINE_HEIGHT,
  DIVIDER_MARGIN,
  ROW_HEIGHT,
  ROW_WIDTH,
  calcContainerHeight,
} from '@/stores/context-menu';

interface Props {
  options: ContextMenuOption[];
  /** 本层容器在视口中的位置（用于子菜单边界计算与递归传参） */
  containerPosition: { top: number; left: number };
}

interface Emits {
  select: [option: ContextMenuOption];
}

const { options = [], containerPosition = { top: 0, left: 0 } } = defineProps<Props>();
const emit = defineEmits<Emits>();

const { width: windowWidth, height: windowHeight } = useWindowSize();

const activedIndex = ref(-1);
const activedTop = ref(0);

const childrenPosition = computed(() => {
  const childOptions = options[activedIndex.value]?.children;
  if (!childOptions?.length) return { top: 0, left: ROW_WIDTH - 2 };

  const containerHeight = calcContainerHeight(childOptions);

  const top =
    activedTop.value + containerHeight <= windowHeight.value
      ? -CONTAINER_PADDING
      : -(containerHeight - (windowHeight.value - activedTop.value));
  const left =
    containerPosition.left + CONTAINER_WIDTH * 2 <= windowWidth.value
      ? ROW_WIDTH - 2
      : -CONTAINER_WIDTH;

  return { top, left };
});

/** 子菜单在视口中的位置 = 本层位置 + 相对偏移，传给递归子层用于边界计算 */
const childContainerPosition = computed(() => ({
  top: containerPosition.top + childrenPosition.value.top,
  left: containerPosition.left + childrenPosition.value.left,
}));

const handleChildrenShow = (e: MouseEvent, index: number) => {
  // 用 currentTarget 而非 target：li 自身有 relative，
  // 子元素的 offsetTop 相对 li 而非容器，会算偏
  activedTop.value = containerPosition.top + (e.currentTarget as HTMLElement).offsetTop;
  activedIndex.value = index;
};

const handleClick = (option: ContextMenuOption) => {
  if (!option.onClick) return;

  emit('select', option);
  activedIndex.value = -1;
};

// 透传子层冒泡上来的 select 事件
const handleSelect = (option: ContextMenuOption) => {
  emit('select', option);
};
</script>

<template>
  <ul
    class="rounded-lg border border-border bg-background shadow-lg shadow-shadow"
    :style="{ padding: `${CONTAINER_PADDING}px`, width: `${CONTAINER_WIDTH}px` }"
    @contextmenu.stop.prevent
  >
    <template v-for="(option, index) in options" :key="index">
      <li
        v-if="option.divider"
        class="w-full bg-border"
        :style="{ height: `${DIVIDER_LINE_HEIGHT}px`, marginBlock: `${DIVIDER_MARGIN}px` }"
      ></li>

      <li
        v-else
        class="card-hover relative flex cursor-pointer items-center gap-1 rounded-lg px-2 transition-colors"
        :style="{ height: `${ROW_HEIGHT}px` }"
        :data-disabled="option.disabled"
        :data-label="option.label"
        @click="handleClick(option)"
        @mouseenter="handleChildrenShow($event, index)"
        @mouseleave="activedIndex = -1"
      >
        <div class="size-4">
          <slot name="prefix-icon">
            <SvgIcon v-if="option.prefixIcon" :name="option.prefixIcon" size="16" />
          </slot>
        </div>

        <div class="w-0 flex-1 truncate px-1">{{ option.label }}</div>

        <slot name="suffix-icon">
          <SvgIcon v-if="option.suffixIcon" :name="option.suffixIcon" size="16" />
        </slot>

        <Transition name="zoom-fade">
          <ContextMenu
            v-if="option.children?.length && activedIndex === index"
            class="absolute"
            :style="{
              top: `${childrenPosition.top}px`,
              left: `${childrenPosition.left}px`,
            }"
            :options="option.children"
            :container-position="childContainerPosition"
            @select="handleSelect"
          />
        </Transition>
      </li>
    </template>
  </ul>
</template>
