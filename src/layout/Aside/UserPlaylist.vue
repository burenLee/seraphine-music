<script lang="ts" setup>
import Image from '@/components/Image.vue'
import Modal from '@/components/Modal.vue'
import { notify } from '@/components/Notification'
import SelectModal from '@/components/SelectModal.vue'
import SlideBar from '@/components/SlideBar.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useContextMenuStore } from '@/stores/contextMenu'
import { useListStore } from '@/stores/list'
import { useMusicStore } from '@/stores/music'
import { useRefreshStore } from '@/stores/refresh'
import { useUserStore } from '@/stores/user'
import { AddPlaylistType, ListType, PageSize, PlaylistType } from '@/utils/params'
import { getPic, getPlayingOrigin, getPrivilegeTags, invoke } from '@/utils/tools'
import { vOnClickOutside } from '@vueuse/components'

const route = useRoute()
const router = useRouter()

const userStore = useUserStore()
const listStore = useListStore()
const musicStore = useMusicStore()
const refreshStore = useRefreshStore()
const contextMenuStore = useContextMenuStore()

const slideOptions: Array<SelectOption<PlaylistType>> = [
  { label: '自建歌单', value: PlaylistType.User },
  { label: '收藏歌单', value: PlaylistType.Collection }
]
const addOptions: Array<SelectOption<AddPlaylistType>> = [
  { label: '新建歌单', value: AddPlaylistType.Add, prefixIcon: 'Plus' },
  { label: '导入歌单', value: AddPlaylistType.Import, prefixIcon: 'Link', disabled: true }
]

let likePlaylistGid = ''

const slideSelection = ref(slideOptions[0])
const userPlaylist = ref<Playlist[]>([])
const addSelectVisible = ref(false)
const contextMenuPlaylist = ref<Playlist>()
const addModalVisible = ref(false)
const addLoading = ref(false)
const addForm = ref({ name: '', isPri: 0 })
const delModalVisible = ref(false)
const delLoading = ref(false)

// 歌单切换效果
const transition = computed(() =>
  slideSelection.value.value === PlaylistType.User ? 'slide-right' : 'slide-left'
)

const getUserPlaylist = async () => {
  const api_playlist_user = await invoke('api_playlist_user', { pageSize: 300 })
  if (api_playlist_user?.status !== 1) return

  userPlaylist.value = api_playlist_user.data.info.map((playlist) => {
    if (playlist.is_def === 1) {
      // 默认收藏
      playlist.sort = 0
    } else if (playlist.is_def === 2) {
      // 我喜欢
      playlist.sort = 1

      likePlaylistGid = playlist.list_create_gid
      listStore.setLikeList({
        info: {
          id: playlist.list_create_listid,
          cover: '',
          title: '我喜欢',
          artist: '',
          count: playlist.count,
          tags: []
        },
        list: []
      })
    } else {
      // 默认0, +2确保 默认收藏/我喜欢 在顶部
      playlist.sort += 2
    }

    return playlist
  })

  userPlaylist.value.sort((a, b) => a.sort - b.sort)
  userStore.setUserPlaylist(userPlaylist.value.filter((list) => list.type === PlaylistType.User))
}

const clearUserPlaylist = () => {
  userPlaylist.value = []
}

const getLikePlaylist = async () => {
  let page = 1
  const list: ID[] = []

  while (list.length < listStore.like.info.count) {
    const playlist_tracks_all = await invoke('api_playlist_tracks_all', {
      gid: likePlaylistGid,
      page: page++,
      pageSize: PageSize.Max
    })
    if (playlist_tracks_all?.status !== 1) break

    list.push(...playlist_tracks_all.data.songs.map((song) => song.fileid))
  }

  listStore.like.info.count = list.length
  listStore.setLikeList({ info: listStore.like.info, list })
}

const handleAddSelect = (type: AddPlaylistType) => {
  switch (type) {
    case AddPlaylistType.Add:
      addModalVisible.value = true
      break
    case AddPlaylistType.Import:
      break
  }

  addSelectVisible.value = false
}

