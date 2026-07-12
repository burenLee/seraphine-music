<script setup lang="ts">
import Carousel from '@/components/Carousel.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useListStore } from '@/stores/list'
import { useMusicStore } from '@/stores/music'
import { ListType } from '@/utils/params'
import { getFullName, getPic, getPlayingOrigin, invoke } from '@/utils/tools'

const musciStore = useMusicStore()
const listStore = useListStore()

const likeMusicList = ref<MusicList>({
  info: { id: '', cover: '', title: '', artist: '', count: 0, tags: [] },
  list: []
})
const currentLikeMusic = ref<ListMusic>()
const recommendMusicList = ref<MusicList>({
  info: { id: '', cover: '', title: '', artist: '', count: 0, tags: [] },
  list: []
})
const currentRecommendMusic = ref<ListMusic>()
const bannerList = ref([
  {
    title: '猜你喜欢',
    intro: '',
    img: '',
    bgColor: 'var(--color-info)',
    onClick: () => {
      if (!currentLikeMusic.value) return

      musciStore.setMusic(currentLikeMusic.value, {
        origin: getPlayingOrigin(currentLikeMusic.value)
      })
      if (listStore.play.info.id !== likeMusicList.value.info.id) {
        listStore.setList(ListType.Play, likeMusicList.value)
      }
    }
  },
  {
    title: '每日推荐',
    intro: '',
    img: '',
    bgColor: 'var(--color-success)',
    onClick: () => {
      if (!currentRecommendMusic.value) return

      musciStore.setMusic(currentRecommendMusic.value, {
        origin: getPlayingOrigin(currentRecommendMusic.value)
      })
      if (listStore.play.info.id !== recommendMusicList.value.info.id) {
        listStore.setList(ListType.Play, recommendMusicList.value)
      }
    }
  },
  {
    title: '排行榜',
    intro: '',
    img: '',
    bgColor: 'var(--color-warning)',
    disabled: true,
    onClick: () => 'TODO: 排行榜'
  }
])

const getLikeList = async () => {
  const personal_fm = await invoke('api_personal_fm')
  if (personal_fm?.status !== 1) return

  const info: ListInfo = {
    id: 'like',
    cover: '',
    title: '猜你喜欢',
    artist: '',
    count: personal_fm.data.song_list.length,
    tags: []
  }
  const list: ListMusic[] = personal_fm.data.song_list.map((song, index) => ({
    id: song.songid,
    path: null,
    hash: song.hash,
    cover: song.trans_param.union_cover,
    title: song.songname,
    artist: song.author_name,
    album: '',
    duration: song.time_length,
    sort: index
  }))

  likeMusicList.value = { info, list }
  currentLikeMusic.value = list[0]
  bannerList.value[0].intro = getFullName(currentLikeMusic.value)
  bannerList.value[0].img = currentLikeMusic.value.cover
    ? getPic(currentLikeMusic.value.cover, 'md')
    : ''
}

const getRecommendLisd = async () => {
  const music_everyday = await invoke('api_music_everyday')
  if (music_everyday?.status !== 1) return

  const info: ListInfo = {
    id: 'recommend',
    cover: '',
    title: '猜你喜欢',
    artist: '',
    count: music_everyday.data.song_list_size,
    tags: []
  }
  const list: ListMusic[] = music_everyday.data.song_list.map((song, index) => ({
    id: song.songid,
    path: null,
    hash: song.hash,
    cover: song.trans_param.union_cover,
    title: song.songname,
    artist: song.author_name,
    album: '',
    duration: song.time_length,
    sort: index
  }))

  recommendMusicList.value = { info, list }
  currentRecommendMusic.value = list[0]
  bannerList.value[1].intro = getFullName(currentRecommendMusic.value)
  bannerList.value[1].img = currentRecommendMusic.value.cover
    ? getPic(currentRecommendMusic.value.cover, 'md')
    : ''
}

onMounted(() => {
  getLikeList()
  getRecommendLisd()
})
</script>

<template>
  <div class="flex gap-3">
    <div
      v-for="(banner, index) in bannerList"
      :key="index"
      class="card flex basis-1/3 overflow-hidden text-neutral-50 bg-gradient-to-br from-[var(--from-bg)]"
      :style="{ '--from-bg': banner.bgColor }"
      :data-disabled="banner.disabled">
      <div class="w-0 flex-1 pl-4 pr-8 py-2">
        <div class="truncate font-bold text-base leading-8">{{ banner.title }}</div>
        <Carousel class="truncate leading-8" :content="banner.intro" />
      </div>

      <div class="relative size-20">
        <div
          class="absolute right-0 top-0 border size-full origin-bottom-right -rotate-[24deg] rounded-lg bg-[var(--from-bg)]"></div>
        <div
          class="absolute right-0 top-0 border size-full origin-bottom-right -rotate-12 rounded-lg bg-[var(--from-bg)]"></div>

        <div
          class="absolute group right-0 top-0 bg-[var(--from-bg)] overflow-hidden border size-full cursor-pointer rounded-lg"
          @click="banner.onClick">
          <img v-if="banner.img" :src="banner.img" loading="lazy" decoding="async" alt="" />
          <SvgIcon
            v-else
            class="size-full flex justify-center items-center"
            name="Music"
            :size="36" />

          <SvgIcon
            class="absolute inset-0 bg-black/30 text-neutral-50 transition-opacity opacity-0 group-hover:opacity-100"
            name="PlayBold"
            size="32" />
        </div>
      </div>
    </div>
  </div>
</template>
