<script setup lang="ts">
import { withBase } from 'vitepress'

defineProps<{
  lang?: 'en' | 'zh'
}>()

const logo = withBase('/images/logo.svg')
const en = withBase('/en/')
const zh = withBase('/zh/')
</script>

<template>
  <div class="cm-gate">
    <div class="cm-gate__glow" aria-hidden="true" />
    <div class="cm-gate__panel">
      <img class="cm-gate__logo" :src="logo" width="72" height="72" alt="Custom Mail" />
      <p class="cm-gate__brand">Custom Mail</p>
      <p class="cm-gate__lede">Private web mail on the edge.</p>
      <div class="cm-gate__actions">
        <a class="cm-btn cm-btn--primary" :href="en">English</a>
        <a class="cm-btn" :href="zh">中文</a>
      </div>
      <p class="cm-gate__meta">
        <a href="https://mail.xuxuclassmate.com">Live demo</a>
        <span aria-hidden="true">·</span>
        <a href="https://github.com/InnoNestX/Custom-Mail">GitHub</a>
      </p>
    </div>
  </div>
</template>
