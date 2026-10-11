import '@/styles/global.css';

import { createPinia } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import { createApp } from 'vue';

import { interdictHotkeys } from '@/utils/tools';

import MiniPlayerWindow from './MiniPlayer.vue';

interdictHotkeys();

const app = createApp(MiniPlayerWindow).use(createPinia().use(piniaPluginPersistedstate));

app.mount('#app');
