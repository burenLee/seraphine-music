<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import { notify } from '@/components/Notification.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import VirtualList from '@/components/VirtualList.vue';
import { useContextMenuStore } from '@/stores/context-menu';
import { useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { useUserStore } from '@/stores/user';
import { getFullName, getOrigin } from '@/utils/music';
import { ApiInvokeStatus, ListType, PlaylistType } from '@/utils/params';
import { invoke } from '@/utils/tools';

const listType = ListType.Play;

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
const isLike = (id: ID) => listStore.like.list.get(Number(id));

const handlePlay = async (music: MusicInfo) => {
  musicStore.setMusic(music, { origin: getOrigin(music) });
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

const removeLocalPlaylist = (music: MusicInfo) => {
  listStore.removeList(listType, [music.id]);
};

const handleDownload = () => {
  // TODO: 下载
};

const handleContextMenu = (e: MouseEvent, music: MusicInfo) => {
  contextMenuStore.show({
    x: e.clientX,
    y: e.clientY,
    teleport: '#Playlist',
    options: [
      { label: '播放', prefixIcon: 'Play', onClick: () => handlePlay(music) },
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
      { label: '下载', prefixIcon: 'Download', disabled: true, onClick: () => handleDownload() },
      { divider: true },
      { label: '从列表中删除', prefixIcon: 'Bin', onClick: () => removeLocalPlaylist(music) },
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
  const fileId = listStore.like.list.get(Number(music.id));
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
</script>

<template>
  <div id="Playlist" v-on-click-outside="() => (listVisible = false)">
    <SvgIcon class="action-icon" name="Playlist" size="18" @click="listVisible = !listVisible" />

    <Transition name="slide-page-left">
      <div
        v-if="listVisible"
        class="card fixed bottom-[var(--playbar-height)] right-0 top-[var(--header-height)] z-40 my-4 flex w-80 flex-col bg-background shadow-md shadow-shadow"
      >
        <div class="mt-4 px-4">
          <span class="text-lg font-bold">播放队列</span>
          <span class="ml-2">共 {{ listStore.play.info.count }} 首</span>
        </div>

        <div class="mt-2 flex items-center justify-between px-4">
          <div>
            <span>来源: </span>
            <span v-if="listStore.play.info" class="cursor-pointer font-bold">
              {{ listStore.play.info.title }}
            </span>
          </div>

          <div>
            <ActionButton
              class="h-6 px-1"
              theme="error"
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
            <SvgIcon v-if="isPlaying(row.id)" class="text-info" name="Music" />
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
                @contextmenu.stop
              >
                <SvgIcon class="action-icon" name="Play" @click="handlePlay(row)" />

                <template v-if="userStore.userinfo && row.hash">
                  <SvgIcon
                    v-if="isLike(row.id)"
                    class="action-icon text-error"
                    name="HeartBold"
                    @click="handleUnlike(row)"
                  />
                  <SvgIcon v-else class="action-icon" name="Heart" @click="handleLike(row)" />
                </template>

                <SvgIcon class="action-icon" name="More" @click="handleContextMenu($event, row)" />
              </div>
            </div>
          </template>
        </VirtualList>
      </div>
    </Transition>
  </div>
</template>
