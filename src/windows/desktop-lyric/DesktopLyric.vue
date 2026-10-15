<script lang="ts" setup>
import { emitTo, listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { vOnClickOutside } from '@vueuse/components';
import { useThrottleFn } from '@vueuse/core';
import { computed, ref, watch } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import {
  DesktopLyricEmit,
  ForwardDuration,
  Interval,
  LyricFontSize,
  LyricFormat,
  LyricTransMode,
  PresetsColors,
  WindowEvent,
  WindowTarget,
  DesktopLyricSize,
} from '@/constants';
import { getFullName } from '@/utils/music';

import { useDesktopLyricStore } from './stores/desktop-lyric';

const lyricWindow = getCurrentWindow();

const desktopLyricStore = useDesktopLyricStore();

const ContentHeight = DesktopLyricSize.height - 32; // 歌词部分的高度, 32 = 操作栏高度(2rem)

const isHovering = ref(false);
const audio = ref<DesktopLyricAudio>({ isLoading: false, isPlaying: false, music: undefined });
const progress = ref(0);
const lyric = ref<LyricInfo>();
const currentIndex = ref(-1);
const nextIndex = ref(0);
const colorVisible = ref(false);
const fontFamilyVisible = ref(false);
const fontFamilyOptions = ref<Array<SelectOption<FontValue>>>([]);
const isLocked = ref(false);

const fontFamilySelection = computed<SelectOption<FontValue> | undefined>(
  () =>
    fontFamilyOptions.value.find((item) => item.value === desktopLyricStore.fontFamily) ||
    fontFamilyOptions.value[0],
);
// 歌词偏移后的实际播放进度 (s)
const playProgress = computed(
  () => (progress.value + (desktopLyricStore.offsetMap[lyric.value?.id || 0] || 0)) * 1000,
);
// 当前高亮歌词行索引
const activedIndex = computed(() => {
  if (!lyric.value) return -1;

  const lines = lyric.value.lines;
  const pg = playProgress.value + ForwardDuration;
  for (let i = 0; i < lines.length; i++) {
    if (pg < lines[i].offset || pg > (lines[i + 1]?.offset || Infinity)) continue;

    return i;
  }

  return -1;
});

const lyricLine = computed(() => {
  if (!lyric.value) return { current: null, next: null };

  return {
    current: lyric.value.lines[currentIndex.value],
    next: lyric.value.lines[nextIndex.value],
  };
});

const handleMouseenter = () => {
  isHovering.value = true;
};

const handleMouseleave = () => {
  if (colorVisible.value || fontFamilyVisible.value) return;

  isHovering.value = false;
};

const handleFontFamilySelect = (font: FontValue) => {
  desktopLyricStore.setFontFamily(font);
  fontFamilyVisible.value = false;
};

const handleTransClick = (mode: LyricTransMode) => {
  desktopLyricStore.setTransMode(desktopLyricStore.transMode === mode ? LyricTransMode.Off : mode);
};

// 获取单词进度百分比
const getWordProgress = (offset: number, duration: number) =>
  Math.max(0, Math.min(1, (playProgress.value - offset) / duration)) * 100;

const handleSend = (type: DesktopLyricEmit, data?: any) => {
  emitTo(WindowTarget.Main, WindowEvent.DesktopLyric, { type, data });
};

const handleLock = async () => {
  isLocked.value = !isLocked.value;
  await lyricWindow.setIgnoreCursorEvents(isLocked.value);

  if (isLocked.value) isHovering.value = false;
};

// 拖动时鼠标会取消悬停状态, 强制赋值
lyricWindow.onMoved(
  useThrottleFn((e) => {
    isHovering.value = true;
    handleSend(DesktopLyricEmit.Pos, e.payload);
  }, Interval.Long),
);
// 向主窗口发送初始化请求
emitTo(WindowTarget.Main, WindowEvent.DesktopLyric, {
  type: DesktopLyricEmit.Init,
});
listen<{ type: DesktopLyricEmit; data: unknown }>(WindowEvent.DesktopLyric, (e) => {
  switch (e.payload.type) {
    case DesktopLyricEmit.Audio:
      audio.value = e.payload.data as DesktopLyricAudio;
      break;
    case DesktopLyricEmit.Progress:
      progress.value = e.payload.data as number;
      break;
    case DesktopLyricEmit.Lyric:
      lyric.value = e.payload.data as LyricInfo;
      break;
    case DesktopLyricEmit.Fonts:
      const fonts = e.payload.data as FontItem[];
      fontFamilyOptions.value = fonts.map(([label, value]) => ({
        label,
        value,
      }));
      break;
  }
});

watch(activedIndex, (index) => {
  if (index === currentIndex.value) {
    // 情况1：当前播放的 = 上行展示索引
    nextIndex.value = index + 1;
  } else if (index === nextIndex.value) {
    // 情况2：当前播放的 = 下行展示索引
    currentIndex.value = index + 1;
  } else {
    // 情况3：拖动进度条，跳跃很远，直接重置
    currentIndex.value = index;
    nextIndex.value = index + 1;
  }
});
</script>

<template>
  <div
    class="h-screen w-screen select-none overflow-hidden rounded-lg px-2 transition-colors"
    :class="isHovering ? 'bg-black/70' : 'bg-transparent'"
    :style="{
      '--height': `${ContentHeight}px`,
      '--line-height': `${ContentHeight / 2}px`,
    }"
    @mouseenter="handleMouseenter"
    @mouseleave="handleMouseleave"
  >
    <!-- 工具栏 -->
    <div
      data-tauri-drag-region
      class="flex w-full cursor-move items-center justify-center transition-opacity"
      :class="isHovering ? 'pointer-events-auto opacity-100' : 'pointer-events-none opacity-0'"
    >
      <ActionButton
        class="size-8 text-neutral-100 hover:text-neutral-100/50"
        mode="text"
        prefix-icon="Music"
        title="打开主界面"
        @click="handleSend(DesktopLyricEmit.Main)"
      />

      <div class="mx-1 h-4 w-px bg-neutral-100/30" />

      <ActionButton
        class="size-8 text-neutral-100 hover:text-neutral-100/50"
        mode="text"
        prefix-icon="Previous"
        title="上一首"
        @click="handleSend(DesktopLyricEmit.Prev)"
      />
      <ActionButton
        v-if="!audio.isLoading"
        class="size-8 text-neutral-100 hover:text-neutral-100/50"
        mode="text"
        :prefix-icon="audio.isPlaying ? 'Pause' : 'Play'"
        :title="audio.isPlaying ? '暂停' : '播放'"
        @click="handleSend(audio.isPlaying ? DesktopLyricEmit.Pause : DesktopLyricEmit.Play)"
      />
      <ActionButton
        v-else
        class="pointer-events-none size-8 text-neutral-100 hover:text-neutral-100/50"
        prefix-icon="Ring"
        title="加载中"
      />
      <ActionButton
        class="size-8 text-neutral-100 hover:text-neutral-100/50"
        mode="text"
        prefix-icon="Next"
        title="下一首"
        @click="handleSend(DesktopLyricEmit.Next)"
      />

      <div class="mx-1 h-4 w-px bg-neutral-100/30" />

      <ActionButton
        class="size-8 text-neutral-100 hover:text-neutral-100/50"
        mode="text"
        prefix-icon="ForwardLeft"
        title="歌词进度 -0.1 秒"
        @click="lyric && desktopLyricStore.setOffsetMap('sub', lyric.id)"
      />
      <ActionButton
        class="size-8 text-neutral-100 hover:text-neutral-100/50"
        mode="text"
        prefix-icon="Restart"
        title="重置歌词进度"
        @click="lyric && desktopLyricStore.setOffsetMap('restart', lyric.id)"
      />
      <ActionButton
        class="size-8 text-neutral-100 hover:text-neutral-100/50"
        mode="text"
        prefix-icon="ForwardRight"
        title="歌词进度 +0.1 秒"
        @click="lyric && desktopLyricStore.setOffsetMap('add', lyric.id)"
      />

      <div class="mx-1 h-4 w-px bg-neutral-100/30" />

      <ActionButton
        class="size-8 text-neutral-100 hover:text-neutral-100/50"
        mode="text"
        prefix-icon="ZoomOut"
        title="减小歌词字体"
        :disabled="desktopLyricStore.fontSize <= LyricFontSize.Min"
        @click="desktopLyricStore.setFontSize('sub')"
      />
      <ActionButton
        class="size-8 text-neutral-100 hover:text-neutral-100/50"
        mode="text"
        prefix-icon="Restart"
        title="重置歌词字体大小"
        @click="desktopLyricStore.setFontSize('restart')"
      />
      <ActionButton
        class="size-8 text-neutral-100 hover:text-neutral-100/50"
        mode="text"
        prefix-icon="ZoomIn"
        title="增大歌词字体"
        :disabled="desktopLyricStore.fontSize >= LyricFontSize.Max"
        @click="desktopLyricStore.setFontSize('add')"
      />

      <div class="mx-1 h-4 w-px bg-neutral-100/30" />

      <div class="relative" v-on-click-outside="() => (colorVisible = false)">
        <div
          class="action-icon size-8 text-center leading-8 text-neutral-100 hover:text-neutral-100/50"
          @click="colorVisible = !colorVisible"
        >
          颜
        </div>

        <Transition name="zoom-top-right">
          <div
            v-if="colorVisible"
            class="card absolute left-1/2 top-full z-10 flex -translate-x-1/2 cursor-default gap-3 rounded-lg border-neutral-100/10 bg-foreground p-2"
          >
            <div
              v-for="(color, index) in PresetsColors"
              class="size-4 cursor-pointer rounded-full transition-transform hover:scale-110"
              :key="index"
              :style="{ background: color[0] }"
              @click="desktopLyricStore.setTextColors(color)"
            ></div>
          </div>
        </Transition>
      </div>

      <div class="relative" v-on-click-outside="() => (fontFamilyVisible = false)">
        <div
          class="action-icon size-8 text-center leading-8 text-neutral-100 hover:text-neutral-100/50"
          @click="fontFamilyVisible = !fontFamilyVisible"
        >
          字
        </div>

        <Transition name="zoom-top">
          <ul
            v-if="fontFamilyVisible"
            class="card absolute left-1/2 top-full z-10 h-[var(--height)] -translate-x-1/2 cursor-default overflow-y-auto border-neutral-100/10 bg-foreground p-1.5 text-background"
          >
            <li
              v-for="(option, index) in fontFamilyOptions"
              :key="index"
              class="flex h-8 cursor-pointer items-center gap-1 rounded-lg px-2 transition-colors"
              :class="
                fontFamilySelection?.value === option.value
                  ? 'bg-neutral-400 text-foreground'
                  : 'hover:bg-neutral-400/10'
              "
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

      <div
        class="action-icon size-8 text-center leading-8 hover:text-neutral-100/50"
        :class="
          desktopLyricStore.transMode === LyricTransMode.Trans ? 'text-info' : 'text-neutral-100'
        "
        @click="handleTransClick(LyricTransMode.Trans)"
      >
        译
      </div>
      <div
        class="action-icon size-8 text-center leading-8 hover:text-neutral-100/50"
        :class="
          desktopLyricStore.transMode === LyricTransMode.Roman ? 'text-info' : 'text-neutral-100'
        "
        @click="handleTransClick(LyricTransMode.Roman)"
      >
        音
      </div>

      <div class="mx-1 h-4 w-px bg-neutral-100/30" />

      <ActionButton
        class="size-8 text-neutral-100 hover:text-neutral-100/50"
        mode="text"
        prefix-icon="Lock"
        icon-size="16"
        @click="handleLock"
      />

      <ActionButton
        class="size-8 text-neutral-100 hover:text-error"
        mode="text"
        prefix-icon="Close"
        icon-size="20"
        @click="handleSend(DesktopLyricEmit.Close)"
      />
    </div>

    <!-- 歌词区域 -->
    <div
      class="whitespace-nowrap font-bold"
      :style="{
        fontFamily: desktopLyricStore.fontFamily,
        fontSize: `${desktopLyricStore.fontSize}px`,
        '-webkit-text-stroke': '0.8px rgba(0, 0, 0, 0.75)',
        'paint-order': 'stroke fill',
        filter:
          'drop-shadow(0 1px 1px rgba(0, 0, 0, 0.75)) drop-shadow(0 0 4px rgba(0, 0, 0, 0.5))',
        '--color-lyric-base': desktopLyricStore.textBaseColor,
        '--color-lyric-accent': desktopLyricStore.textAccentColor,
      }"
    >
      <template v-if="!lyric?.lines.length">
        <div
          class="h-[var(--line-height)] text-center leading-[var(--line-height)] text-[var(--color-lyric-base)]"
        >
          {{ audio.music ? getFullName(audio.music) : 'Seraphine' }}
        </div>
      </template>

      <template v-else-if="desktopLyricStore.transMode === LyricTransMode.Off">
        <template v-if="lyric?.fmt === LyricFormat.Krc">
          <div class="h-[var(--line-height)] text-left leading-[var(--line-height)]">
            <span
              v-for="(word, wordIndex) in lyricLine.current?.words"
              :key="wordIndex"
              class="music-lyric"
              :style="{ '--word-progress': `${getWordProgress(word.offset, word.duration)}%` }"
            >
              {{ word.text }}
            </span>
          </div>

          <div class="h-[var(--line-height)] text-right leading-[var(--line-height)]">
            <span
              v-for="(word, wordIndex) in lyricLine.next?.words"
              :key="wordIndex"
              class="music-lyric"
              :style="{ '--word-progress': `${getWordProgress(word.offset, word.duration)}%` }"
            >
              {{ word.text }}
            </span>
          </div>
        </template>

        <template v-else-if="lyric?.fmt === LyricFormat.Lrc">
          <div
            class="h-[var(--line-height)] text-left leading-[var(--line-height)] text-[var(--color-lyric-base)]"
          >
            <span v-for="(word, wordIndex) in lyricLine.current?.words" :key="wordIndex">
              {{ word.text }}
            </span>
          </div>

          <div class="h-[var(--line-height)] text-right leading-[var(--line-height)]">
            <span v-for="(word, wordIndex) in lyricLine.next?.words" :key="wordIndex">
              {{ word.text }}
            </span>
          </div>
        </template>
      </template>

      <template v-else>
        <template v-if="lyric?.fmt === LyricFormat.Krc">
          <template v-if="activedIndex === currentIndex">
            <div class="h-[var(--line-height)] text-center leading-[var(--line-height)]">
              <span
                v-for="(word, wordIndex) in lyricLine.current?.words"
                :key="wordIndex"
                class="music-lyric"
                :style="{ '--word-progress': `${getWordProgress(word.offset, word.duration)}%` }"
              >
                {{ word.text }}
              </span>
            </div>

            <div
              class="h-[var(--line-height)] text-center leading-[var(--line-height)] text-[var(--color-lyric-base)]"
            >
              {{ lyricLine.current?.translations[desktopLyricStore.transMode] }}
            </div>
          </template>

          <template v-else-if="activedIndex === nextIndex">
            <div class="h-[var(--line-height)] text-center leading-[var(--line-height)]">
              <span
                v-for="(word, wordIndex) in lyricLine.next?.words"
                :key="wordIndex"
                class="music-lyric"
                :style="{ '--word-progress': `${getWordProgress(word.offset, word.duration)}%` }"
              >
                {{ word.text }}
              </span>
            </div>

            <div
              class="h-[var(--line-height)] text-center leading-[var(--line-height)] text-[var(--color-lyric-base)]"
            >
              {{ lyricLine.next?.translations[desktopLyricStore.transMode] }}
            </div>
          </template>
        </template>

        <template v-else-if="lyric?.fmt === LyricFormat.Lrc">
          <template v-if="activedIndex === currentIndex">
            <div
              class="h-[var(--line-height)] text-left leading-[var(--line-height)] text-[var(--color-lyric-base)]"
            >
              <span v-for="(word, wordIndex) in lyricLine.current?.words" :key="wordIndex">
                {{ word.text }}
              </span>
            </div>

            <div
              class="h-[var(--line-height)] text-center leading-[var(--line-height)] text-[var(--color-lyric-base)]"
            >
              {{ lyricLine.current?.translations[desktopLyricStore.transMode] }}
            </div>
          </template>

          <template v-else-if="activedIndex === nextIndex">
            <div class="h-[var(--line-height)] text-right leading-[var(--line-height)]">
              <span v-for="(word, wordIndex) in lyricLine.next?.words" :key="wordIndex">
                {{ word.text }}
              </span>
            </div>

            <div
              class="h-[var(--line-height)] text-center leading-[var(--line-height)] text-[var(--color-lyric-base)]"
            >
              {{ lyricLine.next?.translations[desktopLyricStore.transMode] }}
            </div>
          </template>
        </template>
      </template>
    </div>
  </div>
</template>

<style scoped>
.music-lyric {
  background-image: linear-gradient(
    90deg,
    var(--color-lyric-accent) 0%,
    var(--color-lyric-accent) var(--word-progress),
    var(--color-lyric-base) var(--word-progress),
    var(--color-lyric-base) 100%
  );
  background-clip: text;
  -webkit-text-fill-color: transparent;
}
</style>
