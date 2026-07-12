<script lang="ts" setup>
import SvgIcon from '@/components/SvgIcon.vue'
import { useListStore } from '@/stores/list'

const { toggleChecked } = useListStore()
</script>

<template>
  <SvgIcon
    class="action-icon card-hover transition-colors rounded-lg hover:text-foreground"
    name="CheckList"
    @click="toggleChecked" />
</template>
