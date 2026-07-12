<script lang="ts" setup>
import SvgIcon from './SvgIcon.vue'
import { useContextMenuStore } from '@/stores/contextMenu'
import { useSettingStore } from '@/stores/setting'
import { vOnClickOutside } from '@vueuse/components'
import { useEventListener, useWindowSize } from '@vueuse/core'

const settingStore = useSettingStore()
const contextMenuStore = useContextMenuStore()

const { width: windowWidth, height: windowHeight } = useWindowSize()

const containerWidth = 176 // 容器宽度
const containerPadding = 4 // 容器内边距
const dividerHeight = 16 // 分割线内容高度
const lineHeight = 32 // 单元行高度
const lineWidth = containerWidth - containerPadding * 2 // 单元行宽度

const activedIndex = ref(-1)
const activedTop = ref(0)

const containerPosition = computed(() => {
  const { position, options } = contextMenuStore

  const containerHeight =
    options.reduce((h, option) => h + (option.divider ? dividerHeight : lineHeight), 0) +
    containerPadding * 2

  const top =
    position.y + containerHeight > windowHeight.value ? position.y - containerHeight : position.y
  const left =
    position.x + containerWidth > windowWidth.value ? position.x - containerWidth : position.x

  return { top, left }
})

const childrenPosition = computed(() => {
  const childOptions = contextMenuStore.options[activedIndex.value].children
  if (!childOptions?.length) return { top: 0, left: lineWidth - 2 }

  const containerHeight =
    childOptions.reduce((h, option) => h + (option.divider ? dividerHeight : lineHeight), 0) +
    containerPadding * 2

  const inBlockArea = activedTop.value + containerHeight <= windowHeight.value
  const top = inBlockArea
    ? -containerPadding
    : -(containerHeight - (windowHeight.value - activedTop.value))

  const inInlineArea = containerPosition.value.left + containerWidth * 2 <= windowWidth.value
  const left = inInlineArea ? lineWidth - 2 : -containerWidth

  return { top, left }
})

const handleChildrenShow = (e: MouseEvent, index: number) => {
  activedTop.value = containerPosition.value.top + (e.target as HTMLDivElement).offsetTop
  activedIndex.value = index
}

const handleClick = (option: ContextMenuOption) => {
  if (!option.onClick) return

  activedIndex.value = -1
  contextMenuStore.hide()
  option.onClick()
}

useEventListener('resize', contextMenuStore.hide)
useEventListener('wheel', contextMenuStore.hide, true)
</script>

<template>
  <Teleport to="body">
    <Transition name="zoom-fade">
      <ul
        v-if="contextMenuStore.visible"
        class="fixed z-50 rounded-lg border border-border bg-background shadow-md shadow-shadow"
        :style="{
          fontFamily: settingStore.fontFamily,
          padding: `${containerPadding}px`,
          width: `${containerWidth}px`,
          top: `${containerPosition.top}px`,
          left: `${containerPosition.left}px`
        }"
        v-on-click-outside="contextMenuStore.hide"
        @contextmenu.stop.prevent>
        <template v-for="(option, index) in contextMenuStore.options" :key="index">
          <li
            v-if="option.divider"
            class="flex items-center justify-center"
            :style="{ height: `${dividerHeight}px` }">
            <div class="h-px w-full bg-border"></div>
          </li>

          <li
            v-else
            class="card-hover transition-colors relative flex cursor-pointer items-center gap-1 rounded-lg px-3"
            :style="{ height: `${lineHeight}px` }"
            :data-disabled="option.disabled"
            @click="handleClick(option)"
            @mouseenter="handleChildrenShow($event, index)"
            @mouseleave="activedIndex = -1">
            <div class="size-4">
              <slot name="prefix-icon">
                <SvgIcon v-if="option.prefixIcon" :name="option.prefixIcon" />
              </slot>
            </div>

            <div class="w-0 flex-1 truncate font-bold px-1">{{ option.label }}</div>

            <slot name="suffix-icon">
              <SvgIcon v-if="option.suffixIcon" :name="option.suffixIcon" />
            </slot>

            <Transition name="zoom-fade">
              <ul
                v-if="option.children?.length && activedIndex === index"
                class="absolute rounded-lg border border-border bg-background shadow-md shadow-shadow"
                :style="{
                  padding: `${containerPadding}px`,
                  width: `${containerWidth}px`,
                  top: `${childrenPosition.top}px`,
                  left: `${childrenPosition.left}px`
                }">
                <li
                  v-for="(childOption, childIndex) in option.children"
                  :key="childIndex"
                  class="card-hover transition-colors flex cursor-pointer items-center gap-1 rounded-lg px-3"
                  :style="{ height: `${lineHeight}px` }"
                  :data-disabled="childOption.disabled"
                  @click="handleClick(childOption)">
                  <div class="size-4">
                    <slot name="prefix-icon">
                      <SvgIcon v-if="childOption.prefixIcon" :name="childOption.prefixIcon" />
                    </slot>
                  </div>

                  <div class="w-0 flex-1 truncate font-bold px-1">{{ childOption.label }}</div>

                  <slot name="suffix-icon">
                    <SvgIcon v-if="childOption.suffixIcon" :name="childOption.suffixIcon" />
                  </slot>
                </li>
              </ul>
            </Transition>
          </li>
        </template>
      </ul>
    </Transition>
  </Teleport>
</template>
