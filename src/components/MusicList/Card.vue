<script lang="ts" setup>
import { useRouter } from 'vue-router';

import Image from '@/components/Image.vue';
import { notify } from '@/components/Notification.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useContextMenuStore } from '@/stores/context-menu';
import { useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { useUserStore } from '@/stores/user';
import { getOrigin, getPic } from '@/utils/music';
import { ApiInvokeStatus, ListType, PlaylistType } from '@/utils/params';
import { invoke } from '@/utils/tools';

interface Props {
  data: CardInfo;
  info: ListInfo;
  list: CardInfo[];
}

const { data, info, list } = defineProps<Props>();

const router = useRouter();

const musicStore = useMusicStore();
const listStore = useListStore();
const userStore = useUserStore();
const contextMenuStore = useContextMenuStore();

const handleClick = () => {
  if (data.musicInfo) {
    handlePlay();
  } else if (data.artistInfo) {
    if (!data.artistInfo.id) return;

    router.push(`/artist-list-table/${data.artistInfo.id}`);
  } else if (data.playlistInfo) {
    if (!data.playlistInfo.gid) return;

    router.push(`/top-playlist-table/${data.playlistInfo.gid}`);
  }
};

const handlePlay = () => {
  if (!data.musicInfo) return;

  const playList: MusicInfo[] = [];
  list.forEach((item) => item.musicInfo && playList.push(item.musicInfo));

  listStore.setList(ListType.Play, { info, list: playList });
  musicStore.setMusic(data.musicInfo, { origin: getOrigin(data.musicInfo) });
};

const handlePlayNext = () => {
  if (!data.musicInfo) return;

  listStore.addNextPlay(data.musicInfo);
};

const handleDownload = () => {
  // TODO: 下载
};

const adUserPlaylist = async (playlist: ApiPlaylist) => {
  if (!data.musicInfo) return;

  try {
    const { status, data: playlist_tracks_add } = await invoke('api_playlist_tracks_add', {
      listId: playlist.list_create_listid,
      musicList: [{ name: data.musicInfo.title, hash: data.musicInfo.hash }],
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('添加失败');
      return;
    }

    notify.success('添加成功');
    // 如果是添加到我喜欢,同步列表
    if (playlist.is_def === 2) {
      const musicList: MusicInfo[] = playlist_tracks_add.info.map((song) => {
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

const handleContextMenu = (e: MouseEvent) => {
  if (!data.musicInfo) return;

  contextMenuStore.show({
    x: e.clientX,
    y: e.clientY,
    options: [
      { label: '播放', prefixIcon: 'Play', onClick: handlePlay },
      { label: '下一首播放', prefixIcon: 'Playlist', onClick: handlePlayNext },
      { divider: true },
      {
        label: '添加到',
        prefixIcon: 'Add',
        suffixIcon: 'Right',
        disabled: !userStore.userinfo,
        children: userStore.userPlaylist
          .filter((playlist) => playlist.type === PlaylistType.User)
          .map((playlist) => ({
            label: playlist.name,
            onClick: () => adUserPlaylist(playlist),
          })),
      },
      { label: '下载', prefixIcon: 'Download', disabled: true, onClick: handleDownload },
    ],
  });
};
</script>

<template>
  <div
    class="card-hover group/card relative flex h-16 cursor-pointer items-center gap-2 rounded-lg px-2 transition-colors"
    @click="handleClick"
    @contextmenu.prevent="handleContextMenu"
  >
    <Image class="size-12" :src="getPic(data.cover)" />

    <SvgIcon
      v-if="data.musicInfo"
      class="card absolute left-2 top-2 flex size-12 items-center justify-center bg-black/30 text-neutral-50 opacity-0 transition-opacity group-hover/card:opacity-100"
      name="PlayBold"
      size="20"
    />

    <div class="flex h-12 w-0 flex-1 flex-col justify-center">
      <div class="music-title">{{ data.title }}</div>
      <div v-if="data.artist" class="music-artist">{{ data.artist }}</div>
    </div>
  </div>
</template>
