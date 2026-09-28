<script lang="ts" setup>
import { convertFileSrc } from '@tauri-apps/api/core';
import { computed, ref, watch } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Image from '@/components/Image.vue';
import Modal from '@/components/Modal.vue';
import { notify } from '@/components/Notification.vue';
import { formatFileSize, invoke, revealPath } from '@/utils/tools';

interface Props {
  path: string;
}

const visible = defineModel({ required: true, default: false });
const { path } = defineProps<Props>();

const isLoading = ref(false);
const musicDetail = ref<MusicDetail>();

const cover = computed(() =>
  musicDetail.value?.cover ? convertFileSrc(musicDetail.value.cover) : '',
);

const handleCancel = () => {
  visible.value = false;
};

watch(visible, async (visible) => {
  if (visible) {
    isLoading.value = true;

    try {
      musicDetail.value = await invoke('music_file_detail', { path });
    } catch {
      notify.error('获取文件详情失败');
    } finally {
      isLoading.value = false;
    }
  } else {
    musicDetail.value = undefined;
  }
});
</script>

<template>
  <Modal v-model="visible" class="w-96" title="歌曲详情" hideFooter @cancel="handleCancel">
    <div v-if="isLoading" class="text-center font-bold leading-[8rem] text-minor">获取中...</div>

    <div v-else-if="!musicDetail" class="text-center font-bold leading-[8rem]">
      未获取到歌曲详情
    </div>

    <div v-else class="space-y-1 px-6 pb-6">
      <div class="flex gap-3">
        <Image class="size-12" :src="cover" />

        <div class="min-w-0 flex-1">
          <div class="music-title cursor-copy truncate" v-copy="musicDetail.artist">
            {{ musicDetail.title }}
          </div>
          <div class="music-artist cursor-copy truncate" v-copy="musicDetail.artist">
            {{ musicDetail.artist }}
          </div>
        </div>
      </div>

      <div>
        专辑：
        <span class="cursor-copy font-bold" v-copy="musicDetail.album">
          {{ musicDetail.album }}
        </span>
      </div>

      <div>
        流派：
        <span class="cursor-copy font-bold" v-copy="musicDetail.genre">
          {{ musicDetail.genre }}
        </span>
      </div>

      <div>
        声道：
        <span class="cursor-copy font-bold" v-copy="musicDetail.channels">
          {{ musicDetail.channels }}
        </span>
      </div>

      <div>
        总比特率：
        <span class="cursor-copy font-bold" v-copy="musicDetail.overall_bitrate">
          {{ musicDetail.overall_bitrate ?? 0 }} kbps
        </span>
      </div>

      <div>
        音频比特率：
        <span class="cursor-copy font-bold" v-copy="musicDetail.audio_bitrate">
          {{ musicDetail.audio_bitrate ?? 0 }} kbps
        </span>
      </div>

      <div>
        采样率：
        <span class="cursor-copy font-bold" v-copy="musicDetail.sample_rate">
          {{ musicDetail.sample_rate ?? 0 }} Hz
        </span>
      </div>

      <div>
        比特深度：
        <span class="cursor-copy font-bold" v-copy="musicDetail.bit_depth">
          {{ musicDetail.bit_depth ?? 0 }} bits
        </span>
      </div>

      <div>
        时长：
        <span class="cursor-copy font-bold" v-copy="musicDetail.duration">
          {{ musicDetail.duration ?? 0 }} s
        </span>
      </div>

      <div>
        文件大小：
        <span class="cursor-copy font-bold" v-copy="musicDetail.size">
          {{ formatFileSize(musicDetail.size) }}
        </span>
      </div>

      <div>
        文件类型：
        <span class="cursor-copy font-bold" v-copy="musicDetail.format">
          {{ musicDetail.format }}
        </span>
      </div>

      <div class="mt-1 flex flex-wrap items-center">
        文件位置：
        <span class="w-0 flex-1 cursor-copy truncate font-bold" v-copy="musicDetail.path">
          {{ musicDetail.path }}
        </span>
        <ActionButton theme="success" @click="revealPath(musicDetail.path)">浏览</ActionButton>
      </div>
    </div>
  </Modal>
</template>
