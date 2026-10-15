<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { computed, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import Image from '@/components/Image.vue';
import Modal from '@/components/Modal.vue';
import { notify } from '@/components/Notification.vue';
import SlideBar from '@/components/SlideBar.vue';
import { InvokeApiStatus, ListType, PageSize, PlaylistType } from '@/constants';
import { useContextMenuStore } from '@/stores/context-menu';
import { defaultListInfo, useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { useStateStore } from '@/stores/state.ts';
import { useUserStore } from '@/stores/user';
import { getOrigin, getPic, getPrivilegeTags } from '@/utils/music';
import { invoke } from '@/utils/tools';

import Add from './Add.vue';

const route = useRoute('UserPlaylistTable');
const router = useRouter();

const userStore = useUserStore();
const listStore = useListStore();
const musicStore = useMusicStore();
const stateStore = useStateStore();
const contextMenuStore = useContextMenuStore();

const slideOptions: Array<SelectOption<PlaylistType>> = [
  { label: '自建歌单', value: PlaylistType.Own },
  { label: '收藏歌单', value: PlaylistType.Favorite },
];

const slideSelection = ref(slideOptions[0]);
const userPlaylist = ref<PlaylistInfo[]>([]);
const favoritePlaylist = ref<PlaylistInfo[]>([]);
const contextMenuPlaylist = ref<PlaylistInfo>();
const delModalVisible = ref(false);

// 歌单切换效果
const transition = computed(() =>
  slideSelection.value.value === PlaylistType.Own ? 'slide-right' : 'slide-left',
);

const getUserPlaylist = async () => {
  const { status, data } = await invoke('api_playlist_user', { pageSize: PageSize.Max });
  if (status !== InvokeApiStatus.Success) return;

  const list = data.info.map((playlist) => {
    if (playlist.is_def === 1) {
      // 默认收藏
      playlist.sort = 0;
    } else if (playlist.is_def === 2) {
      // 我喜欢
      playlist.sort = 1;

      // 初始化 `我喜欢` 列表属性
      listStore.setLikeListInfo({
        ...defaultListInfo,
        id: playlist.list_create_listid,
        title: playlist.name,
        count: playlist.count,
        gid: playlist.list_create_gid,
        sort: 0,
      });
    } else {
      // 默认0, +2确保 `默认收藏`/`我喜欢` 在顶部
      playlist.sort += 2;
    }

    return {
      id: playlist.list_create_listid,
      cover: playlist.pic,
      title: playlist.name,
      artist: playlist.list_create_username,
      count: playlist.count,
      playCount: 0,
      tags: playlist.musiclib_tags.map((tag) => tag.tag_name),
      gid: playlist.list_create_gid,
      is_pri: playlist.is_pri,
      is_def: playlist.is_def,
      sort: playlist.sort,

      type: playlist.type,
    };
  });

  userPlaylist.value = list.filter((playlist) => playlist.type === PlaylistType.Own);
  favoritePlaylist.value = list.filter((playlist) => playlist.type === PlaylistType.Favorite);

  userPlaylist.value.sort((a, b) => a.sort - b.sort);
  favoritePlaylist.value.sort((a, b) => a.sort - b.sort);

  userStore.setUserPlaylist(userPlaylist.value);
};

const clearUserPlaylist = () => {
  userPlaylist.value.length = 0;
};

const getLikePlaylist = async () => {
  const gid = listStore.like.info.gid;
  if (!gid) return;

  let page = 1;
  let total = 0;
  const list: MusicInfo[] = [];

  while (total < listStore.like.info.count) {
    const playlist_tracks_all = await invoke('api_playlist_tracks_all', {
      gid,
      page: page++,
      pageSize: PageSize.Max,
    });
    if (playlist_tracks_all.status !== InvokeApiStatus.Success) break;

    total += playlist_tracks_all.data.count;

    playlist_tracks_all.data.songs.forEach((song) => {
      if (song.shield !== undefined) return;

      const [artist, title] = song.name.split(' - ');

      list.push({
        id: song.mixsongid,
        path: null,
        hash: song.hash,
        cover: song.trans_param.union_cover,
        title: title,
        artist: artist,
        album: song.albuminfo.name,
        duration: song.timelen / 1000,
        sort: song.sort,
        privilegeTags: getPrivilegeTags(song.privilege, song.download[0].pay_type),
        fileId: song.fileid,
      });
    });
  }

  listStore.setLikeListRaw(list);
};

const clearLikePlaylist = () => {
  listStore.clearLikeList();
};

const handlePlay = async (playlist: PlaylistInfo) => {
  if (playlist.id === listStore.play.info.id) {
    musicStore.play();
    return;
  }

  const info: ListInfo = {
    id: playlist.id,
    cover: playlist.cover,
    title: playlist.title,
    artist: playlist.artist,
    tags: playlist.tags,
    count: playlist.count,
    playCount: playlist.playCount,
  };
  const list: MusicInfo[] = [];

  let page = 1;
  let total = 0;

  while (total < playlist.count) {
    const { status, data } = await invoke('api_playlist_tracks_all', {
      gid: playlist.gid,
      page: page++,
      pageSize: PageSize.Max,
    });
    if (status !== InvokeApiStatus.Success) break;

    total += data.count;

    data.songs.forEach((song) => {
      if (song.shield !== undefined) return;

      const [artist, title] = song.name.split(' - ');

      list.push({
        id: song.mixsongid,
        path: null,
        hash: song.hash,
        title: title,
        artist: artist,
        album: song.albuminfo.name,
        cover: song.trans_param.union_cover,
        duration: song.timelen / 1000,
        sort: song.sort,
        privilegeTags: getPrivilegeTags(song.privilege, song.download[0].pay_type),
        fileId: song.fileid,
      });
    });
  }

  listStore.setList(ListType.Play, { info, list });
  musicStore.setMusic(list[0], { origin: getOrigin(list[0]) });
};

const handleShare = () => {
  // TODO: 分享
};

const handlePrivacy = () => {
  // TODO: 设置隐私歌单
};

const handleRename = () => {
  // TODO: 重命名
};

const showRemoveModal = (playlist: PlaylistInfo) => {
  contextMenuPlaylist.value = playlist;
  delModalVisible.value = true;
};

const handleContextMenu = (e: MouseEvent, playlist: PlaylistInfo) => {
  contextMenuStore.show({
    top: e.clientY,
    left: e.clientX,
    options: [
      { label: '播放', prefixIcon: 'Play', onClick: () => handlePlay(playlist) },
      { label: '分享', prefixIcon: 'Share', disabled: true, onClick: () => handleShare() },
      { divider: true },
      {
        label: playlist.is_pri === 0 ? '设为隐私歌单' : '取消设为隐私歌单',
        prefixIcon: playlist.is_pri === 0 ? 'EyeClosed' : 'Eye',
        disabled: true,
        onClick: () => handlePrivacy(),
      },
      { divider: true },
      { label: '重命名', prefixIcon: 'Pen', disabled: true, onClick: () => handleRename() },
      {
        label: '删除',
        prefixIcon: 'Bin',
        disabled: !!playlist.is_def,
        onClick: () => showRemoveModal(playlist),
      },
    ],
  });
};

const handlePlaylistClick = async (playlist: PlaylistInfo) => {
  if (route.params.gid === playlist.gid) return;

  // 如果当前路由为歌单,则replace,否则push
  if (route.name === 'UserPlaylistTable') {
    router.replace(`/user-playlist-table/${playlist.gid}`);
    stateStore.refresh();
  } else {
    router.push(`/user-playlist-table/${playlist.gid}`);
  }
};

const handleDelConfirm = async () => {
  if (!contextMenuPlaylist.value) {
    notify.warning('请选择要删除的歌单');
    return;
  }

  try {
    const { status } = await invoke('api_playlist_del', { listid: +contextMenuPlaylist.value.id });
    if (status !== InvokeApiStatus.Success) {
      notify.error('删除歌单失败');
      return;
    }

    getUserPlaylist();
    handleDelCancel();

    // 如果删除的歌单为当前页面则返回上一级路由
    if (route.params.gid === contextMenuPlaylist.value.gid) router.back();
  } catch {
    notify.error('删除歌单失败');
  }
};

const handleDelCancel = () => {
  delModalVisible.value = false;
};

// 监听用户信息来获取或清空歌单
watch(
  () => userStore.userinfo,
  async (userinfo) => {
    if (userinfo) {
      await getUserPlaylist();
      await getLikePlaylist();
    } else {
      clearUserPlaylist();
      clearLikePlaylist();
    }
  },
  { immediate: true },
);
</script>

<template>
  <div class="flex flex-col pt-8">
    <div class="card mx-4 flex items-center justify-between">
      <SlideBar
        class="border-0 bg-transparent"
        :options="slideOptions"
        :selection="slideSelection"
        @change="slideSelection = $event"
      />

      <Add @refresh="getUserPlaylist" />
    </div>

    <!-- 歌单列表 -->
    <Transition :name="transition" mode="out-in">
      <div
        v-if="slideSelection.value === PlaylistType.Own"
        class="h-0 flex-1 overflow-y-auto px-4 pb-4 pt-2"
      >
        <div
          v-for="playlist in userPlaylist"
          :key="playlist.gid"
          class="flex h-10 cursor-pointer items-center gap-3 rounded-lg px-2"
          :class="route.params.gid === playlist.gid ? 'card-actived' : 'card-hover'"
          @click="handlePlaylistClick(playlist)"
          @contextmenu="handleContextMenu($event, playlist)"
        >
          <Image class="size-7" :src="getPic(playlist.cover)" />
          <div class="w-0 flex-1 truncate font-bold">{{ playlist.title }}</div>
        </div>
      </div>

      <div
        v-else-if="slideSelection.value === PlaylistType.Favorite"
        class="h-0 flex-1 overflow-y-auto px-4 pb-4 pt-2"
      >
        <div
          v-for="playlist in favoritePlaylist"
          :key="playlist.gid"
          class="flex h-10 cursor-pointer items-center gap-3 rounded-lg px-2"
          :class="route.params.gid === playlist.gid ? 'card-actived' : 'card-hover'"
          @click="handlePlaylistClick(playlist)"
          @contextmenu="handleContextMenu($event, playlist)"
        >
          <Image class="size-7" :src="getPic(playlist.cover)" />
          <div class="w-0 flex-1 truncate font-bold">{{ playlist.title }}</div>
        </div>
      </div>
    </Transition>

    <!-- 确认删除弹窗 -->
    <Modal
      :visible="delModalVisible"
      class="w-80"
      title="删除歌单"
      @cancel="handleDelCancel"
      @confirm="handleDelConfirm"
    >
      <div class="px-6">
        确认删除歌单
        <span v-if="contextMenuPlaylist" class="font-bold">{{ contextMenuPlaylist?.title }}</span>
        ?
      </div>
    </Modal>
  </div>
</template>
