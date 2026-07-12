<script lang="ts" setup>
import Cache from './Cache.vue'
import Close from './Close.vue'
import Device from './Device.vue'
import General from './General.vue'
import Shortcut from './Shortcut.vue'
import User from './User.vue'
import Version from './Version.vue'
import SvgIcon from '@/components/SvgIcon.vue'
</script>

<template>
  <div class="main space-y-8">
    <div class="flex items-center gap-2">
      <SvgIcon name="Setting" size="20" />
      <div class="text-xl font-bold">设置</div>
    </div>

    <User />
    <General />
    <Close />
    <Shortcut />
    <Device />
    <Cache />
    <Version />
  </div>
</template>
