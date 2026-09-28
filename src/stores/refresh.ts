import { defineStore } from 'pinia';
import { ref } from 'vue';

/** 局部组件的刷新 */
export const useRefreshStore = defineStore('refresh', () => {
  const key = ref(0);

  const refresh = () => key.value++;

  return {
    key,
    refresh,
  };
});
