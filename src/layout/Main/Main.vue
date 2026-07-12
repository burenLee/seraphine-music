<script setup lang="ts">
import ToTop from '@/components/PageActions/ToTop.vue'
import { useRefreshStore } from '@/stores/refresh'

const refreshStore = useRefreshStore()
const containerRef = useTemplateRef('containerRef')

const handleToTop = () => {
  containerRef.value?.scrollTo({ top: 0, behavior: 'smooth' })
}
</script>

<template>
  <main ref="containerRef" class="overflow-y-auto mt-2 overflow-x-hidden">
    <RouterView v-slot="{ Component, route }">
      <Transition :name="route.meta.transition || 'slide-left'" mode="out-in">
        <component :is="Component" :key="refreshStore.key" />
      </Transition>

      <ToTop v-if="route.meta.toTop" @click="handleToTop" />
    </RouterView>
  </main>
</template>
