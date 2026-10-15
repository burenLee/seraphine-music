<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { watchEffect } from 'vue';

import { useContextMenuStore } from '@/stores/context-menu';
import { useStateStore } from '@/stores/state.ts';

import ContextMenu from './ContextMenu.vue';

const stateStore = useStateStore();
const contextMenuStore = useContextMenuStore();

// 仅在弹窗可见时监听 Esc, 组件卸载时自动清理
watchEffect((onCleanup) => {
  if (!contextMenuStore.visible) return;

  window.addEventListener('resize', contextMenuStore.hide);
  window.addEventListener('wheel', contextMenuStore.hide);
  onCleanup(() => {
    window.removeEventListener('resize', contextMenuStore.hide);
    window.removeEventListener('wheel', contextMenuStore.hide);
  });
});
</script>

<template>
  <Teleport :to="contextMenuStore.teleport">
    <div
      class="pointer-events-none fixed inset-0 z-50 overflow-hidden"
      :style="{ fontFamily: stateStore.fontFamily }"
      v-on-click-outside="contextMenuStore.hide"
    >
      <Transition name="fade">
        <ContextMenu
          v-if="contextMenuStore.visible"
          :options="contextMenuStore.options"
          :position="contextMenuStore.position"
          :level="1"
        />
      </Transition>
    </div>
  </Teleport>
</template>
