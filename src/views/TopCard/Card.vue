<script lang="ts" setup>
import { ref } from 'vue';

import RowList from '@/components/MusicList/RowList.vue';
import { defaultInfo } from '@/stores/list';
import { ApiInvokeStatus } from '@/utils/params';
import { invoke } from '@/utils/tools';

interface Props {
  cardId: number;
}

const { cardId } = defineProps<Props>();

const isLoading = ref(true);
const rowData = ref<RowList>({ info: { ...defaultInfo }, list: [] });

const handleLoad = async () => {
  isLoading.value = true;

  try {
    const { status, data } = await invoke('api_top_card', { cardId });
    if (status === ApiInvokeStatus.Success) {
      const list: CardInfo[] = data.song_list.map((song, index) => ({
        id: song.mixsongid,
        cover: song.trans_param.union_cover,
        title: song.songname,
        artist: song.author_name,
        musicInfo: {
          id: song.mixsongid,
          hash: song.hash,
          path: null,
          cover: song.trans_param.union_cover,
          title: song.songname,
          artist: song.author_name,
          album: song.album_name,
          duration: song.time_length,
          sort: index,
        },
      }));

      rowData.value.info.title = data.rec_desc.replace(/[「」]/g, '');
      rowData.value.info.count = list.length;
      rowData.value.list = list;
    }
  } finally {
    isLoading.value = false;
  }
};
</script>

<template>
  <RowList :loading="isLoading" :data="rowData" not-more @load="handleLoad" @refresh="handleLoad" />
</template>
