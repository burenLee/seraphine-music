<script lang="ts" setup>
import { computed } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import { notify } from '@/components/Notification.vue';
import { useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { useUserStore } from '@/stores/user';
import { ApiInvokeStatus } from '@/utils/params';
import { invoke } from '@/utils/tools';

const listStore = useListStore();
const musicStore = useMusicStore();
const userStore = useUserStore();

const disabled = computed(() => !userStore.userinfo || !musicStore.music);
const isLike = computed(() => musicStore.music && listStore.like.list.get(+musicStore.music.id));

const handleLike = async () => {
  if (!musicStore.music) return;

  try {
    const { status, data } = await invoke('api_playlist_tracks_add', {
      listId: listStore.like.info.id,
      musicList: [{ name: musicStore.music.title, hash: musicStore.music.hash }],
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
    if (status !== ApiInvokeStatus.Success) {
      notify.error('删除失败');
      return;
    }

    notify.success('删除成功');
    listStore.removeLikeList([musicStore.music] as MusicInfo[]);
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
    :disabled="disabled"
    @click="handleUnlike"
  />
  <ActionButton
    v-else
    class="size-8"
    mode="text"
    prefix-icon="Heart"
    icon-size="18"
    :disabled="disabled || !musicStore.music?.hash"
    @click="handleLike"
  />
</template>
