<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue';
import { useListStore } from '@/stores/list';

const { toggleChecked } = useListStore();
</script>

<template>
  <ActionButton class="size-8" mode="text" prefix-icon="CheckList" @click="toggleChecked" />
</template>
