---
# Redirect root to English docs
---

<script setup>
if (typeof window !== 'undefined') {
  const preferZh = /^zh\b/i.test(navigator.language || '')
  window.location.replace(preferZh ? './zh/' : './en/')
}
</script>

# Custom Mail

[English docs](./en/) · [中文文档](./zh/)
