import '@/styles/global.css';

import { createPinia } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import { createApp } from 'vue';

import { setupErrorCapture } from '@/utils/logger';
import { interdictHotkeys } from '@/utils/tools';

import DesktopLyricWindow from './DesktopLyric.vue';

interdictHotkeys();

const app = createApp(DesktopLyricWindow).use(createPinia().use(piniaPluginPersistedstate));

setupErrorCapture(app);

app.mount('#app');
