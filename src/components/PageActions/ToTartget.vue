<script lang="ts" setup>
import SvgIcon from '@/components/SvgIcon.vue';

interface Emits {
  click: [];
}

const emits = defineEmits<Emits>();
</script>

<template>
  <SvgIcon
    class="action-icon card shadow-md shadow-shadow"
    name="Target"
    title="滚到播放音乐"
    @click="emits('click')"
  />
</template>
