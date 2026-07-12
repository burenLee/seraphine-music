<!-- filepath: e:\Code\tauri\SeraphineMusic\src\layout\Header\Login\Sidebar.vue -->
<script lang="ts" setup>
import loginBg from '@/assets/login_bg.jpg'
import { LoginMode } from '@/utils/params'

const mode = defineModel({ required: true, default: LoginMode.Code })
</script>

<template>
  <div
    class="sidebar-login relative z-20 h-full overflow-hidden transition-transform duration-500 before:absolute before:left-0 before:top-0 before:h-full before:w-[40rem] before:bg-cover before:bg-center before:transition-transform before:duration-500 after:absolute after:left-0 after:top-0 after:size-full after:bg-black/30"
    :class="
      mode === LoginMode.Code
        ? 'translate-x-0 before:-translate-x-0'
        : 'translate-x-[26rem] before:-translate-x-[26rem]'
    "
    :style="{ '--login-bg': `url(${loginBg})` }">
    <div
      class="relative z-10 flex h-full select-none flex-col items-center font-poetry text-4xl text-white"
      :style="{ writingMode: 'vertical-lr' }">
      <div
        class="w-28 transition-[opacity.transform] duration-500"
        :class="mode === LoginMode.Code ? 'translate-x-[4.5rem] opacity-100' : 'opacity-0'">
        <div>伤心桥下春波绿</div>
        <div class="indent-12">曾是惊鸿照影来</div>
      </div>

      <div
        class="w-28 pl-8 transition-[opacity.transform] duration-500"
        :class="mode === LoginMode.Code ? 'opacity-0' : 'translate-x-[-4.5rem] opacity-100'">
        <div class="indent-12">事如春梦了无痕</div>
        <div>人似秋鸿来有信</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.sidebar-login::before {
  background-image: var(--login-bg);
}
</style>
