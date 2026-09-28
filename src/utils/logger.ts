import { error } from '@tauri-apps/plugin-log';
import { App } from 'vue';

export function setupErrorCapture(app: App) {
  app.config.errorHandler = async (err, _instance, info) => {
    await error(`Vue错误 ${info}: ${String(err)}`);
  };
}
