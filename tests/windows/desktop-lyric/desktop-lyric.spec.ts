import { getCurrentWindow } from '@tauri-apps/api/window';
import { mount } from '@vue/test-utils';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import SelectModal from '@/components/SelectModal.vue';
import { DesktopLyricEmit, LyricFormat, WindowEvent, WindowTarget } from '@/utils/params';
import DesktopLyric from '@/windows/desktop-lyric/DesktopLyric.vue';

import {
  createTestPinia,
  makeLyricInfo,
  makeLyricLine,
  makeLyricWord,
  makePlayingMusic,
} from '../../helpers/factories';
import { emitToMock, flushAsync, listenMock, triggerListen } from '../../helpers/mocks';

/** 固定 getCurrentWindow 返回值，便于断言窗口方法调用 */
const setupWindow = () => {
  const win = vi.mocked(getCurrentWindow)();
  vi.mocked(getCurrentWindow).mockReturnValue(win);
  return win;
};

const mountComponent = () => mount(DesktopLyric);

beforeEach(() => createTestPinia());

afterEach(() => {
  vi.restoreAllMocks();
});

describe('DesktopLyric 桌面歌词窗口', () => {
  it('挂载时向主窗口发送 Init 并监听事件', () => {
    setupWindow();
    const wrapper = mountComponent();

    expect(emitToMock).toHaveBeenCalledWith(WindowTarget.Main, WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Init,
    });
    expect(listenMock).toHaveBeenCalledWith(WindowEvent.DesktopLyric, expect.any(Function));
    expect(wrapper.text()).toContain('Seraphine');
  });

  it('Audio 事件同步音频信息并渲染歌曲名', async () => {
    setupWindow();
    const wrapper = mountComponent();
    const music = makePlayingMusic({ title: '海阔天空', artist: 'Beyond' });

    triggerListen(WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Audio,
      data: { isLoading: false, isPlaying: true, music },
    });
    await flushAsync();

    expect(wrapper.text()).toContain('海阔天空 - Beyond');
  });

  it('Lyric 事件同步歌词并渲染歌词行', async () => {
    setupWindow();
    const wrapper = mountComponent();
    const lyric = makeLyricInfo({
      id: 'lyric-x',
      fmt: LyricFormat.Lrc,
      lines: [
        makeLyricLine({ offset: 1000, words: [makeLyricWord({ text: '第一句' })] }),
        makeLyricLine({ offset: 2000, words: [makeLyricWord({ text: '第二句' })] }),
      ],
    });

    triggerListen(WindowEvent.DesktopLyric, { type: DesktopLyricEmit.Lyric, data: lyric });
    await flushAsync();

    expect(wrapper.text()).toContain('第一句');
  });

  it('Fonts 事件将字体项映射为 SelectModal 选项', async () => {
    setupWindow();
    const wrapper = mountComponent();

    triggerListen(WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Fonts,
      data: [
        ['系统默认', 'system-ui'],
        ['衬线', 'serif'],
      ],
    });
    await flushAsync();

    expect(wrapper.findComponent(SelectModal).props('options')).toEqual([
      { label: '系统默认', value: 'system-ui' },
      { label: '衬线', value: 'serif' },
    ]);
  });

  it('点击播放/暂停按钮发送 Play / Pause 事件', async () => {
    setupWindow();
    const wrapper = mountComponent();

    // 初始未播放 → 显示「播放」
    emitToMock.mockClear();
    await wrapper.find('[title="播放"]').trigger('click');
    expect(emitToMock).toHaveBeenCalledWith(WindowTarget.Main, WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Play,
      data: undefined,
    });

    // 切换到播放中 → 显示「暂停」
    triggerListen(WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Audio,
      data: { isLoading: false, isPlaying: true, music: null },
    });
    await flushAsync();

    emitToMock.mockClear();
    await wrapper.find('[title="暂停"]').trigger('click');
    expect(emitToMock).toHaveBeenCalledWith(WindowTarget.Main, WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Pause,
      data: undefined,
    });
  });

  it('点击工具栏按钮发送 Main / Prev / Next 事件', async () => {
    setupWindow();
    const wrapper = mountComponent();

    emitToMock.mockClear();
    await wrapper.find('[title="打开主界面"]').trigger('click');
    expect(emitToMock).toHaveBeenCalledWith(WindowTarget.Main, WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Main,
      data: undefined,
    });

    emitToMock.mockClear();
    await wrapper.find('[title="上一首"]').trigger('click');
    expect(emitToMock).toHaveBeenCalledWith(WindowTarget.Main, WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Prev,
      data: undefined,
    });

    emitToMock.mockClear();
    await wrapper.find('[title="下一首"]').trigger('click');
    expect(emitToMock).toHaveBeenCalledWith(WindowTarget.Main, WindowEvent.DesktopLyric, {
      type: DesktopLyricEmit.Next,
      data: undefined,
    });
  });
});
