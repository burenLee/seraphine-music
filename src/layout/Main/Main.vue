<script setup lang="ts">
import { useTemplateRef } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import { useRefreshStore } from '@/stores/refresh';

const refreshStore = useRefreshStore();
const containerRef = useTemplateRef('containerRef');

const handleToTop = () => {
  containerRef.value?.scrollTo({ top: 0, behavior: 'smooth' });
};
</script>

<template>
  <main ref="containerRef" class="mt-2 overflow-y-auto overflow-x-hidden">
    <RouterView v-slot="{ Component, route }">
      <Transition :name="route.meta.transition || 'slide-left'" mode="out-in">
        <component :is="Component" :key="refreshStore.key" />
      </Transition>

      <ActionButton
        v-if="route.meta.toTop"
        class="size-8 p-0"
        prefix-icon="Up"
        title="滚到顶部"
        @click="handleToTop"
      />
    </RouterView>
  </main>
</template>
