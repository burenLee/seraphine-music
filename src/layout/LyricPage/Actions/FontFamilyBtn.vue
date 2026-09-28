<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { computed, ref } from 'vue';

import SelectModal from '@/components/SelectModal.vue';
import { useLyricStore } from '@/stores/lyric';
import { useSettingStore } from '@/stores/setting';

const lyricStore = useLyricStore();
const settingStore = useSettingStore();

const fontFamilyVisible = ref(false);
const fontFamilyOptions = ref<Array<SelectOption<FontValue>>>([]);

const fontFamilySelection = computed(
  () =>
    fontFamilyOptions.value.find((item) => item.value === lyricStore.fontFamily) ||
    fontFamilyOptions.value[0],
);

const handleFontFamilyClick = () => {
  fontFamilyVisible.value = !fontFamilyVisible.value;

  if (settingStore.availableFonts.length === 0) settingStore.getAvailableFonts();
  fontFamilyOptions.value = settingStore.availableFonts.map(
    ([label, value]: [string, FontValue]) => ({ label, value }),
  );
};

const handleFontFamilySelect = (font: FontValue) => {
  lyricStore.setFontFamily(font);
  fontFamilyVisible.value = false;
};
</script>

<template>
  <div class="relative" v-on-click-outside="() => (fontFamilyVisible = false)">
    <div
      class="action-icon card flex items-center justify-center text-base"
      title="歌词字体"
      @click="handleFontFamilyClick"
    >
      A
    </div>

    <SelectModal
      class="absolute right-full top-0 mr-2 h-64 overflow-y-auto"
      :visible="fontFamilyVisible"
      :options="fontFamilyOptions"
      :selection="fontFamilySelection"
      @select="handleFontFamilySelect"
    >
    </SelectModal>
  </div>
</template>
