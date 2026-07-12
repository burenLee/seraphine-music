<script lang="ts" setup>
import Image from '@/components/Image.vue'
import MusicActions from '@/components/MusicTable/MusicActions.vue'
import MusicTable from '@/components/MusicTable/MusicTable.vue'
import ToTop from '@/components/PageActions/ToTop.vue'
import SlideBar from '@/components/SlideBar.vue'
import VirtualList from '@/components/VirtualList.vue'
import { useListStore } from '@/stores/list'
import { ListType, PageSize, SearchType } from '@/utils/params'
import { getPic, invoke } from '@/utils/tools'

provide('listType', ListType.Show)

const route = useRoute()
const router = useRouter()

const listStore = useListStore()

const slideOptions: Array<SlideOption<SearchType>> = [
  { label: '单曲', value: SearchType.Song },
  { label: '歌手', value: SearchType.Author },
  { label: '歌单', value: SearchType.Special }
]
const artistTableColumns: TableColumn[] = [
  { key: 'index', slot: true, width: '3rem', padding: 0, align: 'center' },
  { key: 'info', width: 'auto', slot: true },
  { key: 'fanscount', slot: true, width: 'auto', align: 'center' }
]
const playlistTableColumns: TableColumn[] = [
  { key: 'index', slot: true, width: '3rem', padding: 0, align: 'center' },
  { key: 'info', slot: true, width: 'auto' },
  { key: 'playCount', slot: true, width: '20%', align: 'center' }
]

const tableRef = useTemplateRef('tableRef')

const slideSelection = ref(slideOptions[0])
const isLoading = ref(false)
const isFinished = ref(false)
const page = ref(1)
const artistList = ref<ArtistInfo[]>([])
const playlistList = ref<PlaylistInfo[]>([])

const handleLoad = async (query: string, type: SearchType) => {
  if (!query) return
  isLoading.value = true

  const api_search = await invoke('api_search', {
    keywords: query,
    searchType: type,
    page: page.value,
    pageSize: PageSize.Default
  })
  if (api_search?.status !== 1) {
    isLoading.value = false
    return
  }

  switch (type) {
    case SearchType.Song:
      listStore.isLoading = true

      const info: ListInfo = {
        id: 'search',
        cover: '',
        title: '搜索',
        artist: '',
        count: api_search.data.total,
        tags: []
      }
      const list: ListMusic[] = api_search.data.lists.map((song, index) => {
        const privilegeTags: string[] = []
        if (song.PayType === 2) privilegeTags.push('付费')
        else if (song.PayType === 3) privilegeTags.push('VIP')

        return {
          id: song.Audioid,
          hash: song.FileHash,
          path: null,
          cover: song.trans_param.union_cover,
          title: song.OriSongName,
          artist: song.SingerName,
          album: song.AlbumName,
          duration: song.Duration,
          sort: index,
          privilegeTags
        }
      })

      listStore.setList(ListType.Show, { info, list })
      listStore.isLoading = false
      break
    case SearchType.Author:
      artistList.value = api_search.data.lists.map((artist) => ({
        id: artist.AuthorId,
        cover: artist.Avatar,
        name: artist.AuthorName,
        fanscount: artist.FansNum,
        descibe: '',
        url: ''
      }))
      break
    case SearchType.Special:
      playlistList.value = api_search.data.lists.map((playlist) => ({
        id: playlist.gid,
        cover: playlist.img,
        title: playlist.specialname,
        artist: playlist.nickname,
        play_count: playlist.play_count
      }))
      break
  }

  isLoading.value = false
}

