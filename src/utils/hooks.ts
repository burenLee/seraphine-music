import { ListType } from './params'
import { useListStore } from '@/stores/list'
import { MaybeComputedElementRef, MaybeElement, unrefElement } from '@vueuse/core'

type IntersectionObserverCallback = (
  entries: IntersectionObserverEntry,
  observer: IntersectionObserver
) => void

// 创建一个 IntersectionObserver 管理器
const observerManager = (() => {
  let observer: IntersectionObserver | null = null
  const callback = {
    map: new WeakMap<Element, IntersectionObserverCallback>(),
    size: 0
  }

  const getOrInitObserver = () => {
    if (!observer) {
      observer = new IntersectionObserver((entries) => {
        entries.forEach((entry) => {
          const cb = callback.map.get(entry.target)
          if (cb) cb(entry, observer!)
        })
      })
    }

    return observer
  }

  const observe = (element: Element, cb: IntersectionObserverCallback) => {
    getOrInitObserver().observe(element)
    callback.map.set(element, cb)
    callback.size++
  }

  const unobserve = (element: Element) => {
    observer?.unobserve(element)
    callback.map.delete(element)
    callback.size--

    if (!observer || callback.size === 0) disconnect()
  }

  const disconnect = () => {
    observer?.disconnect()
    observer = null
    callback.size = 0
  }

  return { observe, unobserve, disconnect }
})()

/**
 * 监听元素可见性
 */
export const useObserver = (
  elementRef: MaybeComputedElementRef | MaybeRefOrGetter<MaybeElement>,
  callback: IntersectionObserverCallback
) => {
  const targetElement = computed(() => unrefElement(toValue(elementRef)))

  watch(
    targetElement,
    (targetElement) => targetElement && observerManager.observe(targetElement, callback),
    { immediate: true }
  )

  const unobserve = () => targetElement.value && observerManager.unobserve(targetElement.value)

  onUnmounted(unobserve)

  return { unobserve }
}

export function useListContext() {
  const listStore = useListStore()
  const listType = inject('listType', ListType.Show)
  const list = computed(() => listStore[listType])

  return { listStore, listType, list }
}
