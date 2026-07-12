<script setup lang="ts">
import MusicActions from '@/components/MusicTable/MusicActions.vue'
import MusicHeader from '@/components/MusicTable/MusicHeader.vue'
import MusicTable from '@/components/MusicTable/MusicTable.vue'
import { useListStore } from '@/stores/list'
import { ListType, PageSize } from '@/utils/params'
import { getPrivilegeTags, invoke } from '@/utils/tools'

provide('listType', ListType.Show)

const route = useRoute()
const listStore = useListStore()

const handleLoad = async () => {
  if (!route.query.id) return

  listStore.isLoading = true

  const playlist_detail = await invoke('api_playlist_detail', {
    gids: [route.query.id as string]
  })
  if (playlist_detail?.status !== 1) {
    listStore.isLoading = false
    return
  }

  const { pic, name, musiclib_tags, count, list_create_listid, list_create_username } =
    playlist_detail.data[0]

  let page = 1
  const allSongs: PlaylistSong[] = []

  // 获取全部歌曲
  while (allSongs.length < playlist_detail.data[0].count) {
    const playlist_tracks_all = await invoke('api_playlist_tracks_all', {
      gid: playlist_detail.data[0].list_create_gid,
      page: page++,
      pageSize: PageSize.Max
    })
    if (playlist_tracks_all?.status !== 1) break

    allSongs.push(...playlist_tracks_all.data.songs)
  }

  const info: ListInfo = {
    id: list_create_listid,
    cover: pic,
    title: name,
    artist: list_create_username,
    tags: musiclib_tags.map((tag: any) => tag.tag_name),
    count: count
  }
  const list: ListMusic[] = []

  allSongs.forEach((song) => {
    if (!song.hash) return

    const [artist, title] = song.name.split('-')

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
      privilegeTags: getPrivilegeTags(song.privilege, song.download[0].pay_type)
    })
  })

  listStore.setList(ListType.Show, { info, list })
  listStore.isLoading = false
}

onMounted(handleLoad)
onUnmounted(() => listStore.resetList(ListType.Show))
</script>

<template>
  <div class="relative space-y-3 pt-4 w-full h-full flex flex-col">
    <MusicHeader />
    <MusicActions />
    <MusicTable class="h-0 flex-1" />
  </div>
</template>
