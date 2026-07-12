<script lang="ts" setup>
import ColList from '@/components/MusicList/ColList.vue'
import { invoke } from '@/utils/tools'

const router = useRouter()

const isLoading = ref(true)
const colData = ref<ColList>({
  info: { id: '', cover: '', title: '排行榜歌曲', artist: '', tags: [], count: 0 },
  list: []
})

const handleLoad = async () => {
  isLoading.value = true

  const api_rank_top = await invoke('api_rank_top')
  if (api_rank_top?.status !== 1) {
    isLoading.value = true
    return
  }

  const list: RowList[] = await Promise.all(
    api_rank_top.data.list.map(async (item) => {
      const info: ListInfo = {
        id: item.rankid,
        cover: item.imgurl,
        title: item.rankname,
        artist: '',
        tags: item.intro.split('\r\n'),
        count: 0
      }

      const api_rank_audio = await invoke('api_rank_audio', { rankId: item.rankid, pageSize: 3 })
      if (api_rank_audio?.status !== 1) return { info, list: [] }

      const list = api_rank_audio.data.songlist.map((song) => ({
        id: song.audio_id,
        cover: song.trans_param.union_cover,
        title: song.songname,
        artist: song.author_name,
        musicInfo: {
          id: song.audio_id,
          path: null,
          hash: song.audio_info.hash_128,
          cover: song.trans_param.union_cover,
          title: song.songname,
          artist: song.author_name,
          album: song.album_info.album_name,
          duration: song.audio_info.duration_128 / 1000,
          sort: song.business.original_index
        }
      }))

      info.count = list.length
      return { info, list }
    })
  )

  colData.value.list = list
  isLoading.value = false
}

const handleMore = () => {
  router.push('/rank-top-more')
}
</script>

<template>
  <ColList
    :loading="isLoading"
    :data="colData"
    @load="handleLoad"
    @refresh="handleLoad"
    @more="handleMore" />
</template>
