<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue'
import Image from '@/components/Image.vue'
import ToTop from '@/components/PageActions/ToTop.vue'
import SlideBar from '@/components/SlideBar.vue'
import VirtualList from '@/components/VirtualList.vue'
import { AreaTypes, PageSize, SexTypes } from '@/utils/params'
import { getPic, invoke } from '@/utils/tools'

const router = useRouter()

const tableColumns: TableColumn[] = [
  { key: 'index', slot: true, width: '3rem', padding: 0, align: 'center' },
  { key: 'info', width: 'auto', slot: true },
  { key: 'fanscount', slot: true, width: 'auto', align: 'center' },
  { key: 'qa', slot: true, width: 'auto' }
]
const musicTableRef = useTemplateRef('musicTableRef')

const areaSlideOptions = ref<SlideOption[]>([]) // 区域菜单项
const areaSlideSelection = ref<SlideOption>() // 区域选中项
const sexSlideOptions = ref<SlideOption[]>([]) // 性别菜单项
const sexSlideSelection = ref<SlideOption>() // 性别选中项
const isLoading = ref(true)
const isFinished = ref(false)
const page = ref(1)
const artistList = ref<ArtistInfo[]>([])

const handleSlideGet = async () => {
  areaSlideOptions.value = AreaTypes.map((areaType) => ({
    label: areaType.title,
    value: areaType.type,
    musician: areaType.musician
  }))
  areaSlideSelection.value = areaSlideOptions.value[0]

  sexSlideOptions.value = SexTypes.map((sexType) => ({ label: sexType.value, value: sexType.key }))
  sexSlideSelection.value = sexSlideOptions.value[0]
}

const handleSlideChange = (option: SlideOption, type: 'area' | 'sex') => {
  switch (type) {
    case 'area':
      areaSlideSelection.value = option
      break
    case 'sex':
      sexSlideSelection.value = option
      break
  }

  page.value = 1
  handleToTop()
  handleLoad()
}

const handleLoad = async () => {
  if (!areaSlideSelection.value || !sexSlideSelection.value) return
  isLoading.value = true

  const api_artist_list = await invoke('api_artist_list', {
    areaType: areaSlideSelection.value.value,
    musician: areaSlideSelection.value.musician,
    sexType: sexSlideSelection.value.value,
    page: page.value,
    pageSize: PageSize.Default
  })
  if (api_artist_list?.status !== 1) {
    isLoading.value = false
    return
  }

  const list = api_artist_list.data.info.map((artist) => ({
    id: artist.singerid,
    cover: artist.imgurl,
    name: artist.singername,
    fanscount: artist.fanscount,
    descibe: artist.descibe,
    url: artist.url
  }))

  artistList.value = list
  isLoading.value = false
}

const handleInfinite = async () => {
  if (isFinished.value || !areaSlideSelection.value || !sexSlideSelection.value) return

  const api_artist_list = await invoke('api_artist_list', {
    areaType: areaSlideSelection.value.value,
    musician: areaSlideSelection.value.musician,
    sexType: sexSlideSelection.value.value,
    page: ++page.value,
    pageSize: PageSize.Default
  })
  if (api_artist_list?.status !== 1) return

  const list = api_artist_list.data.info.map((artist) => ({
    id: artist.singerid,
    cover: artist.imgurl,
    name: artist.singername,
    fanscount: artist.fanscount,
    descibe: artist.descibe,
    url: artist.url
  }))

  if (list.length < PageSize.Default) isFinished.value = true

  artistList.value.push(...list)
}

const handleLineClick = (row: ArtistInfo) => {
  router.push({
    path: '/artist-list-table',
    query: { id: row.id, cover: row.cover, name: row.name }
  })
}

const handleToTop = () => {
  musicTableRef.value?.scrollToTop()
}

onMounted(async () => {
  await handleSlideGet()
  await handleLoad()
})
</script>

<template>
  <div class="relative space-y-3 pt-4 w-full h-full flex flex-col">
    <!-- 区域菜单项 -->
    <div class="flex items-center gap-3 mx-8">
      <div class="font-bold text-xl">歌手:</div>
      <SlideBar
        class="flex-1"
        :slideWidth="64"
        :options="areaSlideOptions"
        :selection="areaSlideSelection"
        @change="handleSlideChange($event, 'area')" />
    </div>

    <!-- 性别菜单项 -->
    <div class="flex items-center gap-3 mx-8">
      <div class="font-bold text-xl opacity-0 pointer-events-none">歌手:</div>
      <SlideBar
        class="flex-1"
        :slideWidth="64"
        :options="sexSlideOptions"
        :selection="sexSlideSelection"
        @change="handleSlideChange($event, 'sex')" />
    </div>

    <VirtualList
      ref="musicTableRef"
      class="h-0 w-full flex-1 pl-8 pr-[1.625rem]"
      :columns="tableColumns"
      :loading="isLoading"
      :list="artistList"
      @infinite="handleInfinite"
      @lineClick="handleLineClick">
      <template #index="row">
        {{ row.index + 1 }}
      </template>

      <template #info="row">
        <div class="flex items-center gap-3">
          <Image class="size-12" :img="getPic(row.cover)" />

          <div class="w-0 flex-1 truncate font-bold">{{ row.name }}</div>
        </div>
      </template>

      <template #fanscount="row">{{ (row.fanscount / 1000).toFixed(1) }}万粉丝</template>

      <template #qa="row">
        <div v-if="row.descibe" class="flex justify-end">
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
