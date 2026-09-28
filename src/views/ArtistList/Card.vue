<script lang="ts" setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';

import ColList from '@/components/MusicList/ColList.vue';
import { defaultInfo } from '@/stores/list';
import { ApiInvokeStatus, AreaTypes, PageSize } from '@/utils/params';
import { invoke } from '@/utils/tools';

const router = useRouter();

const isLoading = ref(true);
const colData = ref<ColList>({ info: { ...defaultInfo, title: '推荐歌手' }, list: [] });

const handleLoad = async () => {
  // 不使用isLoading是因为初始值为true
  // if (isLoading.value) return

  isLoading.value = true;

  try {
    colData.value.list = await Promise.all(
      AreaTypes.slice(0, 5).map(async (areaType) => {
        const { status, data } = await invoke('api_artist_list', {
          areaType: areaType.type,
          musician: areaType.musician,
          pageSize: PageSize.Min,
        });

        const info: ListInfo = { ...defaultInfo, title: areaType.title };
        let list: CardInfo[] = [];
        if (status === ApiInvokeStatus.Success) {
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
