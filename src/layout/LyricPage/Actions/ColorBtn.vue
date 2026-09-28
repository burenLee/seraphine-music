<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { watchThrottled } from '@vueuse/core';
import { ref } from 'vue';

import { useLyricStore } from '@/stores/lyric';
import { Interval, PresetsColors } from '@/utils/params';

const lyricStore = useLyricStore();

const pickerVisible = ref(false);
const usedColor = ref(lyricStore.textColor); // 使用的颜色
const lastColor = ref(lyricStore.textColor); // 缓存最后可用的颜色

watchThrottled(
  usedColor,
  (color) => {
    // 校验颜色格式
    if (/^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$/.test(color)) {
      lyricStore.setTextColor(color);
      lastColor.value = color;
    }
  },
  { throttle: Interval.Long },
);
</script>

<template>
  <div class="relative" v-on-click-outside="() => (pickerVisible = false)">
    <div class="action-icon card p-1" title="歌词颜色" @click="pickerVisible = !pickerVisible">
      <div class="size-full rounded-md" :style="{ background: lyricStore.textColor }"></div>
    </div>

    <Transition name="zoom-fade">
      <div
        v-if="pickerVisible"
        class="absolute right-full top-0 mr-2 rounded-lg border border-border bg-background p-4 shadow-md"
      >
        <div class="font-bold">歌词颜色</div>

        <div class="flex items-center gap-2 pt-3">
          <div
            v-for="(color, index) in PresetsColors"
            class="size-4 cursor-pointer rounded-full transition-transform hover:scale-110"
            :key="index"
            :style="{ background: color[0] }"
            @click="usedColor = color[0]"
          ></div>
        </div>

        <div class="flex items-center gap-2 pt-3">
          <input class="size-6 cursor-pointer rounded-md" type="color" v-model="usedColor" />
          <input
            class="rounded-md border border-border bg-card px-2 leading-6"
            type="text"
            v-model="usedColor"
            @blur="usedColor = lastColor"
          />
        </div>
      </div>
    </Transition>
  </div>
</template>
