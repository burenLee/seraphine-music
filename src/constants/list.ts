/** 添加音频方式 */
export const enum AddMusicType {
  Add,
  Scan,
}

/** 列表类型 */
export const enum ListType {
  Local = 'local',
  Show = 'show',
  Play = 'play',
}

/** 排序方式 */
export const enum SortType {
  Default,
  Title,
  Artist,
  Album,
  Duration,
}

/** 排序顺序 */
export const enum SortOrder {
  ASC,
  DESC,
}

/** 分页尺寸 */
export const enum PageSize {
  Min = 3,
  Default = 10,
  More = 30,
  Max = 300,
}

/** 播放列表类型 */
export const enum PlaylistType {
  Own,
  Favorite,
}

/** 添加播放列表方式 */
export const enum AddPlaylistType {
  Create,
  Import,
}

/** 屏幕像素断点 */
export const enum BreakPoint {
  MD = 1280,
  LG = 1536,
}

/** 列显示数量 */
export const enum ColCount {
  SM = 3,
  MD = 4,
  LG = 5,
}
