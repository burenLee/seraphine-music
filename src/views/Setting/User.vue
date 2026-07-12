<script lang="ts" setup>
import Image from '@/components/Image.vue'
import SvgIcon from '@/components/SvgIcon.vue'
import { useSettingStore } from '@/stores/setting'
import { useUserStore } from '@/stores/user'

const settingStore = useSettingStore()
const userStore = useUserStore()
</script>

<template>
  <div class="flex text-base">
    <div class="font-bold w-40">用户:</div>

    <div class="space-y-3">
      <div v-if="userStore.userinfo" class="relative flex cursor-pointer items-center">
        <Image class="size-6 rounded-full" :img="userStore.userinfo.pic" icon="User" />
        <div class="max-w-24 truncate pl-1 font-bold" :title="userStore.userinfo.nickname">
          {{ userStore.userinfo.nickname }}
        </div>
        <SvgIcon class="transition-transform" name="Down" />

        <div
          v-if="userStore.isVip"
          class="absolute -bottom-1 left-3 flex items-center border border-border pl-1 font-bold size-4 rounded-full bg-info scale-[70%] text-xs text-background">
          v
        </div>
      </div>

      <label class="flex items-center gap-2 cursor-pointer" :data-disabled="true">
        <input type="checkbox" v-model="settingStore.autoLiteVipState" />
        自动领取概念版会员
      </label>
    </div>
  </div>
</template>
