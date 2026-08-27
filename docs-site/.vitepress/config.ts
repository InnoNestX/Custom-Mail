import { defineConfig } from 'vitepress'
import { writeFileSync } from 'node:fs'
import { join } from 'node:path'

const base = '/Custom-Mail/'
const siteUrl = 'https://innonestx.github.io/Custom-Mail/'

export default defineConfig({
  title: 'Custom Mail',
  description:
    'Private web mail console on Cloudflare Workers. Compose, preview, attachments, and send history via Brevo.',
  base,
  srcDir: 'docs',
  outDir: '_site',
  cacheDir: '.vitepress/cache',
  cleanUrls: false,
  ignoreDeadLinks: true,
  lastUpdated: true,
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
    ['link', { rel: 'icon', href: `${base}images/logo.svg`, type: 'image/svg+xml' }],
    ['link', { rel: 'canonical', href: siteUrl }],
    ['meta', { name: 'theme-color', content: '#15624f' }],
    ['meta', { name: 'author', content: 'InnoNestX' }],
    [
      'meta',
      {
        name: 'keywords',
        content:
          'Custom Mail, Cloudflare Workers, Brevo, webmail, outbound email, self-hosted mail, InnoNestX',
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
          'Self-hosted outbound mail workspace. Markdown compose, attachments, preview, and send history via Brevo.',
      },
    ],
    ['meta', { property: 'og:url', content: siteUrl }],
    ['meta', { property: 'og:image', content: `${siteUrl}images/logo.png` }],
  ],
  themeConfig: {
    logo: {
      src: '/images/logo.svg',
      alt: 'Custom Mail',
    },
    siteTitle: 'Custom Mail',
    nav: [
      { text: 'Docs', link: '/en/' },
      { text: 'Live demo', link: 'https://mail.xuxuclassmate.com' },
      { text: 'GitHub', link: 'https://github.com/InnoNestX/Custom-Mail' },
    ],
    sidebar: {
      '/en/': [
        {
          text: 'Guide',
          items: [
            { text: 'Overview', link: '/en/' },
            { text: 'Quick start', link: '/en/quick-start' },
            { text: 'Configuration', link: '/en/config' },
            { text: 'Deploy', link: '/en/deploy' },
            { text: 'FAQ', link: '/en/faq' },
          ],
        },
      ],
      '/zh/': [
        {
          text: '指南',
          items: [
            { text: '概览', link: '/zh/' },
            { text: '快速开始', link: '/zh/quick-start' },
            { text: '配置说明', link: '/zh/config' },
            { text: '部署', link: '/zh/deploy' },
            { text: '常见问题', link: '/zh/faq' },
          ],
        },
      ],
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
    appearance: {
      // Keep the calm light paper look as default
      defaultTheme: 'light',
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/InnoNestX/Custom-Mail' },
    ],
  },
  locales: {
    root: {
      label: 'Language',
      lang: 'en-US',
    },
    en: {
      label: 'English',
      lang: 'en-US',
      link: '/en/',
      title: 'Custom Mail',
      description:
        'Private web mail console on Cloudflare Workers. Compose, preview, attachments, and send history via Brevo.',
    },
    zh: {
      label: '中文',
      lang: 'zh-CN',
      link: '/zh/',
      title: 'Custom Mail',
      description:
        '基于 Cloudflare Workers 的私有网页发信控制台。Markdown 撰写、附件、预览与发送记录，经 Brevo 投递。',
      themeConfig: {
        nav: [
          { text: '文档', link: '/zh/' },
          { text: '在线演示', link: 'https://mail.xuxuclassmate.com' },
          { text: 'GitHub', link: 'https://github.com/InnoNestX/Custom-Mail' },
        ],
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
