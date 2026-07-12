export const useRefreshStore = defineStore('refresh', () => {
  const key = ref(0)

  const refresh = () => {
    key.value++
  }

  return {
    key,
    refresh
  }
})