const handleInfinite = async () => {
  if (isFinished.value) return

  const query = route.query.query as string
  const type = slideSelection.value.value
  if (!query) return

  const api_search = await invoke('api_search', {
    keywords: query,
    searchType: type,
    page: ++page.value,
    pageSize: PageSize.Default
  })
  if (api_search?.status !== 1) return

  switch (type) {
    case SearchType.Song:
      const list: ListMusic[] = api_search.data.lists.map((song, index) => {
        const privilegeTags: string[] = []
        if (song.PayType === 2) privilegeTags.push('付费')
        else if (song.PayType === 3) privilegeTags.push('VIP')

        return {
          id: song.Audioid,
          hash: song.FileHash,
          path: null,
          cover: song.trans_param.union_cover,
          title: song.OriSongName,
          artist: song.SingerName,
          album: song.AlbumName,
          duration: song.Duration,
          sort: index,
          privilegeTags
        }
      })

      listStore.addList(ListType.Show, list, false)
      break
    case SearchType.Author:
      artistList.value = artistList.value.concat(
        api_search.data.lists.map((artist) => ({
          id: artist.AuthorId,
          cover: artist.Avatar,
          name: artist.AuthorName,
          fanscount: artist.FansNum,
          descibe: '',
          url: ''
        }))
      )
      break
    case SearchType.Special:
      playlistList.value = playlistList.value.concat(
        api_search.data.lists.map((playlist) => ({
          id: playlist.gid,
          cover: playlist.img,
          title: playlist.specialname,
          artist: playlist.nickname,
          play_count: playlist.play_count
        }))
      )
      break
  }
}

const handleSlideChange = (option: SlideOption) => {
  slideSelection.value = option

  listStore.resetList(ListType.Show)
  artistList.value = []
  playlistList.value = []
  page.value = 1

  handleLoad(route.query.query as string, option.value)
}

const handleArtistClick = (row: ArtistInfo) => {
  router.push({
    path: '/artist-list-table',
    query: { id: row.id, cover: row.cover, name: row.name }
  })
}

const handlePlaylistClick = (row: PlaylistInfo) => {
  router.push({
    path: '/top-playlist-table',
    query: { id: row.id, cover: row.cover, title: row.title }
  })
}

const handleToTop = () => {
  tableRef.value?.scrollToTop()
}

watch(
  () => route.query,
  ({ query, type }) => {
    if (!query || !type) return

    handleLoad(query as string, type as SearchType)
  },
  { immediate: true }
)

onMounted(() => {
  slideSelection.value =
    slideOptions.find((item) => item.value === (route.query.type as SearchType)) || slideOptions[0]
})
onUnmounted(() => listStore.resetList(ListType.Show))
</script>

<template>
  <div class="relative space-y-3 pt-4 w-full h-full flex flex-col">
    <div class="flex items-center gap-3 mx-8">
      <div class="font-bold text-xl">搜索:</div>
      <SlideBar
        class="flex-1"
        :slide-width="88"
        :options="slideOptions"
        :selection="slideSelection"
        @change="handleSlideChange" />
    </div>

    <template v-if="slideSelection.value === SearchType.Song">
      <MusicActions />
      <MusicTable @infinite="handleInfinite" class="h-0 flex-1" />
    </template>

    <VirtualList
      v-else-if="slideSelection.value === SearchType.Author"
      ref="tableRef"
      class="h-0 w-full flex-1 pl-8 pr-[1.625rem]"
      :columns="artistTableColumns"
      :loading="isLoading"
      :list="artistList"
      @infinite="handleInfinite"
      @line-click="handleArtistClick">
      <template #index="row">
        {{ row.index + 1 }}
      </template>

      <template #info="row">
        <div class="flex items-center gap-3">
          <Image class="size-12" :img="getPic(row.cover)" />

          <div class="w-0 flex-1 truncate font-bold">{{ row.name }}</div>
        </div>
      </template>

      <template #fanscount="row">{{ (row.fanscount / 1000).toFixed(1) }}万粉丝</template>
    </VirtualList>

    <VirtualList
      v-else-if="slideSelection.value === SearchType.Special"
      ref="tableRef"
      class="h-0 w-full flex-1 pl-8 pr-[1.625rem]"
      :columns="playlistTableColumns"
      :loading="isLoading"
      :list="playlistList"
      @infinite="handleInfinite"
      @lineClick="handlePlaylistClick">
      <template #index="row">
        {{ row.index + 1 }}
      </template>

      <template #info="row">
        <div class="flex items-center gap-3">
          <Image class="size-12" :img="getPic(row.cover)" />

          <div class="w-0 flex-1 overflow-hidden">
            <div class="music-title">{{ row.title }}</div>
            <div class="music-artist">{{ row.artist }}</div>
          </div>
        </div>
      </template>

      <template #playCount="row">{{ row.play_count }} 次播放</template>
    </VirtualList>

    <div v-if="slideSelection.value !== SearchType.Song" class="absolute right-4 bottom-4">
      <ToTop @click="handleToTop" />
    </div>
  </div>
</template>
