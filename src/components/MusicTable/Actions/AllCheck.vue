<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue'
import { useListContext } from '@/utils/hooks'

const { listStore, list } = useListContext()

const isChecked = computed(
  () => listStore.checkedList.length > 0 && listStore.checkedList.length == list.value.list.length
)

const handleClick = () => {
  listStore.setChecked(isChecked.value ? [] : list.value.list.map((item) => item.id))
}
</script>

<template>
  <ActionButton
    :prefix-icon="isChecked ? 'UnreadBold' : 'Unread'"
    :disabled="list.list.length === 0"
    @click="handleClick">
    全选
  </ActionButton>
</template>