const handlePlay = async (playlist: Playlist) => {
  if (playlist.list_create_listid === listStore.play.info.id) return

  const info: ListInfo = {
    id: playlist.list_create_listid,
    cover: playlist.pic,
    title: playlist.name,
    artist: playlist.list_create_username,
    tags: playlist.musiclib_tags.map((tag) => tag.tag_name),
    count: playlist.count
  }
  const list: ListMusic[] = []

  let page = 1
  let total = 0

  // 获取全部歌曲
  while (total < playlist.count) {
    const playlist_tracks_all = await invoke('api_playlist_tracks_all', {
      gid: playlist.list_create_gid,
      page: page++,
      pageSize: PageSize.Max
    })
    if (playlist_tracks_all?.status !== 1) break

    total += playlist_tracks_all.data.songs.length

    for (const song of playlist_tracks_all.data.songs) {
      if (!song.hash) continue

      const [artist, title] = song.name.split('-')

      list.push({
        id: song.fileid,
        path: null,
        hash: song.hash,
        title: title.trim(),
        artist: artist.trim(),
        album: song.albuminfo.name,
        cover: song.trans_param.union_cover,
        duration: song.timelen / 1000,
        sort: song.sort,
        privilegeTags: getPrivilegeTags(song.privilege, song.download[0].pay_type)
      })
    }
  }

  listStore.setList(ListType.Play, { info, list })
  musicStore.setMusic(list[0], { origin: getPlayingOrigin(list[0]) })
}

const handleContextMenu = (e: MouseEvent, playlist: Playlist) => {
  contextMenuStore.show({
    x: e.clientX,
    y: e.clientY,
    options: [
      { label: '播放', prefixIcon: 'Play', onClick: () => handlePlay(playlist) },
      { label: '分享', prefixIcon: 'Share', disabled: true, onClick: () => 'TODO: 分享' },
      { divider: true },
      {
        label: playlist.is_pri === 0 ? '设为隐私歌单' : '取消设为隐私歌单',
        prefixIcon: playlist.is_pri === 0 ? 'EyeClosed' : 'Eye',
        disabled: true,
        onClick: () => 'TODO: 设置隐私歌单'
      },
      { divider: true },
      { label: '重命名', prefixIcon: 'Pen', disabled: true, onClick: () => 'TODO: 重命名' },
      {
        label: '删除',
        prefixIcon: 'Bin',
        disabled: !!playlist.is_def,
        onClick: () => {
          contextMenuPlaylist.value = playlist
          delModalVisible.value = true
        }
      }
    ]
  })
}

const handlePlaylistClick = async (playlist: Playlist) => {
  if (route.query.id === playlist.list_create_gid) return

  const routeInfo = { path: '/user-playlist-table', query: { id: playlist.list_create_gid } }

  if (route.path === '/user-playlist-table') {
    router.replace(routeInfo)
    refreshStore.refresh()
  } else {
    router.push(routeInfo)
  }
}

const handleAddConfirm = async () => {
  if (addLoading.value) return
  if (!userStore.userinfo) return notify.error('暂不支持本地歌单')
  if (!addForm.value.name) return notify.error('请填写歌单名称')

  addLoading.value = true

  const api_playlist_add = await invoke('api_playlist_add', {
    name: addForm.value.name,
    isPri: addForm.value.isPri,
    listCreateUserid: userStore.userinfo.userid
  })
  if (api_playlist_add?.status !== 1) {
    addLoading.value = false
    return
  }

  notify.success('创建歌单成功')
  handleAddCancel()
  getUserPlaylist()
}

const handleAddCancel = () => {
  addLoading.value = false
  addForm.value = { name: '', isPri: 0 }
  addModalVisible.value = false
}

const handleDelConfirm = async () => {
  if (delLoading.value) return
  if (!contextMenuPlaylist.value) return notify.error('请选择要删除的歌单')

  delLoading.value = true

  const api_playlist_del = await invoke('api_playlist_del', {
    listid: contextMenuPlaylist.value.list_create_listid
  })
  if (api_playlist_del?.status !== 1) {
    delLoading.value = false
    return
  }

  notify.success('删除歌单成功')
  handleDelCancel()

  if (contextMenuPlaylist.value.list_create_gid === route.query.id) {
    router.back()
  } else {
    getUserPlaylist()
  }
}

