<script lang="ts" setup>
import { convertFileSrc } from '@tauri-apps/api/core';
import { computed, inject } from 'vue';

import Image from '@/components/Image.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { ListType, PicSize } from '@/constants';
import { useListStore } from '@/stores/list';
import { getPic } from '@/utils/music';

const listType = inject<ListType>('listType', ListType.Show);

const listStore = useListStore();

const musicList = computed(() => listStore[listType]);
const cover = computed(() => {
  if (!musicList.value.info.cover) return '';

  return listType === ListType.Local
    ? convertFileSrc(musicList.value.info.cover)
    : getPic(musicList.value.info.cover, PicSize.Md);
});
</script>

<template>
  <div class="flex w-full gap-3 px-8">
    <!-- 加载状态 -->
    <template v-if="listStore.isHeaderLoading">
      <div class="size-14 rounded-lg bg-card"></div>
      <div class="space-y-2 py-1">
        <div class="h-6 w-32 rounded-md bg-card"></div>
        <div class="h-4 w-24 rounded-md bg-card"></div>
      </div>
    </template>

    <!-- 无数据状态 -->
    <template v-else-if="!musicList.info">
      <div class="card flex size-14 items-center justify-center">
        <SvgIcon name="Music" :size="32" />
      </div>
      <div class="text-xl font-bold leading-8">播放列表</div>
    </template>

    <!-- 有数据状态 -->
    <template v-else>
      <Image class="size-14" :src="cover" icon-size="28" />

      <div class="flex-1">
        <div class="flex items-baseline gap-3">
          <div class="truncate text-xl font-bold leading-7">
            {{ musicList.info.title || '播放列表' }}
          </div>
          <div class="flex-1">共 {{ musicList.info.count || 0 }} 首</div>
        </div>

        <div class="flex items-center gap-3">
          <div class="whitespace-nowrap font-bold leading-7">{{ musicList.info.artist }}</div>

          <div
            class="card whitespace-nowrap rounded border-info px-1 text-xs font-bold leading-4 text-info"
            v-for="(tag, index) in musicList.info.tags"
            :key="index"
          >
            {{ tag }}
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
