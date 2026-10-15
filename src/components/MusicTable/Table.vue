<script lang="ts" setup>
import { convertFileSrc } from '@tauri-apps/api/core';
import { computed, inject, onMounted, onUnmounted, ref, useTemplateRef } from 'vue';
import { useRoute } from 'vue-router';

import ActionButton from '@/components/ActionButton.vue';
import Image from '@/components/Image.vue';
import Modal from '@/components/Modal.vue';
import MusicDetail from '@/components/MusicTable/Detail.vue';
import { notify } from '@/components/Notification.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import VirtualList from '@/components/VirtualList.vue';
import { InvokeApiStatus, ListType } from '@/constants';
import { useContextMenuStore } from '@/stores/context-menu';
import { useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { useStateStore } from '@/stores/state';
import { useUserStore } from '@/stores/user';
import { getOrigin, getPic } from '@/utils/music';
import { cn, formatDuration, invoke, revealPath } from '@/utils/tools';

interface Emits {
  infinite: [];
}

const emits = defineEmits<Emits>();

const listType = inject<ListType>('listType', ListType.Show);

const route = useRoute();

const listStore = useListStore();
const musicStore = useMusicStore();
const userStore = useUserStore();
const contextMenuStore = useContextMenuStore();

const musicTableRef = useTemplateRef('musicTableRef');

const tableColumns: TableColumn[] = [
  { key: 'index', slot: true, width: '3rem', padding: 0 },
  { key: 'info', slot: true, width: 'auto' },
  { key: 'album', width: '35%' },
  { key: 'duration', slot: true, width: '10%' },
];

const stateStore = useStateStore();
const contextMenuMusic = ref<MusicInfo>();
const musicDetailVisible = ref(false);
const removeVisible = ref(false);

const musicList = computed(() => listStore[listType]);
const getCover = computed(() =>
  // 目前只考虑整个表的类型, 不支持混合类型
  listType === ListType.Local
    ? (cover: string) => convertFileSrc(cover)
    : (cover: string) => getPic(cover),
);

const isPlaying = (id: ID) => id === musicStore.music?.id;
const isLike = (id: ID) => listStore.like.list.get(+id);

const handlePlay = (music: MusicInfo) => {
  musicStore.setMusic(music, { origin: getOrigin(music) });

  // 如果播放列表不是当前列表，则切换
  if (listStore.play.info.id !== musicList.value.info.id) {
    listStore.setList(ListType.Play, {
      info: musicList.value.info,
      list: [...musicList.value.list],
    });
  }
};

const addUserPlaylist = async (music: MusicInfo, playlist: PlaylistInfo) => {
  try {
    const { status, data } = await invoke('api_playlist_tracks_add', {
      listId: +playlist.id,
      musicList: [{ name: music.title, hash: music.hash }],
    });
    if (status !== InvokeApiStatus.Success) {
      notify.error('添加失败');
      return;
    }

    notify.success('添加成功');

    // 如果是我喜欢, 同步喜欢列表
    if (playlist.id === listStore.like.info.id) {
      const likeList: MusicInfo[] = data.info.map((song) => {
        const [artist, title] = song.name.split(' - ');

        return {
          id: song.mixsongid,
          path: null,
          hash: song.hash,
          cover: song.cover,
          title: title,
          artist: artist,
          album: '',
          duration: 0,
          sort: song.sort,
          fileId: song.fileid,
        };
      });

      listStore.addLikeList(likeList);
    }

    if (route.name === 'UserPlaylistTable' && route.params.gid === playlist.gid) {
      stateStore.refresh();
    }
  } catch {
    notify.error('添加失败');
  }
};

const removeUserPlaylist = async (music: MusicInfo, playlist: PlaylistInfo) => {
  const isLike = playlist.id === listStore.like.info.id;
  const fileId = isLike ? listStore.like.list.get(+music.id) : music.fileId;

  if (!fileId) {
    notify.warning('未知歌曲');
    return;
  }

  try {
    const { status } = await invoke('api_playlist_tracks_del', {
      listId: +playlist.id,
      fileIds: [+fileId],
    });
    if (status !== InvokeApiStatus.Success) {
      notify.error('删除失败');
      return;
    }

    notify.success('删除成功');

    // 如果是 我喜欢, 同步列表
    if (isLike) listStore.removeLikeList([music.id]);

    if (route.name === 'UserPlaylistTable' && route.params.gid === playlist.gid) {
      stateStore.refresh();
    }
  } catch {
    notify.error('删除失败');
  }
};

const handleContextMenu = (e: MouseEvent, music: MusicInfo) => {
  contextMenuMusic.value = music;

  contextMenuStore.show({
    top: e.clientY,
    left: e.clientX,
    options: [
      { label: '播放', prefixIcon: 'Play', onClick: () => handlePlay(music) },
      { label: '下一首播放', prefixIcon: 'Playlist', onClick: () => listStore.addNextPlay(music) },
      { divider: true },
      {
        label: '添加到',
        prefixIcon: 'Add',
        disabled: !userStore.userinfo || !music.hash,
        children: userStore.userPlaylist.map((playlist) => ({
          label: playlist.title,
          onClick: () => addUserPlaylist(music, playlist),
        })),
      },
      { label: '下载', prefixIcon: 'Download', disabled: true, onClick: () => 'TODO: 下载' },
      { divider: true },
      {
        label: '歌曲详情',
        prefixIcon: 'Info',
        disabled: !!music.hash,
        onClick: () => (musicDetailVisible.value = true),
      },
      { label: '歌曲修改', prefixIcon: 'Pen', disabled: true, onClick: () => 'TODO: 歌曲设置' },
      {
        label: '打开文件目录',
        prefixIcon: 'Folder',
        disabled: !!music.hash,
        onClick: () => music.path && revealPath(music.path),
      },
      { divider: true },
      { label: '删除', prefixIcon: 'Bin', onClick: () => (removeVisible.value = true) },
    ],
  });
};

const handleRemoveConfirm = async () => {
  if (!contextMenuMusic.value) {
    notify.warning('未知歌曲');
    return;
  }

  if (route.name === 'UserPlaylistTable') {
    const playlist = userStore.userPlaylist.find(
      (playlist) => playlist.id === musicList.value.info.id,
    );
    if (!playlist) {
      notify.error('未知歌单');
      return;
    }

    removeUserPlaylist(contextMenuMusic.value, playlist);
  } else {
    listStore.removeList(listType, [contextMenuMusic.value.id]);
  }

  handleRemoveCancel();
};

const handleRemoveCancel = () => {
  removeVisible.value = false;
  contextMenuMusic.value = undefined;
};

const handleToTarget = () => {
  musicTableRef.value?.scrollToTarget(musicStore.music?.id);
};

const handleToTop = () => {
  musicTableRef.value?.scrollToTop();
};

onMounted(handleToTarget);
onUnmounted(() => listStore.clearSearch());

defineExpose({ handleToTarget, handleToTop });
</script>

<template>
  <VirtualList
    ref="musicTableRef"
    :class="cn('h-full w-full pl-8 pr-[1.625rem]', $attrs.class)"
    :columns="tableColumns"
    :loading="listStore.isTableLoading"
    :list="listStore.searchList || musicList.list"
    :checking="listStore.isChecking"
    :checkedList="listStore.checkedList"
    @infinite="emits('infinite')"
    @check="listStore.handleChecked"
    @contextmenu="handleContextMenu"
    @lineDblClick="handlePlay"
  >
    <template #index="row">
      <div v-if="isPlaying(row.id)" class="flex items-center justify-center text-info">
        <SvgIcon name="Bars" />
      </div>
      <div v-else class="truncate text-center text-minor">{{ row.index + 1 }}</div>
    </template>

    <template #info="row">
      <div class="flex items-center gap-3">
        <Image class="size-12" :src="row.cover ? getCover(row.cover) : ''" icon-size="24" />

        <div class="w-0 flex-1 overflow-hidden">
          <div class="flex items-center gap-3">
            <div class="music-title" :class="isPlaying(row.id) ? 'text-info' : ''">
              {{ row.title }}
            </div>

            <div
              class="card rounded border-info px-1 text-xs font-bold leading-4 text-info"
              v-for="(tag, index) in row.privilegeTags"
              :key="index"
            >
              {{ tag }}
            </div>
          </div>

          <div class="music-artist">{{ row.artist }}</div>
        </div>

        <div
          v-if="!listStore.isChecking"
          class="hidden items-center pl-2 group-hover/line:flex"
          @dblclick.stop
        >
          <ActionButton class="p-2" mode="text" prefixIcon="Play" @click="handlePlay(row)" />

          <ActionButton
            v-if="!isLike(row.id)"
            class="p-2"
            mode="text"
            prefixIcon="Heart"
            :disabled="!userStore.userinfo || !row.hash"
            @click="addUserPlaylist(row, listStore.like.info)"
          />
          <ActionButton
            v-else
            mode="text"
            class="p-2 text-error"
            prefixIcon="HeartBold"
            :disabled="!userStore.userinfo"
            @click="removeUserPlaylist(row, listStore.like.info)"
          />

          <ActionButton
            class="p-2"
            mode="text"
            prefixIcon="More"
            @click="handleContextMenu($event, row)"
          />
        </div>
      </div>
    </template>

    <template #duration="row">
      <div class="text-center text-minor">{{ formatDuration(row.duration) }}</div>
    </template>
  </VirtualList>

  <div class="absolute bottom-4 right-4 space-y-3">
    <ActionButton
      v-if="musicStore.music"
      class="size-8 p-0"
      prefix-icon="Target"
      title="滚到播放"
      @click="handleToTarget"
    />
    <ActionButton class="size-8 p-0" prefix-icon="Up" title="滚到顶部" @click="handleToTop" />
  </div>

  <MusicDetail v-model="musicDetailVisible" :path="contextMenuMusic?.path || ''" />

  <Modal
    :visible="removeVisible"
    class="w-80"
    title="删除"
    @confirm="handleRemoveConfirm"
    @cancel="handleRemoveCancel"
  >
    <div class="px-6">
      确认删除
      <span class="font-bold">{{ contextMenuMusic?.title }}</span>
      ?
    </div>
  </Modal>
</template>
