<script lang="ts" setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';

import ColList from '@/components/MusicList/ColList.vue';
import { InvokeApiStatus, PageSize } from '@/constants';
import { defaultListInfo } from '@/stores/list';
import { genRandomNum, invoke } from '@/utils/tools';

const router = useRouter();

const isLoading = ref(true);
const colData = ref<ColList>({ info: { ...defaultListInfo, title: '歌单排行榜' }, list: [] });

const handleLoad = async () => {
  isLoading.value = true;

  try {
    const playlist_tags = await invoke('api_playlist_tags');
    if (playlist_tags.status !== InvokeApiStatus.Success) return;

    const list: RowList[] = await Promise.all(
      playlist_tags.data.map(async (tag) => {
        const top_playlist = await invoke('api_top_playlist', {
          categoryId: tag.son[genRandomNum(tag.son.length - 1)].tag_id,
          pageSize: PageSize.Min,
        });

        const info: ListInfo = { ...defaultListInfo, title: tag.tag_name };
        let list: CardInfo[] = [];
        if (top_playlist.status === InvokeApiStatus.Success) {
          list = top_playlist.data.special_list.slice(0, PageSize.Min).map((playlist, index) => ({
            id: playlist.global_collection_id,
            cover: playlist.flexible_cover,
            title: playlist.specialname,
            artist: playlist.nickname,
            playlistInfo: {
              id: playlist.specialid,
              cover: playlist.flexible_cover,
              title: playlist.specialname,
              artist: playlist.nickname,
              count: PageSize.Min,
              playCount: playlist.play_count,
              tags: playlist.abtags?.map((tag) => tag.name),
              gid: playlist.global_collection_id,
              sort: index,
            },
          }));
        }

        return { info, list };
      }),
    );

    colData.value.list = list;
  } finally {
    isLoading.value = false;
  }
};
</script>

<template>
  <ColList
    :loading="isLoading"
    :data="colData"
    @load="handleLoad"
    @refresh="handleLoad"
    @more="router.push('/top-playlist-more')"
  />
</template>
