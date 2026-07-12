<script lang="ts" setup>
import { invoke } from '@/utils/tools'

onMounted(() => {
  // 注册设备
  invoke('api_register_dev')
})
</script>

<template>
  <RouterView />
</template>
