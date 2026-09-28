<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { watchThrottled } from '@vueuse/core';
import { inject, ref, useTemplateRef } from 'vue';

import SvgIcon from '@/components/SvgIcon.vue';
import { useListStore } from '@/stores/list';
import { Interval, ListType } from '@/utils/params';

const listType = inject<ListType>('listType', ListType.Show);

const { search } = useListStore();

const searchInputRef = useTemplateRef('searchInputRef');

const searchVisible = ref(false);
const searchQuery = ref('');

const toggleSearch = () => {
  // 存在query时不允许收起
  if (searchQuery.value && searchVisible.value) return;

  searchVisible.value = !searchVisible.value;

  // 展开时聚焦搜索框
  if (searchVisible.value) searchInputRef.value?.focus();
};

const clearSearch = () => {
  searchQuery.value = '';

  searchInputRef.value?.focus();
};

watchThrottled(searchQuery, (query) => search(listType, query), { throttle: Interval.Long });
</script>

<template>
  <div class="flex" v-on-click-outside="() => !searchQuery && (searchVisible = false)">
    <input
      ref="searchInputRef"
      class="bg-transparent transition-[width,padding]"
      :class="searchVisible ? 'w-36 px-2' : 'w-0 px-0'"
      placeholder="关键词搜索"
      v-model="searchQuery"
    />

    <SvgIcon
      v-if="searchQuery"
      class="action-icon card-hover rounded-lg transition-colors hover:text-foreground"
      name="Close"
      @click="clearSearch"
    />
    <SvgIcon
      v-else
      class="action-icon card-hover rounded-lg transition-colors hover:text-foreground"
      name="Search"
      @click="toggleSearch"
    />
  </div>
</template>
