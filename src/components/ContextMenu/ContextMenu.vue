<script lang="ts">
/** 容器内边距 */
const ContainerPadding = 6;
/** 容器宽度 */
const ContainerWidth = 176;
/** 分割线高度 */
const DividerHeight = 12;
/** 行高度 */
const RowHeight = 32;
/** 行宽度 */
const RowWidth = ContainerWidth - ContainerPadding * 2;
</script>

<script lang="ts" setup>
import { useWindowSize } from '@vueuse/core';
import { computed, ref } from 'vue';

import SvgIcon from '@/components/SvgIcon.vue';
import { useContextMenuStore } from '@/stores/context-menu';

interface Props {
  options: ContextMenuOption[];
  /** 本层容器在视口中的预计位置 */
  position: { top: number; left: number };
  /** 层级 */
  level: number;
}

const { options, position, level } = defineProps<Props>();

const contextMenuStore = useContextMenuStore();

const { width: windowWidth, height: windowHeight } = useWindowSize();

const activedIndex = ref(-1);

// 实际显示的位置
const showPosition = computed(() => {
  if (!options.length) return { top: 0, left: 0 };

  const containerHeight = calcHeight(options.length - 1);

  const top =
    position.top + containerHeight <= windowHeight.value
      ? position.top
      : windowHeight.value - containerHeight;
  const left =
    position.left + ContainerWidth <= windowWidth.value
      ? position.left
      : position.left - ContainerWidth * level;

  return { top, left };
});

/**
 * 计算给定索引的菜单高度
 * @param index - 索引
 */
const calcHeight = (index: number) => {
  if (index < 0) return 0;

  return (
    options
      .slice(0, index + 1)
      .reduce((h, option) => h + (option.divider ? DividerHeight : RowHeight), 0) +
    ContainerPadding * 2
  );
};

const handleClick = (option: ContextMenuOption) => {
  contextMenuStore.hide();

  activedIndex.value = -1;
  option.onClick?.();
};
</script>

<template>
  <ul
    class="card pointer-events-auto fixed z-10 bg-background shadow-lg shadow-shadow"
    :style="{
      padding: `${ContainerPadding}px`,
      width: `${ContainerWidth}px`,
      top: `${showPosition.top}px`,
      left: `${showPosition.left}px`,
      '--divider-height': `${DividerHeight}px`,
      '--row-height': `${RowHeight}px`,
      '--row-width': `${RowWidth}px`,
    }"
  >
    <template v-for="(option, index) in options" :key="index">
      <li v-if="option.divider" class="flex h-[var(--divider-height)] items-center">
        <div class="h-px w-full bg-border"></div>
      </li>

      <li
        v-else
        class="card-hover relative flex h-[var(--row-height)] w-[var(--row-width)] cursor-pointer items-center gap-1 rounded-lg px-2"
        :data-disabled="option.disabled"
        @click="handleClick(option)"
        @mouseenter="activedIndex = index"
        @mouseleave="activedIndex = index"
      >
        <div class="size-4">
          <slot name="prefix-icon">
            <SvgIcon v-if="option.prefixIcon" :name="option.prefixIcon" size="16" />
          </slot>
        </div>

        <div class="w-0 flex-1 truncate px-1">{{ option.label }}</div>

        <slot name="suffix-icon">
          <SvgIcon v-if="option.suffixIcon" :name="option.suffixIcon" size="16" />
          <SvgIcon v-else-if="option.children?.length" name="Right" size="16" />
        </slot>

        <Transition name="fade">
          <ContextMenu
            v-if="option.children?.length && activedIndex === index"
            :options="option.children"
            :position="{
              top: showPosition.top + calcHeight(activedIndex - 1) - ContainerPadding * 2,
              left: showPosition.left + ContainerWidth,
            }"
            :level="level + 1"
          />
        </Transition>
      </li>
    </template>
  </ul>
</template>
