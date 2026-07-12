<script lang="ts" setup>
import SvgIcon from '@/components/SvgIcon.vue'
import { IconName } from '@/utils/icons'

interface Menu {
  path: string
  title: string
  icon: IconName
}

const route = useRoute()
const router = useRouter()

const menus: Menu[] = [
  { path: '/home', title: '首页', icon: 'Music' },
  { path: '/local-list-table', title: '本地', icon: 'Download' }
]
</script>

<template>
  <ul class="p-4">
    <li
      v-for="(menu, index) in menus"
      :key="index"
      class="flex cursor-pointer items-center text-base gap-2 rounded-lg px-3 py-2 transition-colors"
      :class="route.path === menu.path ? 'card-actived' : 'card-hover'"
      @click="router.push(menu.path)">
      <SvgIcon :name="menu.icon" />
      <div>{{ menu.title }}</div>
    </li>
  </ul>
</template>
