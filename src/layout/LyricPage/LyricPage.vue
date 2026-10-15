<script lang="ts" setup>
import { ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Image from '@/components/Image.vue';
import Close from '@/components/SystemActions/Close.vue';
import Maximize from '@/components/SystemActions/Maximize.vue';
import Minimize from '@/components/SystemActions/Minimize.vue';
import MiniPlayer from '@/components/SystemActions/MiniPlayer.vue';
import Theme from '@/components/SystemActions/Theme.vue';
import { usePlayingCover } from '@/composables/usePlayingCover.ts';
import { LyricPageMode, PicSize } from '@/constants';
import Playbar from '@/layout/Playbar/Playbar.vue';
import { useContextMenuStore } from '@/stores/context-menu';
import { useLyricStore } from '@/stores/lyric';
import { useMusicStore } from '@/stores/music';
import { useStateStore } from '@/stores/state.ts';

import AlignBtn from './Actions/AlignBtn.vue';
import ColorBtn from './Actions/ColorBtn.vue';
import FontFamilyBtn from './Actions/FontFamilyBtn.vue';
import FontSizeBtn from './Actions/FontSizeBtn.vue';
import OffsetBtn from './Actions/OffsetBtn.vue';
import TransBtn from './Actions/TransBtn.vue';
import BgMode from './BgMode.vue';
import FullScreen from './FullScreen.vue';
import LyricScrollList from './LyricScrollList.vue';
import LyricSearchModal from './LyricSearchModal.vue';
import VinylRecord from './VinylRecord.vue';

const stateStore = useStateStore();
const lyricStore = useLyricStore();
const musicStore = useMusicStore();
const contextMenuStore = useContextMenuStore();

const cover = usePlayingCover(PicSize.Lg);

const lyricSearchVisible = ref(false);

const handleContextMenu = (e: MouseEvent) => {
  contextMenuStore.show({
    top: e.clientY,
    left: e.clientX,
    options: [
      {
        label: '背景模式',
        prefixIcon: 'Picture',
        children: [
          {
            label: '方形封面',
            prefixIcon: 'Picture',
            onClick: () => lyricStore.setPageMode(LyricPageMode.Cover),
          },
          {
            label: '炫胶唱片',
            prefixIcon: 'Album',
            onClick: () => lyricStore.setPageMode(LyricPageMode.Record),
          },
          { label: '歌手写真', prefixIcon: 'User', disabled: true },
        ],
      },
      { divider: true },
      {
        label: '歌词搜索',
        prefixIcon: 'Search',
        disabled: !musicStore.music,
        onClick: () => (lyricSearchVisible.value = true),
      },
      { label: '歌词关联', prefixIcon: 'Link', disabled: true, onClick: () => 'TODO: 歌词关联' },
    ],
  });
};
</script>

<template>
  <Transition name="slide-top-page">
    <div
      v-if="lyricStore.pageVisible"
      class="fixed inset-0 z-10 flex flex-col overflow-hidden bg-background"
    >
      <!-- 封面模糊背景 -->
      <!-- 以 1/4 尺寸光栅化 + scale(5) 放大，等价于全屏 blur-3xl(64px)，但模糊中间层显存降低约 4 倍 -->
      <div v-if="cover" class="absolute inset-0 overflow-hidden">
        <img
          class="absolute left-1/2 top-1/2 h-1/4 w-1/4 -translate-x-1/2 -translate-y-1/2 scale-[5] object-cover blur-md"
          :src="cover"
          alt=""
          draggable="false"
        />
        <div class="absolute inset-0 bg-background opacity-50" />
      </div>

      <div
        :data-tauri-drag-region="!stateStore.isFullscreen"
        class="relative flex h-[var(--header-height)] items-center gap-1 px-5 pt-4"
      >
        <template v-if="!stateStore.isFullscreen">
          <ActionButton
            class="mr-auto size-8"
            mode="text"
            prefix-icon="Down"
            icon-size="24"
            title="收起"
            @click="lyricStore.togglePageVisible"
          />

          <BgMode />
          <FullScreen />

          <div class="mx-1 h-4 w-px bg-minor" />

          <Theme />
          <MiniPlayer />
          <Minimize />
          <Maximize />
          <Close />
        </template>

        <FullScreen v-else class="ml-auto" />
      </div>

      <div class="relative flex h-0 flex-1 p-8" @contextmenu="handleContextMenu">
        <div class="flex w-1/2 items-center justify-center pr-4">
          <Transition name="zoom-fade" mode="out-in">
            <Image
              v-if="lyricStore.pageMode === LyricPageMode.Cover"
              class="rounded-3xl"
              :class="stateStore.isFullscreen || stateStore.isMaximized ? 'size-96' : 'size-72'"
              :src="cover"
              :icon-size="96"
            />
            <VinylRecord v-else-if="lyricStore.pageMode === LyricPageMode.Record" :src="cover" />
          </Transition>
        </div>

        <div class="flex h-full w-1/2 gap-3 pl-4">
          <LyricScrollList class="h-full flex-1" @lyric-search-show="lyricSearchVisible = true" />

          <div v-if="!stateStore.isFullscreen" class="flex w-8 flex-col gap-3 self-end">
            <ColorBtn />
            <FontFamilyBtn />
            <AlignBtn />
            <FontSizeBtn />
            <OffsetBtn />
            <TransBtn />
          </div>
        </div>
      </div>

      <div class="h-[var(--playbar-height)]">
        <Playbar v-if="!stateStore.isFullscreen" />
      </div>
    </div>
  </Transition>

  <LyricSearchModal v-model="lyricSearchVisible" />
</template>
