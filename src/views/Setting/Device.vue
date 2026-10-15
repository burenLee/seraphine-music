<script lang="ts" setup>
import { vOnClickOutside } from '@vueuse/components';
import { computed, onMounted, ref } from 'vue';

import SelectModal from '@/components/SelectModal.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useMusicStore } from '@/stores/music';
import { useSettingStore } from '@/stores/setting';
import { invoke } from '@/utils/tools';

const settingStore = useSettingStore();
const musicStore = useMusicStore();

const deviceVisible = ref(false);
const deviceOptions = ref<SelectOption[]>([]);

const deviceSelection = computed(
  () =>
    deviceOptions.value.find((item) => item.value === settingStore.device) ||
    deviceOptions.value[0],
);

const getDevices = async () => {
  const player_devices = await invoke('music_player_get_devices');

  deviceOptions.value = player_devices.map((item) => ({ label: item.name, value: item.id }));
};

const getDevice = async () => {
  const player_device = await invoke('music_player_get_device');

  settingStore.setDevice(player_device.id);
};

const handleDeviceSelect = async (id: string) => {
  deviceVisible.value = false;

  const lastProgress = musicStore.playProgress;

  await invoke('music_player_set_device', { id });
  settingStore.setDevice(id);

  await musicStore.setMusic(musicStore.music, {
    origin: musicStore.origin,
    loop: true,
    autoPlay: musicStore.isPlaying,
  });
  await musicStore.seek(lastProgress);
};

onMounted(async () => {
  await getDevices();
  await getDevice();
});
</script>

<template>
  <div class="flex items-center text-base">
    <div class="w-40 font-bold">音频输出设备:</div>

    <div class="space-y-3">
      <div class="relative text-sm" v-on-click-outside="() => (deviceVisible = false)">
        <div class="flex cursor-pointer items-center gap-1" @click="deviceVisible = !deviceVisible">
          <div class="whitespace-nowrap font-bold">{{ deviceSelection?.label || '默认设备 ' }}</div>
          <SvgIcon
            class="transition-transform"
            :class="deviceVisible ? 'rotate-180' : ''"
            name="Down"
          />
        </div>

        <SelectModal
          class="absolute -left-4 top-full"
          transition="zoom-top"
          :visible="deviceVisible"
          :options="deviceOptions"
          :selection="deviceSelection"
          @select="handleDeviceSelect"
        />
      </div>
    </div>
  </div>
</template>
