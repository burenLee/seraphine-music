<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue'
import { useListStore } from '@/stores/list'

const { toggleChecked } = useListStore()
</script>

<template>
  <ActionButton prefix-icon="Close" size="10" @click="toggleChecked">取消</ActionButton>
</template>
