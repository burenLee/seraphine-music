<script lang="ts" setup>
import { convertFileSrc } from '@tauri-apps/api/core';
import { computed, inject, onMounted, onUnmounted, ref, useTemplateRef } from 'vue';
import { useRoute } from 'vue-router';

import Image from '@/components/Image.vue';
import Modal from '@/components/Modal.vue';
import { notify } from '@/components/Notification.vue';
import ToTartget from '@/components/PageActions/ToTartget.vue';
import ToTop from '@/components/PageActions/ToTop.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import VirtualList from '@/components/VirtualList.vue';
import { useContextMenuStore } from '@/stores/context-menu';
import { useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { useUserStore } from '@/stores/user';
import { getOrigin, getPic } from '@/utils/music';
import { ApiInvokeStatus, ListType, PlaylistType } from '@/utils/params';
import { cn, formatDuration, invoke, revealPath } from '@/utils/tools';

import MusicDetail from './MusicDetail.vue';

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

const contextMenuMusic = ref<MusicInfo>();
const musicDetailVisible = ref(false);
const removeVisible = ref(false);
const removeMode = ref<'user' | 'local'>('local');

const musicList = computed(() => listStore[listType]);
const getCover = computed(() =>
  // 目前只考虑整个表的类型, 不支持混合类型
  listType === ListType.Local
    ? (cover: string) => convertFileSrc(cover)
    : (cover: string) => getPic(cover),
);

const isPlaying = (id: ID) => musicStore.music?.id == id;
const isLike = (id: ID) => listStore.like.list.get(Number(id));

const handleInfinite = () => {
  emits('infinite');
};

const handleCheck = <T>(value: T) => {
  listStore.handleChecked(value);
};

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

const addUserPlaylist = async (music: MusicInfo, playlist: ApiPlaylist) => {
  try {
    const { status, data } = await invoke('api_playlist_tracks_add', {
      listId: playlist.list_create_listid,
      musicList: [{ name: music.title, hash: music.hash }],
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('添加失败');
      return;
    }

    notify.success('添加成功');

    // 如果是 我喜欢, 同步列表
    if (playlist.is_def === 2) {
      const musicList: MusicInfo[] = data.info.map((song) => {
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
      listStore.addLikeList(musicList);
    }
  } catch {
    notify.error('添加失败');
  }
};

const removeUserPlaylist = async (music: MusicInfo) => {
  if (!music.fileId) return;

  try {
    const { status } = await invoke('api_playlist_tracks_del', {
      listId: Number(musicList.value.info.id),
      fileIds: [Number(music.fileId)],
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('删除失败');
      return;
    }

    notify.success('删除成功');
    if (route.name === 'UserPlaylistTable' && musicList.value.info.gid === route.params.gid) {
      removeLocalPlaylist(music);
    }
    // 如果是 我喜欢, 同步列表
    if (musicList.value.info.id === listStore.like.info.id) listStore.removeLikeList([music]);
  } catch {
    notify.error('删除失败');
  }
};

const removeLocalPlaylist = (music: MusicInfo) => {
  listStore.removeList(listType, [music.id]);
};

const handleDownload = () => {
  // TODO: 下载
};

// 打开音频详情弹窗
const showMusicDetailModal = () => {
  musicDetailVisible.value = true;
};

// 打开音频属性修改弹窗
const showMusicSettingModal = () => {
  // TODO: 歌曲设置
};

// 打开删除确认弹窗
const showRemoveModal = (mode: 'user' | 'local') => {
  removeVisible.value = true;
  removeMode.value = mode;
};

const handleContextMenu = (e: MouseEvent, music: MusicInfo) => {
  contextMenuMusic.value = music;

  contextMenuStore.show({
    x: e.clientX,
    y: e.clientY,
    options: [
      { label: '播放', prefixIcon: 'Play', onClick: () => handlePlay(music) },
      { label: '下一首播放', prefixIcon: 'Playlist', onClick: () => listStore.addNextPlay(music) },
      { divider: true },
      {
        label: '添加到',
        prefixIcon: 'Add',
        suffixIcon: 'Right',
        disabled: !userStore.userinfo || !music.hash,
        children: userStore.userPlaylist
          .filter((playlist) => playlist.type === PlaylistType.User)
          .map((playlist) => ({
            label: playlist.name,
            onClick: () => addUserPlaylist(music, playlist),
          })),
      },
      { label: '下载', prefixIcon: 'Download', disabled: true, onClick: handleDownload },
      { divider: true },
      {
        label: '歌曲详情',
        prefixIcon: 'Info',
        disabled: !!music.hash,
        onClick: showMusicDetailModal,
      },
      { label: '歌曲设置', prefixIcon: 'Setting', disabled: true, onClick: showMusicSettingModal },
      {
        label: '打开文件目录',
        prefixIcon: 'Folder',
        disabled: !!music.hash,
        onClick: () => music.path && revealPath(music.path),
      },
      { divider: true },
      {
        label: '从歌单删除',
        prefixIcon: 'Bin',
        disabled: route.name !== 'UserPlaylistTable',
        onClick: () => showRemoveModal('user'),
      },
      { label: '从列表删除', prefixIcon: 'Bin', onClick: () => showRemoveModal('local') },
    ],
  });
};

const handleLike = async (music: MusicInfo) => {
  try {
    const { status, data } = await invoke('api_playlist_tracks_add', {
      listId: listStore.like.info.id,
      musicList: [{ name: music.title, hash: music.hash }],
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('添加失败');
      return;
    }

    notify.success('添加成功');

    const musicList: MusicInfo[] = data.info.map((song) => {
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
    listStore.addLikeList(musicList);
  } catch {
    notify.error('添加失败');
  }
};

const handleUnlike = async (music: MusicInfo) => {
  // 同一首歌的每个歌单的fileid是不同的, 获取我喜欢歌单的fileid
  const fileId = listStore.like.list.get(Number(music.id))?.toString();
  if (!fileId) return;

  try {
    const { status } = await invoke('api_playlist_tracks_del', {
      listId: Number(listStore.like.info.id),
      fileIds: [Number(fileId)],
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('删除失败');
      return;
    }

    notify.success('删除成功');
    listStore.removeLikeList([music]);
  } catch {
    notify.error('删除失败');
  }
};

const handleRemoveConfirm = async () => {
  if (!contextMenuMusic.value) {
    notify.error('未知歌曲');
    return;
  }

  switch (removeMode.value) {
    case 'user':
      removeUserPlaylist(contextMenuMusic.value);
      break;
    case 'local':
      removeLocalPlaylist(contextMenuMusic.value);
      break;
  }

  handleRemoveCancel();
};

const handleRemoveCancel = () => {
  contextMenuMusic.value = undefined;
  removeVisible.value = false;
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
    @infinite="handleInfinite"
    @check="handleCheck"
    @contextmenu="handleContextMenu"
    @lineDblClick="handlePlay"
  >
    <template #index="row">
      <SvgIcon v-if="isPlaying(row.id)" class="text-info" name="Music" />
      <div v-else class="truncate text-center text-minor">{{ row.index + 1 }}</div>
    </template>

    <template #info="row">
      <div class="flex items-center gap-3">
        <Image class="size-12" :src="row.cover ? getCover(row.cover) : ''" />

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
          @contextmenu.stop
        >
          <SvgIcon class="action-icon" name="Play" @click="handlePlay(row)" />

          <SvgIcon
            v-if="isLike(row.id)"
            class="action-icon text-error"
            name="HeartBold"
            @click="handleUnlike(row)"
          />
          <SvgIcon
            v-else
            class="action-icon"
            name="Heart"
            :disabled="!userStore.userinfo || !row.hash"
            @click="handleLike(row)"
          />

          <SvgIcon class="action-icon" name="More" @click="handleContextMenu($event, row)" />
        </div>
      </div>
    </template>

    <template #duration="row">
      <div class="text-center text-minor">{{ formatDuration(row.duration) }}</div>
    </template>
  </VirtualList>

  <div class="absolute bottom-4 right-4">
    <ToTartget v-if="musicStore.music" @click="handleToTarget" />
    <ToTop class="mt-3" @click="handleToTop" />
  </div>

  <MusicDetail v-model="musicDetailVisible" :path="contextMenuMusic?.path || ''" />

  <Modal
    v-model="removeVisible"
    class="w-80"
    title="删除"
    @confirm="handleRemoveConfirm"
    @cancel="handleRemoveCancel"
  >
    <div class="px-6">
      确认 {{ removeMode === 'user' ? '从歌单' : '从列表' }} 删除
      <span class="font-bold">{{ contextMenuMusic?.title }}</span> ?
    </div>
  </Modal>
</template>
