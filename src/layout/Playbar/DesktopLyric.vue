<script lang="ts" setup>
import { notify } from '@/components/Notification'
import { useLyricStore } from '@/stores/lyric'
import { LyricEmitType } from '@/utils/params'
import { emitTo, listen } from '@tauri-apps/api/event'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { getCurrentWindow } from '@tauri-apps/api/window'

const lyricStore = useLyricStore()

let lyricsWindow: WebviewWindow | undefined

const creatWindow = () => {
  const screenWidth = window.screen.availWidth
  const screenHeight = window.screen.availHeight
  const width = 640
  const height = 160

  lyricsWindow = new WebviewWindow('desktop-lyric', {
    title: '桌面歌词',
    url: '/desktop-lyric.html',
    width,
    height,
    x: Math.round((screenWidth - width) / 2),
    y: Math.round(screenHeight - height),
    transparent: true,
    decorations: false,
    alwaysOnTop: true,
    shadow: false,
    contentProtected: true,
    skipTaskbar: true
  })

  lyricsWindow.once('tauri://error', () => notify.error('桌面歌词创建失败'))
}

const destroyWindow = () => {
  if (!lyricsWindow) return

  lyricsWindow.close()
  lyricsWindow = undefined
}

listen<{ type: LyricEmitType }>('desktop-lyric:handler', (e) => {
  switch (e.payload.type) {
    case LyricEmitType.GetLyric:
      emitTo('desktop-lyric', 'desktop-lyric:handler', {
        type: LyricEmitType.SendLyric,
        data: lyricStore.lyric?.lines
      })
      break

    case LyricEmitType.Main:
      getCurrentWindow().show()
      break

    case LyricEmitType.Close:
      destroyWindow()
      break

    default:
      break
  }
})
</script>

<template>
  <div
    class="action-icon text-center font-bold leading-8"
    :class="lyricsWindow ? 'text-info' : ''"
    title="桌面歌词"
    :data-disabled="true"
    @click="lyricsWindow ? destroyWindow() : creatWindow()">
    词
  </div>
</template>
