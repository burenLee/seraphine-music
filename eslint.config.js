import skipFormatting from '@vue/eslint-config-prettier/skip-formatting'
import { defineConfigWithVueTs, vueTsConfigs } from '@vue/eslint-config-typescript'
import pluginVue from 'eslint-plugin-vue'
import { globalIgnores } from 'eslint/config'

export default defineConfigWithVueTs(
  {
    name: 'app/files-to-lint',
    files: ['**/*.{ts,mts,tsx,vue}']
  },
  globalIgnores(['**/dist/**', '**/dist-ssr/**', '**/coverage/**']),
  pluginVue.configs['flat/essential'],
  vueTsConfigs.recommended,
  skipFormatting,
  { ignores: ['node_modules', 'dist', 'src-tauri', '.env', '.vscode', '.idea'] },
  {
    rules: {
      'vue/multi-word-component-names': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/ban-ts-comment': 'off'
    }
  }
)
