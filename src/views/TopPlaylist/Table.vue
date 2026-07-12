<script lang="ts" setup>
import MusicActions from '@/components/MusicTable/MusicActions.vue'
import MusicHeader from '@/components/MusicTable/MusicHeader.vue'
import MusicTable from '@/components/MusicTable/MusicTable.vue'
import { useListStore } from '@/stores/list'
import { ListType, PageSize } from '@/utils/params'
import { invoke } from '@/utils/tools'

provide('listType', ListType.Show)

const route = useRoute()
const listStore = useListStore()

const isFinished = ref(false)
const page = ref(1)

const handleLoad = async () => {
  if (!route.query.id) return

  listStore.isLoading = true

  const api_playlist_tracks_all = await invoke('api_playlist_tracks_all', {
    gid: route.query.id as string,
    pageSize: PageSize.Default
  })
  if (api_playlist_tracks_all?.status !== 1) {
    listStore.isLoading = false
    return
  }

  const info: ListInfo = {
    id: route.query.id as string,
    cover: route.query.cover as string,
    title: route.query.title as string,
    artist: '',
    tags: [],
    count: api_playlist_tracks_all.data.count
  }

  const list: ListMusic[] = []
  api_playlist_tracks_all.data.songs.forEach((song) => {
    if (!song.fileid) return

    const [artist, title] = song.name.split('-')
    const privilegeTags: string[] = []

    if (song.download[0].pay_type === 2) privilegeTags.push('付费')
    else if (song.download[0].pay_type === 3) privilegeTags.push('VIP')

    list.push({
      id: song.fileid,
      path: null,
      hash: song.hash,
      cover: song.trans_param.union_cover,
      title: title.trim(),
      artist: artist.trim(),
      album: song.albuminfo.name,
      duration: song.timelen / 1000,
      sort: song.sort,
      privilegeTags
    })
  })

  listStore.setList(ListType.Show, { info, list })
  listStore.isLoading = false
}

const handleInfinite = async () => {
  if (!route.query.id || isFinished.value) return

  const api_playlist_tracks_all = await invoke('api_playlist_tracks_all', {
    gid: route.query.id as string,
    page: ++page.value,
    pageSize: PageSize.Default
  })
  if (api_playlist_tracks_all?.status !== 1) return

  const list: ListMusic[] = []
  api_playlist_tracks_all.data.songs.forEach((song: any) => {
    if (!song.audio_id) return

    const [artist, title] = song.name.split('-')

    const privilegeTags: string[] = []
    if (song.download[0].pay_type === 2) privilegeTags.push('付费')
    else if (song.download[0].pay_type === 3) privilegeTags.push('VIP')

    list.push({
      id: song.audio_id,
      path: null,
      hash: song.relate_goods[0].hash,
      cover: song.cover,
      title: title.trim(),
      artist: artist.trim(),
      album: song.albuminfo.name,
      duration: song.timelen / 1000,
      sort: song.sort,
      privilegeTags
    })
  })

  if (list.length < PageSize.Default) isFinished.value = true

  listStore.addList(ListType.Show, list, false)
}

onMounted(handleLoad)
onUnmounted(() => listStore.resetList(ListType.Show))
</script>

<template>
  <div class="relative space-y-3 pt-4 w-full h-full flex flex-col">
    <MusicHeader />
    <MusicActions />
    <MusicTable class="h-0 flex-1" @infinite="handleInfinite" />
  </div>
</template>
