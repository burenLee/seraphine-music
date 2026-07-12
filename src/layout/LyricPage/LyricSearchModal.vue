<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue'
import Modal from '@/components/Modal.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useLyricStore } from '@/stores/lyric'
import { useMusicStore } from '@/stores/music'
import { getFullName, invoke } from '@/utils/tools'

const visible = defineModel({ required: true, default: false })

const lyricStore = useLyricStore()
const musicStore = useMusicStore()

const searchQuery = ref('')
const searchLoading = ref(false)
const searchList = ref<LyricCandidate[]>([])
const selectLyric = ref<LyricCandidate>()

const handleSearch = async () => {
  if (!musicStore.music) return

  searchLoading.value = true

  const lyric_search = await invoke('api_lyric_search', {
    keyword: searchQuery.value,
    hash: musicStore.music.hash
  })
  if (lyric_search?.status !== 200) {
    searchList.value = []
    searchLoading.value = false
    return
  }

  searchList.value = lyric_search.candidates
  searchLoading.value = false
}

const handleReset = (refresh = false) => {
  if (!musicStore.music) return

  searchQuery.value = getFullName(musicStore.music, 'at')
  searchList.value = []
  selectLyric.value = searchList.value.find((item) => item.id === lyricStore.lyric?.id)

  if (refresh) handleSearch()
}

const handleCancel = () => {
  visible.value = false

  handleReset()
}

const handleConfirm = () => {
  if (!musicStore.music || !selectLyric.value) return

  lyricStore.load(musicStore.music, selectLyric.value)
}

watch(visible, (visible) => visible && handleReset(true))
</script>

<template>
  <Modal
    v-model="visible"
    class="w-[40rem]"
    title="歌词搜索"
    :maskClosed="false"
    confirmLabel="选择"
    @cancel="handleCancel"
    @confirm="handleConfirm">
    <div class="px-4">
      <div class="flex justify-between items-center gap-3">
        <div class="relative w-full">
          <input
            class="leading-8 border bg-card border-border pl-3 pr-9 rounded-lg w-full"
            placeholder="请输入搜索关键词"
            v-model="searchQuery" />
          <SvgIcon
            v-if="searchQuery"
            class="absolute right-0 top-0 bottom-0 px-3 hover:text-error flex items-center justify-center cursor-pointer"
            name="Close"
            size="12"
            @click="searchQuery = ''" />
        </div>

        <ActionButton theme="success" @click="handleSearch">搜索</ActionButton>
      </div>

      <div class="card border overflow-y-auto p-2 border-border rounded-lg h-64 mt-4">
        <template v-if="searchLoading">
          <div v-for="count in 3" :key="count" class="h-7 my-1 rounded-lg bg-card w-full"></div>
        </template>

        <div
          v-else-if="!searchList.length"
          class="font-bold text-minor text-xl size-full flex items-center justify-center">
          暂无歌词
        </div>

        <template v-else>
          <div
            v-for="(lyric, index) in searchList"
            :key="index"
            class="p-2 rounded-lg flex justify-between cursor-pointer"
            :class="selectLyric?.id === lyric.id ? 'card-actived' : 'card-hover'"
            @click="selectLyric = lyric">
            <div class="flex-1 w-0 truncate pr-2">{{ lyric.singer }} - {{ lyric.song }}</div>
            <div class="w-28 truncate">{{ lyric.product_from }}</div>
            <div class="w-14 truncate">{{ lyric.score }} 分</div>
            <div class="w-8">
              <SvgIcon v-if="lyricStore.lyric?.id === lyric.id" name="Unread" size="20" />
            </div>
          </div>
        </template>
      </div>
    </div>

    <template #actions>
      <ActionButton @click="handleReset(true)">重置</ActionButton>
    </template>
  </Modal>
</template>
