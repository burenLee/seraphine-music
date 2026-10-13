<script lang="ts" setup>
import { useColorMode } from '@vueuse/core';
import { computed, ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import SelectModal from '@/components/SelectModal.vue';
import { IconName } from '@/utils/icons';
import { ThemeMode } from '@/utils/params';

const colorMode = useColorMode({ emitAuto: true });

const themeOptions: Array<SelectOption<ThemeMode>> = [
  { label: '浅色', value: ThemeMode.Light, prefixIcon: 'Sun' },
  { label: '深色', value: ThemeMode.Dark, prefixIcon: 'Moon' },
  { label: '跟随系统', value: ThemeMode.Auto, prefixIcon: 'Laptop' },
];

const themeVisible = ref(false);

const themeSelection = computed<SelectOption<ThemeMode>>(
  () => themeOptions.find((item) => item.value === colorMode.value) || themeOptions[2],
);

const themeIcon = computed<IconName>(() => {
  switch (colorMode.value) {
    case ThemeMode.Light:
      return 'Sun';
    case ThemeMode.Dark:
      return 'Moon';
    case ThemeMode.Auto:
      return 'Laptop';
    default:
      return 'Sun';
  }
});

const handleTheme = (mode: ThemeMode) => {
  colorMode.value = mode;

  themeVisible.value = false;
};
</script>

<template>
  <div class="relative" v-on-click-outside="() => (themeVisible = false)">
    <ActionButton
      class="size-8"
      mode="text"
      :prefix-icon="themeIcon"
      title="主题"
      icon-size="18"
      @click="themeVisible = !themeVisible"
    />

    <SelectModal
      class="absolute left-1/2 top-full -translate-x-1/2"
      transition="zoom-top"
      :visible="themeVisible"
      :options="themeOptions"
      :selection="themeSelection"
      @select="handleTheme"
    />
  </div>
</template>
