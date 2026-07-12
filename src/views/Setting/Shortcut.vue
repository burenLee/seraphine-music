<script lang="ts" setup>
import ActionButton from '@/components/ActionButton.vue'
import { useSettingStore } from '@/stores/setting'
import { ShortcutKey } from '@/utils/params'

const settingStore = useSettingStore()

const shortcutGroups: Array<Array<{ label: string; key: ShortcutKey }>> = [
  [
    { key: ShortcutKey.PlayOrPause, label: '播放/暂停' },
    { key: ShortcutKey.AddVolumn, label: '增大音量' },
    { key: ShortcutKey.SubVolumn, label: '减小音量' },
    { key: ShortcutKey.Mute, label: '静音' }
  ],
  [
    { key: ShortcutKey.Prev, label: '上一首' },
    { key: ShortcutKey.Next, label: '下一首' },
    { key: ShortcutKey.Forward, label: '快进' },
    { key: ShortcutKey.Backward, label: '快退' }
  ]
]

const handleReset = async () => {
  await settingStore.unregisterAllGlobalShortcut()
  settingStore.resetShortcutMap()

  if (settingStore.globalShortcutState) {
    settingStore.registerAllGlobalShortcut()
  }
}

const handleKeydown = (e: KeyboardEvent, type: ShortcutKey) => {
  if (e.repeat) return
  e.preventDefault()

  if (e.key === 'Escape') {
    ;(e.target as HTMLInputElement).blur()
    return
  }

  if (e.key === 'Backspace' || e.key === 'Delete') {
    settingStore.setShortcutMap(type, '')
    return
  }

  const keys: string[] = []

  if (e.ctrlKey) keys.push('Ctrl')
  if (e.shiftKey) keys.push('Shift')
  if (e.altKey) keys.push('Alt')
  if (e.metaKey) keys.push('Meta')

  if (!['Control', 'Shift', 'Alt', 'Meta'].includes(e.key)) {
    keys.push(e.key.length === 1 ? e.key.toUpperCase() : e.key)
  }

  if (keys.length === 0) return

  const shortcut = keys.join('+')
  if (Object.values(settingStore.shortcutMap).includes(shortcut)) return

  settingStore.setShortcutMap(type, keys.join('+'))
}

const handleBlur = () => {
  settingStore.registerAllGlobalShortcut()
}

const handleClick = () => {
  settingStore.unregisterAllGlobalShortcut()
}
</script>

<template>
  <div class="flex text-base">
    <div class="font-bold w-40">快捷键:</div>

    <div>
      <label class="flex w-fit items-center gap-2 cursor-pointer">
        <input
          type="checkbox"
          :checked="settingStore.globalShortcutState"
          @change="settingStore.toggleGlobalShortcutState" />
        启用全局快捷键

        <ActionButton theme="success" @click="handleReset">重置</ActionButton>
      </label>

      <label class="flex w-fit items-center gap-2 cursor-pointer mt-3">
        <input
          type="checkbox"
          :checked="settingStore.mediaShortcutState"
          @change="settingStore.toggleMediaShortcutState" />
        启用媒体快捷键
      </label>

      <div class="flex gap-6 flex-1 mt-3" :data-disabled="!settingStore.globalShortcutState">
        <div class="flex-1 space-y-3" v-for="(shortcut, index) in shortcutGroups" :key="index">
          <div v-for="item in shortcut" :key="item.key" class="flex items-center gap-3">
            <div class="w-20">{{ item.label }}</div>
            <input
              class="card px-2 py-1"
              :value="settingStore.shortcutMap[item.key]"
              placeholder="按下快捷键"
              readonly
              @keydown.prevent="handleKeydown($event, item.key)"
              @blur="handleBlur"
              @click="handleClick" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
