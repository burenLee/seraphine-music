<script lang="ts" setup>
import { notify } from '@/components/Notification'
import { useDesktopLyricBridge } from '@/composables/useDesktopLyricBridge'
import { WindowName } from '@/utils/params'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

const bridge = useDesktopLyricBridge(destroyWindow)

let lyricsWindow: WebviewWindow | undefined

/** 桌面歌词窗口是否打开 */
const isLyricOpen = () => WebviewWindow.getByLabel(WindowName.DesktopLyric) !== null

const creatWindow = () => {
  const screenWidth = window.screen.availWidth
  const screenHeight = window.screen.availHeight
  const width = 640
  const height = 160

  lyricsWindow = new WebviewWindow(WindowName.DesktopLyric, {
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

  lyricsWindow.once('tauri://created', () => bridge.pushNow())
  lyricsWindow.once('tauri://error', () => notify.error('桌面歌词创建失败'))
}

function destroyWindow() {
  if (!lyricsWindow) return
  lyricsWindow.close()
  lyricsWindow = undefined
}

onMounted(() => {
  bridge.start()
})

onUnmounted(() => {
  bridge.stop()
})
</script>

<template>
  <div
    class="action-icon text-center font-bold leading-8"
    :class="isLyricOpen() ? 'text-info' : ''"
    title="桌面歌词"
    @click="isLyricOpen() ? destroyWindow() : creatWindow()">
    词
  </div>
</template>
