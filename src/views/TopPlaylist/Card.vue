<script lang="ts" setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';

import ColList from '@/components/MusicList/ColList.vue';
import { defaultInfo } from '@/stores/list';
import { ApiInvokeStatus, PageSize } from '@/utils/params';
import { genRandomNum, invoke } from '@/utils/tools';

const router = useRouter();

const isLoading = ref(true);
const colData = ref<ColList>({ info: { ...defaultInfo, title: '推荐歌单' }, list: [] });

const handleLoad = async () => {
  isLoading.value = true;

  try {
    const playlist_tags = await invoke('api_playlist_tags');
    if (playlist_tags.status === ApiInvokeStatus.Success) {
      const list: RowList[] = await Promise.all(
        playlist_tags.data.map(async (tag) => {
          const top_playlist = await invoke('api_top_playlist', {
            categoryId: tag.son[genRandomNum(tag.son.length - 1)].tag_id,
            // 结果有时候会少一个所以+1
            pageSize: PageSize.Min + 1,
          });

          const info: ListInfo = { ...defaultInfo, title: tag.tag_name };
          let list: CardInfo[] = [];
          if (top_playlist.status === ApiInvokeStatus.Success) {
            list = top_playlist.data.special_list.slice(0, PageSize.Min).map((playlist) => ({
              id: playlist.global_collection_id,
              cover: playlist.flexible_cover,
              title: playlist.specialname,
              artist: playlist.nickname,
              playlistInfo: {
                id: playlist.specialid,
                cover: playlist.flexible_cover,
                title: playlist.specialname,
                artist: playlist.nickname,
                count: 0,
                playCount: playlist.play_count,
                tags: playlist.abtags?.map((tag) => tag.name),
                gid: playlist.global_collection_id,
              },
            }));
          }

          return { info, list };
        }),
      );

      colData.value.list = list;
    }
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
