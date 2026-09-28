<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { watchThrottled } from '@vueuse/core';
import { ref, useTemplateRef } from 'vue';
import { useRouter } from 'vue-router';

import { notify } from '@/components/Notification.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useUserStore } from '@/stores/user';
import { getFullName } from '@/utils/music';
import { ApiInvokeStatus, Interval, SearchType } from '@/utils/params';
import { invoke } from '@/utils/tools';

const userStore = useUserStore();

const router = useRouter();
const searchRef = useTemplateRef('searchInputRef');

const isSearching = ref(false);
const searchQuery = ref('');
const searchVisible = ref(false);
const searchResult = ref<{
  [SearchType.Song]: MusicInfo[];
  [SearchType.Author]: ArtistInfo[];
  [SearchType.Collect]: ListInfo[];
}>();

const handleSearch = async (searchQuery: string) => {
  const query = searchQuery.trim();

  if (query) {
    isSearching.value = true;

    try {
      const { status, data } = await invoke('api_search_complex', { keywords: query, pageSize: 5 });
      if (status !== ApiInvokeStatus.Success) {
        notify.error('搜索失败');
      } else {
        searchVisible.value = true;
        data.lists.forEach((item) => {
          if (!searchResult.value) searchResult.value = { song: [], author: [], collect: [] };

          switch (item.type) {
            case SearchType.Song:
              searchResult.value[SearchType.Song] = item.lists.map((song, index) => {
                const artist = Array.isArray(song.Singers)
                  ? song.Singers.map((item: any) => item.name).join('、')
                  : song.Singers;

                return {
                  id: song.Audioid,
                  hash: song.FileHash,
                  path: null,
                  cover: song.trans_param.union_cover,
                  title: song.SongName,
                  artist: artist,
                  album: song.AlbumName,
                  duration: song.Duration,
                  sort: index,
                };
              });
              break;
            case SearchType.Author:
              searchResult.value[SearchType.Author] = item.lists.map((artist) => ({
                id: artist.AuthorId,
                cover: artist.Avatar,
                name: artist.AuthorName,
                fanscount: artist.FansNum,
                descibe: '',
                url: '',
              }));
              break;
            case SearchType.Collect:
              searchResult.value[SearchType.Collect] = item.lists.map((playlist) => ({
                id: playlist.gid,
                cover: playlist.img,
                title: playlist.specialname,
                artist: playlist.nickname,
                count: 0,
                playCount: playlist.play_count,
              }));
              break;
          }
        });
      }
    } catch {
      notify.error('搜索失败');
    } finally {
      isSearching.value = false;
    }
  } else {
    searchVisible.value = false;
    searchResult.value = undefined;
    isSearching.value = false;
  }
};

const handleClear = () => {
  searchQuery.value = '';
  searchResult.value = undefined;
  searchRef.value?.focus();
};

const handleEnter = () => {
  if (!searchQuery.value) return;

  searchVisible.value = false;
  searchRef.value?.blur();
  router.push({ name: 'Search', query: { query: searchQuery.value, type: SearchType.Song } });
};

const handleClick = (query: string, type: SearchType) => {
  if (!query) return;

  searchVisible.value = false;
  searchRef.value?.blur();
  router.push({ name: 'Search', query: { query, type } });
};

watchThrottled(searchQuery, handleSearch, { throttle: Interval.Sec });
</script>

<template>
  <div class="relative px-2" v-on-click-outside="() => (searchVisible = false)">
    <div class="relative" :data-disabled="!userStore.userinfo">
      <SvgIcon
        class="pointer-events-none absolute left-2 top-1/2 -translate-y-1/2"
        :name="!isSearching ? 'Search' : 'Ring'"
      />
      <input
        ref="searchInputRef"
        class="card h-8 w-48 border border-border px-8"
        :placeholder="!userStore.userinfo ? '请登录' : '搜索'"
        v-model="searchQuery"
        @focusin="searchVisible = true"
        @keydown.enter.prevent="handleEnter"
      />
      <SvgIcon
        v-if="searchQuery"
        class="action-icon absolute right-1 top-1/2 size-6 -translate-y-1/2 cursor-pointer"
        name="Close"
        @click="handleClear"
      />
    </div>

    <Transition name="zoom-top">
      <div
        v-if="searchVisible && searchResult"
        class="card absolute left-2 top-full w-72 space-y-4 bg-background p-4 shadow-md shadow-shadow"
      >
        <div class="flex gap-3">
          <div class="w-9 leading-8 text-minor">单曲:</div>
          <div class="w-0 flex-1">
            <div
              v-for="(music, index) in searchResult[SearchType.Song].slice(0, 5)"
              :key="index"
              class="w-full cursor-pointer truncate rounded-lg px-2 py-1.5 font-bold hover:bg-hover"
              @click="handleClick(getFullName(music), SearchType.Song)"
            >
              {{ getFullName(music) }}
            </div>
          </div>
        </div>

        <div class="flex gap-3">
          <div class="w-9 leading-8 text-minor">歌手:</div>
          <div class="w-0 flex-1">
            <div
              v-for="(artist, index) in searchResult[SearchType.Author].slice(0, 5)"
              :key="index"
              class="w-full cursor-pointer truncate rounded-lg px-2 py-1.5 font-bold hover:bg-hover"
              @click="handleClick(artist.name, SearchType.Author)"
            >
              {{ artist.name }}
            </div>
          </div>
        </div>

        <div class="flex gap-3">
          <div class="w-9 leading-8 text-minor">歌单:</div>
          <div class="w-0 flex-1">
            <div
              v-for="(playlist, index) in searchResult[SearchType.Collect].slice(0, 5)"
              :key="index"
              class="w-full cursor-pointer truncate rounded-lg px-2 py-1.5 font-bold hover:bg-hover"
              @click="handleClick(playlist.title, SearchType.Collect)"
            >
              {{ playlist.title }}
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>
