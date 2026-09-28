import { getCurrentWindow } from '@tauri-apps/api/window';
import { mount } from '@vue/test-utils';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import { MiniPlayerEmit, PlayingOrigin, WindowEvent, WindowTarget } from '@/utils/params';
import MiniPlayer from '@/windows/mini-player/MiniPlayer.vue';

import { createTestPinia, makeListMusic, makePlayingMusic } from '../../helpers/factories';
import { emitToMock, flushAsync, listenMock, triggerListen } from '../../helpers/mocks';

/** 固定 getCurrentWindow 返回值，便于断言窗口方法调用 */
const setupWindow = () => {
  const win = vi.mocked(getCurrentWindow)();
  vi.mocked(getCurrentWindow).mockReturnValue(win);
  return win;
};

const mountComponent = () => mount(MiniPlayer);

beforeEach(() => createTestPinia());

afterEach(() => {
  vi.restoreAllMocks();
});

describe('MiniPlayer 迷你播放器窗口', () => {
  it('挂载时向主窗口发送 Init 并监听事件', () => {
    setupWindow();
    const wrapper = mountComponent();

    expect(emitToMock).toHaveBeenCalledWith(WindowTarget.Main, WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Init,
    });
    expect(listenMock).toHaveBeenCalledWith(WindowEvent.MiniPlayer, expect.any(Function));
    expect(wrapper.text()).toContain('Seraphine');
  });

  it('Audio 事件同步音频信息并渲染歌曲名', async () => {
    setupWindow();
    const wrapper = mountComponent();
    const music = makePlayingMusic({ title: '稻香', artist: '周杰伦' });

    triggerListen(WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Audio,
      data: { isLoading: false, isPlaying: true, music, origin: PlayingOrigin.Online },
    });
    await flushAsync();

    expect(wrapper.text()).toContain('稻香 - 周杰伦');
  });

  it('Lyric 事件同步当前歌词并优先展示', async () => {
    setupWindow();
    const wrapper = mountComponent();

    triggerListen(WindowEvent.MiniPlayer, { type: MiniPlayerEmit.Lyric, data: '当前歌词' });
    await flushAsync();

    expect(wrapper.text()).toContain('当前歌词');
  });

  it('点击播放/暂停按钮发送 Play / Pause 事件', async () => {
    setupWindow();
    const wrapper = mountComponent();

    // 初始未播放，点击 PlayBold → Play
    emitToMock.mockClear();
    const playIcon = wrapper
      .findAllComponents({ name: 'SvgIcon' })
      .find((icon) => icon.props('name') === 'PlayBold');
    expect(playIcon).toBeTruthy();
    await playIcon!.trigger('click');

    expect(emitToMock).toHaveBeenCalledWith(WindowTarget.Main, WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Play,
      data: undefined,
    });

    // 切换到播放中 → PauseBold → Pause
    triggerListen(WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Audio,
      data: { isLoading: false, isPlaying: true, music: null, origin: PlayingOrigin.Local },
    });
    await flushAsync();

    emitToMock.mockClear();
    const pauseIcon = wrapper
      .findAllComponents({ name: 'SvgIcon' })
      .find((icon) => icon.props('name') === 'PauseBold');
    expect(pauseIcon).toBeTruthy();
    await pauseIcon!.trigger('click');

    expect(emitToMock).toHaveBeenCalledWith(WindowTarget.Main, WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Pause,
      data: undefined,
    });
  });

  it('点击播放列表按钮切换列表并调整窗口尺寸', async () => {
    const miniWindow = setupWindow();
    const wrapper = mountComponent();

    triggerListen(WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Playlist,
      data: [makeListMusic({ id: 1, title: '第一首', artist: 'A' })],
    });
    await flushAsync();

    await wrapper.find('[title="播放列表"]').trigger('click');
    await flushAsync();

    expect(miniWindow.setSize).toHaveBeenCalledTimes(1);
    expect(wrapper.text()).toContain('第一首 - A');
  });

  it('双击播放列表行发送 Set 事件', async () => {
    setupWindow();
    const wrapper = mountComponent();

    const song = makeListMusic({ id: 1, title: '第一首', artist: 'A' });
    triggerListen(WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Playlist,
      data: [song],
    });
    await flushAsync();

    await wrapper.find('[title="播放列表"]').trigger('click');
    await flushAsync();

    emitToMock.mockClear();
    await wrapper.find('li').trigger('dblclick');

    expect(emitToMock).toHaveBeenCalledWith(WindowTarget.Main, WindowEvent.MiniPlayer, {
      type: MiniPlayerEmit.Set,
      data: expect.objectContaining({ id: 1 }),
    });
  });
});
