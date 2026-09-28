<script lang="ts" setup>
import { computed } from 'vue';

import Image from '@/components/Image.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { useSettingStore } from '@/stores/setting';
import { useUserStore } from '@/stores/user';
import { YouthVip } from '@/utils/params';

const settingStore = useSettingStore();
const userStore = useUserStore();

const isSvip = computed(() => userStore.userinfo?.youthVip === YouthVip.Svip);
</script>

<template>
  <div class="flex text-base">
    <div class="w-40 font-bold">用户:</div>

    <div class="space-y-3">
      <div v-if="userStore.userinfo" class="relative flex cursor-pointer items-center">
        <Image class="size-6 rounded-full" :src="userStore.userinfo.pic" icon="User" />
        <div class="max-w-24 truncate pl-1 font-bold" :title="userStore.userinfo.nickname">
          {{ userStore.userinfo.nickname }}
        </div>
        <SvgIcon class="transition-transform" name="Down" />

        <div
          v-if="isSvip"
          class="absolute -bottom-1 left-3 flex size-4 scale-[70%] items-center rounded-full border border-border bg-info pl-1 text-xs font-bold"
        >
          v
        </div>
      </div>

      <label class="flex cursor-pointer items-center gap-2" :data-disabled="true">
        <input type="checkbox" v-model="settingStore.autoLiteVipState" />
        自动领取概念版会员
      </label>
    </div>
  </div>
</template>
