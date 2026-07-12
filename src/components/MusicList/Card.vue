<script lang="ts" setup>
import Image from '../Image.vue'
import SvgIcon from '../SvgIcon.vue'
import { notify } from '@/components/Notification'
import { useContextMenuStore } from '@/stores/contextMenu'
import { useListStore } from '@/stores/list'
import { useMusicStore } from '@/stores/music'
import { useUserStore } from '@/stores/user'
import { getPic, invoke } from '@/utils/tools'

interface Props {
  data: CardInfo
}
interface Emits {
  click: [e: MouseEvent]
}

const { data } = defineProps<Props>()
const emits = defineEmits<Emits>()

const musicStore = useMusicStore()
const listStore = useListStore()
const userStore = useUserStore()
const contextMenuStore = useContextMenuStore()

const handleContextMenu = (e: MouseEvent) => {
  if (!data.musicInfo) return

  contextMenuStore.show({
    x: e.clientX,
    y: e.clientY,
    options: [
      { label: '播放', prefixIcon: 'Play', onClick: () => {} },
      {
        label: '下一首播放',
        prefixIcon: 'Playlist',
        onClick: () => {
          if (!data.musicInfo) return

          listStore.addNextList(
            listStore.play.list.findIndex((item) => item.id === musicStore.music?.id),
            data.musicInfo
          )
        }
      },
      { divider: true },
      {
        label: '添加到',
        prefixIcon: 'Plus',
        suffixIcon: 'Right',
        disabled: !userStore.userinfo,
        children: userStore.userPlaylist.map((list) => ({
          label: list.name,
          onClick: async () => {
            if (!data.musicInfo) return

            const playlist_tracks_add = await invoke('api_playlist_tracks_add', {
              listId: list.list_create_listid,
              musicList: [{ name: data.musicInfo.title, hash: data.musicInfo.hash }]
            })
            if (playlist_tracks_add?.status !== 1) {
              notify.error('添加失败')
            } else {
              notify.success('添加成功')

              // 如果是添加到我喜欢,同步列表
              if (list.is_def === 2) {
                listStore.addLikeList(data.musicInfo.id)
              }
            }
          }
        }))
      },
      { label: '下载', prefixIcon: 'Download', disabled: true, onClick: () => 'TODO: 下载' }
    ]
  })
}
</script>

<template>
  <div
    class="card-hover relative transition-colors group/card flex h-16 cursor-pointer items-center gap-2 rounded-lg px-2"
    @click="emits('click', $event)"
    @contextmenu.prevent="handleContextMenu">
    <Image class="size-12" :img="getPic(data.cover)" />

    <SvgIcon
      v-if="data.musicInfo"
      class="absolute left-2 top-2 card size-12 bg-black/30 text-neutral-50 flex justify-center items-center transition-opacity opacity-0 group-hover/card:opacity-100"
      name="PlayBold"
      size="20" />

    <div class="flex h-12 w-0 flex-1 flex-col justify-center">
      <div class="music-title">{{ data.title }}</div>
      <div v-if="data.artist" class="music-artist">{{ data.artist }}</div>
    </div>
  </div>
</template>
