<script lang="ts" setup>
import { IconMap, IconName } from '@/utils/icons';

interface Props {
  name: IconName;
  size?: number | string;
  disabled?: boolean;
}

const { name, size = 16, disabled } = defineProps<Props>();
</script>

<template>
  <div class="flex items-center justify-center" :data-disabled="disabled">
    <component :is="IconMap[name]" :height="size" :width="size" />
  </div>
</template>
