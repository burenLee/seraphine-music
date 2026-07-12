<script lang="ts" setup>
import ColList from '@/components/MusicList/ColList.vue'
import { AreaTypes } from '@/utils/params'
import { invoke } from '@/utils/tools'

const router = useRouter()

const isLoading = ref(true)
const colData = ref<ColList>({
  info: { id: '', cover: '', title: '推荐歌手', artist: '', tags: [], count: 0 },
  list: []
})

const handleLoad = async () => {
  isLoading.value = true

  const list: RowList[] = await Promise.all(
    AreaTypes.slice(0, 5).map(async (item) => {
      const info: ListInfo = {
        id: '',
        cover: '',
        title: item.title,
        artist: '',
        tags: [],
        count: 0
      }

      const api_artist_list = await invoke('api_artist_list', {
        areaType: item.type,
        musician: item.musician,
        pageSize: 3
      })
      if (api_artist_list?.status !== 1) return { info, list: [] }

      const list: CardInfo[] = api_artist_list.data.info.map((singer) => ({
        id: singer.singerid,
        cover: singer.imgurl,
        title: singer.singername,
        artist: '',
        artistInfo: {
          id: singer.singerid,
          cover: singer.imgurl,
          name: singer.singername,
          fanscount: singer.fanscount,
          descibe: singer.descibe,
          url: singer.url
        }
      }))

      return { info, list }
    })
  )

  colData.value.list = list
  isLoading.value = false
}

const handleMore = () => {
  router.push('/artist-list-more')
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
