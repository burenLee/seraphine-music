<script lang="ts" setup>
import { computed, inject, ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Modal from '@/components/Modal.vue';
import { useListStore } from '@/stores/list';
import { ListType } from '@/utils/params';

const listType = inject<ListType>('listType', ListType.Show);

const listStore = useListStore();

const visible = ref(false);

const musicList = computed(() => listStore[listType]);

const handleCancel = () => {
  visible.value = false;
};

const handleConfirm = () => {
  listStore.clearList(listType);
  handleCancel();
};
</script>

<template>
  <ActionButton
    theme="error"
    prefix-icon="Bin"
    :disabled="musicList.list.length === 0"
    @click="visible = true"
  >
    清空
  </ActionButton>

  <Modal
    v-model="visible"
    class="w-80"
    title="清空"
    @cancel="handleCancel"
    @confirm="handleConfirm"
  >
    <div class="px-6 font-bold">确认清空列表?</div>
    <div class="px-6 py-2">tips: 仅删除显示</div>
  </Modal>
</template>
