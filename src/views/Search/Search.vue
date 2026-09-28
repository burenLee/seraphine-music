<script lang="ts" setup>
import { onMounted, onUnmounted, provide, ref, useTemplateRef, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import Image from '@/components/Image.vue';
import MusicActions from '@/components/MusicTable/MusicActions.vue';
import MusicTable from '@/components/MusicTable/MusicTable.vue';
import { notify } from '@/components/Notification.vue';
import ToTop from '@/components/PageActions/ToTop.vue';
import SlideBar from '@/components/SlideBar.vue';
import VirtualList from '@/components/VirtualList.vue';
import { defaultInfo, useListStore } from '@/stores/list';
import { getPic, getPrivilegeTags } from '@/utils/music';
import { ApiInvokeStatus, ListType, PageSize, SearchType } from '@/utils/params';
import { invoke } from '@/utils/tools';

const listType = ListType.Show;
provide('listType', listType);

const route = useRoute('Search');
const router = useRouter();

const listStore = useListStore();

const slideOptions: Array<SlideOption<SearchType>> = [
  { label: '单曲', value: SearchType.Song },
  { label: '歌手', value: SearchType.Author },
  { label: '歌单', value: SearchType.Special },
];
const artistTableColumns: TableColumn[] = [
  { key: 'index', slot: true, width: '3rem', padding: 0, align: 'center' },
  { key: 'info', width: 'auto', slot: true },
  { key: 'fanscount', slot: true, width: 'auto', align: 'center' },
];
const playlistTableColumns: TableColumn[] = [
  { key: 'index', slot: true, width: '3rem', padding: 0, align: 'center' },
  { key: 'info', slot: true, width: 'auto' },
  { key: 'playCount', slot: true, width: '20%', align: 'center' },
];

const tableRef = useTemplateRef('tableRef');

const slideSelection = ref(slideOptions[0]);
const isLoading = ref(true);
const isFinishing = ref(false);
const isFinished = ref(false);
const page = ref(1);
const artistList = ref<ArtistInfo[]>([]);
const playlistList = ref<ListInfo[]>([]);

const handleLoad = async (query: string, type: SearchType) => {
  if (!query) return;

  isLoading.value = true;

  try {
    const { status, data } = await invoke('api_search', {
      keywords: query,
      searchType: type,
      page: page.value,
      pageSize: PageSize.More,
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('搜索失败');
    } else {
      switch (type) {
        case SearchType.Song:
          listStore.isTableLoading = true;

          const info: ListInfo = { ...defaultInfo, id: 'search', title: '搜索', count: data.total };
          const list: MusicInfo[] = data.lists.map((song, index) => ({
            id: song.MixSongID,
            hash: song.FileHash,
            path: null,
            cover: song.trans_param.union_cover,
            title: song.OriSongName,
            artist: song.SingerName,
            album: song.AlbumName,
            duration: song.Duration,
            sort: index,
            privilegeTags: getPrivilegeTags(song.AlbumPrivilege, song.PayType),
          }));

          listStore.setList(listType, { info, list });
          listStore.isTableLoading = false;
          break;
        case SearchType.Author:
          artistList.value = data.lists.map((artist) => ({
            id: artist.AuthorId,
            cover: artist.Avatar,
            name: artist.AuthorName,
            fanscount: artist.FansNum,
            descibe: '',
            url: '',
          }));
          break;
        case SearchType.Special:
          playlistList.value = data.lists.map((playlist) => ({
            id: playlist.specialid,
            cover: playlist.img,
            title: playlist.specialname,
            artist: playlist.nickname,
            count: playlist.song_count,
            playCount: 0,
            tags: playlist.abtags?.map((tag) => tag.name),
            gid: playlist.gid,
          }));
          break;
      }
    }
  } catch {
    notify.error('搜索失败');
  } finally {
    isLoading.value = false;
  }
};

const handleInfinite = async () => {
  if (isFinishing.value || isFinished.value) return;

  isFinishing.value = true;

  try {
    const query = route.query.query as string;
    const type = slideSelection.value.value;

    const { status, data } = await invoke('api_search', {
      keywords: query,
      searchType: type,
      page: ++page.value,
      pageSize: PageSize.More,
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('搜索失败');
    } else {
      switch (type) {
        case SearchType.Song:
          const start = page.value * PageSize.More;
          const musics: MusicInfo[] = data.lists.map((song, index) => ({
            id: song.MixSongID,
            hash: song.FileHash,
            path: null,
            cover: song.trans_param.union_cover,
            title: song.OriSongName,
            artist: song.SingerName,
            album: song.AlbumName,
            duration: song.Duration,
            sort: start + index,
            privilegeTags: getPrivilegeTags(song.AlbumPrivilege, song.PayType),
          }));

          listStore.addList(listType, musics);
          break;
        case SearchType.Author:
          const artists = data.lists.map((artist) => ({
            id: artist.AuthorId,
            cover: artist.Avatar,
            name: artist.AuthorName,
            fanscount: artist.FansNum,
            descibe: '',
            url: '',
          }));

          artistList.value.push(...artists);
          break;
        case SearchType.Special:
          const playlists = data.lists.map((playlist) => ({
            id: playlist.specialid,
            cover: playlist.img,
            title: playlist.specialname,
            artist: playlist.nickname,
            count: playlist.song_count,
            playCount: 0,
            tags: playlist.abtags?.map((tag) => tag.name),
            gid: playlist.gid,
          }));

          playlistList.value.push(...playlists);
          break;
      }

      if (data.lists.length < PageSize.More) isFinished.value = true;
    }
  } catch {
    notify.error('搜索失败');
  } finally {
    isFinishing.value = false;
  }
};

const handleSlideChange = (option: SlideOption) => {
  slideSelection.value = option;

  listStore.resetList(listType);
  artistList.value.length = 0;
  playlistList.value.length = 0;

  isLoading.value = true;
  isFinishing.value = false;
  isFinished.value = false;
  page.value = 1;

  handleLoad(route.query.query as string, option.value);
  handleToTop();
};

const handleArtistClick = (row: ArtistInfo) => {
  router.push(`/artist-list-table/${row.id}`);
};

const handlePlaylistClick = (row: ListInfo) => {
  if (!row.gid) return;

  router.push(`/top-playlist-table/${row.gid}`);
};

const handleToTop = () => {
  tableRef.value?.scrollToTop();
};

watch(
  () => route.query,
  ({ query, type }) => {
    if (!query || !type) return;

    handleLoad(query as string, type as SearchType);
  },
  { immediate: true },
);

onMounted(() => {
  slideSelection.value =
    slideOptions.find((item) => item.value === (route.query.type as SearchType)) || slideOptions[0];
});
onUnmounted(() => listStore.resetList(listType));
</script>

<template>
  <div class="relative flex h-full w-full flex-col space-y-3 pt-4">
    <div class="mx-8 flex items-center gap-3">
      <div class="text-xl font-bold">搜索:</div>
      <SlideBar
        class="flex-1"
        :slide-width="88"
        :options="slideOptions"
        :selection="slideSelection"
        @change="handleSlideChange"
      />
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
      @line-click="handleArtistClick"
    >
      <template #index="row">
        {{ row.index + 1 }}
      </template>

      <template #info="row">
        <div class="flex items-center gap-3">
          <Image class="size-12" :src="getPic(row.cover)" />

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
      @lineClick="handlePlaylistClick"
    >
      <template #index="row">
        {{ row.index + 1 }}
      </template>

      <template #info="row">
        <div class="flex items-center gap-3">
          <Image class="size-12" :src="getPic(row.cover)" />

          <div class="w-0 flex-1 overflow-hidden">
            <div class="music-title flex items-center gap-2">
              <div class="min-w-0 truncate">{{ row.title }}</div>
              <div class="flex flex-1 gap-2">
                <div
                  class="card rounded border-info px-1 text-xs font-bold leading-4 text-info"
                  v-for="(tag, index) in row.tags"
                  :key="index"
                >
                  {{ tag }}
                </div>
              </div>
            </div>
            <div class="music-artist">{{ row.artist }}</div>
          </div>
        </div>
      </template>
    </VirtualList>

    <div v-if="slideSelection.value !== SearchType.Song" class="absolute bottom-4 right-4">
      <ToTop @click="handleToTop" />
    </div>
  </div>
</template>
