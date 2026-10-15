<script lang="ts" setup>
import { computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import ActionButton from '@/components/ActionButton.vue';
import { notify } from '@/components/Notification.vue';
import { InvokeApiStatus, ListType } from '@/constants';
import { useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { useStateStore } from '@/stores/state';
import { useUserStore } from '@/stores/user';
import { invoke } from '@/utils/tools';

const route = useRoute();

const stateStore = useStateStore();
const listStore = useListStore();
const musicStore = useMusicStore();
const userStore = useUserStore();

const isLike = computed(() => musicStore.music && listStore.like.list.get(+musicStore.music.id));

const handleLike = async () => {
  if (!musicStore.music) return;

  try {
    const { status, data } = await invoke('api_playlist_tracks_add', {
      listId: listStore.like.info.id,
      musicList: [{ name: musicStore.music.title, hash: musicStore.music.hash }],
    });
    if (status !== InvokeApiStatus.Success) {
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

    if (route.name === 'UserPlaylistTable' && route.params.gid === listStore.like.info.gid) {
      stateStore.refresh();
    }
  } catch {
    notify.error('添加失败');
  }
};

const handleUnlike = async () => {
  if (!musicStore.music) return;

  // 同一首歌的每个歌单的fileid是不同的, 获取我喜欢歌单的fileid
  const fileId = listStore.like.list.get(+musicStore.music.id)?.toString();
  if (!fileId) return;

  try {
    const { status } = await invoke('api_playlist_tracks_del', {
      listId: +listStore.like.info.id,
      fileIds: [+fileId],
    });
    if (status !== InvokeApiStatus.Success) {
      notify.error('删除失败');
      return;
    }

    notify.success('删除成功');

    listStore.removeLikeList([musicStore.music.id]);

    if (route.name === 'UserPlaylistTable' && route.params.gid === listStore.like.info.gid) {
      stateStore.refresh();
    }
  } catch {
    notify.error('删除失败');
  }
};
</script>

<template>
  <ActionButton
    v-if="isLike"
    class="size-8 text-error"
    mode="text"
    prefix-icon="HeartBold"
    icon-size="18"
    :disabled="!userStore.userinfo || !musicStore.music"
    @click="handleUnlike"
  />
  <ActionButton
    v-else
    class="size-8"
    mode="text"
    prefix-icon="Heart"
    icon-size="18"
    :disabled="!userStore.userinfo || !musicStore.music || !musicStore.music?.hash"
    @click="handleLike"
  />
</template>
