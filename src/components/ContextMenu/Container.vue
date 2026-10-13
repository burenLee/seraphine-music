<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { useWindowSize } from '@vueuse/core';
import { computed, watch } from 'vue';

import { CONTAINER_WIDTH, calcContainerHeight, useContextMenuStore } from '@/stores/context-menu';
import { useSettingStore } from '@/stores/setting';

import ContextMenu from './ContextMenu.vue';

const settingStore = useSettingStore();
const contextMenuStore = useContextMenuStore();

const { width: windowWidth, height: windowHeight } = useWindowSize();

/** 根容器在视口中的位置（避免溢出窗口） */
const containerPosition = computed(() => {
  const {
    position: { x, y },
    options,
  } = contextMenuStore;

  const containerHeight = calcContainerHeight(options);

  const top = y + containerHeight > windowHeight.value ? y - containerHeight : y;
  const left = x + CONTAINER_WIDTH > windowWidth.value ? x - CONTAINER_WIDTH : x;

  return { top, left };
});

/** 接收任意层级子项冒泡上来的 select 事件 */
const handleSelect = (option: ContextMenuOption) => {
  if (!option.onClick) return;

  option.onClick();
  contextMenuStore.hide();
};

watch(
  () => contextMenuStore.visible,
  (visible, _, onCleanup) => {
    if (!visible) return;

    window.addEventListener('resize', contextMenuStore.hide);
    window.addEventListener('wheel', contextMenuStore.hide);

    onCleanup(() => {
      window.removeEventListener('resize', contextMenuStore.hide);
      window.removeEventListener('wheel', contextMenuStore.hide);
    });
  },
);
</script>

<template>
  <Teleport :to="contextMenuStore.teleport">
    <Transition name="zoom-fade">
      <div
        v-if="contextMenuStore.visible"
        class="fixed z-50"
        :style="{
          fontFamily: settingStore.fontFamily,
          top: `${containerPosition.top}px`,
          left: `${containerPosition.left}px`,
        }"
        v-on-click-outside="contextMenuStore.hide"
        @contextmenu.stop.prevent
      >
        <ContextMenu
          :options="contextMenuStore.options"
          :container-position="containerPosition"
          @select="handleSelect"
        />
      </div>
    </Transition>
  </Teleport>
</template>
