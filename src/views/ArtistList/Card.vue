<script lang="ts" setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';

import ColList from '@/components/MusicList/ColList.vue';
import { InvokeApiStatus, AreaTypes, PageSize } from '@/constants';
import { defaultListInfo } from '@/stores/list';
import { invoke } from '@/utils/tools';

const router = useRouter();

const isLoading = ref(true);
const colData = ref<ColList>({ info: { ...defaultListInfo, title: '歌手排行榜' }, list: [] });

const handleLoad = async () => {
  isLoading.value = true;

  try {
    colData.value.list = await Promise.all(
      AreaTypes.slice(0, 5).map(async (areaType) => {
        const { status, data } = await invoke('api_artist_list', {
          areaType: areaType.type,
          musician: areaType.musician,
          pageSize: PageSize.Min,
        });

        const info: ListInfo = { ...defaultListInfo, title: areaType.title };
        let list: CardInfo[] = [];
        if (status === InvokeApiStatus.Success) {
          list = data.info.map((singer) => ({
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
              url: singer.url,
            },
          }));
        }

        return { info, list };
      }),
    );
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
    @more="router.push('/artist-list-more')"
  />
</template>
