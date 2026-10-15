import { convertFileSrc } from '@tauri-apps/api/core';
import { computed } from 'vue';

import { PicSize, PlayingOrigin } from '@/constants';
import { useMusicStore } from '@/stores/music';
import { getPic } from '@/utils/music';

/** 获取播放音频封面 */
export function usePlayingCover(size: PicSize = PicSize.Sm) {
  const musicStore = useMusicStore();

  return computed(() => {
    if (!musicStore.music?.cover) return;

    return musicStore.origin === PlayingOrigin.Local
      ? convertFileSrc(musicStore.music.cover)
      : getPic(musicStore.music.cover, size);
  });
}
