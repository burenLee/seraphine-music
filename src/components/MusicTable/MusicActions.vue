<script lang="ts" setup>
import Add from './Actions/Add/Add.vue'
import AllCheckBtn from './Actions/AllCheck.vue'
import CancelBtn from './Actions/Cancel.vue'
import ClearBtn from './Actions/Clear.vue'
import Manage from './Actions/Manage.vue'
import PlayBtn from './Actions/Play.vue'
import RemoveBtn from './Actions/Remove.vue'
import Search from './Actions/Search.vue'
import Sort from './Actions/Sort.vue'
import { useListStore } from '@/stores/list'
import { ListType } from '@/utils/params.ts'

const listType = inject<ListType>('listType', ListType.Show)

const listStore = useListStore()
</script>

<template>
  <div class="flex justify-between w-full px-8">
    <div class="flex items-center gap-2">
      <template v-if="!listStore.isChecking">
        <PlayBtn />
      </template>

      <template v-else>
        <AllCheckBtn />
        <RemoveBtn />
        <ClearBtn />
        <CancelBtn />
      </template>
    </div>

    <div class="card flex items-center border border-border">
      <Search />
      <Sort />
      <Add :data-disabled="listType !== ListType.Local" />
      <Manage :data-disabled="listType !== ListType.Local" />
    </div>
  </div>
</template>
