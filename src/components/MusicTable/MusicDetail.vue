<script lang="ts" setup>
import ActionButton from '../ActionButton.vue'
import Image from '../Image.vue'
import Modal from '../Modal.vue'
import { notify } from '../Notification.tsx'
import { formatFileSize, invoke } from '@/utils/tools'
import { convertFileSrc } from '@tauri-apps/api/core'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'

interface Props {
  path: string
}

const visible = defineModel({ required: true, default: false })
const { path } = defineProps<Props>()

const isLoading = ref(false)
const musicDetail = ref<MusicDetail>()

const handleCancel = () => {
  visible.value = false
}

const handleCopy = async (txt: string | number | null) => {
  if (txt === '' || txt === null) return

  try {
    await writeText(String(txt))
    notify.success('复制成功')
  } catch (error) {
    console.error('复制失败', error)
  }
}

const handleView = () => {
  if (!musicDetail.value?.path) return

  invoke('music_file_open', { path: musicDetail.value.path })
}

watch(visible, async (visible) => {
  if (visible) {
    isLoading.value = true

    const get_detail = await invoke('music_file_detail', { filePath: path })
    if (!get_detail) {
      isLoading.value = false
      return
    }

    musicDetail.value = get_detail
    isLoading.value = false
  } else {
    musicDetail.value = undefined
    isLoading.value = false
  }
})
</script>

<template>
  <Modal
    v-model="visible"
    class="w-96"
    title="歌曲详情"
    hideConfirm
    cancelLabel="返回"
    @cancel="handleCancel">
    <div v-if="isLoading" class="font-bold text-minor text-center leading-[8rem]">获取中...</div>

    <div v-else-if="!musicDetail" class="font-bold text-center leading-[8rem]">
      未获取到歌曲详情
    </div>

    <div v-else class="px-4">
      <div class="flex gap-3">
        <Image class="size-12" :img="musicDetail.cover ? convertFileSrc(musicDetail.cover) : ''" />

        <div>
          <div class="music-title cursor-copy" @click="handleCopy(musicDetail.title)">
            {{ musicDetail.title }}
          </div>
          <div class="music-artist cursor-copy" @click="handleCopy(musicDetail.artist)">
            {{ musicDetail.artist }}
          </div>
        </div>
      </div>

      <div class="mt-2">
        专辑：<span class="font-bold cursor-copy" @click="handleCopy(musicDetail.album)">
          {{ musicDetail.album || '未知' }}
        </span>
      </div>

      <div class="mt-1">
        流派：<span class="font-bold cursor-copy" @click="handleCopy(musicDetail.genre)">
          {{ musicDetail.genre || '未知' }}
        </span>
      </div>

      <div class="mt-1">
        声道：
        <span class="font-bold cursor-copy" @click="handleCopy(musicDetail.channels)">
          {{ musicDetail.channels ?? '未知' }}
        </span>
      </div>

      <div class="mt-1">
        总比特率：
        <span class="font-bold cursor-copy" @click="handleCopy(musicDetail.overall_bitrate)">
          {{ musicDetail.overall_bitrate ?? '未知' }} kbps
        </span>
      </div>

      <div class="mt-1">
        音频比特率：
        <span class="font-bold cursor-copy" @click="handleCopy(musicDetail.audio_bitrate)">
          {{ musicDetail.audio_bitrate ?? '未知' }} kbps
        </span>
      </div>

      <div class="mt-1">
        采样率：
        <span class="font-bold cursor-copy" @click="handleCopy(musicDetail.sample_rate)">
          {{ musicDetail.sample_rate ?? '未知' }} Hz
        </span>
      </div>

      <div class="mt-1">
        比特深度：
        <span class="font-bold cursor-copy" @click="handleCopy(musicDetail.bit_depth)">
          {{ musicDetail.bit_depth ?? '未知' }} bits
        </span>
      </div>

      <div class="mt-1">
        时长：
        <span class="font-bold cursor-copy" @click="handleCopy(musicDetail.duration)">
          {{ musicDetail.duration ?? '未知' }} s
        </span>
      </div>

      <div class="mt-1">
        文件大小：
        <span class="font-bold cursor-copy" @click="handleCopy(musicDetail.size)">
          {{ formatFileSize(musicDetail.size) }}
        </span>
      </div>

      <div class="mt-1">
        文件类型：
        <span class="font-bold cursor-copy" @click="handleCopy(musicDetail.format)">
          {{ musicDetail.format }}
        </span>
      </div>

      <div class="flex items-center flex-wrap mt-1">
        文件位置：
        <span
          class="font-bold cursor-copy w-0 flex-1 truncate"
          @click="handleCopy(musicDetail.path)">
          {{ musicDetail.path }}
        </span>
        <ActionButton theme="success" @click="handleView">浏览</ActionButton>
      </div>
    </div>
  </Modal>
</template>
