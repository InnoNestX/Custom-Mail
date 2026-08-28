<script setup lang="ts">
import { ref } from 'vue'
import { withBase } from 'vitepress'

const props = withDefaults(
  defineProps<{
    lang?: 'en' | 'zh'
  }>(),
  { lang: 'en' },
)

const isZh = props.lang === 'zh'

const copy = isZh
  ? {
      kicker: 'Cloudflare Workers · 五类可插拔插件 · MIT',
      brand: 'Custom Mail',
      headline: '安静的发信控制台',
      lede: '撰写、预览、发送。服务商、主题、版式、Logo、配置都是仓库里的目录 — 跑在边缘，不需要邮件服务器。',
      start: '快速开始',
      startHref: withBase('/zh/quick-start.html'),
      config: '配置',
      configHref: withBase('/zh/config.html'),
      plugins: '插件',
      pluginsHref: withBase('/zh/plugins.html'),
      deploy: '部署',
      deployHref: withBase('/zh/deploy.html'),
      docker: 'Docker',
      dockerHref: withBase('/zh/docker.html'),
      skill: 'OpenClaw',
      skillHref: withBase('/zh/openclaw.html'),
      console: '控制台',
      consoleHref: withBase('/zh/console.html'),
      faqHref: withBase('/zh/faq.html'),
      demo: '在线演示',
      treeTitle: '插件目录',
      treeHint: '点开文件夹，跳到对应文档。',
      folders: [
        {
          name: 'plugins/',
          children: [
            { name: 'providers/', detail: 'brevo · resend · sendgrid', href: withBase('/zh/plugins.html#服务商') },
            { name: 'themes/', detail: 'forest · nord · aurora', href: withBase('/zh/plugins.html#主题') },
            { name: 'layouts/', detail: 'banner · compact · card', href: withBase('/zh/plugins.html#版式') },
            { name: 'features/', detail: 'markdown · history · attachments', href: withBase('/zh/plugins.html#功能') },
            { name: 'logos/', detail: 'SVG / PNG → /plugins/logos/', href: withBase('/zh/plugins.html#logo') },
          ],
        },
        {
          name: 'config/',
          children: [
            { name: 'mail.json', detail: '当前 provider / theme / layout / logo', href: withBase('/zh/config.html') },
            { name: 'overlays/', detail: '可选，编译时深度合并', href: withBase('/zh/plugins.html#配置覆盖') },
          ],
        },
      ],
      stepsTitle: '三步上手',
      stepsHint: '从克隆到上线，配置都在仓库里。',
      steps: [
        { title: '克隆仓库', body: '安装依赖，复制 .dev.vars', href: withBase('/zh/quick-start.html') },
        { title: '选出插件', body: '编辑 mail.json，或往 plugins/ 添加文件', href: withBase('/zh/plugins.html') },
        { title: '部署 Worker', body: '写入密钥，然后 npm run deploy', href: withBase('/zh/deploy.html') },
      ],
      urlsTitle: '地址',
      docsUrl: 'https://innonestx.github.io/Custom-Mail/',
      demoUrl: 'https://mail.xuxuclassmate.com',
    }
  : {
      kicker: 'Cloudflare Workers · five drop-in plugins · MIT',
      brand: 'Custom Mail',
      headline: 'A quiet place to send mail',
      lede: 'Compose, preview, and deliver. Provider, theme, layout, logo, and config are folders in the repo — edge-hosted, no mail server to run.',
      start: 'Quick start',
      startHref: withBase('/quick-start.html'),
      config: 'Configure',
      configHref: withBase('/config.html'),
      plugins: 'Plugins',
      pluginsHref: withBase('/plugins.html'),
      deploy: 'Deploy',
      deployHref: withBase('/deploy.html'),
      docker: 'Docker',
      dockerHref: withBase('/docker.html'),
      skill: 'OpenClaw',
      skillHref: withBase('/openclaw.html'),
      console: 'Console',
      consoleHref: withBase('/console.html'),
      faqHref: withBase('/faq.html'),
      demo: 'Live demo',
      treeTitle: 'Plugin folders',
      treeHint: 'Open a folder, then jump to the matching guide.',
      folders: [
        {
          name: 'plugins/',
          children: [
            { name: 'providers/', detail: 'brevo · resend · sendgrid', href: withBase('/plugins.html#provider') },
            { name: 'themes/', detail: 'forest · nord · aurora', href: withBase('/plugins.html#theme') },
            { name: 'layouts/', detail: 'banner · compact · card', href: withBase('/plugins.html#layout') },
            { name: 'features/', detail: 'markdown · history · attachments', href: withBase('/plugins.html#features') },
            { name: 'logos/', detail: 'SVG / PNG → /plugins/logos/', href: withBase('/plugins.html#logo') },
          ],
        },
        {
          name: 'config/',
          children: [
            { name: 'mail.json', detail: 'active provider / theme / layout / logo', href: withBase('/config.html') },
            { name: 'overlays/', detail: 'optional JSON, deep-merged at build', href: withBase('/plugins.html#config-overlays') },
          ],
        },
      ],
      stepsTitle: 'Three steps',
      stepsHint: 'Clone, pick plugins, deploy. Nothing lives outside the repo.',
      steps: [
        { title: 'Clone the repo', body: 'Install deps, copy .dev.vars', href: withBase('/quick-start.html') },
        { title: 'Pick plugins', body: 'Set ids in mail.json, or drop files under plugins/', href: withBase('/plugins.html') },
        { title: 'Deploy the Worker', body: 'Set secrets, then npm run deploy', href: withBase('/deploy.html') },
      ],
      urlsTitle: 'Addresses',
      docsUrl: 'https://innonestx.github.io/Custom-Mail/',
      demoUrl: 'https://mail.xuxuclassmate.com',
    }

