<script lang="ts">
const notifyState: Notify.State = {
  counter: 0,
  subscribers: [],
  subscribe: (subscriber) => notifyState.subscribers.push(subscriber),
  publish: (data) => notifyState.subscribers.forEach((subscriber) => subscriber(data)),
  create: (type: Notify.Type) => (content: string | Notify.Options) => {
    const id = notifyState.counter++;
    const options = typeof content === 'string' ? { message: content } : content;
    notifyState.publish({ ...options, id, type });
  },
};

export const notify = {
  success: notifyState.create('success'),
  info: notifyState.create('info'),
  warning: notifyState.create('warning'),
  error: notifyState.create('error'),
};
</script>

<script lang="ts" setup>
import { onBeforeUnmount, ref } from 'vue';

import SvgIcon from '@/components/SvgIcon.vue';

defineOptions({ name: 'NotificationContainer' });

const typeTheme = {
  success: 'bg-success-bg text-success border-success',
  warning: 'bg-warning-bg text-warning border-warning',
  info: 'bg-info-bg text-info border-info',
  error: 'bg-error-bg text-error border-error',
};

const notifications = ref<Notify.Info[]>([]);
const timers = new Map<number, ReturnType<typeof setTimeout>>();

const remove = (id: number) => {
  const index = notifications.value.findIndex((n) => n.id === id);
  if (index === -1) return;

  notifications.value.splice(index, 1);
  const timer = timers.get(id);
  if (timer) {
    clearTimeout(timer);
    timers.delete(id);
  }
};

const addTimer = (info: Notify.Info) => {
  const duration = info.duration ?? 3000;

  const timer = setTimeout(() => remove(info.id), duration);
  timers.set(info.id, timer);
};

notifyState.subscribe((data) => {
  notifications.value.push(data);
  addTimer(data);
});

onBeforeUnmount(() => {
  timers.forEach((timer) => clearTimeout(timer));
  timers.clear();
});
</script>

<template>
  <ul class="pointer-events-none fixed right-0 top-[var(--header-height)] z-50 m-4 max-h-screen">
    <TransitionGroup name="notification">
      <li
        v-for="notification in notifications"
        :key="notification.id"
        class="pointer-events-auto mb-4 flex w-48 items-center gap-2 overflow-hidden rounded-lg border p-3 shadow-lg shadow-shadow"
        :class="typeTheme[notification.type]"
      >
        <div class="line-clamp-3 min-w-0 flex-1 font-bold">
          {{ notification.message }}
        </div>
        <SvgIcon
          class="size-5 shrink-0 cursor-pointer"
          name="Close"
          :size="20"
          @click="remove(notification.id)"
        />
      </li>
    </TransitionGroup>
  </ul>
</template>
