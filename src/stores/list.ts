import { defineStore } from 'pinia';
import { ref } from 'vue';

import { notify } from '@/components/Notification.vue';
import { ListType, SortOrder, SortType } from '@/utils/params';

import { useMusicStore } from './music';

export const defaultInfo: ListInfo = {
  id: '',
  cover: '',
  title: '',
  artist: '',
  count: 0,
  playCount: 0,
};

/** 列表配置 */
export const useListStore = defineStore(
  'list',
  () => {
    // 本地列表
    const local = ref<MusicList>({
      info: { ...defaultInfo, id: 'local', title: '本地歌曲' },
      list: [],
    });
    // 展示列表
    const show = ref<MusicList>({ info: { ...defaultInfo }, list: [] });
    // 播放列表
    const play = ref<MusicList>({ info: { ...defaultInfo }, list: [] });
    // 喜欢列表
    const like = ref<MusicList<Map<number, ID>>>({ info: { ...defaultInfo }, list: new Map() });

    const isHeaderLoading = ref(true); // 页头加载中
    const isTableLoading = ref(true); // 表格加载中
    const isInfiniting = ref(false); // 无限加载中
    const isInfinited = ref(false); // 无限加载完毕
    const isChecking = ref(false); // 框选中
    const checkedList = ref<any[]>([]); // 框选列表
    const searchList = ref<MusicInfo[]>(); // 查询结果, undefined代表不在查询中, []代表查询结果为空
    const sortMap = ref<Record<ID, SortInfo>>({}); // 排序列表

    // 列表映射
    const LIST_MAP = {
      [ListType.Local]: local,
      [ListType.Show]: show,
      [ListType.Play]: play,
    } as const;

    const musicStore = useMusicStore();

    /** 设置列表 */
    const setList = (type: ListType, newMusicList: MusicList) => {
      const musicList = LIST_MAP[type];

      musicList.value = newMusicList;
      sort(type);
    };

    /** 设置列表属性 */
    const setListInfo = (type: ListType, newInfo: ListInfo) => {
      const musicList = LIST_MAP[type];

      musicList.value.info = newInfo;
    };

    /** 设置列表项 */
    const setListRaw = (type: ListType, newList: MusicInfo[]) => {
      if (newList.length === 0) return;
      const musicList = LIST_MAP[type];

      musicList.value.list = newList;
      sort(type);
    };

    /**
     * 添加列表项
     * @returns 实际添加的数量
     */
    const addList = (type: ListType, newList: MusicInfo[]) => {
      if (newList.length === 0) return 0;
      const musicList = LIST_MAP[type];

      const oldIds = new Set(musicList.value.list.map((music) => music.id));
      const filterList = newList.filter((newMusic) => !oldIds.has(newMusic.id));

      musicList.value.list.push(...filterList);
      musicList.value.info.count += filterList.length;
      sort(type);

      return filterList.length;
    };

    /**
     * 添加列表项
     * @description 不做 `去重` 和 `同步count` 的操作
     */
    const addListRaw = (type: ListType, newList: MusicInfo[]) => {
      if (newList.length === 0) return;
      const musicList = LIST_MAP[type];

      musicList.value.list.push(...newList);
      sort(type);
    };

    /** 移除列表项 */
    // const removeList = (type: ListType, id: ID) => {
    //   const musicList = LIST_MAP[type];

    //   const index = musicList.value.list.findIndex((music) => music.id === id);
    //   if (index === -1) return;

    //   musicList.value.list.splice(index, 1);
    //   musicList.value.info.count--;
    // };

    /** 移除列表项 */
    const removeList = (type: ListType, ids: ID[]) => {
      const musicList = LIST_MAP[type];

      const filterList = musicList.value.list.filter((music) => !ids.includes(music.id));

      musicList.value.list = filterList;
      musicList.value.info.count = filterList.length;

      clearCheckedList();

      // 如果删除当前的是播放歌曲,则清除播放
      if (musicStore.music && ids.includes(musicStore.music.id)) musicStore.setMusic(undefined);
    };

    /** 清空列表项 */
    const clearList = (type: ListType) => {
      const musicList = LIST_MAP[type];

      // 如果删除当前的是播放歌曲,则清除播放
      if (musicList.value.list.some((item) => item.id === musicStore.music?.id))
        musicStore.setMusic(undefined);

      musicList.value.list.length = 0;
      musicList.value.info.count = 0;

      clearCheckedList();
    };

    /** 重置列表 */
    const resetList = (type: ListType) => {
      const musicList = LIST_MAP[type];

      // 如果删除当前的是播放歌曲,则清除播放
      if (musicList.value.list.some((item) => item.id === musicStore.music?.id))
        musicStore.setMusic(undefined);

      musicList.value.list.length = 0;
      musicList.value.info = { ...defaultInfo };

      resetChecked();
    };

    /** 设置喜欢列表属性 */
    const setLikeListInfo = (newInfo: ListInfo) => {
      like.value.info = newInfo;
    };

    /** 设置喜欢列表项 */
    const setLikeListRaw = (newList: MusicInfo[]) => {
      like.value.list = new Map(newList.map((item) => [Number(item.id), item.fileId || '']));
    };

    /** 添加喜欢列表项 */
    const addLikeList = (newList: MusicInfo[]) => {
      newList.forEach((item) => like.value.list.set(Number(item.id), item.fileId || ''));
    };

    /** 移除喜欢列表项 */
    const removeLikeList = (newList: MusicInfo[]) => {
      newList.forEach((item) => like.value.list.delete(Number(item.id)));
    };

    /** 清空喜欢列表项 */
    const clearLikeList = () => {
      like.value.list.clear();
      like.value.info.count = 0;
    };

    /** 重置喜欢列表 */
    const resetLikeList = () => {
      like.value.list.clear();
      like.value.info = { ...defaultInfo };
    };

    /** 下一首播放 */
    const addNextPlay = (newMusic: MusicInfo) => {
      const playingIndex = play.value.list.findIndex((item) => item.id === musicStore.music?.id);

      // 会出现同一首歌曲, 所以 id 要不同
      // 后面操作这条数据的时候去掉 ` - `及后面的数据
      if (play.value.list.some((item) => item.id === newMusic.id)) {
        newMusic.id = `${newMusic.id} - ${crypto.randomUUID()}`;
      }

      play.value.list.splice(playingIndex + 1, 0, newMusic);
      play.value.info.count++;
      notify.success('已添加到下一首播放');
    };

    /** 切换框选状态 */
    const toggleChecked = () => {
      isChecking.value = !isChecking.value;
      clearCheckedList();
    };

    /** 设置框选列表项 */
    const setCheckedList = <T>(newList: T[]) => {
      if (!isChecking.value) return;

      checkedList.value = newList;
    };

    /** 添加/删除框选项 */
    const handleChecked = <T>(newChecked: T) => {
      if (!isChecking.value) return;

      const index = checkedList.value.findIndex((checked) => checked === newChecked);
      if (index === -1) {
        checkedList.value.push(newChecked);
      } else {
        checkedList.value.splice(index, 1);
      }
    };

    /** 清空框选项 */
    const clearCheckedList = () => {
      checkedList.value.length = 0;
    };

    /** 重置框选状态 */
    const resetChecked = () => {
      isChecking.value = false;
      checkedList.value.length = 0;
    };

    /** 移除框选的列表项 */
    const removeListChecked = (type: ListType) => {
      if (!isChecking.value || checkedList.value.length === 0) return;
      const musicList = LIST_MAP[type];

      const filterList = musicList.value.list.filter(
        (music) => !checkedList.value.includes(music.id),
      );

      musicList.value.list = filterList;
      musicList.value.info.count = filterList.length;

      clearCheckedList();
    };

    /** 列表搜索 */
    const search = (type: ListType, searchQuery: string) => {
      const musicList = LIST_MAP[type];
      const query = searchQuery.trim();

      if (query) {
        const list = [];
        for (const music of musicList.value.list) {
          if (music.title.includes(query)) {
            list.push(music);
            continue;
          }

          if (music.artist?.includes(query)) {
            list.push(music);
            continue;
          }

          if (music.album?.includes(query)) {
            list.push(music);
          }
        }

        searchList.value = list;
      } else {
        searchList.value = undefined;
      }
    };

    /** 清除搜索状态 */
    const clearSearch = () => {
      searchList.value = undefined;
    };

    /** 设置排序信息 */
    const setSortMap = (id: ID, newSortInfo: SortInfo) => {
      sortMap.value[id] = newSortInfo;
    };

    /** 列表排序 */
    const sort = (type: ListType) => {
      const musicList = LIST_MAP[type];

      let sortInfo = sortMap.value[musicList.value.info.id];
      // 如果不存在就赋默认值
      if (!sortInfo) {
        // 本地列表默认使用倒序
        const isLocalList = musicList.value.info.id === local.value.info.id;
        sortInfo = { type: SortType.Default, order: isLocalList ? SortOrder.DESC : SortOrder.ASC };
        setSortMap(musicList.value.info.id, sortInfo);
      }

      musicList.value.list.sort((a: MusicInfo, b: MusicInfo) => {
        const conditions = {
          [SortType.Default]: a.sort - b.sort,
          [SortType.Title]: a.title.localeCompare(b.title),
          [SortType.Artist]: (a.artist || '').localeCompare(b.artist || ''),
          [SortType.Album]: (a.album || '').localeCompare(b.album || ''),
          [SortType.Duration]: a.duration - b.duration,
        };

        const sort = conditions[sortInfo.type];
        return sortInfo.order === SortOrder.ASC ? sort : -sort;
      });
    };

    return {
      local,
      show,
      play,
      like,
      isHeaderLoading,
      isTableLoading,
      isInfiniting,
      isInfinited,
      isChecking,
      checkedList,
      searchList,
      sortMap,

      setList,
      setListInfo,
      setListRaw,
      addList,
      addListRaw,
      removeList,
      clearList,
      resetList,
      setLikeListInfo,
      setLikeListRaw,
      addLikeList,
      removeLikeList,
      clearLikeList,
      resetLikeList,
      addNextPlay,
      toggleChecked,
      setCheckedList,
      handleChecked,
      clearCheckedList,
      resetChecked,
      removeListChecked,
      search,
      clearSearch,
      setSortMap,
      sort,
    };
  },
  {
    persist: {
      key: 'list-store',
      pick: ['local', 'play', 'sortMap'],
    },
  },
);
