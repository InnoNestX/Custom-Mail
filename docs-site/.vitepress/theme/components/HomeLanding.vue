<script setup lang="ts">
import { withBase } from 'vitepress'

const props = withDefaults(
  defineProps<{
    lang?: 'en' | 'zh'
  }>(),
  { lang: 'en' },
)

const isZh = props.lang === 'zh'
const logo = withBase('/images/logo.svg')
const base = '/Custom-Mail'

const copy = isZh
  ? {
      kicker: 'Cloudflare Workers · Brevo · MIT',
      brand: 'Custom Mail',
      headline: '安静的发信控制台',
      lede: '撰写、预览、发送。跑在边缘，不需要邮件服务器。',
      start: '快速开始',
      startHref: `${base}/zh/quick-start.html`,
      config: '配置',
      configHref: `${base}/zh/config.html`,
      deploy: '部署',
      deployHref: `${base}/zh/deploy.html`,
      demo: '在线演示',
      alt: 'English',
      altHref: `${base}/en/`,
      stepsTitle: '三步上手',
      steps: [
        ['克隆仓库', '安装依赖，复制 .dev.vars'],
        ['配置 mail.json', '域名、品牌、发件人、通讯录'],
        ['部署 Worker', '写入密钥，npm run deploy'],
      ],
      urlsTitle: '地址',
      docsUrl: 'https://innonestx.github.io/Custom-Mail/',
      demoUrl: 'https://mail.xuxuclassmate.com',
    }
  : {
      kicker: 'Cloudflare Workers · Brevo · MIT',
      brand: 'Custom Mail',
      headline: 'A quiet place to send mail',
      lede: 'Compose, preview, and deliver. Edge-hosted — no mail server to run.',
      start: 'Quick start',
      startHref: `${base}/en/quick-start.html`,
      config: 'Configure',
      configHref: `${base}/en/config.html`,
      deploy: 'Deploy',
      deployHref: `${base}/en/deploy.html`,
      demo: 'Live demo',
      alt: '中文',
      altHref: `${base}/zh/`,
      stepsTitle: 'Three steps',
      steps: [
        ['Clone the repo', 'Install deps, copy .dev.vars'],
        ['Edit mail.json', 'Host, brand, sender, address book'],
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
      <span class="cm-orb cm-orb--a" />
      <span class="cm-orb cm-orb--b" />
      <span class="cm-envelope" />
    </div>

    <header class="cm-home__nav">
      <a class="cm-home__brand" :href="withBase(isZh ? '/zh/' : '/en/')">
        <img :src="logo" width="36" height="36" alt="" />
        <span>Custom Mail</span>
      </a>
      <div class="cm-home__nav-links">
        <a :href="copy.altHref">{{ copy.alt }}</a>
        <a href="https://github.com/InnoNestX/Custom-Mail">GitHub</a>
      </div>
    </header>

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
        <a class="cm-btn cm-btn--ghost" :href="copy.configHref">{{ copy.config }}</a>
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
      <div class="cm-home__links">
        <a :href="copy.deployHref">{{ copy.deploy }} →</a>
        <a :href="withBase(isZh ? '/zh/faq.html' : '/en/faq.html')">FAQ →</a>
      </div>
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
