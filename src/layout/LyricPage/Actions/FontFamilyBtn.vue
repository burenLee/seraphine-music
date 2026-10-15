<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { computed, ref } from 'vue';

import SelectModal from '@/components/SelectModal.vue';
import { useLyricStore } from '@/stores/lyric';
import { useStateStore } from '@/stores/state';

const stateStore = useStateStore();
const lyricStore = useLyricStore();

const fontFamilyVisible = ref(false);
const fontFamilyOptions = ref<Array<SelectOption<FontValue>>>([]);

const fontFamilySelection = computed(
  () =>
    fontFamilyOptions.value.find((item) => item.value === lyricStore.fontFamily) ||
    fontFamilyOptions.value[0],
);

const handleFontFamilyClick = () => {
  fontFamilyVisible.value = !fontFamilyVisible.value;
  fontFamilyOptions.value = stateStore.availableFonts.map(
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
      class="action-icon card h-8 w-full text-center font-serif text-base leading-8"
      title="歌词字体"
      @click="handleFontFamilyClick"
    >
      A
    </div>

    <Transition name="zoom-fade">
      <ul
        v-if="fontFamilyVisible"
        class="card absolute right-full top-0 z-10 mr-2 h-64 overflow-y-auto bg-background p-1.5 shadow-lg shadow-shadow"
      >
        <li
          v-for="(option, index) in fontFamilyOptions"
          :key="index"
          class="flex h-8 cursor-pointer items-center gap-1 rounded-lg px-2 transition-colors"
          :class="fontFamilySelection?.value === option.value ? 'card-actived' : 'card-hover'"
          :style="{ fontFamily: option.value }"
          :data-disabled="option.disabled"
          @click="handleFontFamilySelect(option.value)"
        >
          <div class="whitespace-nowrap px-1">{{ option.label }}</div>
        </li>

        <div v-if="!fontFamilyOptions.length" class="whitespace-nowrap p-4">暂无数据</div>
      </ul>
    </Transition>
  </div>
</template>
