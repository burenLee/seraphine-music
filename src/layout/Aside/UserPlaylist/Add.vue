<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { computed, ref } from 'vue';

import ActionButton from '@/components/ActionButton.vue';
import Modal from '@/components/Modal.vue';
import { notify } from '@/components/Notification.vue';
import SelectModal from '@/components/SelectModal.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useUserStore } from '@/stores/user';
import { AddPlaylistType, ApiInvokeStatus } from '@/utils/params';
import { invoke } from '@/utils/tools';

interface Emits {
  refresh: [];
}

const emits = defineEmits<Emits>();

const userStore = useUserStore();

const addSelectVisible = ref(false);
const addModalVisible = ref(false);
const addForm = ref({ name: '', isPri: 0 });

const addOptions = computed<Array<SelectOption<AddPlaylistType>>>(() => [
  { label: '新建歌单', value: AddPlaylistType.Add, prefixIcon: 'Add' },
  { label: '导入歌单', value: AddPlaylistType.Import, prefixIcon: 'Link', disabled: true },
]);

const handleAddSelect = (type: AddPlaylistType) => {
  addSelectVisible.value = false;

  switch (type) {
    case AddPlaylistType.Add:
      addModalVisible.value = true;
      break;
    case AddPlaylistType.Import:
      break;
  }
};

const handleAddConfirm = async () => {
  if (!userStore.userinfo) {
    notify.error('请先登录');
    return;
  }
  if (!addForm.value.name) {
    notify.error('请填写歌单名称');
    return;
  }

  try {
    const { status } = await invoke('api_playlist_add', {
      ...addForm.value,
      userid: userStore.userinfo.userid,
    });
    if (status !== ApiInvokeStatus.Success) {
      notify.error('创建歌单失败');
      return;
    }

    notify.success('创建歌单成功');
    handleAddCancel();
    emits('refresh');
  } catch {
    notify.error('创建歌单失败');
  }
};

const handleAddCancel = () => {
  addModalVisible.value = false;

  addForm.value.name = '';
  addForm.value.isPri = 0;
};
</script>

<template>
  <div class="relative mr-1" v-on-click-outside="() => (addSelectVisible = false)">
    <ActionButton
      class="size-8"
      mode="text"
      prefix-icon="Add"
      icon-size="20"
      :disabled="!userStore.userinfo"
      @click="addSelectVisible = !addSelectVisible"
    />

    <SelectModal
      class="absolute -right-1 top-full"
      transition="zoom-top-right"
      :visible="addSelectVisible"
      :options="addOptions"
      @select="handleAddSelect"
    />
  </div>

  <Modal
    class="w-80"
    v-model="addModalVisible"
    title="新建歌单"
    confirmLabel="新建"
    @cancel="handleAddCancel"
    @confirm="handleAddConfirm"
  >
    <form class="space-y-3 px-6" @submit.prevent="handleAddConfirm">
      <input
        class="card h-8 w-full px-3 leading-8"
        v-model="addForm.name"
        placeholder="请输入歌单名称"
      />

      <label class="flex cursor-pointer items-center gap-1">
        <input type="checkbox" v-model="addForm.isPri" :true-value="1" :false-value="0" />
        <span>设为隐私歌单</span>
        <SvgIcon name="Info" title="仅自己可见, 且无法分享" size="16" />
      </label>
    </form>
  </Modal>
</template>
