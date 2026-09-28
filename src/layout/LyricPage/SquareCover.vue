<script lang="ts" setup>
import { convertFileSrc } from '@tauri-apps/api/core';
import { computed } from 'vue';

import Image from '@/components/Image.vue';
import { useMusicStore } from '@/stores/music';
import { useSettingStore } from '@/stores/setting';
import { getPic } from '@/utils/music';
import { PicSize, PlayingOrigin } from '@/utils/params';

const musicStore = useMusicStore();
const settingStore = useSettingStore();

const cover = computed(() => {
  if (!musicStore.music?.cover) return '';

  return musicStore.origin === PlayingOrigin.Online
    ? getPic(musicStore.music.cover, PicSize.Lg)
    : convertFileSrc(musicStore.music.cover);
});
</script>

<template>
  <Image
    class="rounded-3xl"
    :class="settingStore.isFullscreen || settingStore.isMaximized ? 'size-96' : 'size-80'"
    :src="cover"
    :icon-size="96"
  />
</template>
