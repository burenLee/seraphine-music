<script lang="ts" setup>
import { emitTo, listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { vOnClickOutside } from '@vueuse/components';
import { useThrottleFn } from '@vueuse/core';
import { computed, ref, watch } from 'vue';

import SelectModal from '@/components/SelectModal.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { getFullName } from '@/utils/music';
import {
  DesktopLyricEmit,
  FORWARD_DURATION,
  Interval,
  LyricFontSize,
  LyricFormat,
  LyricTransMode,
  PresetsColors,
  WindowEvent,
  WindowTarget,
  desktopLyricSize,
} from '@/utils/params';

import { useDesktopLyricStore } from './stores/desktop-lyric';

const lyricWindow = getCurrentWindow();

const desktopLyricStore = useDesktopLyricStore();

const ContentHeight = desktopLyricSize.height - 32; // 歌词部分的高度, 32 = 操作栏高度(2rem)

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

// 当前歌词的偏移量
const offset = computed(() => (lyric.value ? desktopLyricStore.offsetMap[lyric.value.id] || 0 : 0));
const fontFamilySelection = computed<SelectOption<FontValue> | undefined>(
  () =>
    fontFamilyOptions.value.find((item) => item.value === desktopLyricStore.fontFamily) ||
    fontFamilyOptions.value[0],
);

// 当前高亮歌词行索引
const activedIndex = computed(() => {
  if (!lyric.value) return -1;

  const lines = lyric.value.lines;
  const pg = (progress.value + offset.value) * 1000 + FORWARD_DURATION;
  for (let i = 0; i < lines.length; i++) {
    if (pg < lines[i].offset || pg > (lines[i + 1]?.offset || Infinity)) continue;

    return i;
  }

  return -1;
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

const lyricLine = computed(() => {
  if (!lyric.value) return { current: null, next: null };

  return {
    current: lyric.value.lines[currentIndex.value],
    next: lyric.value.lines[nextIndex.value],
  };
});

const handleFontFamilySelect = (font: FontValue) => {
  desktopLyricStore.setFontFamily(font);
  fontFamilyVisible.value = false;
};

const handleTransClick = (mode: LyricTransMode) => {
  desktopLyricStore.setTransMode(desktopLyricStore.transMode === mode ? LyricTransMode.Off : mode);
};

// 获取单词进度百分比
const getWordProgress = (word: LyricWord) =>
  `${Math.max(0, Math.min(1, ((progress.value + offset.value) * 1000 - word.offset) / word.duration)) * 100}%`;

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
</script>

<template>
  <div
    class="h-screen w-screen select-none overflow-hidden rounded-lg px-2 text-neutral-200 transition-colors"
    :class="isHovering ? 'bg-black/80' : 'bg-transparent'"
    :style="{
      '--height': `${ContentHeight}px`,
      '--line-height': `${ContentHeight / 2}px`,
    }"
    @mouseenter="isHovering = true"
    @mouseleave="isHovering = false"
  >
    <!-- 工具栏 -->
    <div
      data-tauri-drag-region
      class="flex w-full cursor-move items-center justify-center transition-opacity"
      :class="isHovering ? 'pointer-events-auto opacity-100' : 'pointer-events-none opacity-0'"
    >
      <SvgIcon
        class="action-icon"
        name="Music"
        title="打开主界面"
        @click="handleSend(DesktopLyricEmit.Main)"
      />

      <div class="mx-1 h-4 w-px bg-minor" />

      <SvgIcon
        class="action-icon"
        name="Previous"
        title="上一首"
        @click="handleSend(DesktopLyricEmit.Prev)"
      />
      <SvgIcon
        v-if="!audio.isLoading"
        class="action-icon"
        :name="audio.isPlaying ? 'Pause' : 'Play'"
        :title="audio.isPlaying ? '暂停' : '播放'"
        @click="handleSend(audio.isPlaying ? DesktopLyricEmit.Pause : DesktopLyricEmit.Play)"
      />
      <SvgIcon v-else class="action-icon pointer-events-none" name="Ring" title="加载中" />
      <SvgIcon
        class="action-icon"
        name="Next"
        title="下一首"
        @click="handleSend(DesktopLyricEmit.Next)"
      />

      <div class="mx-1 h-4 w-px bg-minor" />

      <SvgIcon
        class="action-icon"
        name="ForwardLeft"
        title="歌词进度 -0.2 秒"
        @click="lyric && desktopLyricStore.setOffsetMap('sub', lyric.id)"
      />
      <SvgIcon
        class="action-icon"
        name="Restart"
        title="重置歌词进度"
        @click="lyric && desktopLyricStore.setOffsetMap('restart', lyric.id)"
      />
      <SvgIcon
        class="action-icon"
        name="ForwardRight"
        title="歌词进度 +0.2 秒"
        @click="lyric && desktopLyricStore.setOffsetMap('add', lyric.id)"
      />

      <div class="mx-1 h-4 w-px bg-minor" />

      <SvgIcon
        class="action-icon"
        name="ZoomOut"
        title="减小歌词字体"
        :disabled="desktopLyricStore.fontSize <= LyricFontSize.Min"
        @click="desktopLyricStore.setFontSize('sub')"
      />
      <SvgIcon
        class="action-icon"
        name="Restart"
        title="重置歌词字体大小"
        @click="desktopLyricStore.setFontSize('restart')"
      />
      <SvgIcon
        class="action-icon"
        name="ZoomIn"
        title="增大歌词字体"
        :disabled="desktopLyricStore.fontSize >= LyricFontSize.Max"
        @click="desktopLyricStore.setFontSize('add')"
      />

      <div class="mx-1 h-4 w-px bg-minor" />

      <div class="relative" v-on-click-outside="() => (colorVisible = false)">
        <div
          class="action-icon flex items-center justify-center"
          @click="colorVisible = !colorVisible"
        >
          颜
        </div>

        <Transition name="zoom-top-right">
          <div
            v-if="colorVisible"
            class="absolute right-0 top-full flex cursor-default gap-2 rounded-lg bg-neutral-600 p-2"
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
          class="action-icon flex items-center justify-center"
          @click="fontFamilyVisible = !fontFamilyVisible"
        >
          字
        </div>

        <SelectModal
          class="absolute right-0 top-full h-[var(--height)] bg-neutral-600"
          transition="zoom-top-right"
          :visible="fontFamilyVisible"
          :options="fontFamilyOptions"
          :selection="fontFamilySelection"
          @select="handleFontFamilySelect"
        />
      </div>

      <div
        class="action-icon flex items-center justify-center"
        :class="desktopLyricStore.transMode === LyricTransMode.Trans ? 'text-info' : ''"
        @click="handleTransClick(LyricTransMode.Trans)"
      >
        译
      </div>
      <div
        class="action-icon flex items-center justify-center"
        :class="desktopLyricStore.transMode === LyricTransMode.Roman ? 'text-info' : ''"
        @click="handleTransClick(LyricTransMode.Roman)"
      >
        音
      </div>

      <div class="mx-1 h-4 w-px bg-minor" />

      <SvgIcon class="action-icon" name="Lock" @click="handleLock" />

      <SvgIcon
        class="action-icon hover:text-error"
        name="Close"
        size="20"
        @click="handleSend(DesktopLyricEmit.Close)"
      />
    </div>

    <!-- 歌词区域 -->
    <div
      class="whitespace-nowrap font-bold"
      :style="{
        fontFamily: desktopLyricStore.fontFamily,
        fontSize: `${desktopLyricStore.fontSize}px`,
        '-webkit-text-stroke': '0.4px #000',
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
              :style="{ '--word-progress': getWordProgress(word) }"
            >
              {{ word.text }}
            </span>
          </div>

          <div class="h-[var(--line-height)] text-right leading-[var(--line-height)]">
            <span
              v-for="(word, wordIndex) in lyricLine.next?.words"
              :key="wordIndex"
              class="music-lyric"
              :style="{ '--word-progress': getWordProgress(word) }"
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
                :style="{ '--word-progress': getWordProgress(word) }"
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
                :style="{ '--word-progress': getWordProgress(word) }"
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
