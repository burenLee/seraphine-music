<script lang="ts" setup>
import SelectModal from '@/components/SelectModal.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useSettingStore } from '@/stores/setting'
import { AutoStartMode } from '@/utils/params'
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart'
import { vOnClickOutside } from '@vueuse/components'

const settingStore = useSettingStore()

const autoStartOptions: Array<SelectOption<AutoStartMode>> = [
  { label: '前台', value: AutoStartMode.Foreground },
  { label: '最小化', value: AutoStartMode.Background, disabled: true }
]

const fontFamilyVisible = ref(false)
const autoStartModeVisible = ref(false)

const fontFamilyOptions = computed<Array<SelectOption<FontValue>>>(() =>
  settingStore.availableFonts.map(([label, value]) => ({ label, value }))
)
const fontFamilySelection = computed<SelectOption<FontValue> | undefined>(
  () =>
    fontFamilyOptions.value.find((item) => item.value === settingStore.fontFamily) ||
    fontFamilyOptions.value[0]
)
const autoStartModeSelection = computed(
  () =>
    autoStartOptions.find((item) => item.value === settingStore.autoStartMode) ||
    autoStartOptions[0]
)

const getAvailableFonts = () => {
  if (settingStore.availableFonts.length) return

  settingStore.getAvailableFonts()
}

const handleFontSelect = (font: FontValue) => {
  settingStore.setFontFamily(font)
  fontFamilyVisible.value = false
}

const toggleAutoStartState = async () => {
  const autoStartState = await isEnabled()
  settingStore.toggleAutoStartState(autoStartState)
}

const handleAutoStartChange = async (e: Event) => {
  if ((e.target as HTMLInputElement).checked) {
    await enable()
  } else {
    await disable()
  }

  toggleAutoStartState()
}

onMounted(() => {
  getAvailableFonts()
  toggleAutoStartState()
})
</script>

<template>
  <div class="flex text-base">
    <div class="font-bold w-40">常规:</div>

    <div class="space-y-3">
      <div class="relative text-sm" v-on-click-outside="() => (fontFamilyVisible = false)">
        <div
          class="flex cursor-pointer items-center whitespace-nowrap font-bold"
          @click="fontFamilyVisible = !fontFamilyVisible">
          <div>{{ fontFamilySelection?.label || '默认字体 ' }}</div>
          <SvgIcon
            class="transition-transform"
            :class="fontFamilyVisible ? 'rotate-180' : ''"
            name="Down" />
        </div>

        <SelectModal
          class="absolute -left-4 top-full h-64"
          transition="zoom-top"
          :visible="fontFamilyVisible"
          :options="fontFamilyOptions"
          :selection="fontFamilySelection"
          @select="handleFontSelect" />
      </div>

      <div class="flex gap-3 items-center">
        <label class="flex items-center gap-2 cursor-pointer">
          <input
            type="checkbox"
            v-model="settingStore.autoStartState"
            @change="handleAutoStartChange" />
          开机启动
        </label>

        <div class="relative text-sm" v-on-click-outside="() => (autoStartModeVisible = false)">
          <div
            class="flex cursor-pointer items-center whitespace-nowrap font-bold"
            @click="autoStartModeVisible = !autoStartModeVisible">
            <div>{{ autoStartModeSelection.label }}</div>
            <SvgIcon
              class="transition-transform"
              :class="autoStartModeVisible ? 'rotate-180' : ''"
              name="Down" />
          </div>

          <SelectModal
            class="absolute -left-4 top-full"
            transition="zoom-top"
            :visible="autoStartModeVisible"
            :options="autoStartOptions"
            :selection="autoStartModeSelection" />
        </div>
      </div>

      <label class="flex items-center gap-2 cursor-pointer" :data-disabled="true">
        <input type="checkbox" />
        最小化时, 开启节省性能模式
      </label>

      <label class="flex items-center gap-2 cursor-pointer" :data-disabled="true">
        <input type="checkbox" />
        将 Seraphine 设为默认播放器
      </label>

      <label class="flex items-center gap-2 cursor-pointer" :data-disabled="true">
        <input type="checkbox" />
        禁用动画效果
      </label>

      <!-- <label class="flex items-center gap-2 cursor-pointer mt-3" :data-disabled="true">
          <input type="checkbox" />
          禁用系统缩放比例
        </label> -->
    </div>
  </div>
</template>
