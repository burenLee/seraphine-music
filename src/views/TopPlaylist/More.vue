<script lang="ts" setup>
import { onMounted, ref, useTemplateRef } from 'vue';
import { useRouter } from 'vue-router';

import Image from '@/components/Image.vue';
import { notify } from '@/components/Notification.vue';
import ToTop from '@/components/PageActions/ToTop.vue';
import SlideBar from '@/components/SlideBar.vue';
import VirtualList from '@/components/VirtualList.vue';
import { getPic } from '@/utils/music';
import { ApiInvokeStatus, PageSize } from '@/utils/params';
import { formatCount, invoke } from '@/utils/tools';

const router = useRouter();

const tableColumns: TableColumn[] = [
  { key: 'index', slot: true, width: '3rem', padding: 0, align: 'center' },
  { key: 'info', slot: true, width: 'auto' },
  { key: 'playCount', slot: true, width: '12rem', align: 'center' },
];
const musicTableRef = useTemplateRef('musicTableRef');

const slideOptions = ref<SlideOption[]>([]);
const firstSelection = ref<SlideOption>(); // 第一级选中项
const secondSelection = ref<SlideOption>(); // 第二级选中项
const isLoading = ref(true);
const isFinishing = ref(false);
const isFinished = ref(false);
const page = ref(1);
const playlistList = ref<ListInfo[]>([]);

const getSlideOptions = async () => {
  try {
    const { status, data } = await invoke('api_playlist_tags');
    if (status !== ApiInvokeStatus.Success) {
      notify.error('获取歌单标签列表失败');
      return;
    }

    slideOptions.value = data.map((item) => ({
      label: item.tag_name,
      value: item.tag_id,
      children: item.son.slice(0, 10).map((son) => ({ label: son.tag_name, value: son.tag_id })),
    }));

    firstSelection.value = slideOptions.value[0];
    secondSelection.value = firstSelection.value.children?.[0];
  } catch {
    notify.error('获取歌单标签列表失败');
  }
};

const changeSlideSelection = (option: SlideOption, type: 'first' | 'second') => {
  switch (type) {
    case 'first':
      if (firstSelection.value?.value === option.value) return;

      firstSelection.value = option;
      secondSelection.value = firstSelection.value.children?.[0];
      break;
    case 'second':
      if (secondSelection.value?.value === option.value) return;

      secondSelection.value = option;
      break;
  }

  page.value = 1;
  handleLoad();
  handleToTop();
};

const handleLoad = async () => {
  if (!secondSelection.value) return;

  isLoading.value = true;

  try {
    const { status, data } = await invoke('api_top_playlist', {
      categoryId: secondSelection.value.value,
      page: page.value,
      pageSize: PageSize.More,
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('获取歌单歌曲失败');
    } else {
      playlistList.value = data.special_list.map((playlist) => ({
        id: playlist.specialid,
        cover: playlist.flexible_cover,
        title: playlist.specialname,
        artist: playlist.nickname,
        count: 0,
        playCount: playlist.play_count,
        tags: playlist.abtags?.map((tag) => tag.name),
        gid: playlist.global_collection_id,
      }));
    }
  } catch {
    notify.error('获取歌单歌曲失败');
  } finally {
    isLoading.value = false;
  }
};

const handleInfinite = async () => {
  if (isFinishing.value || isFinished.value || !secondSelection.value) return;

  isFinishing.value = true;

  try {
    const { status, data } = await invoke('api_top_playlist', {
      categoryId: secondSelection.value.value,
      page: ++page.value,
      pageSize: PageSize.More,
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('获取歌单歌曲失败');
    } else {
      const list = data.special_list.map((playlist) => ({
        id: playlist.specialid,
        cover: playlist.flexible_cover,
        title: playlist.specialname,
        artist: playlist.nickname,
        count: 0,
        playCount: playlist.play_count,
        tags: playlist.abtags?.map((tag) => tag.name),
        gid: playlist.global_collection_id,
      }));

      playlistList.value.push(...list);
      if (data.special_list.length < PageSize.More) isFinished.value = true;
    }
  } catch {
    notify.error('获取歌单歌曲失败');
  } finally {
    isFinishing.value = false;
  }
};

const handleLineClick = (row: ListInfo) => {
  if (!row.gid) return;

  router.push(`/top-playlist-table/${row.gid}`);
};

const handleToTop = () => {
  musicTableRef.value?.scrollToTop();
};

onMounted(async () => {
  await getSlideOptions();
  await handleLoad();
});
</script>

<template>
  <div class="relative flex h-full w-full flex-col space-y-3 pt-4">
    <!-- 第一级菜单 -->
    <div class="mx-8 flex items-center gap-3">
      <div class="font-bold">歌单:</div>
      <SlideBar
        class="flex-1"
        :slideWidth="64"
        :options="slideOptions"
        :selection="firstSelection"
        @change="changeSlideSelection($event, 'first')"
      />
    </div>

    <!-- 第二级菜单 -->
    <div class="mx-8 flex items-center gap-3">
      <div class="pointer-events-none font-bold opacity-0">歌单:</div>
      <SlideBar
        class="flex-1"
        :slideWidth="64"
        :options="firstSelection?.children || []"
        :selection="secondSelection"
        @change="changeSlideSelection($event, 'second')"
      />
    </div>

    <VirtualList
      ref="musicTableRef"
      class="h-0 w-full flex-1 pl-8 pr-[1.625rem]"
      :columns="tableColumns"
      :loading="isLoading"
      :list="playlistList"
      @infinite="handleInfinite"
      @lineClick="handleLineClick"
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

      <template #playCount="row"> {{ formatCount(row.playCount) }}</template>
    </VirtualList>

    <div class="absolute bottom-4 right-4">
      <ToTop v-if="playlistList.length" @click="handleToTop" />
    </div>
  </div>
</template>
