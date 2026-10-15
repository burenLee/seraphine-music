<script lang="ts" setup>
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
import { vOnClickOutside } from '@vueuse/components';
import { computed, onMounted, ref } from 'vue';

import SelectModal from '@/components/SelectModal.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { AutoStartMode } from '@/constants';
import { useSettingStore } from '@/stores/setting';
import { useStateStore } from '@/stores/state';

const stateStore = useStateStore();
const settingStore = useSettingStore();

const autoStartOptions: Array<SelectOption<AutoStartMode>> = [
  { label: '前台', value: AutoStartMode.Foreground },
  { label: '最小化', value: AutoStartMode.Background, disabled: true },
];

const fontFamilyVisible = ref(false);
const autoStartModeVisible = ref(false);

const fontFamilyOptions = computed<Array<SelectOption<FontValue>>>(() =>
  stateStore.availableFonts.map(([label, value]: [string, FontValue]) => ({ label, value })),
);
const fontFamilySelection = computed<SelectOption<FontValue> | undefined>(
  () =>
    fontFamilyOptions.value.find((item) => item.value === stateStore.fontFamily) ||
    fontFamilyOptions.value[0],
);
const autoStartModeSelection = computed(
  () =>
    autoStartOptions.find((item) => item.value === settingStore.autoStartMode) ||
    autoStartOptions[0],
);

const handleFontFamilySelect = (font: FontValue) => {
  fontFamilyVisible.value = false;

  stateStore.setFontFamily(font);
};

const handleAutoStartChange = async (e: Event) => {
  if ((e.target as HTMLInputElement).checked) {
    await enable();
  } else {
    await disable();
  }

  settingStore.toggleAutoStart(await isEnabled());
};

onMounted(async () => {
  settingStore.toggleAutoStart(await isEnabled());
});
</script>

<template>
  <div class="flex text-base">
    <div class="w-40 font-bold">常规:</div>

    <div class="space-y-3">
      <div class="relative text-sm" v-on-click-outside="() => (fontFamilyVisible = false)">
        <div
          class="inline-flex cursor-pointer items-center gap-1"
          @click="fontFamilyVisible = !fontFamilyVisible"
        >
          <div class="whitespace-nowrap font-bold">
            {{ fontFamilySelection?.label || '默认字体 ' }}
          </div>
          <SvgIcon
            class="transition-transform"
            :class="fontFamilyVisible ? 'rotate-180' : ''"
            name="Down"
          />
        </div>

        <Transition name="zoom-top">
          <ul
            v-if="fontFamilyVisible"
            class="card absolute -left-4 top-full z-10 h-64 overflow-y-auto bg-background p-1.5 shadow-lg shadow-shadow"
          >
            <li
              v-for="(option, index) in fontFamilyOptions"
              :key="index"
              class="flex h-8 cursor-pointer items-center gap-1 rounded-lg px-2 transition-colors"
              :class="fontFamilySelection?.value === option.value ? 'card-actived' : 'card-hover'"
              :style="{ fontFamily: option.value }"
              :data-disabled="option.disabled"
              @click="handleFontFamilySelect(option.value)"
            >
              <div class="whitespace-nowrap px-1">{{ option.label }}</div>
            </li>

            <div v-if="!fontFamilyOptions.length" class="whitespace-nowrap p-4">暂无数据</div>
          </ul>
        </Transition>
      </div>

      <div class="flex items-center gap-3">
        <label class="flex cursor-pointer items-center gap-1">
          <input type="checkbox" v-model="settingStore.autoStart" @change="handleAutoStartChange" />
          开机启动
        </label>

        <div class="relative text-sm" v-on-click-outside="() => (autoStartModeVisible = false)">
          <div
            class="flex cursor-pointer items-center gap-1"
            @click="autoStartModeVisible = !autoStartModeVisible"
          >
            <div class="whitespace-nowrap font-bold">{{ autoStartModeSelection.label }}</div>
            <SvgIcon
              class="transition-transform"
              :class="autoStartModeVisible ? 'rotate-180' : ''"
              name="Down"
            />
          </div>

          <SelectModal
            class="absolute left-1/2 top-full -translate-x-1/2"
            transition="zoom-top"
            :visible="autoStartModeVisible"
            :options="autoStartOptions"
            :selection="autoStartModeSelection"
          />
        </div>
      </div>

      <label class="flex cursor-pointer items-center gap-1" :data-disabled="true">
        <input type="checkbox" />
        将 Seraphine 设为默认播放器
      </label>

      <label class="flex cursor-pointer items-center gap-1" :data-disabled="true">
        <input type="checkbox" />
        最小化时, 开启轻量模式
      </label>

      <label class="flex cursor-pointer items-center gap-1" :data-disabled="true">
        <input type="checkbox" />
        禁用动画效果
      </label>

      <!-- <label class="flex items-center gap-1 cursor-pointer mt-3" :data-disabled="true">
          <input type="checkbox" />
          禁用系统缩放比例
        </label> -->
    </div>
  </div>
</template>
