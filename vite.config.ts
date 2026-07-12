import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import AutoImport from 'unplugin-auto-import/vite'
import { FileSystemIconLoader } from 'unplugin-icons/loaders'
import Icons from 'unplugin-icons/vite'
import { defineConfig } from 'vite'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST

// https://vite.dev/config/
export default defineConfig({
  resolve: { alias: { '@': '/src' } },
  plugins: [
    vue(),
    vueJsx(),
    Icons({ customCollections: { 'custom-icons': FileSystemIconLoader('./src/assets/svg') } }),
    AutoImport({
      imports: ['vue', 'pinia', 'vue-router'],
      dts: './auto-imports.d.ts',
      eslintrc: { enabled: true, filepath: './.eslintrc-auto-import.json' }
    })
  ],
  build: {
    rollupOptions: {
      input: {
        main: './index.html',
        desktopLyric: './desktop-lyric.html'
      }
    }
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: 'ws', host, port: 1421 } : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**']
    }
  }
})
