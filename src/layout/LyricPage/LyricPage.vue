<script lang="ts" setup>
import { ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Image from '@/components/Image.vue';
import SystemActions from '@/components/SystemActions.vue';
import { useContextMenuStore } from '@/stores/context-menu';
import { useLyricStore } from '@/stores/lyric';
import { useMusicStore } from '@/stores/music';
import { useSettingStore } from '@/stores/setting';
import { useCover } from '@/utils/hooks';
import { LyricPageMode, PicSize } from '@/utils/params';

import Playbar from '../Playbar/Playbar.vue';

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

const settingStore = useSettingStore();
const lyricStore = useLyricStore();
const musicStore = useMusicStore();
const contextMenuStore = useContextMenuStore();

const cover = useCover(PicSize.Lg);

const lyricSearchVisible = ref(false);

const handleContextMenu = (e: MouseEvent) => {
  contextMenuStore.show({
    x: e.clientX,
    y: e.clientY,
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
      <template v-if="cover">
        <img
          class="absolute left-0 top-0 w-full object-cover blur-3xl"
          :src="cover"
          alt=""
          draggable="false"
        />
        <div class="absolute inset-0 bg-background opacity-50" />
      </template>

      <div
        :data-tauri-drag-region="!settingStore.isFullscreen"
        class="relative flex h-[var(--header-height)] items-center gap-1 px-5 pt-4"
      >
        <template v-if="!settingStore.isFullscreen">
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
          <SystemActions />
        </template>

        <FullScreen v-else class="ml-auto" />
      </div>

      <div class="relative flex h-0 flex-1 p-8" @contextmenu.prevent="handleContextMenu">
        <div class="flex w-1/2 items-center justify-center pr-4">
          <Transition name="zoom-fade" mode="out-in">
            <Image
              v-if="lyricStore.pageMode === LyricPageMode.Cover"
              class="rounded-3xl"
              :class="settingStore.isFullscreen || settingStore.isMaximized ? 'size-96' : 'size-80'"
              :src="cover"
              :icon-size="96"
            />
            <VinylRecord v-else-if="lyricStore.pageMode === LyricPageMode.Record" :src="cover" />
          </Transition>
        </div>

        <div class="flex h-full w-1/2 gap-3 pl-4">
          <LyricScrollList class="h-full flex-1" @lyric-search-show="lyricSearchVisible = true" />

          <div v-if="!settingStore.isFullscreen" class="flex w-8 flex-col gap-3 self-end">
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
        <Playbar v-if="!settingStore.isFullscreen" />
      </div>
    </div>
  </Transition>

  <LyricSearchModal v-model="lyricSearchVisible" />
</template>
