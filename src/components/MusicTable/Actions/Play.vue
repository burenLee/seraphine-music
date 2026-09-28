<script lang="ts" setup>
import { computed, inject } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import { useListStore } from '@/stores/list';
import { useMusicStore } from '@/stores/music';
import { getOrigin } from '@/utils/music';
import { ListType } from '@/utils/params';

const listType = inject<ListType>('listType', ListType.Show);

const listStore = useListStore();
const musicStore = useMusicStore();

const musicList = computed(() => listStore[listType]);

const handlePlay = () => {
  if (musicList.value.list.length === 0) return;

  let music: MusicInfo;
  if (listStore.play.info.id === musicList.value.info.id) {
    // 同一列表：使用播放列表
    if (musicStore.isPlaying) return;
    if (musicStore.music) {
      musicStore.play();
      return;
    }

    music = listStore.play.list[0];
  } else {
    // 不同列表：使用target列表
    music = musicList.value.list[0];
    listStore.setList(ListType.Play, {
      info: musicList.value.info,
      list: [...musicList.value.list],
    });
  }

  musicStore.setMusic(music, { origin: getOrigin(music) });
};
</script>

<template>
  <ActionButton prefix-icon="Play" @click="handlePlay">播放</ActionButton>
</template>
