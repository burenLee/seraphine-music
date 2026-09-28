import { defineConfig, mergeConfig } from 'vitest/config';

import viteConfig from './vite.config.ts';

export default mergeConfig(
  viteConfig,
  defineConfig({
    plugins: [],
    test: {
      environment: 'happy-dom',
      include: ['tests/**/*.spec.ts'],
      setupFiles: ['./tests/setup.ts'],
    },
  }),
);
