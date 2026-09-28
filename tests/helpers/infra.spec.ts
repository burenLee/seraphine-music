import { describe, expect, it, vi } from 'vitest';

import { invoke } from '@/utils/tools';

import {
  createTestPinia,
  makeListMusic,
  makeLyricLine,
  makePlayingMusic,
  makeUserInfo,
} from './factories';
import { emitToMock, invokeMock, mockInvoke, notifyMock, triggerListen } from './mocks';
import { listenMock } from './mocks';

describe('测试基础设施冒烟验证', () => {
  it('createTestPinia 可创建并激活 pinia 实例', () => {
    const pinia = createTestPinia();
    expect(pinia).toBeDefined();
  });

  it('夹具工厂产出符合类型结构的测试数据', () => {
    const music = makePlayingMusic();
    const listMusic = makeListMusic({ privilegeTags: ['privilegeTags'] });
    const lyricLine = makeLyricLine();
    const user = makeUserInfo();

    expect(music.title).toMatch(/^Song /);
    expect(listMusic.privilegeTags).toStrictEqual(['privilegeTags']);
    expect(lyricLine.words).toHaveLength(1);
    expect(user.nickname).toMatch(/^User /);
  });

  it('mockInvoke 可拦截 @/utils/tools 的 invoke 封装', async () => {
    mockInvoke({
      api_song_url: { status: 1, backupUrl: ['https://example.com/a.mp3'] },
    });

    const result = await invoke('api_song_url', { hash: 'abc' });

    expect(invokeMock).toHaveBeenCalledWith('api_song_url', { hash: 'abc' });
    expect(result).toStrictEqual({ status: 1, backupUrl: ['https://example.com/a.mp3'] });
  });

  it('未配置的 invoke 命令默认 resolve undefined', async () => {
    await expect(invoke('music_player_play')).resolves.toBeUndefined();
  });

  it('triggerListen 可触发已注册的事件回调', () => {
    const handler = vi.fn();
    listenMock('test:event', handler);

    triggerListen('test:event', { value: 42 });

    expect(handler).toHaveBeenCalledWith({ event: 'test:event', payload: { value: 42 } });
  });

  it('notify / emitTo / IntersectionObserver mock 可用', () => {
    expect(vi.isMockFunction(notifyMock.error)).toBe(true);
    expect(vi.isMockFunction(emitToMock)).toBe(true);
    expect(typeof IntersectionObserver).toBe('function');

    const observer = new IntersectionObserver(() => {});
    expect(() => observer.observe(document.createElement('div'))).not.toThrow();
  });

  it('每个测试前 invokeMock 实现被重置', async () => {
    // 上个测试配置过 mockInvoke，此处应已恢复默认行为
    await expect(invoke('api_song_url', { hash: 'abc' })).resolves.toBeUndefined();
  });
});
