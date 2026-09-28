import '@/styles/global.css';

import { getCurrentWindow } from '@tauri-apps/api/window';
import { createPinia } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import { createApp } from 'vue';

import router from '@/router/index';
import { interdictHotkeys } from '@/utils/tools';
import { invoke } from '@/utils/tools';

import App from './App.vue';
import vCopy from './directives/copy';
import { setupErrorCapture } from './utils/logger.ts';

// 由于tauri应用启动时在webview没有完成渲染前会出现白屏
// 所以在此延迟打开窗口
const main = getCurrentWindow();
main.show();
main.setFocus();

// 获取dfid, 很多接口要用所以暂时放到这里获取
invoke('api_register_dev');
// 禁用浏览器快捷键
interdictHotkeys();

const app = createApp(App)
  .use(createPinia().use(piniaPluginPersistedstate))
  .use(router)
  .directive('copy', vCopy);

setupErrorCapture(app);

app.mount('#app');
