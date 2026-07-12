<script lang="ts" setup>
import SelectModal from '@/components/SelectModal.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useMusicStore } from '@/stores/music'
import { useSettingStore } from '@/stores/setting'
import { invoke } from '@/utils/tools'
import { vOnClickOutside } from '@vueuse/components'

const settingStore = useSettingStore()
const musicStore = useMusicStore()

const deviceVisible = ref(false)
const deviceOptions = ref<SelectOption[]>([])

const deviceSelection = computed(
  () =>
    deviceOptions.value.find((item) => item.value === settingStore.device) || deviceOptions.value[0]
)

const getDevices = async () => {
  const player_devices = await invoke('music_player_get_devices')
  if (!player_devices) return

  deviceOptions.value = player_devices.map((item) => ({ label: item.name, value: item.id }))
}

const getDevice = async () => {
  const player_device = await invoke('music_player_get_device')
  if (!player_device) return

  settingStore.setDevice(player_device.id)
}

const handleDeviceSelect = async (id: string) => {
  deviceVisible.value = false

  const pg = musicStore.playProgress
  await invoke('music_player_set_device', { id })
  await musicStore.setMusic(musicStore.music, {
    origin: musicStore.origin,
    loop: true,
    autoPlay: musicStore.isPlaying
  })
  await musicStore.seek(pg)

  settingStore.setDevice(id)
}

onMounted(() => {
  getDevices()
  getDevice()
})
</script>

<template>
  <div class="flex items-center text-base">
    <div class="font-bold w-40">音频输出设备:</div>

    <div>
      <div class="relative text-sm" v-on-click-outside="() => (deviceVisible = false)">
        <div
          class="flex cursor-pointer items-center whitespace-nowrap font-bold"
          @click="deviceVisible = !deviceVisible">
          <div>{{ deviceSelection?.label }}</div>
          <SvgIcon
            class="transition-transform"
            :class="deviceVisible ? 'rotate-180' : ''"
            name="Down" />
        </div>

        <SelectModal
          class="absolute -left-4 top-full"
          transition="zoom-top"
          :visible="deviceVisible"
          :options="deviceOptions"
          :selection="deviceSelection"
          @select="handleDeviceSelect" />
      </div>
    </div>
  </div>
</template>
