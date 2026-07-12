<script lang="ts" setup>
import Image from '@/components/Image.vue'
import ToTop from '@/components/PageActions/ToTop.vue'
import SlideBar from '@/components/SlideBar.vue'
import VirtualList from '@/components/VirtualList.vue'
import { PageSize } from '@/utils/params'
import { getPic, invoke } from '@/utils/tools'

const router = useRouter()

const tableColumns: TableColumn[] = [
  { key: 'index', slot: true, width: '3rem', padding: 0, align: 'center' },
  { key: 'info', slot: true, width: 'auto' },
  { key: 'playCount', slot: true, width: '20%', align: 'center' }
]
const musicTableRef = useTemplateRef('musicTableRef')

const firstSlideOptions = ref<SlideOption[]>([]) // 第一级菜单项
const firstSlideSelection = ref<SlideOption>() // 第一级选中项
const secondSlideOptions = ref<SlideOption[]>([]) // 第二级菜单项
const secondSlideSelection = ref<SlideOption>() // 第二级选中项
const isLoading = ref(true)
const isFinished = ref(false)
const page = ref(1)
const playlistList = ref<PlaylistInfo[]>([])

const handleSlideGet = async () => {
  const api_playlist_tags = await invoke('api_playlist_tags')
  if (api_playlist_tags?.status !== 1) return

  firstSlideOptions.value = api_playlist_tags.data.map((item) => ({
    label: item.tag_name,
    value: item.tag_id,
    son: item.son
  }))
  firstSlideSelection.value = firstSlideOptions.value[0]

  secondSlideOptions.value = firstSlideSelection.value.son
    .slice(0, 10)
    .map((item: PlaylistTag) => ({
      label: item.tag_name,
      value: item.tag_id
    }))
  secondSlideSelection.value = secondSlideOptions.value[0]
}

const handleSlideChange = (option: SlideOption, type: 'first' | 'second') => {
  switch (type) {
    case 'first':
      firstSlideSelection.value = option

      secondSlideOptions.value = firstSlideSelection.value.son
        .slice(0, 10)
        .map((item: PlaylistTag) => ({
          label: item.tag_name,
          value: item.tag_id
        }))
      secondSlideSelection.value = secondSlideOptions.value[0]
      break
    case 'second':
      secondSlideSelection.value = option
      break
  }

  page.value = 1
  handleLoad()
}

const handleLoad = async () => {
  if (!secondSlideSelection.value) return
  isLoading.value = true

  const api_top_playlist = await invoke('api_top_playlist', {
    categoryId: secondSlideSelection.value.value,
    page: page.value,
    pageSize: PageSize.Default
  })
  if (api_top_playlist?.status !== 1) {
    isLoading.value = false
    return
  }

  const list = api_top_playlist.data.special_list.map((playlist) => ({
    id: playlist.global_collection_id,
    cover: playlist.flexible_cover,
    title: playlist.specialname,
    artist: playlist.nickname,
    play_count: playlist.play_count
  }))

  playlistList.value = list
  isLoading.value = false
}

const handleInfinite = async () => {
  if (isFinished.value || !secondSlideSelection.value) return

  const api_top_playlist = await invoke('api_top_playlist', {
    categoryId: secondSlideSelection.value.value,
    page: ++page.value,
    pageSize: PageSize.Default
  })
  if (api_top_playlist?.status !== 1) return

  const list = api_top_playlist.data.special_list.map((item) => ({
    id: item.global_collection_id,
    cover: item.flexible_cover,
    title: item.specialname,
    artist: item.nickname,
    play_count: item.play_count
  }))

  if (list.length < PageSize.Default) isFinished.value = true

  playlistList.value.push(...list)
}

const handleLineClick = (row: PlaylistInfo) => {
  router.push({
    path: '/top-playlist-table',
    query: { id: row.id, cover: row.cover, title: row.title }
  })
}

const handleToTop = () => {
  musicTableRef.value?.scrollToTop()
}

onMounted(async () => {
  await handleSlideGet()
  await handleLoad()
})
</script>

<template>
  <div class="relative space-y-3 pt-4 w-full h-full flex flex-col">
    <!-- 第一级菜单 -->
    <div class="flex items-center gap-3 mx-8">
      <div class="font-bold">歌单:</div>
      <SlideBar
        class="flex-1"
        :slideWidth="64"
        :options="firstSlideOptions"
        :selection="firstSlideSelection"
        @change="handleSlideChange($event, 'first')" />
    </div>

    <!-- 第二级菜单 -->
    <div class="flex items-center gap-3 mx-8">
      <div class="font-bold opacity-0 pointer-events-none">歌单:</div>
      <SlideBar
        class="flex-1"
        :slideWidth="64"
        :options="secondSlideOptions"
        :selection="secondSlideSelection"
        @change="handleSlideChange($event, 'second')" />
    </div>

    <VirtualList
      ref="musicTableRef"
      class="h-0 w-full flex-1 pl-8 pr-[1.625rem]"
      :columns="tableColumns"
      :loading="isLoading"
      :list="playlistList"
      @infinite="handleInfinite"
      @lineClick="handleLineClick">
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

      <template #playCount="row"> {{ (row.play_count / 10000).toFixed(1) }} 万播放</template>
    </VirtualList>

    <div class="absolute bottom-4 right-4">
      <ToTop v-if="playlistList.length" @click="handleToTop" />
    </div>
  </div>
</template>
