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

  const api_artist_audios = await invoke('api_artist_audios', {
    id: Number(route.query.id),
    pageSize: PageSize.Default
  })
  if (api_artist_audios?.status !== 1) {
    listStore.isLoading = false
    return
  }

  // TODO: 后续路由只传递 id, 其他数据请求 歌手详情 获取
  const info: ListInfo = {
    id: String(route.query.id),
    cover: String(route.query.cover),
    title: String(route.query.name),
    artist: '',
    tags: [],
    count: api_artist_audios.extra.page_total
  }

  const list: ListMusic[] = []

  api_artist_audios.data.forEach((song, index) => {
    if (!song.audio_id) return

    const privilegeTags: string[] = []
    if (song.pay_type === 2) privilegeTags.push('付费')
    else if (song.pay_type === 3) privilegeTags.push('VIP')

    list.push({
      id: song.audio_id,
      path: null,
      hash: song.hash,
      cover: song.trans_param.union_cover,
      title: song.audio_name,
      artist: song.author_name,
      album: song.album_name,
      duration: song.timelength / 1000,
      sort: index,
      privilegeTags
    })
  })

  listStore.setList(ListType.Show, { info, list })
  listStore.isLoading = false
}

const handleInfinite = async () => {
  if (!route.query.id || isFinished.value) return

  const api_artist_audios = await invoke('api_artist_audios', {
    id: Number(route.query.id),
    page: ++page.value,
    pageSize: PageSize.Default
  })
  if (api_artist_audios?.status !== 1) return

  const list: ListMusic[] = []
  api_artist_audios.data.map((song, index) => {
    if (!song.audio_id) return

    const privilegeTags: string[] = []
    if (song.pay_type === 2) privilegeTags.push('付费')
    else if (song.pay_type === 3) privilegeTags.push('VIP')

    list.push({
      id: song.audio_id,
      path: null,
      hash: song.hash,
      cover: song.trans_param.union_cover,
      title: song.audio_name,
      artist: song.author_name,
      album: song.album_name,
      duration: song.timelength / 1000,
      sort: index,
      privilegeTags
    })
  })

  if (api_artist_audios.data.length < PageSize.Default) isFinished.value = true

  listStore.addList(ListType.Show, list, false)
  listStore.isLoading = false
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
