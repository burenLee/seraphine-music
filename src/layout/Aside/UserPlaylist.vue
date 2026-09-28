<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { computed, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import Image from '@/components/Image.vue';
import Modal from '@/components/Modal.vue';
import { notify } from '@/components/Notification.vue';
import SelectModal from '@/components/SelectModal.vue';
import SlideBar from '@/components/SlideBar.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useContextMenuStore } from '@/stores/context-menu';
import { defaultInfo, useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { useRefreshStore } from '@/stores/refresh';
import { useUserStore } from '@/stores/user';
import { getOrigin, getPic, getPrivilegeTags } from '@/utils/music';
import { AddPlaylistType, ApiInvokeStatus, ListType, PageSize, PlaylistType } from '@/utils/params';
import { invoke } from '@/utils/tools';

const route = useRoute('UserPlaylistTable');
const router = useRouter();

const userStore = useUserStore();
const listStore = useListStore();
const musicStore = useMusicStore();
const refreshStore = useRefreshStore();
const contextMenuStore = useContextMenuStore();

const slideOptions: Array<SelectOption<PlaylistType>> = [
  { label: '自建歌单', value: PlaylistType.User },
  { label: '收藏歌单', value: PlaylistType.Collection },
];
const addOptions = computed<Array<SelectOption<AddPlaylistType>>>(() => [
  {
    label: '新建歌单',
    value: AddPlaylistType.Add,
    prefixIcon: 'Add',
    disabled: !userStore.userinfo,
  },
  { label: '导入歌单', value: AddPlaylistType.Import, prefixIcon: 'Link', disabled: true },
]);

const slideSelection = ref(slideOptions[0]);
const userPlaylist = ref<ApiPlaylist[]>([]);
const addSelectVisible = ref(false);
const contextMenuPlaylist = ref<ApiPlaylist>();
const addModalVisible = ref(false);
const addForm = ref({ name: '', isPri: 0 });
const delModalVisible = ref(false);

// 歌单切换效果
const transition = computed(() =>
  slideSelection.value.value === PlaylistType.User ? 'slide-right' : 'slide-left',
);

const getUserPlaylist = async () => {
  try {
    const { status, data } = await invoke('api_playlist_user', { pageSize: PageSize.Max });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('无法获取歌单歌曲');
      return;
    }

    userPlaylist.value = data.info.map((playlist) => {
      if (playlist.is_def === 1) {
        // 默认收藏
        playlist.sort = 0;
      } else if (playlist.is_def === 2) {
        // 我喜欢
        playlist.sort = 1;

        // 初始化 我喜欢 列表, info 数据供getLikePlaylist使用
        listStore.setLikeListInfo({
          ...defaultInfo,
          id: playlist.list_create_listid,
          title: playlist.name,
          count: playlist.count,
          gid: playlist.list_create_gid,
        });
      } else {
        // 默认0, +2确保 默认收藏/我喜欢 在顶部
        playlist.sort += 2;
      }

      return playlist;
    });

    userPlaylist.value.sort((a, b) => a.sort - b.sort);
    userStore.setUserPlaylist(userPlaylist.value);
  } catch {
    notify.error('无法获取歌单歌曲');
  }
};

const clearUserPlaylist = () => {
  userPlaylist.value.length = 0;
};

const getLikePlaylist = async () => {
  const gid = listStore.like.info.gid;
  if (!gid) return;

  try {
    let page = 1;
    let total = 0;
    const list: MusicInfo[] = [];

    while (total < listStore.like.info.count) {
      const playlist_tracks_all = await invoke('api_playlist_tracks_all', {
        gid,
        page: page++,
        pageSize: PageSize.Max,
      });
      if (playlist_tracks_all.status !== ApiInvokeStatus.Success) break;

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
  } catch {}
};

const clearLikePlaylist = () => {
  listStore.clearLikeList();
};

const handleAddSelect = (type: AddPlaylistType) => {
  switch (type) {
    case AddPlaylistType.Add:
      addModalVisible.value = true;
      break;
    case AddPlaylistType.Import:
      break;
  }

  addSelectVisible.value = false;
};

const handlePlay = async (playlist: ApiPlaylist) => {
  if (playlist.list_create_listid === listStore.play.info.id) {
    musicStore.play();
    return;
  }

  try {
    const info: ListInfo = {
      ...defaultInfo,
      id: playlist.list_create_listid,
      cover: playlist.pic,
      title: playlist.name,
      artist: playlist.list_create_username,
      tags: playlist.musiclib_tags.map((tag) => tag.tag_name),
      count: playlist.count,
      gid: playlist.list_create_gid,
    };
    const list: MusicInfo[] = [];

    let page = 1;
    let total = 0;

    while (total < playlist.count) {
      const { status, data } = await invoke('api_playlist_tracks_all', {
        gid: playlist.list_create_gid,
        page: page++,
        pageSize: PageSize.Max,
      });
      if (status !== ApiInvokeStatus.Success) break;

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
  } catch {}
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

const showRemoveModal = (playlist: ApiPlaylist) => {
  contextMenuPlaylist.value = playlist;
  delModalVisible.value = true;
};

const handleContextMenu = (e: MouseEvent, playlist: ApiPlaylist) => {
  contextMenuStore.show({
    x: e.clientX,
    y: e.clientY,
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

const handlePlaylistClick = async ({ list_create_gid }: ApiPlaylist) => {
  if (route.params.gid === list_create_gid) return;

  // 如果当前路由为歌单,则replace,否则push
  if (route.name === 'UserPlaylistTable') {
    router.replace(`/user-playlist-table/${list_create_gid}`);
    refreshStore.refresh();
  } else {
    router.push(`/user-playlist-table/${list_create_gid}`);
  }
};

const handleAddConfirm = async () => {
  if (!userStore.userinfo) {
    notify.error('请先登录');
    return;
  }
  if (!addForm.value.name) {
    notify.error('请填写歌单名称');
    return;
  }

  try {
    const { status } = await invoke('api_playlist_add', {
      name: addForm.value.name,
      isPri: addForm.value.isPri,
      userid: userStore.userinfo.userid,
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('创建歌单失败');
    }

    notify.success('创建歌单成功');
    handleAddCancel();
    getUserPlaylist();
  } catch {
    notify.error('创建歌单失败');
  }
};

const handleAddCancel = () => {
  addForm.value = { name: '', isPri: 0 };
  addModalVisible.value = false;
};

const handleDelConfirm = async () => {
  if (!contextMenuPlaylist.value) {
    notify.error('请选择要删除的歌单');
    return;
  }

  try {
    const { status } = await invoke('api_playlist_del', {
      listid: contextMenuPlaylist.value.list_create_listid,
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.success('删除歌单失败');
      return;
    }

    notify.success('删除歌单成功');
    handleDelCancel();

    // 如果删除的歌单为当前页面则返回上一级路由
    if (contextMenuPlaylist.value.list_create_gid === route.query.id) {
      router.back();
    } else {
      getUserPlaylist();
    }
  } catch {
    notify.success('删除歌单失败');
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
    <div class="card mx-4 flex items-center justify-between pr-1">
      <SlideBar
        class="border-0 bg-transparent"
        :options="slideOptions"
        :selection="slideSelection"
        @change="slideSelection = $event"
      />

      <div class="relative" v-on-click-outside="() => (addSelectVisible = false)">
        <SvgIcon
          class="action-icon card-hover rounded-lg transition-colors hover:text-foreground"
          name="Add"
          size="20"
          @click="addSelectVisible = !addSelectVisible"
        />

        <SelectModal
          class="absolute -right-1 top-full"
          transition="zoom-top-right"
          :visible="addSelectVisible"
          :options="addOptions"
          @select="handleAddSelect"
        />
      </div>
    </div>

    <!-- 歌单列表 -->
    <Transition :name="transition" mode="out-in">
      <div
        v-if="slideSelection.value === PlaylistType.User"
        class="h-0 flex-1 overflow-y-auto px-4 py-2"
      >
        <div
          v-for="playlist in userPlaylist.filter((list) => list.type === PlaylistType.User)"
          :key="playlist.list_create_gid"
          class="flex cursor-pointer items-center gap-2 rounded-lg px-2 py-1 transition-colors"
          :class="route.query.id === playlist.list_create_gid ? 'card-actived' : 'card-hover'"
          @click="handlePlaylistClick(playlist)"
          @contextmenu.prevent="handleContextMenu($event, playlist)"
        >
          <Image class="size-7" :src="getPic(playlist.pic)" />

          <div class="w-0 flex-1 truncate font-bold">{{ playlist.name }}</div>
        </div>
      </div>

      <div
        v-else-if="slideSelection.value === PlaylistType.Collection"
        class="h-0 flex-1 overflow-y-auto px-4 py-2"
      >
        <div
          v-for="playlist in userPlaylist.filter((list) => list.type === PlaylistType.Collection)"
          :key="playlist.list_create_gid"
          class="flex cursor-pointer items-center gap-2 rounded-lg px-2 py-1 transition-colors"
          :class="route.query.id === playlist.list_create_gid ? 'card-actived' : 'card-hover'"
          @click="handlePlaylistClick(playlist)"
          @contextmenu.prevent="handleContextMenu($event, playlist)"
        >
          <Image class="size-7" :src="getPic(playlist.pic)" />

          <div class="w-0 flex-1 truncate font-bold">{{ playlist.name }}</div>
        </div>
      </div>
    </Transition>

    <!-- 新增歌单弹窗 -->
    <Modal
      class="w-80"
      v-model="addModalVisible"
      title="新建歌单"
      confirmLabel="新建"
      @cancel="handleAddCancel"
      @confirm="handleAddConfirm"
    >
      <form class="px-6" @submit.prevent="handleAddConfirm">
        <input
          class="h-8 w-full rounded-md border border-minor bg-background px-2 leading-8"
          v-model="addForm.name"
          placeholder="请输入歌单名称"
        />

        <label class="flex cursor-pointer items-center gap-1 pt-2">
          <input type="checkbox" v-model="addForm.isPri" :true-value="1" :false-value="0" />
          <span class="text-sm">设为隐私歌单</span>
          <SvgIcon name="Info" title="仅自己可见, 且无法分享" />
        </label>
      </form>
    </Modal>

    <!-- 确认删除弹窗 -->
    <Modal
      class="w-80"
      v-model="delModalVisible"
      title="删除歌单"
      @cancel="handleDelCancel"
      @confirm="handleDelConfirm"
    >
      <div class="px-6">
        确认删除歌单 <span class="font-bold">{{ contextMenuPlaylist?.name }}</span> ?
      </div>
    </Modal>
  </div>
</template>
