import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import { useListStore } from '@/stores/list';
import { useLyricStore } from '@/stores/lyric';
import { useMusicStore } from '@/stores/music';
import { useUserStore } from '@/stores/user';
import { ListType, LyricFormat, PlayingOrigin, YouthVip } from '@/utils/params';

import {
  createTestPinia,
  makeListMusic,
  makeLyricCandidate,
  makeMusicList,
  makeUserInfo,
} from '../helpers/factories';
import { invokeMock, mockInvoke } from '../helpers/mocks';

// user store 依赖 useRoute/useRouter，文件级 mock vue-router
const { routeMock } = vi.hoisted(() => ({
  routeMock: { name: undefined as string | undefined },
}));

vi.mock('vue-router', () => ({
  useRoute: () => routeMock,
  useRouter: () => ({ replace: vi.fn() }),
}));

// music store 的水合 watch（pause/monitor）与 setMusic/loadLyric 链路均为微任务，
// fake timers 下用 advanceTimersByTimeAsync(0) 排空
const flush = () => vi.advanceTimersByTimeAsync(0);

beforeEach(() => {
  vi.useFakeTimers();
  createTestPinia();
  routeMock.name = undefined;
});

afterEach(() => {
  vi.useRealTimers();
});

describe('N1 选歌↔歌词↔播放↔会员 联动', () => {
  it('选歌→播放→歌词：在线歌取 URL 加载并联动加载歌词', async () => {
    const listStore = useListStore();
    const musicStore = useMusicStore();
    const lyricStore = useLyricStore();
    await flush();
    invokeMock.mockClear();

    const song = makeListMusic({
      hash: 'hash-n1',
      path: null,
      title: 'N1 Song',
      artist: 'N1 Artist',
    });
    listStore.setList(ListType.Play, makeMusicList([song], { id: 'play' }));

    const official = makeLyricCandidate({ id: 'c-n1', product_from: '官方推荐歌词' });
    mockInvoke({
      api_song_url: { status: 1, backupUrl: ['https://cdn.example.com/n1.mp3'] },
      api_lyric_search: { status: 200, candidates: [official] },
      api_lyric_get: { id: 'c-n1', fmt: LyricFormat.Lrc, content: '[00:01.00]N1 lyric line' },
    });

    await musicStore.setMusic(song, { origin: PlayingOrigin.Online });
    await flush();

    // 播放链路
    expect(invokeMock).toHaveBeenCalledWith('api_song_url', expect.anything());
    expect(invokeMock).toHaveBeenCalledWith('music_player_load_url', expect.anything());
    expect(musicStore.isLoaded).toBe(true);
    expect(musicStore.isPlaying).toBe(true);

    // 歌词联动：music.load 的 finally 触发 lyricStore.load（搜索 + 获取 + 解析）
    expect(invokeMock).toHaveBeenCalledWith('api_lyric_search', expect.anything());
    expect(invokeMock).toHaveBeenCalledWith('api_lyric_get', expect.anything());
    expect(lyricStore.lyric?.id).toBe('c-n1');
    expect(lyricStore.lyric?.lines).toHaveLength(1);
    expect(lyricStore.lyric?.lines[0].words[0].text).toBe('N1 lyric line');
  });

  it('切歌→歌词：切换歌曲后歌词随之重新加载', async () => {
    const listStore = useListStore();
    const musicStore = useMusicStore();
    const lyricStore = useLyricStore();
    await flush();
    invokeMock.mockClear();

    const list = [
      makeListMusic({ id: 1, hash: null, path: '/music/1.mp3', title: 'T1', artist: 'A1' }),
      makeListMusic({ id: 2, hash: null, path: '/music/2.mp3', title: 'T2', artist: 'A2' }),
    ];
    listStore.setList(ListType.Play, makeMusicList(list, { id: 'play' }));

    // 按关键词动态返回不同候选，验证歌词随歌曲切换而刷新
    mockInvoke({
      api_lyric_search: (args) => {
        const keyword = (args as { keyword: string }).keyword;
        const id = keyword.startsWith('A1') ? 'lyric-1' : 'lyric-2';
        return { status: 200, candidates: [makeLyricCandidate({ id })] };
      },
      api_lyric_get: (args) => ({
        id: (args as { id: string }).id,
        fmt: LyricFormat.Lrc,
        content: '[00:01.00]line',
      }),
    });

    await musicStore.setMusic(list[0], { origin: PlayingOrigin.Local });
    await flush();
    expect(lyricStore.lyric?.id).toBe('lyric-1');

    await musicStore.setMusic(list[1], { origin: PlayingOrigin.Local });
    await flush();
    expect(lyricStore.lyric?.id).toBe('lyric-2');
  });

  it('会员联动：KgLite 登录后获取概念版 VIP 等级', async () => {
    const userStore = useUserStore();
    await flush();

    mockInvoke({
      api_youth_union_vip: {
        status: 1,
        data: { busi_vip: [{ is_vip: 1, product_type: 'svip' }], is_vip: 1 },
      },
    });

    await userStore.login(makeUserInfo({ userid: 1001 }));

    expect(userStore.userinfo?.userid).toBe(1001);
    expect(userStore.userinfo?.youthVip).toBe(YouthVip.Svip);
  });
});
