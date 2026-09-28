<script lang="ts" setup>
import { useThrottleFn, useWindowSize } from '@vueuse/core';
import { computed, ref, useTemplateRef } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useObserver } from '@/utils/hooks';
import { BreakPoint, ColCount, Interval } from '@/utils/params';

import Card from './Card.vue';

interface Props {
  /** 列表加载中 */
  loading?: boolean;
  /** 列表数据 */
  data: RowList;
  /** 列表行数 */
  rows?: number;
  /** 不显示更多按钮 */
  notMore?: boolean;
}

interface IEmits {
  load: [];
  refresh: [];
  more: [data: RowList];
}

const { data, loading, rows = 3, notMore } = defineProps<Props>();
const emits = defineEmits<IEmits>();

const { width: windowWidth } = useWindowSize();
const listRef = useTemplateRef('listRef');

const TotalHeight = 1.5 + 4.75 * rows;

const isIntersecting = ref(false);

// 可见列数
const cols = computed(() => {
  if (windowWidth.value > BreakPoint.LG) return ColCount.LG;
  else if (windowWidth.value > BreakPoint.MD) return ColCount.MD;
  else return ColCount.SM;
});
// 可见数据数
const visibleList = computed(() => data.list.slice(0, rows * cols.value));

const handleRefresh = useThrottleFn(() => emits('refresh'), Interval.Sec, true);

const handleMore = () => {
  if (notMore) return;
  emits('more', data);
};

const { unobserve } = useObserver(listRef, (entry) => {
  if (!entry.isIntersecting) return;

  isIntersecting.value = true;
  emits('load');
  unobserve();
});
</script>

<template>
  <!-- 未到视口状态 -->
  <div
    v-if="!isIntersecting"
    ref="listRef"
    :style="{ width: '100%', height: `${TotalHeight}rem` }"
  ></div>

  <!-- 加载状态 -->
  <div v-else-if="loading">
    <div class="flex justify-between">
      <div class="flex h-6 flex-1 items-center gap-3">
        <div class="h-full w-1 rounded bg-minor"></div>
        <div class="h-full w-24 rounded bg-card"></div>
      </div>

      <SvgIcon class="action-icon size-6" name="Refresh" @click="handleRefresh" />
    </div>

    <div class="mt-3 grid gap-3" :style="{ gridTemplateColumns: `repeat(${cols}, 1fr)` }">
      <div v-for="index in rows * cols" :key="index" class="h-16 rounded-lg bg-card"></div>
    </div>
  </div>

  <!-- 无数据状态 -->
  <div
    v-else-if="!data"
    class="card flex items-center justify-center gap-3 font-bold"
    :style="{ height: `${TotalHeight}rem` }"
  >
    <div class="text-xl font-bold">无数据或请求失败</div>

    <ActionButton
      class="px-1 text-xl hover:text-info"
      mode="text"
      theme="info"
      suffix-icon="Refresh"
      size="20"
      @click="handleRefresh"
    >
      重试
    </ActionButton>
  </div>

  <!-- 存在数据状态 -->
  <div v-else>
    <div class="flex items-center justify-between gap-3">
      <div class="flex h-6 flex-1 items-center gap-3 overflow-hidden whitespace-nowrap">
        <div class="h-full w-1 rounded bg-minor"></div>
        <div class="text-base font-bold">{{ data.info.title }}</div>

        <div
          class="card rounded border-info px-1 text-xs font-bold leading-4 text-info"
          v-for="(tag, index) in data.info.tags"
          :key="index"
        >
          {{ tag }}
        </div>
      </div>

      <div class="flex h-6 items-center gap-2">
        <SvgIcon class="action-icon size-6" name="Refresh" @click="handleRefresh" />

        <ActionButton
          v-if="!notMore"
          class="h-6 px-1 text-sm hover:text-minor"
          mode="text"
          suffix-icon="Right"
          @click="handleMore"
        >
          更多
        </ActionButton>
      </div>
    </div>

    <div class="mt-3 grid gap-3" :style="{ gridTemplateColumns: `repeat(${cols}, 1fr)` }">
      <Card
        v-for="item in visibleList"
        :key="item.id"
        :data="item"
        :info="data.info"
        :list="data.list"
      />
    </div>
  </div>
</template>
