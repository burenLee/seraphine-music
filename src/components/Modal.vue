<script lang="ts" setup>
import { watch } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useSettingStore } from '@/stores/setting';
import { cn } from '@/utils/tools';

defineOptions({ inheritAttrs: false });

interface Props {
  title?: string;
  /** 点击遮罩是否关闭模态框 */
  maskClosed?: boolean;
  /** 隐藏标题栏 */
  hideHeader?: boolean;
  /** 隐藏操作栏 */
  hideFooter?: boolean;
  /** 隐藏确认按钮 */
  hideConfirm?: boolean;
  /** 隐藏取消按钮 */
  hideCancel?: boolean;
  /** 确认按钮显示名称 */
  confirmLabel?: string;
  /** 取消按钮显示名称 */
  cancelLabel?: string;
}

interface Emits {
  cancel: [];
  confirm: [];
}

const visible = defineModel<boolean>({ required: true });
const {
  title,
  maskClosed = true,
  hideHeader,
  hideFooter,
  hideConfirm,
  hideCancel,
  confirmLabel = '确认',
  cancelLabel = '取消',
} = defineProps<Props>();
const emits = defineEmits<Emits>();

const settingStore = useSettingStore();

const handleCancel = (e: KeyboardEvent) => {
  if (e.key !== 'Escape') return;

  e.preventDefault();
  emits('cancel');
};

watch(visible, (visible) => {
  if (visible) {
    window.addEventListener('keyup', handleCancel);
  } else {
    window.removeEventListener('keyup', handleCancel);
  }
});
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="visible"
        class="fixed inset-0 z-40 flex items-center justify-center bg-shadow backdrop-blur-sm"
        :style="{ fontFamily: settingStore.fontFamily }"
        @click="maskClosed && emits('cancel')"
      >
        <div
          class="modal-container overflow-hidden transition-transform duration-300"
          :class="cn('z-40 rounded-lg bg-background shadow-md shadow-shadow', $attrs.class)"
          @click.stop
        >
          <slot name="header">
            <div v-if="!hideHeader" class="flex items-center justify-between py-4 pl-6 pr-4">
              <div class="text-base font-bold">{{ title }}</div>
              <SvgIcon
                class="action-icon hover:text-error"
                name="Close"
                size="20"
                @click="emits('cancel')"
              />
            </div>
          </slot>

          <slot></slot>

          <slot name="footer">
            <div v-if="!hideFooter" class="flex items-center justify-end gap-3 px-6 py-4">
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
