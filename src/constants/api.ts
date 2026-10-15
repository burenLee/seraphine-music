/**  invoke 的 api接口返回的状态码 */
export const enum InvokeApiStatus {
  Success = 1,
}

/** 查询类型 */
export const enum SearchType {
  Song = 'song',
  Album = 'album',
  Author = 'author',
  Mv = 'mv',
  Lyric = 'lyric',
  Special = 'special',
  Collect = 'collect',
}

/** 歌手区域配置   */
export const AreaTypes = [
  { type: 1, musician: 0, title: '华语' },
  { type: 2, musician: 0, title: '欧美' },
  { type: 5, musician: 0, title: '日本' },
  { type: 6, musician: 0, title: '韩国' },
  { type: 7, musician: 0, title: '粤语' },
  { type: 8, musician: 0, title: '闽南语' },
  { type: 0, musician: 3, title: '音乐人' },
  { type: 4, musician: 0, title: '其他' },
] as const;

/** 歌手性别配置 */
export const SexTypes = [
  { key: 0, value: '全部' },
  { key: 1, value: '男' },
  { key: 2, value: '女' },
  { key: 3, value: '组合' },
] as const;
