<script setup lang="ts">
import { onMounted, ref } from 'vue';

import Carousel from '@/components/Carousel.vue';
import Image from '@/components/Image.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { InvokeApiStatus, ListType, PicSize } from '@/constants';
import { defaultListInfo, useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { getFullName, getOrigin, getPic } from '@/utils/music';
import { invoke } from '@/utils/tools';

const musciStore = useMusicStore();
const listStore = useListStore();

const likeMusicList = ref<MusicList>({ info: { ...defaultListInfo }, list: [] });
const currentLikeMusic = ref<MusicInfo>();
const recommendMusicList = ref<MusicList>({ info: { ...defaultListInfo }, list: [] });
const currentRecommendMusic = ref<MusicInfo>();
const bannerList = ref([
  {
    title: '猜你喜欢',
    intro: '',
    img: '',
    bgColor: 'var(--color-info)',
    onClick: () => {
      if (!currentLikeMusic.value) return;

      musciStore.setMusic(currentLikeMusic.value, { origin: getOrigin(currentLikeMusic.value) });
      if (listStore.play.info.id !== likeMusicList.value.info.id) {
        listStore.setList(ListType.Play, likeMusicList.value);
      }
    },
  },
  {
    title: '每日推荐',
    intro: '',
    img: '',
    bgColor: 'var(--color-success)',
    onClick: () => {
      if (!currentRecommendMusic.value) return;

      musciStore.setMusic(currentRecommendMusic.value, {
        origin: getOrigin(currentRecommendMusic.value),
      });
      if (listStore.play.info.id !== recommendMusicList.value.info.id) {
        listStore.setList(ListType.Play, recommendMusicList.value);
      }
    },
  },
  {
    title: '排行榜',
    intro: '',
    img: '',
    bgColor: 'var(--color-warning)',
    disabled: true,
    onClick: () => 'TODO: 排行榜',
  },
]);

const getLikeList = async () => {
  const { status, data } = await invoke('api_personal_fm');
  if (status !== InvokeApiStatus.Success) return;

  const info: ListInfo = {
    ...defaultListInfo,
    id: 'like',
    title: '猜你喜欢',
    count: data.song_list.length,
  };
  const list: MusicInfo[] = data.song_list.map((song, index) => ({
    id: song.songid,
    path: null,
    hash: song.hash,
    cover: song.trans_param.union_cover,
    title: song.songname,
    artist: song.author_name,
    album: '',
    duration: song.time_length,
    sort: index,
  }));

  likeMusicList.value = { info, list };
  currentLikeMusic.value = list[0];
  bannerList.value[0].intro = getFullName(currentLikeMusic.value);
  bannerList.value[0].img = currentLikeMusic.value.cover
    ? getPic(currentLikeMusic.value.cover, PicSize.Md)
    : '';
};

const getRecommendLisd = async () => {
  const { status, data } = await invoke('api_music_everyday_recommend');
  if (status !== InvokeApiStatus.Success) return;

  const info: ListInfo = {
    ...defaultListInfo,
    id: 'recommend',
    title: '每日推荐',
    count: data.song_list_size,
  };
  const list: MusicInfo[] = data.song_list.map((song, index) => ({
    id: song.songid,
    path: null,
    hash: song.hash,
    cover: song.trans_param.union_cover,
    title: song.songname,
    artist: song.author_name,
    album: '',
    duration: song.time_length,
    sort: index,
  }));

  recommendMusicList.value = { info, list };
  currentRecommendMusic.value = list[0];
  bannerList.value[1].intro = getFullName(currentRecommendMusic.value);
  bannerList.value[1].img = currentRecommendMusic.value.cover
    ? getPic(currentRecommendMusic.value.cover, PicSize.Md)
    : '';
};

onMounted(() => {
  getLikeList();
  getRecommendLisd();
});
</script>

<template>
  <div class="flex gap-3">
    <div
      v-for="(banner, index) in bannerList"
      :key="index"
      class="card flex basis-1/3 overflow-hidden bg-gradient-to-br from-[var(--from-bg)] text-neutral-50"
      :style="{ '--from-bg': banner.bgColor }"
      :data-disabled="banner.disabled"
    >
      <div class="w-0 flex-1 py-2 pl-4 pr-8">
        <div class="truncate text-base font-bold leading-8">{{ banner.title }}</div>
        <Carousel class="truncate leading-8" :content="banner.intro" />
      </div>

      <div class="relative size-20">
        <div
          class="absolute right-0 top-0 size-full origin-bottom-right -rotate-[24deg] rounded-lg border bg-[var(--from-bg)]"
        ></div>
        <div
          class="absolute right-0 top-0 size-full origin-bottom-right -rotate-12 rounded-lg border bg-[var(--from-bg)]"
        ></div>

        <div
          class="group absolute right-0 top-0 size-full cursor-pointer overflow-hidden rounded-lg border bg-[var(--from-bg)]"
          @click="banner.onClick"
        >
          <Image class="size-full" :src="banner.img" icon-size="32" />

          <div
            class="absolute inset-0 flex items-center justify-center bg-black/30 text-neutral-50 opacity-0 transition-opacity group-hover:opacity-100"
          >
            <SvgIcon name="PlayBold" size="32" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
