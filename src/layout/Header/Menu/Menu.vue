<script lang="ts" setup>
import { notify } from '@/components/Notification'
import SelectModal from '@/components/SelectModal.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useUserStore } from '@/stores/user'
import { MenuAction } from '@/utils/params'
import { invoke } from '@/utils/tools'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { vOnClickOutside } from '@vueuse/components'

const router = useRouter()

const userStore = useUserStore()

const menuOptions = computed<Array<SelectOption<MenuAction>>>(() => [
  { label: '恢复默认窗口', value: MenuAction.Restore, prefixIcon: 'Restart' },
  { label: '检查更新', value: MenuAction.Update, prefixIcon: 'Refresh', disabled: true },
  { label: '设置', value: MenuAction.Setting, prefixIcon: 'Setting' },
  {
    label: '退出登录',
    value: MenuAction.Logout,
    prefixIcon: 'Logout',
    disabled: !userStore.userinfo
  },
  { label: '退出播放器', value: MenuAction.Exit, prefixIcon: 'Exit' }
])

const menuVisible = ref(false)

const handleSelect = async (action: MenuAction) => {
  switch (action) {
    case MenuAction.Restore:
      await invoke('system_setting_restore_window')
      break
    case MenuAction.Update:
      notify.info('检查更新中...')
      break
    case MenuAction.Setting:
      router.push('/setting')
      break
    case MenuAction.Logout:
      userStore.logout()
      break
    case MenuAction.Exit:
      getCurrentWindow().close()
      break
  }

  menuVisible.value = false
}
</script>

<template>
  <div class="relative" v-on-click-outside="() => (menuVisible = false)">
    <SvgIcon class="action-icon" name="Menu" size="20" @click="menuVisible = !menuVisible" />

    <SelectModal
      class="absolute left-1/2 -translate-x-1/2 top-full"
      transition="zoom-top"
      :visible="menuVisible"
      :options="menuOptions"
      @select="handleSelect" />
  </div>
</template>
