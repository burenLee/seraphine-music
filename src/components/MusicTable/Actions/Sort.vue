<script lang="ts" setup>
import SelectModal from '@/components/SelectModal.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useListContext } from '@/utils/hooks'
import { SortOrder, SortType } from '@/utils/params'
import { vOnClickOutside } from '@vueuse/components'

const { listStore, listType, list } = useListContext()

const sortVisible = ref(false)
const sortOptions = ref<Array<SelectOption<SortType>>>([
  { label: '默认', value: SortType.Default, prefixIcon: 'Sort', suffixIcon: 'SortUp' },
  { label: '歌名', value: SortType.Title, prefixIcon: 'Title' },
  { label: '歌手', value: SortType.Artist, prefixIcon: 'User' },
  { label: '专辑', value: SortType.Album, prefixIcon: 'Album' },
  { label: '时长', value: SortType.Duration, prefixIcon: 'Timer' }
])

const listSort = computed<SortInfo | undefined>(() => listStore.sortMap[list.value.info.id])
const sortSelection = computed(
  () =>
    sortOptions.value.find((option) => option.value === listSort.value?.type) ||
    sortOptions.value[0]
)

const handleSortSelect = (newSortType: SortType) => {
  const sortInfo = { type: newSortType, order: SortOrder.ASC }

  if (listSort.value?.type === newSortType) {
    sortInfo.order = listSort.value.order === SortOrder.ASC ? SortOrder.DESC : SortOrder.ASC
  }

  listStore.sortMap[list.value.info.id] = sortInfo
  sortVisible.value = false
}

watch(
  listSort,
  (sortInfo) => {
    if (!sortInfo) return

    sortOptions.value = sortOptions.value.map((option) => {
      const icon = sortInfo.order === SortOrder.ASC ? 'SortUp' : 'SortDown'
      return { ...option, suffixIcon: option.value === sortInfo.type ? icon : undefined }
    })

    listStore.handleSort(listType)
  },
  { immediate: true }
)
</script>

<template>
  <div class="relative" v-on-click-outside="() => (sortVisible = false)">
    <SvgIcon
      class="action-icon card-hover transition-colors rounded-lg hover:text-foreground"
      :name="sortSelection.prefixIcon!"
      @click="sortVisible = !sortVisible" />

    <SelectModal
      class="absolute left-1/2 top-full -translate-x-1/2"
      transition="zoom-top"
      :visible="sortVisible"
      :options="sortOptions"
      :selection="sortSelection"
      @select="handleSortSelect" />
  </div>
</template>