const handleDelCancel = () => {
  delLoading.value = false
  delModalVisible.value = false
}

// 监听用户信息来获取或清空歌单
watch(
  () => userStore.userinfo,
  async (userinfo) => {
    if (userinfo) {
      await getUserPlaylist()
      await getLikePlaylist()
    } else {
      clearUserPlaylist()
      listStore.clearLikeList()
    }
  },
  { immediate: true }
)
</script>

<template>
  <div class="flex flex-col">
    <div class="flex items-center mx-4 justify-between card pr-1">
      <SlideBar
        class="border-0 bg-transparent"
        :options="slideOptions"
        :selection="slideSelection"
        @change="slideSelection = $event" />

      <div class="relative" v-on-click-outside="() => (addSelectVisible = false)">
        <SvgIcon
          class="action-icon card-hover transition-colors rounded-lg hover:text-foreground"
          name="Plus"
          @click="addSelectVisible = !addSelectVisible" />

        <SelectModal
          class="absolute -right-1 top-full"
          transition="zoom-top-right"
          :visible="addSelectVisible"
          :options="addOptions"
          @select="handleAddSelect" />
      </div>
    </div>

    <!-- 歌单列表 -->
    <Transition :name="transition" mode="out-in">
      <div v-if="slideSelection.value === PlaylistType.User" class="flex-1 h-0 p-4 overflow-y-auto">
        <div
          v-for="playlist in userPlaylist.filter((list) => list.type === PlaylistType.User)"
          :key="playlist.list_create_gid"
          class="flex cursor-pointer items-center gap-2 rounded-lg px-3 py-1 transition-colors"
          :class="route.query.id === playlist.list_create_gid ? 'card-actived' : 'card-hover'"
          @click="handlePlaylistClick(playlist)"
          @contextmenu.prevent="handleContextMenu($event, playlist)">
          <Image class="size-8" :img="getPic(playlist.pic)" />

          <div class="w-0 flex-1 truncate font-bold">{{ playlist.name }}</div>
        </div>
      </div>

      <div
        v-else-if="slideSelection.value === PlaylistType.Collection"
        class="flex-1 h-0 overflow-y-auto p-4">
        <div
          v-for="playlist in userPlaylist.filter((list) => list.type === PlaylistType.Collection)"
          :key="playlist.list_create_gid"
          class="flex cursor-pointer items-center gap-2 rounded-lg px-3 py-1 transition-colors"
          :class="route.query.id === playlist.list_create_gid ? 'card-actived' : 'card-hover'"
          @click="handlePlaylistClick(playlist)"
          @contextmenu.prevent="handleContextMenu($event, playlist)">
          <Image class="size-8" :img="getPic(playlist.pic)" />

          <div class="w-0 flex-1 truncate font-bold">{{ playlist.name }}</div>
        </div>
      </div>
    </Transition>

    <!-- 新增歌单弹窗 -->
    <Modal
      class="w-80"
      v-model="addModalVisible"
      title="新建歌单"
      confirmLabel="新建"
      @cancel="handleAddCancel"
      @confirm="handleAddConfirm">
      <form class="px-4" @submit.prevent="handleAddConfirm">
        <input
          class="h-8 w-full rounded-md border border-minor bg-background px-2 leading-8"
          v-model="addForm.name"
          placeholder="请输入歌单名称" />

        <label class="flex cursor-pointer items-center gap-1 pt-2">
          <input type="checkbox" v-model="addForm.isPri" :true-value="1" :false-value="0" />
          <span class="text-sm">设为隐私歌单</span>
          <SvgIcon name="Info" size="16" title="仅自己可见, 且无法分享" />
        </label>
      </form>
    </Modal>

    <!-- 确认删除弹窗 -->
    <Modal
      class="w-80"
      v-model="delModalVisible"
      title="删除歌单"
      @cancel="handleDelCancel"
      @confirm="handleDelConfirm">
      <div class="px-4">
        确认删除歌单 <span class="font-bold">{{ contextMenuPlaylist?.name }}</span> ?
      </div>
    </Modal>
  </div>
</template>
