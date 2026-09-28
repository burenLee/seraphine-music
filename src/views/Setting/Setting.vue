<script lang="ts" setup>
import SvgIcon from '@/components/SvgIcon.vue';

import Cache from './Cache.vue';
import Close from './Close.vue';
import Device from './Device.vue';
import General from './General.vue';
import Shortcut from './Shortcut.vue';
import User from './User.vue';
import Version from './Version.vue';
</script>

<template>
  <div class="main space-y-8">
    <div class="flex items-center gap-2">
      <SvgIcon name="Setting" size="20" />
      <div class="text-xl font-bold">设置</div>
    </div>

    <div class="h-px w-full bg-border"></div>
    <User />
    <div class="h-px w-full bg-border"></div>
    <General />
    <div class="h-px w-full bg-border"></div>
    <Close />
    <div class="h-px w-full bg-border"></div>
    <Shortcut />
    <div class="h-px w-full bg-border"></div>
    <Device />
    <div class="h-px w-full bg-border"></div>
    <Cache />
    <div class="h-px w-full bg-border"></div>
    <Version />
  </div>
</template>
