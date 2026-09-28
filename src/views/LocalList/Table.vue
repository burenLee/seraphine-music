<script setup lang="ts">
import { onMounted, onUnmounted, provide } from 'vue';

import MusicActions from '@/components/MusicTable/MusicActions.vue';
import MusicHeader from '@/components/MusicTable/MusicHeader.vue';
import MusicTable from '@/components/MusicTable/MusicTable.vue';
import { useListStore } from '@/stores/list';
import { ListType } from '@/utils/params';

provide('listType', ListType.Local);

const listStore = useListStore();

onMounted(() => {
  listStore.isHeaderLoading = false;
  listStore.isTableLoading = false;
});
onUnmounted(() => {
  listStore.isHeaderLoading = true;
  listStore.isTableLoading = true;
});
</script>

<template>
  <div class="relative flex h-full w-full flex-col space-y-3 pt-4">
    <MusicHeader />
    <MusicActions />
    <MusicTable class="h-0 flex-1" />
  </div>
</template>
