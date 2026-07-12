import { ListType, SortOrder, SortType } from '@/utils/params'

export const useListStore = defineStore(
  'list',
  () => {
    // 本地列表
    const local = ref<MusicList>({
      info: { id: 'local', cover: '', title: '本地歌曲', artist: '', count: 0, tags: [] },
      list: []
    })
    // 展示列表
    const show = ref<MusicList>({
      info: { id: '', cover: '', title: '', artist: '', count: 0, tags: [] },
      list: []
    })
    // 播放列表
    const play = ref<MusicList>({
      info: { id: '', cover: '', title: '', artist: '', count: 0, tags: [] },
      list: []
    })
    // 我喜欢列表
    const like = ref<{ info: ListInfo; list: ID[] }>({
      info: { id: '', cover: '', title: '', artist: '', count: 0, tags: [] },
      list: []
    })

    const isLoading = ref(false) // 加载状态
    const isInfiniting = ref(false) // 无限加载中
    const isChecking = ref(false) // 框选状态
    const checkedList = ref<any[]>([]) // 框选列表
    const searchList = ref<ListMusic[]>() // 查询结果
    const sortMap = ref<Record<ID, SortInfo>>({}) // 排序列表

    // 列表映射
    const LIST_MAP = {
      [ListType.Local]: local,
      [ListType.Show]: show,
      [ListType.Play]: play
    } as const

    // 添加列表项
    const addList = (type: ListType, newList: ListMusic[], start: boolean = true) => {
      const target = LIST_MAP[type]
      if (!target || newList.length === 0) return 0

      // 去重
      const idSet = new Set(target.value.list.map((music) => music.id))
      const filterList = newList.filter((music) => !idSet.has(music.id))

      target.value.list = start
        ? filterList.concat(target.value.list)
        : target.value.list.concat(filterList)
      target.value.info.count += filterList.length

      return filterList.length
    }

    // 设置列表
    const setList = (type: ListType, newTargetList: MusicList) => {
      const target = LIST_MAP[type]
      if (!target) return

      target.value = newTargetList

      // 如果存在排序则执行一次排序
      if (sortMap.value[target.value.info.id]) handleSort(type)
    }

    //移除列表项
    const removeList = (type: ListType, id: ID) => {
      const target = LIST_MAP[type]
      if (!target || id === '') return

      checkedList.value = checkedList.value.filter((checkedId) => checkedId !== id)
      target.value.list = target.value.list.filter((music) => music.id !== id)
      target.value.info.count -= 1
    }

    // 清空列表
    const clearList = (type: ListType) => {
      const target = LIST_MAP[type]
      if (!target) return

      target.value.list = []
      target.value.info.count = 0

      clearChecked()
    }

    // 重置列表
    const resetList = (type: ListType) => {
      const target = LIST_MAP[type]
      if (!target) return

      target.value = {
        info: { id: '', cover: '', title: '', artist: '', count: 0, tags: [] },
        list: []
      }

      resetChecked()
    }

    // 下一首播放
    const addNextList = (index: number, music: ListMusic) => {
      // 会出现同一首歌曲, 所以 id要不同
      play.value.list.splice(index + 1, 0, { ...music, id: `${music.id}-${crypto.randomUUID()}` })
      play.value.info.count += 1
    }

    // 设置最喜欢列表
    const setLikeList = (newTarget: { info: ListInfo; list: ID[] }) => {
      like.value = newTarget
    }

    const addLikeList = (newId: ID) => {
      like.value.list.push(newId)
      like.value.info.count++
    }

    const removeLikeList = (newId: ID) => {
      like.value.list = like.value.list.filter((id) => id !== newId)
      like.value.info.count--
    }

    const clearLikeList = () => {
      like.value.list = []
      like.value.info.count = 0
    }

    // 切换框选状态
    const toggleChecked = () => {
      isChecking.value = !isChecking.value
      clearChecked()
    }

    // 设置框选列表
    const setChecked = <T>(newList: T[]) => {
      if (!isChecking.value) return

      checkedList.value = newList
    }

    // 处理框选项
    const handleChecked = <T>(checked: T) => {
      if (!isChecking.value || !checked) return

      checkedList.value = checkedList.value.includes(checked)
        ? checkedList.value.filter((checkedId) => checkedId !== checked)
        : checkedList.value.concat(checked)
    }

    // 清空框选列表
    const clearChecked = () => {
      checkedList.value = []
    }

    // 重置框选状态
    const resetChecked = () => {
      isChecking.value = false
      checkedList.value = []
    }

    // 移除框选的列表项
    const removeCheckedList = (type: ListType) => {
      if (!isChecking.value || checkedList.value.length === 0) return

      const target = LIST_MAP[type]
      if (!target) return

      const len = target.value.list.length
      target.value.list = target.value.list.filter((music) => !checkedList.value.includes(music.id))
      target.value.info.count -= len - target.value.list.length

      clearChecked()
    }

    // 搜索
    const handleSearch = (type: ListType, query: string) => {
      const target = LIST_MAP[type]
      if (!target) return

      searchList.value = query
        ? target.value.list.filter(({ title, artist, album }) =>
            [title, artist, album].filter(Boolean).some((item) => item!.includes(query))
          )
        : undefined
    }

    const clearSearch = () => {
      searchList.value = undefined
    }

    // 设置排序信息
    const setSortMap = (id: ID, newSortInfo: SortInfo) => {
      sortMap.value[id] = newSortInfo
    }

    // 排序列表
    const handleSort = (type: ListType) => {
      const target = LIST_MAP[type]
      if (!target) return

      const sortInfo = sortMap.value[target.value.info.id]
      if (!sortInfo) return

      target.value.list.sort((a: ListMusic, b: ListMusic) => {
        const conditions = {
          [SortType.Default]: a.sort - b.sort,
          [SortType.Title]: a.title.localeCompare(b.title),
          [SortType.Artist]: (a.artist || '').localeCompare(b.artist || ''),
          [SortType.Album]: (a.album || '').localeCompare(b.album || ''),
          [SortType.Duration]: a.duration - b.duration
        }

        const result = conditions[sortInfo.type]
        return sortInfo.order === SortOrder.ASC ? result : -result
      })
    }

    return {
      local,
      show,
      play,
      like,
      isLoading,
      isInfiniting,
      isChecking,
      searchList,
      checkedList,
      sortMap,

      addList,
      setList,
      addNextList,
      removeList,
      clearList,
      resetList,
      setLikeList,
      addLikeList,
      removeLikeList,
      clearLikeList,
      toggleChecked,
      handleChecked,
      setChecked,
      clearChecked,
      removeCheckedList,
      handleSearch,
      clearSearch,
      setSortMap,
      handleSort
    }
  },
  {
    persist: {
      key: 'list-store',
      pick: ['local', 'play', 'sortMap']
    }
  }
)
