<script lang="ts" setup generic="T extends Record<string, any>">
import SvgIcon from './SvgIcon.vue'
import { useEventListener, useThrottleFn } from '@vueuse/core'

type Key = keyof T

interface Props {
  /**
   * 行的高度
   * @default 64
   */
  lineHeight?: number
  /**
   * 底部填充高度
   * @default 16
   */
  bottomPadding?: number
  /**
   * 行的主键名称
   * @default 'id'
   */
  lineKey?: Key
  /** 行配置 */
  columns: TableColumn[]
  loading?: boolean
  list: T[]
  /** 是否显示复选框 */
  checking?: boolean
  /** 选中项列表 */
  checkedList?: T[Key][]
}

interface IEmits {
  /** 滚动事件 */
  scroll: [e: Event]
  /** 鼠标滚轮事件 */
  wheel: [e: WheelEvent]
  /** 无限滚动事件 */
  infinite: [e: Event]
  /** 选中项点击事件 */
  check: [value: T[Key]]
  /** 右键菜单事件 */
  contextmenu: [e: MouseEvent, data: T]
  /** 行点击事件 */
  lineClick: [data: T]
  /** 行双击事件 */
  lineDblClick: [data: T]
}

const {
  lineHeight = 64,
  bottomPadding = 16,
  lineKey = 'id',
  columns,
  loading,
  list,
  checking,
  checkedList
} = defineProps<Props>()
const emits = defineEmits<IEmits>()

const containerRef = useTemplateRef('containerRef')

let lastScrollTop = 0 // 最后滚动位置

const containerHeight = ref(0) // 容器高度
const containerOffsetY = ref(0) // 容器滚动距离

// 内容总高度
const totalHeight = computed(() => lineHeight * list.length + bottomPadding)
// 开始索引
const startIndex = computed(() => Math.floor(containerOffsetY.value / lineHeight))
// 可见列表
const visibleLines = computed(() => {
  // 可见数量, 可能出现上下都为半个的情况,所以多 +1
  const visibleCount = Math.ceil(containerHeight.value / lineHeight) + 1
  return list.slice(startIndex.value, startIndex.value + visibleCount)
})

// 设置容器高度
const setContainerHeight = () => {
  containerHeight.value = containerRef.value?.clientHeight || 0
}

const handleInfinite = useThrottleFn((e: Event) => emits('infinite', e), 100)

// 滚动事件
const handleScroll = (e: Event) => {
  const target = e.target as HTMLDivElement
  containerOffsetY.value = target.scrollTop
  emits('scroll', e)

  const toBottom = target.scrollTop > lastScrollTop
  const onBottom = target.clientHeight + target.scrollTop + 1 >= target.scrollHeight
  if (toBottom && onBottom) handleInfinite(e)
  lastScrollTop = target.scrollTop
}

// 鼠标滚轮事件
const handleWheel = (e: WheelEvent) => {
  emits('wheel', e)
}

// 行点击事件
const handleLineClick = (line: T) => {
  if (!checking) {
    emits('lineClick', line)
  } else {
    emits('check', line[lineKey])
  }
}

// 行双击事件
const handleLineDblClick = (line: T) => {
  if (checking) return

  emits('lineDblClick', line)
}

// 滚动到顶部
const scrollToTop = (behavior: ScrollBehavior = 'auto') => {
  containerRef.value?.scrollTo({ top: 0, behavior })
}

// 滚动到指定行
const scrollToIndex = (
  index: number,
  options: ScrollOptions = { position: 'top', behavior: 'auto' }
) => {
  if (index < 0 || index >= list.length) return

  let scrollTop = lineHeight * index

  switch (options?.position) {
    case 'top':
      break
    case 'center':
      scrollTop += (lineHeight - containerHeight.value) / 2
      break
    case 'bottom':
      scrollTop += lineHeight - containerHeight.value
      break
  }

  const maxScrollTop = totalHeight.value - containerHeight.value + bottomPadding
  const top = Math.min(Math.max(0, scrollTop), maxScrollTop)

  containerRef.value?.scrollTo({ top, behavior: options?.behavior })
}

// 滚动到指定项
const scrollToTarget = (
  target: T[Key],
  options: ScrollOptions = { position: 'top', behavior: 'auto' }
) => {
  if (!target) return

  const index = list.findIndex((item) => item[lineKey] === target)
  if (index === -1) return

  scrollToIndex(index, options)
}

onMounted(setContainerHeight)
useEventListener('resize', setContainerHeight)

defineExpose({ scrollToTarget, scrollToIndex, scrollToTop })
</script>

<template>
  <div
    ref="containerRef"
    class="relative overflow-y-auto"
    :style="{ '--line-height': `${lineHeight}px` }"
    @scroll.passive="handleScroll"
    @wheel.passive="handleWheel">
    <!-- 撑起容器高度 -->
    <div class="absolute inset-0" :style="{ height: `${totalHeight}px` }"></div>

    <div v-if="loading">
      <div v-for="count in 5" :key="count" class="card mb-2 h-16 border-0"></div>
    </div>

    <div
      v-else-if="!list.length"
      class="flex items-center justify-center size-full flex-col text-minor">
      <SvgIcon name="LinesRemove" size="56" />
      <div class="text-xl font-bold">列表为空</div>
    </div>

    <ul v-else :style="{ transform: `translateY(${`${lineHeight * startIndex}`}px)` }">
      <li
        v-for="(line, lineIndex) in visibleLines"
        :key="startIndex + lineIndex"
        class="group/line card-hover flex h-[var(--line-height)] items-center rounded-lg"
        @click="handleLineClick(line)"
        @dblclick="handleLineDblClick(line)"
        @contextmenu.prevent="!checking && emits('contextmenu', $event, line)">
        <input
          v-if="checking"
          class="basis-8"
          type="checkbox"
          :checked="checkedList?.includes(line[lineKey])" />

        <div
          v-for="(column, columnIndex) in columns"
          :key="columnIndex"
          class="truncate"
          :class="column.width === 'auto' ? 'w-0 flex-1' : 'basis-[var(--basis)]'"
          :style="{
            textAlign: column.align || 'left',
            paddingInline:
              typeof column.padding === 'number' ? `${column.padding}px` : column.padding || '8px',
            '--basis': typeof column.width === 'number' ? `${column.width}px` : column.width
          }">
          <slot :name="column.key" v-bind="{ ...line, index: startIndex + lineIndex }">
            <template v-if="line[column.key]">{{ line[column.key] }}</template>
          </slot>
        </div>
      </li>
    </ul>
  </div>
</template>
