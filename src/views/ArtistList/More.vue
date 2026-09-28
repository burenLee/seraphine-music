<script lang="ts" setup>
import { onMounted, ref, useTemplateRef } from 'vue';
import { useRouter } from 'vue-router';

import ActionButton from '@/components/ActionButton.vue';
import Image from '@/components/Image.vue';
import { notify } from '@/components/Notification.vue';
import ToTop from '@/components/PageActions/ToTop.vue';
import SlideBar from '@/components/SlideBar.vue';
import VirtualList from '@/components/VirtualList.vue';
import { getPic } from '@/utils/music';
import { ApiInvokeStatus, AreaTypes, PageSize, SexTypes } from '@/utils/params';
import { formatCount, invoke } from '@/utils/tools';

const router = useRouter();

const tableColumns: TableColumn[] = [
  { key: 'index', slot: true, width: '3rem', padding: 0, align: 'center' },
  { key: 'info', slot: true, width: 'auto' },
  { key: 'fanscount', slot: true, width: 'auto', align: 'center' },
  { key: 'qa', slot: true, width: 'auto' },
];
const musicTableRef = useTemplateRef('musicTableRef');

const areaOptions = ref<SlideOption[]>([]); // 区域菜单项
const areaSelection = ref<SlideOption>(); // 区域选中项
const sexOptions = ref<SlideOption[]>([]); // 性别菜单项
const sexSelection = ref<SlideOption>(); // 性别选中项
const isLoading = ref(true);
const isFinishing = ref(false);
const isFinished = ref(false);
const page = ref(1);
const artistList = ref<ArtistInfo[]>([]);

const getSlideOptions = () => {
  areaOptions.value = AreaTypes.map((areaType) => ({
    label: areaType.title,
    value: areaType.type,
    musician: areaType.musician,
  }));
  areaSelection.value = areaOptions.value[0];

  sexOptions.value = SexTypes.map((sexType) => ({ label: sexType.value, value: sexType.key }));
  sexSelection.value = sexOptions.value[0];
};

const changeSlideSelection = (option: SlideOption, mode: 'area' | 'sex') => {
  switch (mode) {
    case 'area':
      if (areaSelection.value?.value === option.value) return;

      areaSelection.value = option;
      break;
    case 'sex':
      if (sexSelection.value?.value === option.value) return;

      sexSelection.value = option;
      break;
  }

  page.value = 1;
  handleLoad();
  handleToTop();
};

const handleLoad = async () => {
  // 不使用isLoading是因为初始值为true
  // if (isLoading.value) return

  if (!areaSelection.value || !sexSelection.value) return;

  isLoading.value = true;

  try {
    const { status, data } = await invoke('api_artist_list', {
      areaType: areaSelection.value.value,
      musician: areaSelection.value.musician,
      sexType: sexSelection.value.value,
      page: page.value,
      pageSize: PageSize.More,
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('获取歌手列表失败');
    } else {
      artistList.value = data.info.map((artist) => ({
        id: artist.singerid,
        cover: artist.imgurl,
        name: artist.singername,
        fanscount: artist.fanscount,
        descibe: artist.descibe,
        url: artist.url,
      }));
    }
  } catch {
    notify.error('获取歌手列表失败');
  } finally {
    isLoading.value = false;
  }
};

const handleInfinite = async () => {
  if (isFinishing.value || isFinished.value || !areaSelection.value || !sexSelection.value) return;

  isFinishing.value = true;

  try {
    const { status, data } = await invoke('api_artist_list', {
      areaType: areaSelection.value.value,
      musician: areaSelection.value.musician,
      sexType: sexSelection.value.value,
      page: ++page.value,
      pageSize: PageSize.More,
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('获取歌手列表失败');
    } else {
      data.info.forEach((artist) => {
        artistList.value.push({
          id: artist.singerid,
          cover: artist.imgurl,
          name: artist.singername,
          fanscount: artist.fanscount,
          descibe: artist.descibe,
          url: artist.url,
        });
      });

      if (data.info.length < PageSize.More) isFinished.value = true;
    }
  } catch {
    notify.error('获取歌手列表失败');
  } finally {
    isFinishing.value = false;
  }
};

const handleLineClick = (row: ArtistInfo) => {
  router.push(`/artist-list-table/${row.id}`);
};

const handleToTop = () => {
  musicTableRef.value?.scrollToTop();
};

onMounted(() => {
  getSlideOptions();
  handleLoad();
});
</script>

<template>
  <div class="relative flex h-full w-full flex-col space-y-3 pt-4">
    <!-- 区域菜单项 -->
    <div class="mx-8 flex items-center gap-3">
      <div class="text-xl font-bold">歌手:</div>
      <SlideBar
        class="flex-1"
        :slideWidth="64"
        :options="areaOptions"
        :selection="areaSelection"
        @change="changeSlideSelection($event, 'area')"
      />
    </div>

    <!-- 性别菜单项 -->
    <div class="mx-8 flex items-center gap-3">
      <div class="pointer-events-none text-xl font-bold opacity-0">歌手:</div>
      <SlideBar
        class="flex-1"
        :slideWidth="64"
        :options="sexOptions"
        :selection="sexSelection"
        @change="changeSlideSelection($event, 'sex')"
      />
    </div>

    <VirtualList
      ref="musicTableRef"
      class="h-0 w-full flex-1 pl-8 pr-[1.625rem]"
      :columns="tableColumns"
      :loading="isLoading"
      :list="artistList"
      @infinite="handleInfinite"
      @lineClick="handleLineClick"
    >
      <template #index="row">
        {{ row.index + 1 }}
      </template>

      <template #info="row">
        <div class="flex items-center gap-3">
          <Image class="size-12" :src="getPic(row.cover)" />

          <div class="w-0 flex-1 truncate font-bold">{{ row.name }}</div>
        </div>
      </template>

      <template #fanscount="row"> {{ formatCount(row.fanscount) }} 粉丝</template>

      <template #qa="row">
        <div v-if="row.descibe" class="flex justify-end" :data-disabled="true">
          <ActionButton class="text-sm" mode="text" theme="info" suffixIcon="Right" @click.stop>
            {{ row.descibe }}
          </ActionButton>
        </div>
      </template>
    </VirtualList>

    <div class="absolute bottom-4 right-4">
      <ToTop @click="handleToTop" />
    </div>
  </div>
</template>
