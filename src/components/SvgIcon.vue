<script lang="ts" setup>
import { IconMap, IconName } from '@/utils/icons'

interface Props {
  name: IconName
  size?: number | string
  disabled?: boolean

  // 此处声明是为了tsx语法
  onClick?: (e: MouseEvent) => any | Promise<any>
}

const { name, size = 16, disabled, onClick } = defineProps<Props>()
</script>

<template>
  <div class="flex items-center justify-center" :data-disabled="disabled" @click="onClick">
    <component :is="IconMap[name]" :height="size" :width="size" />
  </div>
</template>
