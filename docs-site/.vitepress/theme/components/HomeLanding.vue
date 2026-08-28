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
      faqHref: withBase('/zh/faq.html'),
      demo: '在线演示',
      treeTitle: '仓库目录',
      treeHint: '往对应文件夹丢文件，再在 mail.json 里选中。',
      tree: [
        { kind: 'dir', name: 'plugins/' },
        { kind: 'item', name: 'providers/', detail: 'brevo.json · resend.json · sendgrid.json …' },
        { kind: 'item', name: 'themes/', detail: 'forest.json · nord.json · aurora.json …' },
        { kind: 'item', name: 'layouts/', detail: 'banner.json · compact.json · card.json …' },
        { kind: 'item', name: 'features/', detail: 'markdown.json · history.json · attachments.json' },
        { kind: 'item', name: 'logos/', detail: '放入 SVG / PNG，构建后走 /plugins/logos/' },
        { kind: 'dir', name: 'config/' },
        { kind: 'item', name: 'mail.json', detail: '当前启用的 provider / theme / layout / logo' },
        { kind: 'item', name: 'overlays/', detail: '可选 JSON，编译时深度合并' },
      ],
      stepsTitle: '三步上手',
      steps: [
        ['克隆仓库', '安装依赖，复制 .dev.vars'],
        ['编辑 mail.json 与 plugins/', '域名、品牌、主题、版式、Logo、服务商'],
        ['部署 Worker', '写入密钥，npm run deploy'],
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
      faqHref: withBase('/faq.html'),
      demo: 'Live demo',
      treeTitle: 'Repository layout',
      treeHint: 'Drop a file in the matching folder, then select its id in mail.json.',
      tree: [
        { kind: 'dir', name: 'plugins/' },
        { kind: 'item', name: 'providers/', detail: 'brevo.json · resend.json · sendgrid.json …' },
        { kind: 'item', name: 'themes/', detail: 'forest.json · nord.json · aurora.json …' },
        { kind: 'item', name: 'layouts/', detail: 'banner.json · compact.json · card.json …' },
        { kind: 'item', name: 'features/', detail: 'markdown.json · history.json · attachments.json' },
        { kind: 'item', name: 'logos/', detail: 'drop an SVG/PNG → /plugins/logos/' },
        { kind: 'dir', name: 'config/' },
        { kind: 'item', name: 'mail.json', detail: 'active provider / theme / layout / logo' },
        { kind: 'item', name: 'overlays/', detail: 'optional JSON, deep-merged at build' },
      ],
      stepsTitle: 'Three steps',
      steps: [
        ['Clone the repo', 'Install deps, copy .dev.vars'],
        ['Edit mail.json + plugins/', 'Host, brand, theme, layout, logo, provider'],
        ['Deploy the Worker', 'Set secrets, npm run deploy'],
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
      <span class="cm-envelope"></span>
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

    <section class="cm-home__tree" :aria-label="copy.treeTitle">
      <h2>{{ copy.treeTitle }}</h2>
      <p class="cm-home__tree-hint">{{ copy.treeHint }}</p>
      <ol class="cm-tree">
        <li
          v-for="row in copy.tree"
          :key="row.name + row.kind"
          :class="['cm-tree__row', 'cm-tree__row--' + row.kind]"
        >
          <code class="cm-tree__name">{{ row.name }}</code>
          <span v-if="row.detail" class="cm-tree__detail">{{ row.detail }}</span>
        </li>
      </ol>
      <div class="cm-home__links">
        <a :href="copy.pluginsHref">{{ copy.plugins }} →</a>
        <a :href="copy.deployHref">{{ copy.deploy }} →</a>
        <a :href="copy.faqHref">FAQ →</a>
      </div>
    </section>

    <section class="cm-home__steps" :aria-label="copy.stepsTitle">
      <h2>{{ copy.stepsTitle }}</h2>
      <ol>
        <li v-for="(step, i) in copy.steps" :key="step[0]">
          <span class="cm-home__n">{{ String(i + 1).padStart(2, '0') }}</span>
          <div>
            <strong>{{ step[0] }}</strong>
            <span>{{ step[1] }}</span>
          </div>
        </li>
      </ol>
    </section>

    <section class="cm-home__urls" :aria-label="copy.urlsTitle">
      <h2>{{ copy.urlsTitle }}</h2>
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
