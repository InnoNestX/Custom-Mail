<script setup lang="ts">
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
      treeHint: '把文件丢进对应文件夹，再在 mail.json 里选中它的 id。',
      folders: [
        {
          name: 'plugins/',
          children: [
            { name: 'providers/', detail: 'brevo · resend · sendgrid' },
            { name: 'themes/', detail: 'forest · nord · aurora' },
            { name: 'layouts/', detail: 'banner · compact · card' },
            { name: 'features/', detail: 'markdown · history · attachments' },
            { name: 'logos/', detail: 'SVG / PNG，构建后走 /plugins/logos/' },
          ],
        },
        {
          name: 'config/',
          children: [
            { name: 'mail.json', detail: '当前 provider / theme / layout / logo' },
            { name: 'overlays/', detail: '可选，编译时深度合并' },
          ],
        },
      ],
      stepsTitle: '三步上手',
      stepsHint: '从克隆到上线，配置都在仓库里。',
      steps: [
        ['克隆仓库', '安装依赖，复制 .dev.vars'],
        ['选出插件', '编辑 mail.json，或往 plugins/ 添加文件'],
        ['部署 Worker', '写入密钥，然后 npm run deploy'],
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
      treeHint: 'Drop a file in the matching folder, then select its id in mail.json.',
      folders: [
        {
          name: 'plugins/',
          children: [
            { name: 'providers/', detail: 'brevo · resend · sendgrid' },
            { name: 'themes/', detail: 'forest · nord · aurora' },
            { name: 'layouts/', detail: 'banner · compact · card' },
            { name: 'features/', detail: 'markdown · history · attachments' },
            { name: 'logos/', detail: 'SVG / PNG → /plugins/logos/' },
          ],
        },
        {
          name: 'config/',
          children: [
            { name: 'mail.json', detail: 'active provider / theme / layout / logo' },
            { name: 'overlays/', detail: 'optional JSON, deep-merged at build' },
          ],
        },
      ],
      stepsTitle: 'Three steps',
      stepsHint: 'Clone, pick plugins, deploy. Nothing lives outside the repo.',
      steps: [
        ['Clone the repo', 'Install deps, copy .dev.vars'],
        ['Pick plugins', 'Set ids in mail.json, or drop files under plugins/'],
        ['Deploy the Worker', 'Set secrets, then npm run deploy'],
      ],
      urlsTitle: 'Addresses',
      docsUrl: 'https://innonestx.github.io/Custom-Mail/',
      demoUrl: 'https://mail.xuxuclassmate.com',
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
          <article v-for="folder in copy.folders" :key="folder.name" class="cm-folder">
            <h3 class="cm-folder__name">{{ folder.name }}</h3>
            <ul class="cm-folder__files">
              <li v-for="child in folder.children" :key="child.name" class="cm-file">
                <span class="cm-file__name">{{ child.name }}</span>
                <span class="cm-file__meta">{{ child.detail }}</span>
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
          <li v-for="(step, i) in copy.steps" :key="step[0]">
            <span class="cm-steps__n" aria-hidden="true">{{ String(i + 1).padStart(2, '0') }}</span>
            <div>
              <strong>{{ step[0] }}</strong>
              <span>{{ step[1] }}</span>
            </div>
          </li>
        </ol>
        <nav class="cm-home__links" :aria-label="copy.stepsTitle">
          <a :href="copy.consoleHref">{{ copy.console }}</a>
          <a :href="copy.dockerHref">{{ copy.docker }}</a>
          <a :href="copy.skillHref">{{ copy.skill }}</a>
          <a :href="copy.pluginsHref">{{ copy.plugins }}</a>
          <a :href="copy.deployHref">{{ copy.deploy }}</a>
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
