<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue'
import { useMusicStore } from '@/stores/music'
import { useListContext } from '@/utils/hooks'
import { ListType } from '@/utils/params'
import { getPlayingOrigin } from '@/utils/tools'

const { listStore, list } = useListContext()

const musicStore = useMusicStore()

const handlePlay = () => {
  if (musicStore.isPlaying || list.value.list.length === 0) return

  let music: ListMusic
  let shouldUpdate = false

  if (listStore.play.info.id === list.value.info.id) {
    // 同一列表：使用播放列表
    if (musicStore.music) {
      musicStore.play()
      return
    }

    music = listStore.play.list[0]
  } else {
    // 不同列表：使用target列表
    music = list.value.list[0]
    shouldUpdate = true
  }

  musicStore.setMusic(music, { origin: getPlayingOrigin(music) })

  if (shouldUpdate)
    listStore.setList(ListType.Play, { info: list.value.info, list: [...list.value.list] })
}
</script>

<template>
  <ActionButton prefix-icon="Play" @click="handlePlay">播放</ActionButton>
</template>
