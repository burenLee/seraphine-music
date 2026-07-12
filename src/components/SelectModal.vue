<script lang="ts" setup>
import SvgIcon from './SvgIcon.vue'

interface Props {
  visible: boolean
  /**
   * 过渡类型
   * @param "zoom-fade" | "zoom-top" | "zoom-top-right" | "zoom-bottom"
   * @default "zoom-fade"
   */
  transition?: 'zoom-fade' | 'zoom-top' | 'zoom-top-right' | 'zoom-bottom'
  options: SelectOption[]
  selection?: SelectOption
}

interface IEmits {
  select: [value: SelectOption['value'], option: SelectOption]
}

const { visible, transition = 'zoom-fade', options, selection } = defineProps<Props>()
const emits = defineEmits<IEmits>()
</script>

<template>
  <Transition :name="transition">
    <ul
      v-if="visible"
      class="z-10 rounded-lg border border-border overflow-auto bg-background p-1 shadow-md shadow-shadow">
      <li
        v-for="(option, index) in options"
        :key="index"
        class="flex cursor-pointer items-center gap-1 rounded-lg px-3 h-8 transition-colors"
        :class="option.value === selection?.value ? 'card-actived' : 'card-hover'"
        :data-disabled="option.disabled"
        @click="emits('select', option.value, option)">
        <slot name="prefix-icon">
          <SvgIcon v-if="option.prefixIcon" :name="option.prefixIcon" />
        </slot>

        <div class="whitespace-nowrap font-bold px-1">{{ option.label }}</div>

        <slot name="suffix-icon">
          <SvgIcon v-if="option.suffixIcon" :name="option.suffixIcon" />
        </slot>
      </li>

      <div v-if="!options.length" class="p-4 font-bold whitespace-nowrap">暂无数据</div>
    </ul>
  </Transition>
</template>
