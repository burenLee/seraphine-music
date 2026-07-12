<script lang="ts" setup>
import SvgIcon from '../SvgIcon.vue'

interface Emits {
  click: []
}

const emits = defineEmits<Emits>()
</script>

<template>
  <SvgIcon
    class="action-icon card shadow-md shadow-shadow"
    name="Up"
    title="滚到顶部"
    @click="emits('click')" />
</template>
