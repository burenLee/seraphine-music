import { createPinia, setActivePinia, type Pinia } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';

import { LyricFormat, LyricTransMode, SortOrder, SortType, YouthVip } from '@/utils/params';

/**
 * 类型安全的测试夹具工厂。
 * 所有字段均以 `src/types/global.d.ts` 与 `src/utils/params.ts` 为准，禁止臆造字段。
 */

/** 创建带持久化插件的测试用 Pinia 实例并激活（persist 走 localStorage，每个测试前已由 setup 清空） */
export const createTestPinia = (): Pinia => {
  const pinia = createPinia();
  pinia.use(piniaPluginPersistedstate);
  // Pinia 3 的 use() 只有在 install(app) 后才会把插件从暂存列表刷入内部 _p；
  // 传入最小 app 对象触发 install，使持久化插件（hydrate/$persist）真正生效
  pinia.install({
    provide: () => {},
    config: { globalProperties: {} },
  } as never);
  setActivePinia(pinia);
  return pinia;
};

let seq = 0;
const nextId = () => {
  seq += 1;
  return seq;
};

// ---------- 音频 ----------

export const makePlayingMusic = (overrides: Partial<PlayingMusicInfo> = {}): PlayingMusicInfo => {
  const id = nextId();
  return {
    id,
    hash: `hash-${id}`,
    path: `/music/song-${id}.mp3`,
    cover: null,
    title: `Song ${id}`,
    artist: `Artist ${id}`,
    duration: 180,
    ...overrides,
  };
};

export const makeListMusic = (overrides: Partial<MusicInfo> = {}): MusicInfo => {
  const id = nextId();
  return {
    id,
    hash: `hash-${id}`,
    path: `/music/song-${id}.mp3`,
    cover: null,
    title: `Song ${id}`,
    artist: `Artist ${id}`,
    album: `Album ${id}`,
    duration: 180,
    sort: 0,
    ...overrides,
  };
};

export const makeListInfo = (overrides: Partial<ListInfo> = {}): ListInfo => {
  const id = nextId();
  return {
    id,
    cover: '',
    title: `List ${id}`,
    artist: '',
    count: 0,
    playCount: 0,
    tags: [],
    ...overrides,
  };
};

export const makeMusicList = (
  list: MusicInfo[] = [],
  overrides: Partial<ListInfo> = {},
): MusicList => ({
  info: makeListInfo({ count: list.length, ...overrides }),
  list,
});

// ---------- 歌词 ----------

export const makeLyricWord = (overrides: Partial<LyricWord> = {}): LyricWord => ({
  offset: 0,
  duration: 500,
  text: 'word',
  ...overrides,
});

export const makeLyricLine = (overrides: Partial<LyricLine> = {}): LyricLine => ({
  offset: 0,
  duration: 2000,
  words: [makeLyricWord()],
  translations: { [LyricTransMode.Roman]: '', [LyricTransMode.Trans]: '' },
  ...overrides,
});

export const makeLyricInfo = (overrides: Partial<LyricInfo> = {}): LyricInfo => {
  const id = nextId();
  return {
    id: `lyric-${id}`,
    fmt: LyricFormat.Lrc,
    lines: [makeLyricLine()],
    ...overrides,
  };
};

export const makeLyricCandidate = (
  overrides: Partial<ApiLyricCandidate> = {},
): ApiLyricCandidate => {
  const id = nextId();
  return {
    id: `candidate-${id}`,
    accesskey: `key-${id}`,
    product_from: 'kg',
    singer: 'Artist',
    song: 'Song',
    score: 100,
    contenttype: 0,
    ...overrides,
  };
};

// ---------- 用户 ----------

export const makeUserInfo = (overrides: Partial<UserInfo> = {}): UserInfo => {
  const id = nextId();
  return {
    userid: id,
    nickname: `User ${id}`,
    pic: '',
    youthVip: YouthVip.Not,
    ...overrides,
  };
};

export const makePlaylist = (overrides: Partial<ApiPlaylist> = {}): ApiPlaylist => {
  const id = nextId();
  return {
    list_create_gid: `gid-${id}`,
    list_create_listid: id,
    list_create_username: 'tester',
    pic: '',
    name: `Playlist ${id}`,
    count: 0,
    sort: 0,
    musiclib_tags: [],
    is_pri: 0,
    type: 0,
    ...overrides,
  };
};

// ---------- 其他 ----------

export const makeContextMenuOption = (
  overrides: Partial<ContextMenuOption> = {},
): ContextMenuOption => ({
  label: 'Option',
  ...overrides,
});

export const makeDeviceInfo = (overrides: Partial<ApiDeviceInfo> = {}): ApiDeviceInfo => {
  const id = nextId();
  return { id: `device-${id}`, name: `Device ${id}`, ...overrides };
};

export const makeMusicDetail = (overrides: Partial<MusicDetail> = {}): MusicDetail => {
  const id = nextId();
  return {
    path: `/music/song-${id}.mp3`,
    cover: null,
    title: `Song ${id}`,
    artist: `Artist ${id}`,
    album: `Album ${id}`,
    genre: null,
    duration: 180,
    overall_bitrate: 320,
    audio_bitrate: 320,
    sample_rate: 44100,
    bit_depth: 16,
    channels: 2,
    format: 'mp3',
    size: 8 * 1024 * 1024,
    ...overrides,
  };
};

export const makeSortInfo = (overrides: Partial<SortInfo> = {}): SortInfo => ({
  type: SortType.Default,
  order: SortOrder.ASC,
  ...overrides,
});
