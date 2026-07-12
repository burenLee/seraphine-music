<script lang="ts" setup>
import ActionButton from '../ActionButton.vue'
import SvgIcon from '../SvgIcon.vue'
import Card from './Card.vue'
import { useListStore } from '@/stores/list.ts'
import { useMusicStore } from '@/stores/music.ts'
import { useObserver } from '@/utils/hooks.ts'
import { BreakPoint, ColCount, Interval, ListType } from '@/utils/params'
import { getPlayingOrigin } from '@/utils/tools.ts'
import { useThrottleFn, useWindowSize } from '@vueuse/core'

interface Props {
  /** 列表数据 */
  data: ColList
  /** 列表加载中 */
  loading?: boolean
  /** 列表行数 */
  rows?: number
  /** 不显示更多按钮 */
  notMore?: boolean
}

interface IEmits {
  load: []
  refresh: []
  more: [data: ColList]
}

const { data, loading, rows = 3, notMore } = defineProps<Props>()
const emits = defineEmits<IEmits>()

const musicStore = useMusicStore()
const listStore = useListStore()

const router = useRouter()
const { width: windowWidth } = useWindowSize()

const listRef = useTemplateRef('listRef')

const TotalHeight = 4.825 + 4.5 * rows

const isIntersecting = ref(false)

const cols = computed(() => {
  if (windowWidth.value > BreakPoint.LG) return ColCount.LG
  else if (windowWidth.value > BreakPoint.MD) return ColCount.MD
  else return ColCount.SM
})
const visibleList = computed(() => data.list.slice(0, cols.value))

const handleRefresh = useThrottleFn(() => emits('refresh'), Interval.Sec, true)
const handleMore = () => !notMore && emits('more', data)

const handleClick = (info: CardInfo, index: number) => {
  if (info.musicInfo) {
    const list: ListMusic[] = []
    data.list[index].list.forEach((item) => item.musicInfo && list.push(item.musicInfo))

    listStore.setList(ListType.Play, { info: data.list[index].info, list })
    musicStore.setMusic(info.musicInfo, { origin: getPlayingOrigin(info.musicInfo) })
  } else if (info.artistInfo) {
    const { id, cover, name } = info.artistInfo
    router.push({ path: '/artist-list-table', query: { id, cover, name } })
  } else if (info.playlistInfo) {
    const { id, cover, title } = info.playlistInfo
    router.push({ path: '/top-playlist-table', query: { id, cover, title } })
  }
}

const { unobserve } = useObserver(
  () => listRef.value,
  (entry) => {
    if (!entry.isIntersecting) return

    isIntersecting.value = entry.isIntersecting
    emits('load')
    unobserve()
  }
)
</script>

<template>
  <!-- 未到视口状态 -->
  <div
    v-if="!isIntersecting"
    ref="listRef"
    :style="{ width: '100%', height: `${TotalHeight}rem` }"></div>

  <!-- 加载状态 -->
  <div v-if="loading">
    <div class="flex justify-between">
      <div class="flex h-6 flex-1 items-center gap-3">
        <div class="h-full w-1 rounded-lg bg-minor"></div>
        <div class="h-full w-24 rounded-lg bg-card"></div>
      </div>

      <SvgIcon class="action-icon size-6" name="Refresh" @click="handleRefresh" />
    </div>

    <div class="mt-3 flex gap-3">
      <div v-for="col in cols" :key="col" class="card flex-1 p-2">
        <div class="flex justify-center">
          <div class="h-6 w-24 rounded-lg bg-card"></div>
        </div>

        <div v-for="row in rows" :key="row" class="mt-2 h-16 rounded-lg bg-card" />
      </div>
    </div>
  </div>

  <!-- 无数据状态 -->
  <div
    v-else-if="!data"
    class="flex items-center justify-center card gap-3"
    :style="{ height: `${4.825 + 4.5 * rows}rem` }">
    <div class="font-bold text-xl">无数据或请求失败</div>

    <ActionButton
      class="text-xl px-1 hover:text-info"
      mode="text"
      theme="info"
      suffix-icon="Refresh"
      size="20"
      @click="handleRefresh">
      重试
    </ActionButton>
  </div>

  <!-- 存在数据状态 -->
  <div v-else>
    <div class="flex items-center justify-between gap-3">
      <div class="flex h-6 flex-1 items-center gap-3 overflow-hidden whitespace-nowrap">
        <div class="h-full w-1 rounded bg-minor"></div>
        <div class="font-bold text-base">{{ data.info.title }}</div>

        <div
          class="card border-info text-info text-xs rounded px-1 leading-4 font-bold"
          v-for="(tag, index) in data.info.tags"
          :key="index">
          {{ tag }}
        </div>
      </div>

      <div class="flex h-6 items-center gap-2">
        <SvgIcon class="action-icon size-6" name="Refresh" @click="handleRefresh" />

        <ActionButton
          v-if="!notMore"
          class="px-1 hover:text-minor text-sm h-6"
          mode="text"
          suffix-icon="Right"
          @click="handleMore">
          更多
        </ActionButton>
      </div>
    </div>

    <div class="mt-3 flex gap-3">
      <div v-for="(item, index) in visibleList" :key="index" class="card space-y-2 flex-1 p-2">
        <div class="text-center font-bold truncate text-base">{{ item.info.title }}</div>

        <Card
          v-for="(childItem, childIndex) in item.list.slice(0, rows)"
          :key="childIndex"
          :data="childItem"
          @click="handleClick(childItem, index)" />
      </div>
    </div>
  </div>
</template>
