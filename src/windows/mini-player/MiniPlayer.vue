<script lang="ts" setup>
import { convertFileSrc } from '@tauri-apps/api/core';
import { LogicalSize } from '@tauri-apps/api/dpi';
import { emitTo, listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useColorMode, useThrottleFn } from '@vueuse/core';
import { computed, ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Image from '@/components/Image.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import VirtualList from '@/components/VirtualList.vue';
import { getFullName, getPic } from '@/utils/music';
import {
  Interval,
  MiniPlayerEmit,
  PicSize,
  PlayingOrigin,
  WindowEvent,
  WindowTarget,
  miniPlayerSize,
} from '@/utils/params';

// 同步主题
useColorMode();

const miniWindow = getCurrentWindow();

const tableColumns: TableColumn[] = [
  { key: 'index', slot: true, width: '3rem', padding: 0 },
  { key: 'info', slot: true, width: 'auto' },
];

const audio = ref<MiniPlayerAudio>({
  isPlaying: false,
  isLoading: false,
  music: undefined,
  origin: PlayingOrigin.Local,
});

const lyric = ref('');
const playlist = ref<MusicInfo[]>([]);
const playlistVisible = ref(false);

// 封面基于主窗口同步过来的 audio 状态计算，避免在迷你播放器中实例化主窗口 store
const cover = computed(() => {
  if (!audio.value.music?.cover) return '';

  return audio.value.origin === PlayingOrigin.Local
    ? convertFileSrc(audio.value.music.cover)
    : getPic(audio.value.music.cover, PicSize.Md);
});
const showContent = computed(
  () => lyric.value || (audio.value.music ? getFullName(audio.value.music) : 'Seraphine'),
);

const isPlaying = (id: ID) => id === audio.value.music?.id;

const handleSend = (type: MiniPlayerEmit, data?: any) => {
  emitTo(WindowTarget.Main, WindowEvent.MiniPlayer, { type, data });
};

const showPlaylist = () => {
  playlistVisible.value = !playlistVisible.value;

  miniWindow.setSize(
    new LogicalSize(
      miniPlayerSize.width,
      playlistVisible.value ? miniPlayerSize.width : miniPlayerSize.height,
    ),
  );
};

// 向主窗口发送位置信息并缓存
miniWindow.onMoved(useThrottleFn((e) => handleSend(MiniPlayerEmit.Pos, e.payload), Interval.Long));
// 向主窗口发送初始化请求
emitTo(WindowTarget.Main, WindowEvent.MiniPlayer, { type: MiniPlayerEmit.Init });
listen<{ type: MiniPlayerEmit; data: unknown }>(WindowEvent.MiniPlayer, (e) => {
  switch (e.payload.type) {
    case MiniPlayerEmit.Audio:
      audio.value = e.payload.data as MiniPlayerAudio;
      break;
    case MiniPlayerEmit.Lyric:
      lyric.value = e.payload.data as string;
      break;
    case MiniPlayerEmit.Playlist:
      playlist.value = e.payload.data as MusicInfo[];
      break;
  }
});
</script>

<template>
  <div
    class="flex h-screen w-screen select-none flex-col rounded-lg border border-border bg-background"
  >
    <!-- 主体内容 -->
    <div class="flex items-center">
      <!-- 专辑封面 -->
      <Image data-tauri-drag-region class="size-16 cursor-move" :src="cover" icon-size="32" />

      <!-- 歌名/歌词区域 -->
      <div class="group relative w-0 flex-1 px-3">
        <div class="line-clamp-2 font-bold">{{ showContent }}</div>

        <div
          class="absolute inset-0 flex justify-between bg-background pl-6 pr-2 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
        >
          <div class="flex items-center gap-3">
            <ActionButton
              class="size-8"
              mode="text"
              prefix-icon="PreviousBold"
              icon-size="20"
              @click="handleSend(MiniPlayerEmit.Prev)"
            />
            <ActionButton
              v-if="!audio.isLoading"
              class="size-8"
              mode="text"
              :prefix-icon="audio.isPlaying ? 'PauseBold' : 'PlayBold'"
              icon-size="24"
              @click="handleSend(audio.isPlaying ? MiniPlayerEmit.Pause : MiniPlayerEmit.Play)"
            />
            <ActionButton
              v-else
              class="pointer-events-none size-8"
              mode="text"
              prefix-icon="Ring"
              icon-size="24"
            />
            <ActionButton
              class="size-8"
              mode="text"
              prefix-icon="NextBold"
              icon-size="20"
              @click="handleSend(MiniPlayerEmit.Next)"
            />
          </div>

          <div class="flex items-center">
            <ActionButton
              class="size-8"
              mode="text"
              prefix-icon="Playlist"
              title="播放列表"
              icon-size="20"
              @click="showPlaylist"
            />
            <ActionButton
              class="size-8 hover:text-error"
              mode="text"
              prefix-icon="Close"
              icon-size="20"
              @click="handleSend(MiniPlayerEmit.Close)"
            />
          </div>
        </div>
      </div>
    </div>

    <VirtualList
      v-if="playlistVisible"
      class="mt-2 h-0 flex-1 px-2"
      :line-height="40"
      :columns="tableColumns"
      :list="playlist"
      @lineDblClick="handleSend(MiniPlayerEmit.Set, $event)"
    >
      <template #index="row">
        <div v-if="isPlaying(row.id)" class="flex items-center justify-center text-info">
          <SvgIcon name="Bars" />
        </div>
        <div v-else class="truncate text-center text-minor">
          {{ row.index + 1 }}
        </div>
      </template>

      <template #info="row">
        <div class="flex">
          <div
            class="w-0 flex-1 truncate font-bold leading-10"
            :class="isPlaying(row.id) ? 'text-info' : ''"
          >
            {{ getFullName(row) }}
          </div>

          <div
            class="hidden items-center pl-2 group-hover/line:flex"
            @dblclick.stop
            @contextmenu.stop
          >
            <ActionButton
              class="size-8"
              mode="text"
              prefix-icon="Play"
              @click="handleSend(MiniPlayerEmit.Set, row)"
            />
          </div>
        </div>
      </template>
    </VirtualList>
  </div>
</template>
