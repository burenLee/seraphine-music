<script lang="ts" setup>
import { onUnmounted, provide, ref, watch } from 'vue';
import { useRoute } from 'vue-router';

import MusicActions from '@/components/MusicTable/Actions.vue';
import MusicHeader from '@/components/MusicTable/Header.vue';
import MusicTable from '@/components/MusicTable/Table.vue';
import { InvokeApiStatus, ListType, PageSize } from '@/constants';
import { defaultListInfo, useListStore } from '@/stores/list';
import { getPrivilegeTags } from '@/utils/music';
import { invoke } from '@/utils/tools';

const listType = ListType.Show;
provide('listType', listType);

const route = useRoute('ArtistListTable');
const listStore = useListStore();

// 竞态保护：请求序号与卸载标记，避免快速切换歌手时旧请求覆盖新数据
let requestId = 0;
let isUnmounted = false;

const isFinishing = ref(false);
const isFinished = ref(false);
const page = ref(1);

const handleLoad = async (id: string) => {
  const reqId = ++requestId;

  listStore.isHeaderLoading = true;
  listStore.isTableLoading = true;

  // 切换歌手时重置分页状态
  isFinishing.value = false;
  isFinished.value = false;
  page.value = 1;

  try {
    const artist_detail = await invoke('api_artist_detail', { id });
    if (isUnmounted || reqId !== requestId || artist_detail.status !== InvokeApiStatus.Success)
      return;

    const info: ListInfo = {
      ...defaultListInfo,
      id: artist_detail.data.author_id,
      cover: artist_detail.data.sizable_avatar,
      title: artist_detail.data.author_name,
      count: artist_detail.data.song_count,
    };
    listStore.setListInfo(listType, info);
    listStore.isHeaderLoading = false;

    const list: MusicInfo[] = [];

    const artist_audios = await invoke('api_artist_audios', { id, pageSize: PageSize.More });
    if (isUnmounted || reqId !== requestId || artist_audios.status !== InvokeApiStatus.Success)
      return;

    artist_audios.data.forEach((song, index) => {
      if (!song.album_audio_id) return;

      list.push({
        id: song.album_audio_id,
        path: null,
        hash: song.hash,
        cover: song.trans_param.union_cover,
        title: song.audio_name,
        artist: song.author_name,
        album: song.album_name,
        duration: song.timelength / 1000,
        sort: index,
        privilegeTags: getPrivilegeTags(song.privilege, song.pay_type),
      });
    });

    listStore.setListRaw(listType, list);
  } finally {
    if (!isUnmounted && reqId === requestId) {
      listStore.isHeaderLoading = false;
      listStore.isTableLoading = false;
    }
  }
};

const handleInfinite = async () => {
  if (isFinishing.value || isFinished.value) return;

  const reqId = requestId;
  isFinishing.value = true;

  try {
    const { status, data } = await invoke('api_artist_audios', {
      id: route.params.id,
      page: ++page.value,
      pageSize: PageSize.More,
    });
    if (isUnmounted || reqId !== requestId || status !== InvokeApiStatus.Success) return;

    const list: MusicInfo[] = [];
    const start = page.value * PageSize.More;

    data.forEach((song, index) => {
      if (!song.album_audio_id) return;

      list.push({
        id: song.album_audio_id,
        path: null,
        hash: song.hash,
        cover: song.trans_param.union_cover,
        title: song.audio_name,
        artist: song.author_name,
        album: song.album_name,
        duration: song.timelength / 1000,
        sort: start + index,
        privilegeTags: getPrivilegeTags(song.privilege, song.pay_type),
      });
    });

    listStore.addListRaw(listType, list);
    if (data.length < PageSize.More) isFinished.value = true;
  } finally {
    if (!isUnmounted && reqId === requestId) {
      isFinishing.value = false;
    }
  }
};

watch(() => route.params.id, handleLoad, { immediate: true });
onUnmounted(() => {
  isUnmounted = true;
  listStore.isHeaderLoading = true;
  listStore.isTableLoading = true;
  listStore.resetList(listType);
});
</script>

<template>
  <div class="relative flex h-full w-full flex-col space-y-3 pt-4">
    <MusicHeader />
    <MusicActions />
    <MusicTable class="h-0 flex-1" @infinite="handleInfinite" />
  </div>
</template>
