<script lang="ts" setup>
import { useLyricStore } from '@/stores/lyric'
import { Interval, LyricTextColor } from '@/utils/params'
import { vOnClickOutside } from '@vueuse/components'
import { watchThrottled } from '@vueuse/core'

const lyricStore = useLyricStore()

const presetsColors = [
  LyricTextColor.Red,
  LyricTextColor.Orange,
  LyricTextColor.Yellow,
  LyricTextColor.Green,
  LyricTextColor.Cyan,
  LyricTextColor.Blue,
  LyricTextColor.Purple
]

const pickerVisible = ref(false)
const usedColor = ref(lyricStore.setting.textColor) // 使用的颜色
const lastColor = ref(lyricStore.setting.textColor) // 缓存最后可用的颜色

watchThrottled(
  usedColor,
  (color) => {
    // 校验颜色格式
    if (/^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6})$/.test(color)) {
      lastColor.value = color
      lyricStore.setTextColor(color)
    }
  },
  { throttle: Interval.Long }
)
</script>

<template>
  <div class="relative" v-on-click-outside="() => (pickerVisible = false)">
    <div class="action-icon card p-1" title="歌词颜色" @click="pickerVisible = !pickerVisible">
      <div class="rounded-md size-full" :style="{ background: lyricStore.setting.textColor }"></div>
    </div>

    <Transition name="zoom-fade">
      <div
        v-if="pickerVisible"
        class="p-4 bg-background mr-2 border border-border rounded-lg shadow-md absolute right-full top-0">
        <div class="font-bold">歌词颜色</div>

        <div class="flex items-center gap-2 pt-3">
          <div
            v-for="(color, index) in presetsColors"
            class="rounded-full size-4 cursor-pointer hover:scale-110 transition-transform"
            :key="index"
            :style="{ background: color }"
            @click="usedColor = color"></div>
        </div>

        <div class="flex items-center gap-2 pt-3">
          <input class="size-6 cursor-pointer rounded-md" type="color" v-model="usedColor" />
          <input
            class="border border-border rounded-md leading-6 px-2 bg-card"
            type="text"
            v-model="usedColor"
            @blur="usedColor = lastColor" />
        </div>
      </div>
    </Transition>
  </div>
</template>
