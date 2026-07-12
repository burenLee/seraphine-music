<script lang="ts" setup>
import RowList from '@/components/MusicList/RowList.vue'
import { invoke } from '@/utils/tools'

interface Props {
  cardId: number
}

const { cardId } = defineProps<Props>()

const isLoading = ref(true)
const rowData = ref<RowList>({
  info: { id: '', cover: '', title: '', artist: '', count: 0, tags: [] },
  list: []
})

const handleLoad = async () => {
  isLoading.value = true

  const topCard = await invoke('api_top_card', { cardId })
  if (topCard?.status !== 1) {
    isLoading.value = false
    return
  }

  const list: CardInfo[] = topCard.data.song_list.map((song, index) => ({
    id: song.songid,
    cover: song.trans_param.union_cover,
    title: song.songname,
    artist: song.author_name,
    musicInfo: {
      id: song.songid,
      hash: song.hash,
      path: null,
      cover: song.trans_param.union_cover,
      title: song.songname,
      artist: song.author_name,
      album: song.album_name,
      duration: song.time_length,
      sort: index
    }
  }))

  rowData.value.info.title = topCard.data.rec_desc.replace(/[「」]/g, '')
  rowData.value.info.count = list.length
  rowData.value.list = list
  isLoading.value = false
}
</script>

<template>
  <RowList :loading="isLoading" :data="rowData" not-more @load="handleLoad" @refresh="handleLoad" />
</template>