const openFolder = ref(copy.folders[0]?.name ?? '')

function toggleFolder(name: string) {
  openFolder.value = openFolder.value === name ? '' : name
}
</script>

<template>
  <div class="cm-home">
    <div class="cm-home__atmosphere" aria-hidden="true">
      <span class="cm-orb cm-orb--a"></span>
      <span class="cm-orb cm-orb--b"></span>
    </div>

    <section class="cm-home__hero">
      <p class="cm-home__kicker">{{ copy.kicker }}</p>
      <h1 class="cm-home__title">
        <span class="cm-home__brand-word">{{ copy.brand }}</span>
      </h1>
      <p class="cm-home__headline">{{ copy.headline }}</p>
      <p class="cm-home__lede">{{ copy.lede }}</p>
      <div class="cm-home__cta">
        <a class="cm-btn cm-btn--primary" :href="copy.startHref">{{ copy.start }}</a>
        <a class="cm-btn" :href="copy.demoUrl" target="_blank" rel="noreferrer">{{ copy.demo }}</a>
        <a class="cm-btn cm-btn--ghost" :href="copy.pluginsHref">{{ copy.plugins }}</a>
        <a class="cm-btn cm-btn--ghost" :href="copy.configHref">{{ copy.config }}</a>
      </div>
    </section>

    <div class="cm-home__board">
      <section class="cm-panel" :aria-label="copy.treeTitle">
        <header class="cm-panel__head">
          <h2>{{ copy.treeTitle }}</h2>
          <p>{{ copy.treeHint }}</p>
        </header>
        <div class="cm-folders">
          <article
            v-for="folder in copy.folders"
            :key="folder.name"
            class="cm-folder"
            :class="{ 'is-open': openFolder === folder.name }"
          >
            <button
              type="button"
              class="cm-folder__toggle"
              :aria-expanded="openFolder === folder.name"
              :aria-controls="`cm-folder-${folder.name.replace('/', '')}`"
              @click="toggleFolder(folder.name)"
            >
              <span class="cm-folder__name">{{ folder.name }}</span>
              <span class="cm-folder__count">{{ folder.children.length }}</span>
              <span class="cm-folder__caret" aria-hidden="true"></span>
            </button>
            <ul
              v-show="openFolder === folder.name"
              class="cm-folder__files"
              :id="`cm-folder-${folder.name.replace('/', '')}`"
            >
              <li v-for="child in folder.children" :key="child.name" class="cm-file">
                <a class="cm-file__link" :href="child.href">
                  <span class="cm-file__name">{{ child.name }}</span>
                  <span class="cm-file__meta">{{ child.detail }}</span>
                </a>
              </li>
            </ul>
          </article>
        </div>
      </section>

      <section class="cm-panel cm-panel--steps" :aria-label="copy.stepsTitle">
        <header class="cm-panel__head">
          <h2>{{ copy.stepsTitle }}</h2>
          <p>{{ copy.stepsHint }}</p>
        </header>
        <ol class="cm-steps">
          <li v-for="(step, i) in copy.steps" :key="step.title">
            <a class="cm-steps__link" :href="step.href">
              <span class="cm-steps__n" aria-hidden="true">{{ String(i + 1).padStart(2, '0') }}</span>
              <div>
                <strong>{{ step.title }}</strong>
                <span>{{ step.body }}</span>
              </div>
            </a>
          </li>
        </ol>
        <nav class="cm-home__links" :aria-label="copy.stepsTitle">
          <a :href="copy.consoleHref">{{ copy.console }}</a>
          <a :href="copy.dockerHref">{{ copy.docker }}</a>
          <a :href="copy.skillHref">{{ copy.skill }}</a>
          <a :href="copy.faqHref">FAQ</a>
        </nav>
      </section>
    </div>

    <section class="cm-home__urls" :aria-label="copy.urlsTitle">
      <p>
        <span>Docs</span>
        <a :href="copy.docsUrl">{{ copy.docsUrl }}</a>
      </p>
      <p>
        <span>Demo</span>
        <a :href="copy.demoUrl">{{ copy.demoUrl }}</a>
      </p>
    </section>
  </div>
</template>
