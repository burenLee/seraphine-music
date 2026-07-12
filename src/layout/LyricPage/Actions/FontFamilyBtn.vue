<script lang="ts" setup>
import SelectModal from '@/components/SelectModal.vue'
import { useLyricStore } from '@/stores/lyric'
import { useSettingStore } from '@/stores/setting'
import { vOnClickOutside } from '@vueuse/components'

const lyricStore = useLyricStore()
const settingStore = useSettingStore()

const fontFamilyVisible = ref(false)
const fontFamilyOptions = ref<Array<SelectOption<FontValue>>>([])

const fontFamilySelection = computed(
  () =>
    fontFamilyOptions.value.find((item) => item.value === lyricStore.setting.fontFamily) ||
    fontFamilyOptions.value[0]
)

const handleClick = () => {
  fontFamilyVisible.value = !fontFamilyVisible.value

  if (settingStore.availableFonts.length === 0) settingStore.getAvailableFonts()
  fontFamilyOptions.value = settingStore.availableFonts.map(([label, value]) => ({ label, value }))
}

const handleSelect = (font: FontValue) => {
  lyricStore.setFontFamily(font)
  fontFamilyVisible.value = false
}
</script>

<template>
  <div class="relative" v-on-click-outside="() => (fontFamilyVisible = false)">
    <div
      class="action-icon text-base card flex justify-center items-center"
      title="歌词字体"
      @click="handleClick">
      A
    </div>

    <SelectModal
      class="absolute right-full top-0 mr-2 h-64 overflow-y-auto"
      :visible="fontFamilyVisible"
      :options="fontFamilyOptions"
      :selection="fontFamilySelection"
      @select="handleSelect">
    </SelectModal>
  </div>
</template>
