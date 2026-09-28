<script lang="ts" setup>
import { convertFileSrc } from '@tauri-apps/api/core';
import { computed, inject } from 'vue';

import Image from '@/components/Image.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useListStore } from '@/stores/list';
import { getPic } from '@/utils/music';
import { ListType } from '@/utils/params';

const listType = inject<ListType>('listType', ListType.Show);

const listStore = useListStore();

const musicList = computed(() => listStore[listType]);
const cover = computed(() => {
  if (!musicList.value.info.cover) return '';

  return listType === ListType.Local
    ? convertFileSrc(musicList.value.info.cover, 'md')
    : getPic(musicList.value.info.cover);
});
</script>

<template>
  <div class="flex w-full gap-3 px-8">
    <!-- 加载状态 -->
    <template v-if="listStore.isHeaderLoading">
      <div class="size-16 rounded-lg bg-card"></div>
      <div class="py-1">
        <div class="h-6 w-32 rounded-md bg-card"></div>
        <div class="mt-2 h-6 w-24 rounded-md bg-card"></div>
      </div>
    </template>

    <!-- 无数据状态 -->
    <template v-else-if="!musicList.info">
      <SvgIcon class="card size-16" name="Music" :size="32" />
      <div class="text-xl font-bold leading-8">播放列表</div>
    </template>

    <!-- 有数据状态 -->
    <template v-else>
      <Image class="size-16" :src="cover" :icon-size="32" />

      <div class="flex-1">
        <div class="flex h-8 items-baseline">
          <div class="truncate text-xl font-bold leading-8">
            {{ musicList.info.title || '播放列表' }}
          </div>
          <div class="ml-4 flex-1">共 {{ musicList.info.count || 0 }} 首</div>
        </div>

        <div class="flex h-8 items-center gap-2 whitespace-nowrap">
          <div class="font-bold leading-8">{{ musicList.info.artist }}</div>

          <div
            class="card rounded border-info px-1 text-xs font-bold leading-4 text-info"
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
