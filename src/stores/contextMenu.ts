interface ShowOptions {
  x: number
  y: number
  options: ContextMenuOption[]
}

export const useContextMenuStore = defineStore('contextMenu', () => {
  const visible = ref(false)
  const position = ref({ x: 0, y: 0 })
  const options = ref<ContextMenuOption[]>([])

  const show = ({ x, y, options: newOptions }: ShowOptions) => {
    visible.value = false
    position.value = { x, y }
    options.value = newOptions

    nextTick(() => (visible.value = true))
  }

  const hide = () => {
    visible.value = false
  }

  return {
    visible,
    position,
    options,

    show,
    hide
  }
})
