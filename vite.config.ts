import vue from '@vitejs/plugin-vue';
import Icons from 'unplugin-icons/vite';
import { defineConfig } from 'vite';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  resolve: {
    alias: { '@': '/src' },
  },
  plugins: [vue(), Icons()],
  build: {
    target: 'es2020',
    minify: 'oxc',
    cssMinify: 'lightningcss',
    rolldownOptions: {
      input: {
        main: './index.html',
        desktopLyric: './desktop-lyric.html',
        miniPlayer: './mini-player.html',
      },
      output: {
        minify: {
          compress: { dropConsole: true },
        },
      },
    },
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
      ignored: ['**/src-tauri/**'],
    },
  },
});
