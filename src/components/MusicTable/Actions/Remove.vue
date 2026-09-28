<script lang="ts" setup>
import { inject, ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Modal from '@/components/Modal.vue';
import { useListStore } from '@/stores/list';
import { ListType } from '@/utils/params';

const listType = inject<ListType>('listType', ListType.Show);

const listStore = useListStore();

const visible = ref(false);

const handleCancel = () => {
  visible.value = false;
};

const handleConfirm = () => {
  listStore.removeListChecked(listType);
  visible.value = false;
};
</script>

<template>
  <ActionButton
    theme="error"
    prefix-icon="Bin"
    :disabled="listStore.checkedList.length === 0"
    @click="visible = true"
  >
    删除
  </ActionButton>

  <Modal
    v-model="visible"
    class="w-80"
    title="删除"
    @cancel="handleCancel"
    @confirm="handleConfirm"
  >
    <div class="px-4 font-bold">确认删除选中歌曲?</div>
    <div class="px-4 py-2">tips: 仅删除显示</div>
  </Modal>
</template>
