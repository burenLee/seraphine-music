<script lang="ts" setup>
import { onMounted, onUnmounted, provide, ref, useTemplateRef } from 'vue';

import MusicActions from '@/components/MusicTable/MusicActions.vue';
import MusicTable from '@/components/MusicTable/MusicTable.vue';
import { notify } from '@/components/Notification.vue';
import SlideBar from '@/components/SlideBar.vue';
import { defaultInfo, useListStore } from '@/stores/list';
import { getPrivilegeTags } from '@/utils/music';
import { ApiInvokeStatus, ListType, PageSize } from '@/utils/params';
import { invoke } from '@/utils/tools';

const listType = ListType.Show;
provide('listType', listType);

const listStore = useListStore();

const musicTableRef = useTemplateRef('musicTableRef');

const slideOptions = ref<SlideOption[]>([]);
const slideSelection = ref<SlideOption>();

const getSlideOptions = async () => {
  try {
    const { status, data } = await invoke('api_rank_top');
    if (status !== ApiInvokeStatus.Success) return;

    slideOptions.value = data.list.map((item) => ({
      label: item.rankname,
      value: item.rankid,
      imgurl: item.imgurl,
      intro: item.intro,
    }));
    slideSelection.value = slideOptions.value[0];
  } catch {
    notify.error('获取排行榜歌曲标签失败');
  }
};

const changeSlideSelection = (option: SlideOption) => {
  if (slideSelection.value?.value === option.value) return;

  slideSelection.value = option;
  handleLoad();
  musicTableRef.value?.handleToTop();
};

const handleLoad = async () => {
  if (!slideSelection.value) return;

  listStore.isTableLoading = true;

  try {
    const { status, data } = await invoke('api_rank_audio', {
      rankId: slideSelection.value.value,
      pageSize: PageSize.Max,
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('获取排行榜歌曲失败');
    } else {
      const info: ListInfo = {
        ...defaultInfo,
        id: slideSelection.value.value,
        cover: slideSelection.value.imgurl,
        title: slideSelection.value.label,
        tags: slideSelection.value.intro.split('\r\n'),
        count: data.songlist.length,
      };
      const list: MusicInfo[] = data.songlist.map((song) => ({
        id: song.album_audio_id,
        path: null,
        hash: song.audio_info.hash_128,
        cover: song.trans_param.union_cover,
        title: song.songname,
        artist: song.author_name,
        album: song.album_info.album_name,
        duration: song.audio_info.duration_128 / 1000,
        sort: song.business.original_index,
        privilegeTags: getPrivilegeTags(
          song.privilege_download.privilege,
          song.deprecated.pay_type,
        ),
      }));

      listStore.setList(listType, { info, list });
    }
  } catch {
    notify.error('获取排行榜歌曲失败');
  } finally {
    listStore.isTableLoading = false;
  }
};

onMounted(async () => {
  await getSlideOptions();
  await handleLoad();
});

onUnmounted(() => listStore.resetList(listType));
</script>

<template>
  <div class="relative flex h-full w-full flex-col space-y-3 pt-4">
    <div class="mx-8 flex items-center gap-3">
      <div class="text-xl font-bold">排行榜歌曲:</div>
      <SlideBar
        class="flex-1"
        :slide-width="88"
        :options="slideOptions"
        :selection="slideSelection"
        @change="changeSlideSelection"
      />
    </div>

    <MusicActions />
    <MusicTable ref="musicTableRef" class="h-0 flex-1" />
  </div>
</template>
