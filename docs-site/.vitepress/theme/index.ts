import DefaultTheme from 'vitepress/theme'
import type { Theme } from 'vitepress'
import './custom.css'
import HomeLanding from './components/HomeLanding.vue'

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component('HomeLanding', HomeLanding)
    if (typeof document !== 'undefined') {
      document.documentElement.classList.remove('dark')
      document.documentElement.style.colorScheme = 'light'
      try {
        localStorage.setItem('vitepress-theme-appearance', 'light')
      } catch {
        /* ignore private-mode storage */
      }
    }
  },
} satisfies Theme
