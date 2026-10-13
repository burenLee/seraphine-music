<script lang="ts" setup>
import { useEventListener } from '@vueuse/core';
import { computed, nextTick, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import { useLyricStore } from '@/stores/lyric';
import { useMusicStore } from '@/stores/music';
import { useSettingStore } from '@/stores/setting';
import { LyricFormat, LyricTransMode } from '@/utils/params';
import { formatDuration } from '@/utils/tools';

interface Emits {
  lyricSearchShow: [];
}

const emits = defineEmits<Emits>();

const lyricStore = useLyricStore();
const musicStore = useMusicStore();
const settingStore = useSettingStore();

const lyricRef = useTemplateRef('lyricRef');

let wheelTimer: ReturnType<typeof setTimeout> | null = null; // 滚轮定时器

const lyricPadding = ref(0); // 歌词容器内边距
const isWheeling = ref(false); // 鼠标滚轮是否正在滚动

// 当前歌词的偏移量
const lyricOffset = computed(() => lyricStore.offsetMap[lyricStore.lyric.id] || 0);
// 歌词偏移后的实际播放进度 (s)
const adjustedProgress = computed(() => musicStore.playProgress + lyricOffset.value);
const fontSize = computed(
  () => lyricStore.fontSize + (settingStore.isFullscreen || settingStore.isMaximized ? 4 : 0),
);
const wordHeight = computed(() => fontSize.value + 4);
const linePadding = computed(() => fontSize.value / 2);
// 行高（根据翻译显示模式动态计算）
const lineHeight = computed(() => {
  const baseHeight = wordHeight.value + linePadding.value * 2;
  return lyricStore.transMode === LyricTransMode.Off
    ? baseHeight
    : baseHeight + lyricStore.fontSize;
});

// 当前高亮歌词行索引
const currentIndex = computed(() => {
  if (!lyricStore.lyric.id) return -1;

  const pg = adjustedProgress.value * 1000 + 100;
  return lyricStore.lyric.lines.findIndex((line, index, list) => {
    const start = line.offset;
    const end = list[index + 1]?.offset || Infinity;

    return pg >= start && pg < end;
  });
});

// 设置歌词容器内边距（使首行歌词居中）
const setLyricPadding = async () => {
  lyricPadding.value = 0;

  await nextTick(() => {
    if (!lyricRef.value) return;

    lyricPadding.value = (lyricRef.value.clientHeight - lineHeight.value) / 2;
  });

  scrollToLine();
};

// 获取单词进度百分比
const getWordProgress = (offset: number, duration: number) =>
  Math.max(0, Math.min(1, (adjustedProgress.value * 1000 - offset) / duration)) * 100;

// 滚动到当前行
const scrollToLine = () => {
  if (musicStore.isDragging || isWheeling.value) return;

  const line = lyricRef.value?.querySelector<HTMLElement>(`#lyric-line-${currentIndex.value}`);
  if (!line || !lyricRef.value) return;

  lyricRef.value.scrollTo({
    top: line.offsetTop - lyricPadding.value,
    behavior: 'smooth',
  });
};

// 点击歌词行跳转
const handleLineClick = async (offset: number) => {
  isWheeling.value = false;
  clearWheelTimer();

  musicStore.startChangeProgress();
  await musicStore.stopChangeProgress(offset / 1000);

  await nextTick();

  scrollToLine();
};

const setWheelTimer = () => {
  isWheeling.value = true;

  if (wheelTimer !== null) clearTimeout(wheelTimer);
  wheelTimer = setTimeout(() => {
    isWheeling.value = false;
    wheelTimer = null;

    scrollToLine();
  }, 2000);
};

const clearWheelTimer = () => {
  if (wheelTimer === null) return;

  clearTimeout(wheelTimer);
  wheelTimer = null;
};

watch(
  () => musicStore.isDragging,
  (isDragging) => !isDragging && scrollToLine(),
);

watch(currentIndex, scrollToLine, { flush: 'post' });

// 字体大小 / 翻译模式改变会导致行高变化，需重算内边距保持首行居中
watch(() => lineHeight.value, setLyricPadding);

useEventListener('resize', setLyricPadding);

onMounted(setLyricPadding);
onUnmounted(clearWheelTimer);
</script>

<template>
  <div
    ref="lyricRef"
    class="lyric-fade hide-scrollbar relative overflow-y-scroll"
    :style="{
      paddingBlock: `${lyricPadding}px`,
      fontFamily: lyricStore.fontFamily,
      '--line-padding': `${linePadding}px`,
      '--word-font-size': `${fontSize}px`,
      '--fade-size': `${fontSize * 4}px`,
      '--trans-font-size': `${fontSize - 4}px`,
      '--word-height': `${wordHeight}px`,
      '--trans-height': `${lyricStore.fontSize}px`,
      '--color-lyric': lyricStore.textColor,
    }"
    @wheel.passive="setWheelTimer"
  >
    <!-- 加载中状态 -->
    <template v-if="lyricStore.isLoading">
      <div class="flex h-full items-center justify-center text-xl font-bold">加载中...</div>
    </template>

    <!-- 无歌词状态 -->
    <template v-else-if="!lyricStore.lyric.lines.length">
      <div class="flex h-full items-center justify-center">
        <span class="text-xl font-bold text-minor">暂无歌词</span>
        <ActionButton
          v-if="musicStore.music"
          class="px-1 text-xl hover:text-info"
          mode="text"
          theme="info"
          suffix-icon="Right"
          @click="emits('lyricSearchShow')"
        >
          去搜索
        </ActionButton>
      </div>
    </template>

    <!-- 歌词列表 -->
    <template v-else>
      <div
        v-for="(line, lineIndex) in lyricStore.lyric.lines"
        :key="lineIndex"
        :id="`lyric-line-${lineIndex}`"
        class="group/line card-hover relative cursor-pointer rounded-lg px-3 py-[var(--line-padding)] text-[length:var(--word-font-size)] font-bold leading-[var(--word-height)]"
        :class="lyricStore.textAlign"
        @click="handleLineClick(line.offset)"
      >
        <template v-if="lyricStore.lyric.fmt === LyricFormat.Krc">
          <template v-if="currentIndex === lineIndex">
            <span
              v-for="(word, wordIndex) in line.words"
              :key="wordIndex"
              class="lyric"
              :style="{ '--word-progress': `${getWordProgress(word.offset, word.duration)}%` }"
            >
              {{ word.text }}
            </span>
          </template>

          <template v-else>
            <span v-for="(word, wordIndex) in line.words" :key="wordIndex" class="text-minor">
              {{ word.text }}
            </span>
          </template>
        </template>

        <template v-if="lyricStore.lyric.fmt === LyricFormat.Lrc">
          <span
            v-for="(word, wordIndex) in line.words"
            :key="wordIndex"
            :class="currentIndex === lineIndex ? 'text-foreground' : 'text-minor'"
          >
            {{ word.text }}
          </span>
        </template>

        <!-- 翻译文本 -->
        <div
          v-if="
            lyricStore.transMode !== LyricTransMode.Off && line.translations[lyricStore.transMode]
          "
          class="text-[length:var(--trans-font-size)] leading-[var(--trans-height)]"
          :class="currentIndex === lineIndex ? 'text-accent' : 'text-minor'"
        >
          {{ line.translations[lyricStore.transMode] }}
        </div>

        <!-- 跳转时间提示 -->
        <div
          class="pointer-events-none absolute bottom-0 right-0 top-0 flex w-[4.5rem] items-center justify-center rounded-r-lg bg-neutral-200 text-base opacity-0 group-hover/line:opacity-100 dark:bg-neutral-600"
        >
          {{ formatDuration(line.offset / 1000) }}
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.lyric-fade {
  mask-image: linear-gradient(
    to bottom,
    transparent 0,
    #000 var(--fade-size),
    #000 calc(100% - var(--fade-size)),
    transparent 100%
  );
  -webkit-mask-image: linear-gradient(
    to bottom,
    transparent 0,
    #000 var(--fade-size),
    #000 calc(100% - var(--fade-size)),
    transparent 100%
  );
}

.lyric {
  background-image: linear-gradient(
    90deg,
    var(--color-lyric) 0%,
    var(--color-lyric) var(--word-progress),
    var(--color-accent) var(--word-progress),
    var(--color-accent) 100%
  );
  background-clip: text;
  -webkit-text-fill-color: transparent;
}
</style>
