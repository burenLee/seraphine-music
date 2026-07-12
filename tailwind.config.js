/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  darkMode: 'selector',
  theme: {
    extend: {
      fontFamily: {
        poetry: 'poetry'
      },
      colors: {
        background: 'var(--color-background)',
        foreground: 'var(--color-foreground)',
        accent: 'var(--color-accent)',
        minor: 'var(--color-minor)',
        card: 'var(--color-card)',
        border: 'var(--color-border)',
        shadow: 'var(--color-shadow)',
        hover: 'var(--color-hover)',
        actived: 'var(--color-actived)',

        info: 'var(--color-info)',
        'info-bg': 'var(--color-info-bg)',
        success: 'var(--color-success)',
        'success-bg': 'var(--color-success-bg)',
        warning: 'var(--color-warning)',
        'warning-bg': 'var(--color-warning-bg)',
        error: 'var(--color-error)',
        'error-bg': 'var(--color-error-bg)'
      },
      animation: {
        'spin-slow': 'spin 15s linear infinite',
        carousel: 'carousel linear infinite'
      },
      keyframes: {
        carousel: {
          '0%': { transform: 'translateX(0)' },
          '100%': { transform: 'translateX(-50%)' }
        }
      }
    }
  },
  plugins: []
}
