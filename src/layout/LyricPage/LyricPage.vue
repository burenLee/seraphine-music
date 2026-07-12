<script lang="ts" setup>
import AlignBtn from './Actions/AlignBtn.vue'
import ColorBtn from './Actions/ColorBtn.vue'
import FontFamilyBtn from './Actions/FontFamilyBtn.vue'
import FontSizeBtn from './Actions/FontSizeBtn.vue'
import OffsetBtn from './Actions/OffsetBtn.vue'
import TransBtn from './Actions/TransBtn.vue'
import BgMode from './BgMode.vue'
import FullScreen from './FullScreen.vue'
import LyricScrollList from './LyricScrollList.vue'
import LyricSearchModal from './LyricSearchModal.vue'
import SquareCover from './SquareCover.vue'
import VinylRecord from './VinylRecord.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import SystemActions from '@/components/SystemActions.vue'
import { useContextMenuStore } from '@/stores/contextMenu.ts'
import { useLyricStore } from '@/stores/lyric'
import { useMusicStore } from '@/stores/music.ts'
import { useSettingStore } from '@/stores/setting.ts'
import { LyricPageMode } from '@/utils/params'

const settingStore = useSettingStore()
const lyricStore = useLyricStore()
const musicStore = useMusicStore()
const contextMenuStore = useContextMenuStore()

const lyricSearchVisible = ref(false)

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
            onClick: () => lyricStore.setPageMode(LyricPageMode.Cover)
          },
          {
            label: '炫胶唱片',
            prefixIcon: 'Album',
            onClick: () => lyricStore.setPageMode(LyricPageMode.Record)
          },
          { label: '歌手写真', prefixIcon: 'User', disabled: true }
        ]
      },
      { divider: true },
      {
        label: '歌词搜索',
        prefixIcon: 'Search',
        disabled: !musicStore.music,
        onClick: () => (lyricSearchVisible.value = true)
      },
      { label: '歌词关联', prefixIcon: 'Link', disabled: true, onClick: () => 'TODO: 歌词关联' }
    ]
  })
}
</script>

<template>
  <Transition name="slide-page-top">
    <div
      v-if="lyricStore.pageVisible"
      class="fixed bottom-[var(--playbar-height)] z-10 left-0 right-0 top-0 flex flex-col bg-background">
      <!-- <Transition name="fade">
        <SingerPhoto v-if="lyricPageMode === LyricPageMode.Photo" />
      </Transition> -->

      <div
        :data-tauri-drag-region="!settingStore.isFullscreen"
        class="flex h-14 w-full justify-between px-6 pt-6">
        <!-- 左侧 -->
        <div class="flex items-center gap-1">
          <SvgIcon
            v-if="!settingStore.isFullscreen"
            class="action-icon"
            name="Down"
            size="28"
            title="收起"
            @click="lyricStore.togglePageVisible" />
        </div>

        <!-- 右侧 -->
        <div class="flex items-center gap-1">
          <template v-if="!settingStore.isFullscreen">
            <BgMode />
            <FullScreen />
            <div class="mx-1 h-4 w-px bg-minor" />
            <SystemActions />
          </template>

          <FullScreen v-else />
        </div>
      </div>

      <div class="flex h-0 flex-1 p-8" @contextmenu.prevent="handleContextMenu">
        <div class="flex items-center justify-center w-1/2 pr-4">
          <Transition name="zoom-fade" mode="out-in">
            <SquareCover v-if="lyricStore.pageMode === LyricPageMode.Cover" />
            <VinylRecord v-else-if="lyricStore.pageMode === LyricPageMode.Record" />
          </Transition>
        </div>

        <div class="relative flex h-full w-1/2 gap-3 pl-4">
          <LyricScrollList
            class="h-full w-0 flex-1"
            @lyric-search-show="lyricSearchVisible = true" />
          <div
            class="pointer-events-none absolute left-0 right-10 top-0 h-32 bg-gradient-to-b from-background to-transparent"></div>
          <div
            class="pointer-events-none absolute bottom-0 left-0 right-10 h-32 bg-gradient-to-t from-background to-transparent"></div>

          <div class="flex w-8 flex-col gap-3 self-end">
            <template v-if="!settingStore.isFullscreen">
              <ColorBtn />
              <FontFamilyBtn />
              <AlignBtn />
              <FontSizeBtn />
              <OffsetBtn />
              <TransBtn />
            </template>
          </div>
        </div>
      </div>
    </div>
  </Transition>

  <LyricSearchModal v-model="lyricSearchVisible" />
</template>
