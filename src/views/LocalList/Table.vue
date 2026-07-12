<script setup lang="ts">
import MusicActions from '@/components/MusicTable/MusicActions.vue'
import MusicHeader from '@/components/MusicTable/MusicHeader.vue'
import MusicTable from '@/components/MusicTable/MusicTable.vue'
import { useListStore } from '@/stores/list'
import { ListType } from '@/utils/params'

provide('listType', ListType.Local)

const listStore = useListStore()

onMounted(() => (listStore.isLoading = false))
onUnmounted(() => (listStore.isLoading = true))
</script>

<template>
  <div class="relative space-y-3 pt-4 w-full h-full flex flex-col">
    <MusicHeader />
    <MusicActions />
    <MusicTable class="h-0 flex-1" />
  </div>
</template>
