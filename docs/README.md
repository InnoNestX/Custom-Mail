Welcome to the Custom Mail docs. Prefer the published site: **[innonestx.github.io/Custom-Mail](https://innonestx.github.io/Custom-Mail/)**.

Source Markdown also lives under `docs/` and `docs-site/docs/`.

## Guides

| Guide | Description |
|-------|-------------|
| [Configuration](CONFIG.md) | `config/mail.json` — host, branding, mail defaults, address book |
| [Deployment](DEPLOY.md) | Cloudflare Worker, secrets, domain, GitHub Actions |
| [FAQ](FAQ.md) | Common errors and fixes |

## Quick links

- [Published docs](https://innonestx.github.io/Custom-Mail/)
- [Repository README](../README.md)
- [Live demo](https://mail.xuxuclassmate.com)
- [InnoNestX organization](https://github.com/InnoNestX)
- [Brevo API](https://developers.brevo.com/)

## Typical setup flow

```text
1. Fork / clone this repo
2. Edit config/mail.json (host, brand, from address)
3. Create KV namespace → update wrangler.jsonc
4. wrangler secret put ADMIN_PASSWORD + BREVO_API_KEY
5. npm run deploy
6. Open https://<your-host> and sign in
```

For development, only `.dev.vars` and `npm run dev` are required — no Cloudflare deploy until you are ready.
