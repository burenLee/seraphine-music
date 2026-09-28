<script lang="ts" setup>
import { onMounted, onUnmounted, provide } from 'vue';
import { useRoute } from 'vue-router';

import MusicActions from '@/components/MusicTable/MusicActions.vue';
import MusicHeader from '@/components/MusicTable/MusicHeader.vue';
import MusicTable from '@/components/MusicTable/MusicTable.vue';
import { notify } from '@/components/Notification.vue';
import { defaultInfo, useListStore } from '@/stores/list';
import { getPrivilegeTags } from '@/utils/music';
import { ApiInvokeStatus, ListType, PageSize } from '@/utils/params';
import { invoke } from '@/utils/tools';

const listType = ListType.Show;
provide('listType', listType);

const route = useRoute('TopPlaylistTable');
const listStore = useListStore();

const handleLoad = async () => {
  listStore.isHeaderLoading = true;
  listStore.isTableLoading = true;

  try {
    const gid = route.params.gid;

    const playlist_detail = await invoke('api_playlist_detail', { gids: [gid] });
    if (playlist_detail.status !== ApiInvokeStatus.Success || !playlist_detail.data[0]) {
      notify.error('获取歌单信息失败');
    } else {
      const playlist = playlist_detail.data[0];

      const info: ListInfo = {
        ...defaultInfo,
        id: playlist.list_create_listid,
        cover: playlist.pic,
        title: playlist.name,
        artist: playlist.list_create_username,
        tags: playlist.musiclib_tags.map((tag) => tag.tag_name),
        count: playlist.count,
        gid: playlist.list_create_gid,
      };
      listStore.setListInfo(listType, info);
      listStore.isHeaderLoading = false;

      let page = 1;
      let total = 0;
      const list: MusicInfo[] = [];

      while (total < playlist.count) {
        const playlist_tracks_all = await invoke('api_playlist_tracks_all', {
          gid,
          page: page++,
          pageSize: PageSize.Max,
        });
        if (playlist_tracks_all.status !== ApiInvokeStatus.Success) break;

        total += playlist_tracks_all.data.count;

        playlist_tracks_all.data.songs.forEach((song) => {
          if (song.shield !== undefined) return;

          const [artist, title] = song.name.split(' - ');

          list.push({
            id: song.mixsongid,
            path: null,
            hash: song.hash,
            cover: song.trans_param.union_cover,
            title: title,
            artist: artist,
            album: song.albuminfo.name,
            duration: song.timelen / 1000,
            sort: song.sort,
            privilegeTags: getPrivilegeTags(song.privilege, song.download[0].pay_type),
            fileId: song.fileid,
          });
        });
      }

      listStore.setListRaw(listType, list);
    }
  } catch {
    notify.error('获取歌单歌曲失败');
  } finally {
    listStore.isTableLoading = false;
  }
};

onMounted(handleLoad);
onUnmounted(() => {
  listStore.resetList(listType);
  listStore.isHeaderLoading = true;
  listStore.isTableLoading = true;
});
</script>

<template>
  <div class="relative flex h-full w-full flex-col space-y-3 pt-4">
    <MusicHeader />
    <MusicActions />
    <MusicTable class="h-0 flex-1" />
  </div>
</template>
