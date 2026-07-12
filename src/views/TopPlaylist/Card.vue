<script lang="ts" setup>
import ColList from '@/components/MusicList/ColList.vue'
import { invoke } from '@/utils/tools'

const router = useRouter()

const isLoading = ref(true)
const colData = ref<ColList>({
  info: { id: '', cover: '', title: '推荐歌单', artist: '', count: 0, tags: [] },
  list: []
})

const handleLoad = async () => {
  isLoading.value = true

  const api_playlist_tags = await invoke('api_playlist_tags')
  if (api_playlist_tags?.status !== 1) return

  const list: RowList[] = await Promise.all(
    api_playlist_tags.data.map(async (item) => {
      const info = { id: '', cover: '', title: item.tag_name, artist: '', count: 0, tags: [] }

      const api_top_playlist = await invoke('api_top_playlist', {
        categoryId: item.son[0].tag_id,
        pageSize: 4
      })
      if (api_top_playlist?.status !== 1) return { info, list: [] }

      const list: CardInfo[] = api_top_playlist.data.special_list.slice(0, 3).map((item) => ({
        id: item.global_collection_id,
        cover: item.flexible_cover,
        title: item.specialname,
        artist: item.nickname,
        playlistInfo: {
          id: item.global_collection_id,
          cover: item.flexible_cover,
          title: item.specialname,
          artist: item.nickname,
          play_count: item.play_count
        }
      }))

      return { info, list }
    })
  )

  colData.value.list = list
  isLoading.value = false
}

const handleMore = () => {
  router.push('/top-playlist-more')
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
