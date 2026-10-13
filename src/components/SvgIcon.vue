<script lang="ts" setup>
import { IconMap, IconName } from '@/utils/icons';

interface Props {
  name: IconName;
  size?: number | string;
  disabled?: boolean;
}

const { name, size = 18, disabled } = defineProps<Props>();
</script>

<template>
  <component
    :is="IconMap[name]"
    :height="size"
    :width="size"
    :data-disabled="disabled || undefined"
  />
</template>
