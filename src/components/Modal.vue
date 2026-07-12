<script lang="ts" setup>
import ActionButton from './ActionButton.vue'
import SvgIcon from './SvgIcon.vue'
import { useSettingStore } from '@/stores/setting'
import { cn } from '@/utils/tools'

defineOptions({ inheritAttrs: false })

interface Props {
  title?: string
  /** 点击遮罩是否关闭模态框 */
  maskClosed?: boolean
  /** 隐藏标题栏 */
  hideHeader?: boolean
  /** 隐藏操作栏 */
  hideFooter?: boolean
  /** 隐藏确认按钮 */
  hideConfirm?: boolean
  /** 隐藏取消按钮 */
  hideCancel?: boolean
  /** 确认按钮显示名称 */
  confirmLabel?: string
  /** 取消按钮显示名称 */
  cancelLabel?: string
}

interface Emits {
  cancel: []
  confirm: []
}

const visible = defineModel<boolean>({ required: true })
const {
  title,
  maskClosed = true,
  hideHeader,
  hideFooter,
  hideConfirm,
  hideCancel,
  confirmLabel = '确认',
  cancelLabel = '取消'
} = defineProps<Props>()
const emits = defineEmits<Emits>()

const settingStore = useSettingStore()

const handleKeydown = (e: KeyboardEvent) => {
  if (e.key !== 'Escape') return

  e.preventDefault()
  emits('cancel')
}

watch(
  visible,
  (visible) => {
    if (visible) {
      document.addEventListener('keyup', handleKeydown)
    } else {
      document.removeEventListener('keyup', handleKeydown)
    }
  },
  { immediate: true }
)

onUnmounted(() => document.removeEventListener('keyup', handleKeydown))
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="visible"
        ref="modalRef"
        class="flex items-center justify-center fixed inset-0 z-40 bg-shadow backdrop-blur-sm"
        :style="{ fontFamily: settingStore.fontFamily }"
        @click="maskClosed && emits('cancel')">
        <div
          class="modal-container overflow-hidden transition-transform duration-300"
          :class="cn('rounded-lg z-40 bg-background shadow-md shadow-shadow', $attrs.class)"
          @click.stop>
          <slot name="header">
            <div v-if="!hideHeader" class="flex items-center justify-between p-4">
              <div class="font-bold text-base">{{ title }}</div>
              <SvgIcon
                class="action-icon size-4 hover:text-error"
                name="Close"
                size="12"
                @click="emits('cancel')" />
            </div>
          </slot>

          <slot></slot>

          <slot name="footer">
            <div v-if="!hideFooter" class="flex items-center justify-end gap-3 p-4">
              <slot name="actions"></slot>

              <ActionButton v-if="!hideCancel" @click="emits('cancel')">
                {{ cancelLabel }}
              </ActionButton>
              <ActionButton v-if="!hideConfirm" theme="info" @click="emits('confirm')">
                {{ confirmLabel }}
              </ActionButton>
            </div>
          </slot>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>
