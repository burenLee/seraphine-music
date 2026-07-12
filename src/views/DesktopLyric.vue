<script lang="ts" setup>
import SvgIcon from '@/components/SvgIcon.vue'
import { useDesktopLyricStore } from '@/stores/desktopLyric'
import { LyricEmitType } from '@/utils/params'
import { emitTo, listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'

const desktopLyricStore = useDesktopLyricStore()

const isHovering = ref(false)
const isPlaying = ref(false)
const lyric = ref<LyricLine[]>([])

const currentWindow = getCurrentWindow()

const handleClick = (type: LyricEmitType) => {
  emitTo('main', 'desktop-lyric:handler', { type })
}

onMounted(() => {
  // 监测移动
  currentWindow.onMoved(() => (isHovering.value = true))

  // 获取歌词
  emitTo('main', 'desktop-lyric:handler', { type: LyricEmitType.GetLyric })
  listen<{ type: LyricEmitType; data: LyricLine[] }>('desktop-lyric:handler', (e) => {
    if (e.payload.type === LyricEmitType.SendLyric) lyric.value = e.payload.data
  })
})
</script>

<template>
  <div
    class="h-screen select-none w-screen flex text-background transition-colors flex-col rounded-lg overflow-hidden"
    :class="isHovering ? 'bg-black/50' : 'bg-transparent'"
    @mouseleave="isHovering = false">
    <div
      data-tauri-drag-region
      class="w-full flex p-1 transition-opacity cursor-move justify-center items-center"
      :class="isHovering ? 'opacity-100 pointer-events-auto' : 'opacity-0 pointer-events-none'">
      <SvgIcon class="action-icon" name="MusicBold" @click="handleClick(LyricEmitType.Main)" />

      <div class="mx-1 h-4 w-px bg-minor" />

      <SvgIcon class="action-icon" name="PreviousBold" @click="handleClick(LyricEmitType.Prev)" />
      <SvgIcon
        class="action-icon"
        :name="isPlaying ? 'PauseBold' : 'PlayBold'"
        @click="handleClick(isPlaying ? LyricEmitType.Pause : LyricEmitType.Play)" />
      <SvgIcon class="action-icon" name="NextBold" @click="handleClick(LyricEmitType.Next)" />

      <div class="mx-1 h-4 w-px bg-minor" />

      <SvgIcon class="action-icon" name="ForwardLeftBold" />
      <SvgIcon class="action-icon" name="ForwardRightBold" />
      <SvgIcon class="action-icon" name="ZoomInBold" />
      <SvgIcon class="action-icon" name="ZoomOutBold" />
      <div class="action-icon flex justify-center items-center">字</div>
      <div class="action-icon flex justify-center items-center">颜</div>

      <div class="mx-1 h-4 w-px bg-minor" />

      <SvgIcon class="action-icon" name="LockBold" />
      <SvgIcon
        class="action-icon hover:text-error"
        name="Close"
        size="12"
        @click="handleClick(LyricEmitType.Close)" />
    </div>

    <div
      class="flex-1 px-4 overflow-auto"
      :style="{
        fontFamily: desktopLyricStore.fontFamily,
        fontSize: `${desktopLyricStore.fontSize}px`
      }">
      <div
        class="overflow-hidden whitespace-nowrap"
        v-for="(line, lineIndex) in lyric"
        :key="lineIndex"
        @mouseenter="isHovering = true">
        <span v-for="(word, wordIndex) in line.words" :key="wordIndex">
          {{ word.text }}
        </span>
      </div>
    </div>
  </div>
</template>
