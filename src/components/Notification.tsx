import SvgIcon from '@/components/SvgIcon.vue'
import { TransitionGroup } from 'vue'

const typeTheme = {
  success: 'bg-success-bg text-success border-success',
  warning: 'bg-warning-bg text-warning border-warning',
  info: 'bg-info-bg text-info border-info',
  error: 'bg-error-bg text-error border-error'
}

interface NotificationProps {
  info: Notify.Info
  onClose: (id: number) => void
}

const Notification = defineComponent<NotificationProps>({
  props: {
    info: {
      type: Object as PropType<Notify.Info>,
      required: true
    },
    onClose: {
      type: Function as PropType<(id: number) => void>,
      required: true
    }
  },
  setup(props) {
    const duration = props.info.duration ?? 3000
    let timer: ReturnType<typeof setTimeout> | null = null

    onMounted(() => {
      if (!duration) return

      timer = setTimeout(() => {
        props.onClose(props.info.id)
      }, duration)
    })

    onUnmounted(() => {
      if (timer === null) return

      clearTimeout(timer)
      timer = null
    })

    return () => (
      <li
        class={`pointer-events-auto mb-4 flex w-56 items-center gap-3 overflow-hidden rounded-lg border p-4 shadow-lg shadow-shadow ${typeTheme[props.info.type]}`}>
        <div class="line-clamp-5 min-w-0 flex-1 font-bold">{props.info.message}</div>
        <SvgIcon
          class="size-5 flex-shrink-0 cursor-pointer"
          name="Close"
          size={10}
          onClick={() => props.onClose(props.info.id)}
        />
      </li>
    )
  }
})

export const NotificationContainer = defineComponent({
  setup() {
    const notifications = ref<Notify.Info[]>([])

    const remove = (id: number) => {
      const index = notifications.value.findIndex((n) => n.id === id)
      if (index === -1) return

      notifications.value.splice(index, 1)
    }

    notifyState.subscribe((data) => notifications.value.push(data))

    return () => (
      <ul class="pointer-events-none fixed right-0 top-[var(--header-height)] z-50 m-4 max-h-screen">
        <TransitionGroup name="notification">
          {notifications.value.map((notification) => (
            <Notification key={notification.id} info={notification} onClose={remove} />
          ))}
        </TransitionGroup>
      </ul>
    )
  }
})

const notifyState: Notify.State = {
  counter: 0,
  subscribers: [],
  subscribe: (subscriber) => notifyState.subscribers.push(subscriber),
  publish: (data) => notifyState.subscribers.forEach((subscriber) => subscriber(data)),
  create: (type: Notify.Type) => (content: string | Notify.Options) => {
    const id = notifyState.counter++
    const options = typeof content === 'string' ? { message: content } : content
    notifyState.publish({ ...options, id, type })
  }
}

export const notify = {
  success: notifyState.create('success'),
  info: notifyState.create('info'),
  warning: notifyState.create('warning'),
  error: notifyState.create('error')
}
