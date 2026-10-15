<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { ref } from 'vue';
import { useRoute } from 'vue-router';

import ActionButton from '@/components/ActionButton.vue';
import { notify } from '@/components/Notification.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import VirtualList from '@/components/VirtualList.vue';
import { InvokeApiStatus, ListType } from '@/constants';
import { useContextMenuStore } from '@/stores/context-menu';
import { useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { useStateStore } from '@/stores/state';
import { useUserStore } from '@/stores/user';
import { getFullName, getOrigin } from '@/utils/music';
import { invoke } from '@/utils/tools';

const listType = ListType.Play;

const route = useRoute();

const stateStore = useStateStore();
const listStore = useListStore();
const musicStore = useMusicStore();
const userStore = useUserStore();
const contextMenuStore = useContextMenuStore();

const tableColumns: TableColumn[] = [
  { key: 'index', slot: true, width: '3rem', padding: 0 },
  { key: 'info', slot: true, width: 'auto' },
];

const listVisible = ref(false);

const isPlaying = (id: ID) => id === musicStore.music?.id;
const isLike = (id: ID) => listStore.like.list.get(+id);

const handlePlay = async (music: MusicInfo) => {
  musicStore.setMusic(music, { origin: getOrigin(music) });
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
  contextMenuStore.show({
    top: e.clientY,
    left: e.clientX,
    teleport: '#Playlist',
    options: [
      { label: '播放', prefixIcon: 'Play', onClick: () => handlePlay(music) },
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
        label: '删除',
        prefixIcon: 'Bin',
        onClick: () => listStore.removeList(listType, [music.id]),
      },
    ],
  });
};
</script>

<template>
  <div id="Playlist" v-on-click-outside="() => (listVisible = false)">
    <ActionButton
      class="size-8"
      mode="text"
      prefix-icon="Playlist"
      icon-size="18"
      @click="listVisible = !listVisible"
    />

    <Transition name="slide-left-page">
      <div
        v-if="listVisible"
        class="fixed bottom-[var(--playbar-height)] right-0 top-[var(--header-height)] z-40 my-4 flex w-80 flex-col rounded-bl-lg rounded-tl-lg border border-r-0 border-border bg-background shadow-lg shadow-shadow"
      >
        <div class="flex items-center gap-3 px-4 pt-4">
          <SvgIcon name="Playlist" size="24" />
          <div class="text-lg font-bold">播放队列</div>
          <div>共 {{ listStore.play.info.count }} 首</div>
        </div>

        <div class="mt-2 flex items-center justify-between px-4">
          <div>
            <span>来源: </span>
            <span v-if="listStore.play.info" class="animate-underline cursor-pointer font-bold">
              {{ listStore.play.info.title }}
            </span>
          </div>

          <div>
            <ActionButton
              theme="error"
              size="small"
              prefix-icon="Bin"
              @click="listStore.resetList(listType)"
            >
              清空
            </ActionButton>
          </div>
        </div>

        <VirtualList
          ref="musicTableRef"
          class="mt-4 h-0 w-full flex-1 pl-2 pr-0.5"
          :line-height="40"
          :columns="tableColumns"
          :list="listStore.play.list"
          :checked-list="listStore.checkedList"
          :isChecking="listStore.isChecking"
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
            <div class="flex">
              <div
                class="w-0 flex-1 truncate font-bold leading-10"
                :class="isPlaying(row.id) ? 'text-info' : ''"
              >
                {{ getFullName(row) }}
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
        </VirtualList>
      </div>
    </Transition>
  </div>
</template>
