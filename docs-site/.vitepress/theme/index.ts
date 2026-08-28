import DefaultTheme from 'vitepress/theme'
import type { Theme } from 'vitepress'
import './custom.css'
import HomeLanding from './components/HomeLanding.vue'

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component('HomeLanding', HomeLanding)
    if (typeof document !== 'undefined') {
      // appearance:false still leaves html.dark if localStorage remembers it.
      document.documentElement.classList.remove('dark')
      document.documentElement.style.colorScheme = 'light'
    }
  },
} satisfies Theme
