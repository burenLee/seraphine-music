<script lang="ts" setup>
import { useRoute, useRouter } from 'vue-router';

import SvgIcon from '@/components/SvgIcon.vue';
import { IconName } from '@/utils/icons';

interface Menu {
  path: string;
  title: string;
  icon: IconName;
}

const route = useRoute();
const router = useRouter();

const menus: Menu[] = [
  { path: '/home', title: '首页', icon: 'Music' },
  { path: '/local-list-table', title: '本地', icon: 'Download' },
];
</script>

<template>
  <ul class="px-4 pt-8">
    <li
      v-for="(menu, index) in menus"
      :key="index"
      class="flex h-10 cursor-pointer items-center gap-3 rounded-lg px-3 text-base"
      :class="route.path === menu.path ? 'card-actived' : 'card-hover'"
      @click="router.push(menu.path)"
    >
      <SvgIcon :name="menu.icon" />
      {{ menu.title }}
    </li>
  </ul>
</template>
