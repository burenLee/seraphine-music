<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue'
import { notify } from '@/components/Notification'
import SvgIcon from '@/components/SvgIcon.vue'
import VirtualList from '@/components/VirtualList.vue'
import { useContextMenuStore } from '@/stores/contextMenu'
import { useListStore } from '@/stores/list'
import { useMusicStore } from '@/stores/music'
import { useUserStore } from '@/stores/user'
import { ListType } from '@/utils/params'
import { getFullName, getPlayingOrigin, invoke } from '@/utils/tools'
import { vOnClickOutside } from '@vueuse/components'

provide('listType', ListType.Play)

const listStore = useListStore()
const musicStore = useMusicStore()
const userStore = useUserStore()
const contextMenuStore = useContextMenuStore()

const tableColumns: TableColumn[] = [
  { key: 'index', slot: true, width: '3rem', padding: 0 },
  { key: 'info', slot: true, width: 'auto' }
]

const listVisible = ref(false)
const contextMenuMusic = ref<ListMusic>()
const removeVisible = ref(false)

const isPlaying = (row: ListMusic) => {
  return row.id === musicStore.music?.id
}

const isLike = (id: ID) => {
  return listStore.like.list.includes(id)
}

const handleContextMenu = (e: MouseEvent, music: ListMusic) => {
  contextMenuMusic.value = music

  contextMenuStore.show({
    x: e.clientX,
    y: e.clientY,
    options: [
      { label: '播放', prefixIcon: 'Play', onClick: () => handlePlay(music) },
      { divider: true },
      {
        label: '添加到',
        prefixIcon: 'Plus',
        suffixIcon: 'Right',
        disabled: !userStore.userinfo,
        children: userStore.userPlaylist.map((list) => ({
          label: list.name,
          onClick: async () => {
            const playlist_tracks_add = await invoke('api_playlist_tracks_add', {
              listId: list.list_create_listid,
              musicList: [{ name: music.title, hash: music.hash }]
            })
            if (playlist_tracks_add?.status !== 1) {
              notify.error('添加失败')
            } else {
              notify.success('添加成功')

              // 如果是添加到我喜欢,同步列表
              if (list.is_def === 2) {
                listStore.addLikeList(music.id)
              }
            }
          }
        }))
      },
      { label: '下载', prefixIcon: 'Download', disabled: true, onClick: () => 'TODO: 下载' },
      { divider: true },
      { label: '从列表中删除', prefixIcon: 'Bin', onClick: () => (removeVisible.value = true) }
    ]
  })
}

const handlePlay = async (music: ListMusic) => {
  musicStore.setMusic(music, { origin: getPlayingOrigin(music) })
}

const handleLike = async (music: ListMusic) => {
  if (isLike(music.id)) {
    const playlist_tracks_del = await invoke('api_playlist_tracks_del', {
      listId: listStore.show.info.id,
      fileIds: [music.id]
    })
    if (playlist_tracks_del?.status !== 1) return

    listStore.removeLikeList(music.id)
  } else {
    const playlist_tracks_add = await invoke('api_playlist_tracks_add', {
      listId: +listStore.like.info.id,
      musicList: [{ name: music.title, hash: music.hash }]
    })
    if (playlist_tracks_add?.status !== 1) return

    listStore.addLikeList(music.id)
  }
}
</script>

<template>
  <div v-on-click-outside="() => (listVisible = false)">
    <SvgIcon class="action-icon" name="Musiclist" size="20" @click="listVisible = !listVisible" />

    <Transition name="slide-page-left">
      <div
        v-if="listVisible"
        class="card fixed bottom-[var(--playbar-height)] right-0 top-[var(--header-height)] z-40 my-4 flex w-80 flex-col bg-background shadow-md shadow-shadow">
        <div class="mt-4 px-4">
          <span class="font-bold text-lg">播放队列</span>
          <span class="ml-2">共 {{ listStore.play.info.count }} 首</span>
        </div>

        <div class="mt-2 flex items-center justify-between px-4">
          <div>
            <span>来源: </span>
            <span v-if="listStore.play.info" class="cursor-pointer font-bold">
              {{ listStore.play.info.title }}
            </span>
          </div>

          <div>
            <ActionButton
              class="px-1 h-6"
              theme="error"
              prefix-icon="Bin"
              @click="listStore.resetList(ListType.Play)">
              清空
            </ActionButton>
          </div>
        </div>

        <VirtualList
          ref="musicTableRef"
          class="mt-4 h-0 w-full flex-1 pl-2 pr-0.5"
          :line-height="40"
          :columns="tableColumns"
          :list="listStore.play.list"
          :checked-list="listStore.checkedList"
          :isChecking="listStore.isChecking"
          @contextmenu="handleContextMenu"
          @lineDblClick="handlePlay">
          <template #index="row">
            <SvgIcon v-if="isPlaying(row)" class="text-info" name="Music" />
            <div v-else class="truncate text-center text-minor">{{ row.index + 1 }}</div>
          </template>

          <template #info="row">
            <div class="flex">
              <div
                class="w-0 flex-1 truncate font-bold leading-10"
                :class="isPlaying(row) ? 'text-info' : ''">
                {{ getFullName(row) }}
              </div>

              <div
                v-if="!listStore.isChecking"
                class="hidden items-center pl-2 group-hover/line:flex"
                @dblclick.stop
                @contextmenu.stop>
                <SvgIcon class="action-icon" name="Play" @click="handlePlay(row)" />
                <SvgIcon
                  v-if="userStore.userinfo && row.hash"
                  class="action-icon"
                  :class="isLike(row.id) ? 'text-error' : ''"
                  :name="isLike(row.id) ? 'HeartBold' : 'Heart'"
                  size="18"
                  @click="handleLike(row)" />
                <SvgIcon class="action-icon" name="More" @click="handleContextMenu($event, row)" />
              </div>
            </div>
          </template>
        </VirtualList>
      </div>
    </Transition>
  </div>
</template>
