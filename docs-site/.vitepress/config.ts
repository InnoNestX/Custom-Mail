import { defineConfig } from 'vitepress'
import { writeFileSync } from 'node:fs'
import { join } from 'node:path'

const base = '/Custom-Mail/'
const siteUrl = 'https://innonestx.github.io/Custom-Mail/'

const enSidebar = [
  {
    text: 'Start',
    collapsed: true,
    items: [
      { text: 'Overview', link: '/' },
      { text: 'Quick start', link: '/quick-start' },
    ],
  },
  {
    text: 'Run',
    collapsed: true,
    items: [
      { text: 'Console', link: '/console' },
      { text: 'Docker', link: '/docker' },
      { text: 'OpenClaw', link: '/openclaw' },
    ],
  },
  {
    text: 'Customize',
    collapsed: true,
    items: [
      { text: 'Plugins', link: '/plugins' },
      { text: 'Configuration', link: '/config' },
    ],
  },
  {
    text: 'Ship',
    collapsed: true,
    items: [
      { text: 'Deploy', link: '/deploy' },
      { text: 'FAQ', link: '/faq' },
    ],
  },
]

const zhSidebar = [
  {
    text: '开始',
    collapsed: true,
    items: [
      { text: '概览', link: '/zh/' },
      { text: '快速开始', link: '/zh/quick-start' },
    ],
  },
  {
    text: '运行',
    collapsed: true,
    items: [
      { text: '控制台', link: '/zh/console' },
      { text: 'Docker', link: '/zh/docker' },
      { text: 'OpenClaw', link: '/zh/openclaw' },
    ],
  },
  {
    text: '定制',
    collapsed: true,
    items: [
      { text: '插件', link: '/zh/plugins' },
      { text: '配置说明', link: '/zh/config' },
    ],
  },
  {
    text: '上线',
    collapsed: true,
    items: [
      { text: '部署', link: '/zh/deploy' },
      { text: '常见问题', link: '/zh/faq' },
    ],
  },
]

export default defineConfig({
  title: 'Custom Mail',
  description:
    'Private web mail console on Cloudflare Workers. Drop-in provider, theme, layout, logo, and config plugins.',
  base,
  srcDir: 'docs',
  outDir: '_site',
  cacheDir: '.vitepress/cache',
  cleanUrls: false,
  ignoreDeadLinks: true,
  lastUpdated: true,
  // Site chrome is designed for the light paper look. Disable appearance
  // toggle so OS / leftover localStorage dark mode cannot wash out nav text.
  appearance: false,
  sitemap: {
    hostname: siteUrl,
  },
  buildEnd(siteConfig) {
    writeFileSync(
      join(siteConfig.outDir, 'robots.txt'),
      `User-agent: *\nAllow: /\n\nSitemap: ${siteUrl}sitemap.xml\n`,
    )
  },
  head: [
    [
      'script',
      {},
      `document.documentElement.classList.remove('dark');document.documentElement.style.colorScheme='light';try{localStorage.setItem('vitepress-theme-appearance','light')}catch(e){}`,
    ],
    ['link', { rel: 'icon', href: `${base}favicon.ico`, sizes: 'any' }],
    ['link', { rel: 'icon', href: `${base}images/favicon.svg`, type: 'image/svg+xml' }],
    ['link', { rel: 'icon', href: `${base}images/favicon-32.png`, type: 'image/png', sizes: '32x32' }],
    ['link', { rel: 'apple-touch-icon', href: `${base}images/apple-touch-icon.png`, sizes: '180x180' }],
    ['link', { rel: 'canonical', href: siteUrl }],
    ['meta', { name: 'theme-color', content: '#15624f' }],
    ['meta', { name: 'author', content: 'InnoNestX' }],
    [
      'meta',
      {
        name: 'keywords',
        content:
          'Custom Mail, Cloudflare Workers, plugins, Brevo, Resend, SendGrid, webmail, outbound email, self-hosted mail, InnoNestX',
      },
    ],
    ['meta', { name: 'robots', content: 'index,follow' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:site_name', content: 'Custom Mail' }],
    ['meta', { property: 'og:title', content: 'Custom Mail — Private Web Mail on Cloudflare Workers' }],
    [
      'meta',
      {
        property: 'og:description',
        content:
          'Self-hosted outbound mail. Drop-in catalogs for ESP, theme, layout, logo, and config.',
      },
    ],
    ['meta', { property: 'og:url', content: siteUrl }],
    ['meta', { property: 'og:image', content: `${siteUrl}images/logo.png` }],
  ],
  themeConfig: {
    logo: {
      src: '/images/logo.svg',
      alt: '',
    },
    siteTitle: 'Custom Mail',
    nav: [
      { text: 'Docs', link: '/' },
      { text: 'Plugins', link: '/plugins' },
      { text: 'Live demo', link: 'https://mail.xuxuclassmate.com' },
      { text: 'GitHub', link: 'https://github.com/InnoNestX/Custom-Mail' },
    ],
    sidebar: {
      '/': enSidebar,
      '/zh/': zhSidebar,
    },
    footer: {
      message: 'Open Source · MIT License',
      copyright: '© 2026 InnoNestX',
    },
    editLink: {
      pattern: 'https://github.com/InnoNestX/Custom-Mail/edit/main/docs-site/docs/:path',
      text: 'Edit this page on GitHub',
    },
    search: {
      provider: 'local',
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/InnoNestX/Custom-Mail' },
    ],
  },
  locales: {
    root: {
      label: 'English',
      lang: 'en-US',
      title: 'Custom Mail',
      description:
        'Private web mail console on Cloudflare Workers. Drop-in provider, theme, layout, logo, and config plugins.',
    },
    zh: {
      label: '中文',
      lang: 'zh-CN',
      link: '/zh/',
      title: 'Custom Mail',
      description:
        '基于 Cloudflare Workers 的私有网页发信控制台。服务商、主题、版式、Logo、配置均可插拔。',
      themeConfig: {
        nav: [
          { text: '文档', link: '/zh/' },
          { text: '插件', link: '/zh/plugins' },
          { text: '在线演示', link: 'https://mail.xuxuclassmate.com' },
          { text: 'GitHub', link: 'https://github.com/InnoNestX/Custom-Mail' },
        ],
        sidebar: zhSidebar,
        editLink: {
          pattern: 'https://github.com/InnoNestX/Custom-Mail/edit/main/docs-site/docs/:path',
          text: '在 GitHub 上编辑此页',
        },
        docFooter: {
          prev: '上一页',
          next: '下一页',
        },
        footer: {
          message: '开源 · MIT License',
          copyright: '© 2026 InnoNestX',
        },
      },
    },
  },
  markdown: {
    theme: {
      light: 'github-light',
      dark: 'github-dark',
    },
    lineNumbers: true,
  },
})
