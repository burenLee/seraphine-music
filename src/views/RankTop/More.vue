<script lang="ts" setup>
import MusicActions from '@/components/MusicTable/MusicActions.vue'
import MusicTable from '@/components/MusicTable/MusicTable.vue'
import SlideBar from '@/components/SlideBar.vue'
import { useListStore } from '@/stores/list'
import { ListType } from '@/utils/params'
import { getPrivilegeTags, invoke } from '@/utils/tools'

provide('listType', ListType.Show)

const listStore = useListStore()

const slideOptions = ref<SlideOption[]>([])
const slideSelection = ref<SlideOption>()

const handleSlideGet = async () => {
  const api_rank_top = await invoke('api_rank_top')
  if (api_rank_top?.status !== 1) return

  slideOptions.value = api_rank_top.data.list.map((item) => ({
    label: item.rankname,
    value: item.rankid,
    imgurl: item.imgurl,
    intro: item.intro
  }))
  slideSelection.value = slideOptions.value[0]
}

const handleSlideChange = (option: SlideOption) => {
  slideSelection.value = option
  handleLoad()
}

const handleLoad = async () => {
  if (!slideSelection.value) return
  listStore.isLoading = true

  const api_rank_audio = await invoke('api_rank_audio', {
    rankId: slideSelection.value.value,
    pageSize: 100
  })
  if (api_rank_audio?.status !== 1) {
    listStore.isLoading = false
    return
  }

  const info: ListInfo = {
    id: slideSelection.value.value,
    cover: slideSelection.value.imgurl,
    title: slideSelection.value.label,
    artist: '',
    tags: slideSelection.value.intro.split('\r\n'),
    count: api_rank_audio.data.songlist.length
  }
  const list: ListMusic[] = api_rank_audio.data.songlist.map((song) => ({
    id: song.audio_id,
    path: null,
    hash: song.audio_info.hash_128,
    cover: song.trans_param.union_cover,
    title: song.songname,
    artist: song.author_name,
    album: song.album_info.album_name,
    duration: song.audio_info.duration_128 / 1000,
    sort: song.business.original_index,
    privilegeTags: getPrivilegeTags(song.privilege_download.privilege, song.deprecated.pay_type)
  }))

  listStore.setList(ListType.Show, { info, list })
  listStore.isLoading = false
}

onMounted(async () => {
  await handleSlideGet()
  await handleLoad()
})

onUnmounted(() => listStore.resetList(ListType.Show))
</script>

<template>
  <div class="relative space-y-3 pt-4 w-full h-full flex flex-col">
    <div class="flex items-center gap-3 mx-8">
      <div class="font-bold text-xl">排行榜歌曲:</div>
      <SlideBar
        class="flex-1"
        :slide-width="88"
        :options="slideOptions"
        :selection="slideSelection"
        @change="handleSlideChange" />
    </div>

    <MusicActions />
    <MusicTable class="h-0 flex-1" />
  </div>
</template>
