<script lang="ts" setup>
import { watchEffect } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import { useStateStore } from '@/stores/state';
import { cn } from '@/utils/tools';

defineOptions({ inheritAttrs: false });

interface Props {
  visible: boolean;
  title?: string;
  /** 隐藏标题栏 */
  hideHeader?: boolean;
  /** 隐藏操作栏 */
  hideFooter?: boolean;
  /** 确认按钮显示名称 */
  confirmLabel?: string;
  /** 取消按钮显示名称 */
  cancelLabel?: string;
  /** 点击遮罩是否关闭模态框 */
  maskClosed?: boolean;
}

interface Emits {
  cancel: [];
  confirm: [];
}

const {
  visible,
  title,
  hideHeader,
  hideFooter,
  confirmLabel = '确认',
  cancelLabel = '取消',
  maskClosed = true,
} = defineProps<Props>();
const emits = defineEmits<Emits>();

const stateStore = useStateStore();

const handleCancel = (e: KeyboardEvent) => {
  if (e.key !== 'Escape') return;

  emits('cancel');
};

watchEffect((onCleanup) => {
  if (!visible) return;

  window.addEventListener('keyup', handleCancel);
  onCleanup(() => window.removeEventListener('keyup', handleCancel));
});
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="visible"
        class="fixed inset-0 z-40 flex items-center justify-center bg-shadow backdrop-blur"
        :style="{ fontFamily: stateStore.fontFamily }"
        @click="maskClosed && emits('cancel')"
      >
        <div
          class="modal-container overflow-hidden"
          :class="cn('card bg-background shadow-lg shadow-shadow', $attrs.class)"
          @click.stop
        >
          <slot name="header">
            <div v-if="!hideHeader" class="flex items-center justify-between px-6 py-4">
              <div class="text-base font-bold">{{ title }}</div>
              <ActionButton
                class="hover:text-error"
                mode="text"
                prefix-icon="Close"
                icon-size="24"
                @click="emits('cancel')"
              />
            </div>
          </slot>

          <slot></slot>

          <slot name="footer">
            <div v-if="!hideFooter" class="flex items-center justify-end gap-3 px-6 py-4">
              <slot name="actions"></slot>

              <ActionButton @click="emits('cancel')">
                {{ cancelLabel }}
              </ActionButton>
              <ActionButton theme="info" @click="emits('confirm')">
                {{ confirmLabel }}
              </ActionButton>
            </div>
          </slot>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>
