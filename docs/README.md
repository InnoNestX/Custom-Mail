# Custom Mail — Documentation

**Published site (use this):** https://innonestx.github.io/Custom-Mail/

| Language | URL |
|----------|-----|
| English | https://innonestx.github.io/Custom-Mail/ |
| 中文 | https://innonestx.github.io/Custom-Mail/zh/ |

Source Markdown also lives under `docs/` (this folder) and `docs-site/docs/` (VitePress).

## Guides (in-repo)

| Guide | Description |
|-------|-------------|
| [Plugins](../docs-site/docs/plugins.md) | Drop-in catalogs — provider, theme, layout, logo, config |
| [Configuration](CONFIG.md) | `config/mail.json` — host, branding, plugins, mail defaults, address book |
| [Deployment](DEPLOY.md) | Cloudflare Worker, secrets, domain, GitHub Actions |
| [FAQ](FAQ.md) | Common errors and fixes |

## Quick links

- Docs site: https://innonestx.github.io/Custom-Mail/
- Live demo: https://mail.xuxuclassmate.com
- [Repository README](../README.md)
- [InnoNestX organization](https://github.com/InnoNestX)
- [Brevo API](https://developers.brevo.com/) (default provider; others listed in CONFIG.md)

## Typical setup flow

```text
1. Fork / clone this repo
2. Edit config/mail.json and drop-in files under plugins/
3. Create KV namespace → update wrangler.jsonc
4. wrangler secret put ADMIN_PASSWORD + provider API key
5. npm run deploy
6. Open https://<your-host> and sign in
```

For development, only `.dev.vars` and `npm run dev` are required — no Cloudflare deploy until you are ready.
