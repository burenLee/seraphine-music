<script lang="ts" setup>
import { computed } from 'vue';

import Image from '@/components/Image.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import { YouthVip } from '@/constants';
import { useSettingStore } from '@/stores/setting';
import { useUserStore } from '@/stores/user';

const settingStore = useSettingStore();
const userStore = useUserStore();

const isSvip = computed(() => userStore.userinfo?.youthVip === YouthVip.Svip);
</script>

<template>
  <div class="flex text-base">
    <div class="w-40 font-bold">用户:</div>

    <div class="space-y-3">
      <div v-if="userStore.userinfo" class="relative flex cursor-pointer items-center gap-1">
        <Image class="size-8 rounded-full" :src="userStore.userinfo.pic" icon="User" />
        <div class="max-w-24 truncate font-bold" :title="userStore.userinfo.nickname">
          {{ userStore.userinfo.nickname }}
        </div>
        <SvgIcon class="transition-transform" name="Down" />

        <div
          v-if="isSvip"
          class="absolute bottom-0 left-5 flex size-4 scale-75 items-center justify-center rounded-full bg-info text-xs font-bold text-neutral-100"
        >
          v
        </div>
      </div>

      <label class="flex cursor-pointer items-center gap-1">
        <input type="checkbox" v-model="settingStore.autoYouthVip" />
        自动领取概念版会员
      </label>
    </div>
  </div>
</template>
