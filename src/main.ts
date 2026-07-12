import router from '@/router/index'
import '@/styles/global.css'
import App from '@/views/App.vue'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

if (!import.meta.env.DEV) {
  window.addEventListener('contextmenu', (e) => e.preventDefault(), { capture: true })

  window.addEventListener(
    'keydown',
    (e) => {
      const ctrlMeta = e.ctrlKey || e.metaKey

      const isBrowserShortcut =
        e.key === 'Tab' ||
        e.key === 'F5' ||
        e.key === 'F12' ||
        (ctrlMeta && (e.key === 'r' || e.key === 'R')) || // 刷新
        (ctrlMeta && e.shiftKey && (e.key === 'r' || e.key === 'R')) || // 强制刷新
        (ctrlMeta && e.key === 'w') || // 关闭标签
        (ctrlMeta && e.key === 't') || // 新建标签
        (ctrlMeta && e.key === 'p') || // 打印
        (ctrlMeta && e.key === 's') || // 保存
        (ctrlMeta && e.key === 'f') || // 查找
        (ctrlMeta && e.key === 'Tab') || // 切换标签
        (ctrlMeta && e.shiftKey && /[ijc]/i.test(e.key)) || // 开发者工具 / 控制台 / 检查
        (e.altKey && (e.key === 'ArrowLeft' || e.key === 'ArrowRight')) // 后退/前进

      if (isBrowserShortcut) {
        e.preventDefault()
        e.stopImmediatePropagation()
      }
    },
    { capture: true }
  )
}

const pinia = createPinia().use(piniaPluginPersistedstate)

createApp(App).use(pinia).use(router).mount('#app')
