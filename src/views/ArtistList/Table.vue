<script lang="ts" setup>
import { onMounted, onUnmounted, provide, ref } from 'vue';
import { useRoute } from 'vue-router';

import MusicActions from '@/components/MusicTable/MusicActions.vue';
import MusicHeader from '@/components/MusicTable/MusicHeader.vue';
import MusicTable from '@/components/MusicTable/MusicTable.vue';
import { notify } from '@/components/Notification.vue';
import { defaultInfo, useListStore } from '@/stores/list';
import { getPrivilegeTags } from '@/utils/music';
import { ApiInvokeStatus, ListType, PageSize } from '@/utils/params';
import { invoke } from '@/utils/tools';

const listType = ListType.Show;
provide('listType', listType);

const route = useRoute('ArtistListTable');
const listStore = useListStore();

const isFinishing = ref(false);
const isFinished = ref(false);
const page = ref(1);

const handleLoad = async () => {
  listStore.isHeaderLoading = true;
  listStore.isTableLoading = true;

  try {
    const id = route.params.id;

    const artist_detail = await invoke('api_artist_detail', { id });
    if (artist_detail.status !== ApiInvokeStatus.Success) {
      notify.error('获取歌手信息失败');
    } else {
      const info: ListInfo = {
        ...defaultInfo,
        id: artist_detail.data.author_id,
        cover: artist_detail.data.sizable_avatar,
        title: artist_detail.data.author_name,
        count: artist_detail.data.song_count,
      };
      listStore.setListInfo(listType, info);
      listStore.isHeaderLoading = false;

      const list: MusicInfo[] = [];
      const artist_audios = await invoke('api_artist_audios', { id, pageSize: PageSize.More });
      if (artist_audios.status !== ApiInvokeStatus.Success) {
        notify.error('获取歌手歌曲失败');
      } else {
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
      }
    }
  } catch {
    notify.error('获取歌手歌曲失败');
  } finally {
    listStore.isTableLoading = false;
  }
};

const handleInfinite = async () => {
  if (isFinishing.value || isFinished.value) return;

  isFinishing.value = true;

  try {
    const id = route.params.id;

    const { status, data } = await invoke('api_artist_audios', {
      id,
      page: ++page.value,
      pageSize: PageSize.More,
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('获取歌手歌曲失败');
    } else {
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
    }
  } catch {
    notify.error('获取歌手歌曲失败');
  } finally {
    isFinishing.value = false;
  }
};

onMounted(handleLoad);
onUnmounted(() => {
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
