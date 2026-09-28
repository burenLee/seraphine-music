<script lang="ts" setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';

import ColList from '@/components/MusicList/ColList.vue';
import { defaultInfo } from '@/stores/list';
import { ApiInvokeStatus } from '@/utils/params';
import { invoke } from '@/utils/tools';

const router = useRouter();

const isLoading = ref(true);
const colData = ref<ColList>({ info: { ...defaultInfo, title: '排行榜歌曲' }, list: [] });

const handleLoad = async () => {
  isLoading.value = true;

  try {
    const rank_top = await invoke('api_rank_top');
    if (rank_top.status === ApiInvokeStatus.Success) {
      const list: RowList[] = await Promise.all(
        rank_top.data.list.map(async (item) => {
          const rank_audio = await invoke('api_rank_audio', { rankId: item.rankid, pageSize: 3 });

          const info: ListInfo = {
            ...defaultInfo,
            id: item.rankid,
            cover: item.imgurl,
            title: item.rankname,
            tags: item.intro.split('\r\n'),
          };
          let list: CardInfo[] = [];
          if (rank_audio.status === ApiInvokeStatus.Success) {
            list = rank_audio.data.songlist.map((song) => ({
              id: song.album_audio_id,
              cover: song.trans_param.union_cover,
              title: song.songname,
              artist: song.author_name,
              musicInfo: {
                id: song.album_audio_id,
                path: null,
                hash: song.audio_info.hash_128,
                cover: song.trans_param.union_cover,
                title: song.songname,
                artist: song.author_name,
                album: song.album_info.album_name,
                duration: song.audio_info.duration_128 / 1000,
                sort: song.business.original_index,
              },
            }));
            info.count = list.length;
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
    @more="router.push('/rank-top-more')"
  />
</template>
