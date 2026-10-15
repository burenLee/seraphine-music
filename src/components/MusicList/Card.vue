<script lang="ts" setup>
import { useRoute, useRouter } from 'vue-router';

import Image from '@/components/Image.vue';
import { notify } from '@/components/Notification.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { InvokeApiStatus, ListType } from '@/constants';
import { useContextMenuStore } from '@/stores/context-menu';
import { useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { useStateStore } from '@/stores/state';
import { useUserStore } from '@/stores/user';
import { getOrigin, getPic } from '@/utils/music';
import { invoke } from '@/utils/tools';

interface Props {
  data: CardInfo;
  info: ListInfo;
  list: CardInfo[];
}

const { data, info, list } = defineProps<Props>();

const route = useRoute();
const router = useRouter();

const stateStore = useStateStore();
const musicStore = useMusicStore();
const listStore = useListStore();
const userStore = useUserStore();
const contextMenuStore = useContextMenuStore();

const handleClick = () => {
  if (data.musicInfo) {
    handlePlay();
  } else if (data.artistInfo) {
    router.push(`/artist-list-table/${data.artistInfo.id}`);
  } else if (data.playlistInfo) {
    router.push(`/top-playlist-table/${data.playlistInfo.gid}`);
  }
};

const handlePlay = () => {
  if (!data.musicInfo) return;

  musicStore.setMusic(data.musicInfo, { origin: getOrigin(data.musicInfo) });

  const playList: MusicInfo[] = [];
  list.forEach((item) => item.musicInfo && playList.push(item.musicInfo));
  listStore.setList(ListType.Play, { info, list: playList });
};

const handlePlayNext = () => {
  if (!data.musicInfo) return;

  listStore.addNextPlay(data.musicInfo);
};

const handleDownload = () => {
  // TODO: 下载
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

const handleContextMenu = (e: MouseEvent) => {
  if (!data.musicInfo) return;

  contextMenuStore.show({
    top: e.clientY,
    left: e.clientX,
    options: [
      { label: '播放', prefixIcon: 'Play', onClick: handlePlay },
      { label: '下一首播放', prefixIcon: 'Playlist', onClick: handlePlayNext },
      { divider: true },
      {
        label: '添加到',
        prefixIcon: 'Add',
        disabled: !userStore.userinfo,
        children: userStore.userPlaylist.map((playlist) => ({
          label: playlist.title,
          onClick: () => addUserPlaylist(data.musicInfo!, playlist),
        })),
      },
      { label: '下载', prefixIcon: 'Download', disabled: true, onClick: handleDownload },
    ],
  });
};
</script>

<template>
  <div
    class="card-hover group/card relative flex h-16 cursor-pointer items-center gap-3 rounded-lg px-2 transition-colors"
    @click="handleClick"
    @contextmenu="handleContextMenu"
  >
    <Image class="size-12" :src="getPic(data.cover)" icon-size="24" />

    <div
      class="card absolute left-2 top-2 flex size-12 items-center justify-center bg-black/30 text-neutral-50 opacity-0 transition-opacity group-hover/card:opacity-100"
    >
      <SvgIcon v-if="data.musicInfo" name="PlayBold" size="24" />
    </div>

    <div class="w-0 flex-1">
      <div class="music-title">{{ data.title }}</div>
      <div v-if="data.artist" class="music-artist">{{ data.artist }}</div>
    </div>
  </div>
</template>
