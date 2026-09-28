import '@/styles/global.css';

import { createPinia } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import { createApp } from 'vue';

import { setupErrorCapture } from '@/utils/logger';
import { interdictHotkeys } from '@/utils/tools';

import MiniPlayerWindow from './MiniPlayer.vue';

const app = createApp(MiniPlayerWindow).use(createPinia().use(piniaPluginPersistedstate));

interdictHotkeys();
setupErrorCapture(app);

app.mount('#app');
