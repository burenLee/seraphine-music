import { convertFileSrc } from '@tauri-apps/api/core';
import { ShallowRef, computed, onUnmounted, watchEffect } from 'vue';

import { useMusicStore } from '@/stores/music';
import { getPic } from '@/utils/music';
import { PicSize, PlayingOrigin } from '@/utils/params';

interface IntersectionObserverCallback {
  (entry: IntersectionObserverEntry, observer: IntersectionObserver): void;
}

/**
 * IntersectionObserver 管理器
 **/
class ObserverManager {
  private observer: IntersectionObserver | null = null;
  private callbackMap = new WeakMap<Element, IntersectionObserverCallback>();
  private callbackSize = 0;

  constructor() {
    this.observer = new IntersectionObserver((entries, observer) => {
      entries.forEach((entry) => {
        const cb = this.callbackMap.get(entry.target);
        if (cb) cb(entry, observer);
      });
    });
  }

  /**
   * 观察元素可见性
   * @param element - 要观察的元素
   * @param cb - 可见性变化时调用的回调函数
   */
  observe(element: Element, cb: IntersectionObserverCallback) {
    if (!this.observer) return;

    this.observer.observe(element);
    this.callbackMap.set(element, cb);
    this.callbackSize++;
  }

  /**
   * 取消观察元素
   * @param element - 要取消观察的元素
   */
  unobserve(element: Element) {
    if (!this.observer) return;

    this.observer.unobserve(element);
    this.callbackMap.delete(element);
    this.callbackSize--;
  }

  /**
   * 断开观察器连接
   */
  disconnect() {
    if (!this.observer) return;

    this.observer.disconnect();
    this.observer = null;
    this.callbackSize = 0;
  }
}

const observerManager = new ObserverManager();

/**
 * 监听元素可见性
 */
export const useObserver = (
  elementRef: ShallowRef<HTMLDivElement | null>,
  callback: IntersectionObserverCallback,
) => {
  const unobserve = () => {
    if (elementRef.value) observerManager.unobserve(elementRef.value);
  };

  watchEffect(() => {
    if (elementRef.value) observerManager.observe(elementRef.value, callback);
  });

  onUnmounted(unobserve);

  return { unobserve };
};

export const useCover = (size: PicSize = PicSize.Sm) => {
  const musicStore = useMusicStore();

  return computed(() => {
    if (!musicStore.music?.cover) return;

    return musicStore.origin === PlayingOrigin.Local
      ? convertFileSrc(musicStore.music.cover)
      : getPic(musicStore.music.cover, size);
  });
};
