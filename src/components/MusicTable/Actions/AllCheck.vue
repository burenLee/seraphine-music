<script lang="ts" setup>
import { computed, inject } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import { useListStore } from '@/stores/list';
import { ListType } from '@/utils/params';

const listType = inject<ListType>('listType', ListType.Show);

const listStore = useListStore();

const musicList = computed(() => listStore[listType]);
const isAllChecked = computed(
  () =>
    listStore.checkedList.length > 0 && listStore.checkedList.length == musicList.value.list.length,
);

const handleClick = () => {
  if (isAllChecked.value) {
    listStore.clearCheckedList();
  } else {
    listStore.setCheckedList(musicList.value.list.map((item) => item.id));
  }
};
</script>

<template>
  <ActionButton
    :prefix-icon="isAllChecked ? 'UnreadBold' : 'Unread'"
    :disabled="musicList.list.length === 0"
    @click="handleClick"
  >
    全选
  </ActionButton>
</template>
