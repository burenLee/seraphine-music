<script lang="ts" setup>
import { ref, watch } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Modal from '@/components/Modal.vue';
import { notify } from '@/components/Notification.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useLyricStore } from '@/stores/lyric';
import { useMusicStore } from '@/stores/music';
import { getFullName } from '@/utils/music';
import { invoke } from '@/utils/tools';

const visible = defineModel({ required: true, default: false });

const lyricStore = useLyricStore();
const musicStore = useMusicStore();

const searchQuery = ref('');
const searchLoading = ref(false);
const searchList = ref<ApiLyricCandidate[]>([]);
const selectLyric = ref<ApiLyricCandidate>();

const handleSearch = async () => {
  if (searchLoading.value || !musicStore.music) return;
  searchLoading.value = true;

  try {
    const { status, candidates } = await invoke('api_lyric_search', {
      keyword: searchQuery.value,
      hash: musicStore.music.hash,
    });
    if (status !== 200) {
      notify.error('获取歌词列表失败');
      searchList.value.length = 0;
    } else {
      searchList.value = candidates;
    }
  } catch {
    notify.error('获取歌词列表失败');
  } finally {
    searchLoading.value = false;
  }
};

const handleReset = (refresh = false) => {
  if (!musicStore.music) return;

  searchQuery.value = getFullName(musicStore.music, 'at');
  searchList.value.length = 0;
  selectLyric.value = searchList.value.find((item) => item.id === lyricStore.lyric?.id);

  if (refresh) handleSearch();
};

const handleCancel = () => {
  visible.value = false;

  handleReset();
};

const handleConfirm = () => {
  if (!musicStore.music || !selectLyric.value) return;

  lyricStore.load(musicStore.music, selectLyric.value);
};

watch(visible, (visible) => visible && handleReset(true));
</script>

<template>
  <Modal
    v-model="visible"
    class="w-[40rem]"
    title="歌词搜索"
    :maskClosed="false"
    confirmLabel="选择"
    @cancel="handleCancel"
    @confirm="handleConfirm"
  >
    <div class="px-6">
      <div class="flex items-center justify-between gap-3">
        <div class="relative w-full">
          <input
            class="w-full rounded-lg border border-border bg-card pl-3 pr-9 leading-8"
            placeholder="请输入搜索关键词"
            v-model="searchQuery"
          />
          <SvgIcon
            v-if="searchQuery"
            class="absolute bottom-0 right-0 top-0 flex cursor-pointer items-center justify-center px-3 hover:text-error"
            name="Close"
            @click="searchQuery = ''"
          />
        </div>

        <ActionButton theme="success" @click="handleSearch">搜索</ActionButton>
      </div>

      <div class="card mt-4 h-64 overflow-y-auto rounded-lg border border-border p-2">
        <template v-if="searchLoading">
          <div v-for="count in 3" :key="count" class="my-1 h-7 w-full rounded-lg bg-card"></div>
        </template>

        <div
          v-else-if="!searchList.length"
          class="flex size-full items-center justify-center text-xl font-bold text-minor"
        >
          暂无歌词
        </div>

        <template v-else>
          <div
            v-for="(lyric, index) in searchList"
            :key="index"
            class="flex cursor-pointer justify-between rounded-lg p-2"
            :class="selectLyric?.id === lyric.id ? 'card-actived' : 'card-hover'"
            @click="selectLyric = lyric"
          >
            <div class="w-0 flex-1 truncate pr-2">{{ lyric.singer }} - {{ lyric.song }}</div>
            <div class="w-28 truncate">{{ lyric.product_from }}</div>
            <div class="w-14 truncate">{{ lyric.score }} 分</div>
            <div class="w-8">
              <SvgIcon v-if="lyricStore.lyric?.id === lyric.id" name="Unread" />
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
