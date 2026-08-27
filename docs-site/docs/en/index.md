# Custom Mail

Private **web mail console** on Cloudflare Workers. Compose Markdown, attach files, preview HTML, and keep a short send history — outbound delivery through **Brevo**.

| | |
|---|---|
| **This docs site** | https://innonestx.github.io/Custom-Mail/ |
| **Live demo** | https://mail.xuxuclassmate.com |
| **Repository** | https://github.com/InnoNestX/Custom-Mail |
| **License** | MIT |

## Why Custom Mail

- **No VPS** — Worker + KV + static assets on the edge
- **One password** — session login for a private compose UI
- **Brandable** — title, colors, copy, and address book in `config/mail.json`
- **Markdown body** — preview before send; optional attachments
- **Audit trail** — recent sends stored in KV (desktop + mobile detail view)

## Architecture

```text
Browser ──► Cloudflare Worker (custom-mail)
              ├── Workers Assets (UI)
              ├── KV MAIL_LOG_KV (sessions + send log)
              └── Brevo API (transactional send)
```

## Next steps

1. [Quick start](./quick-start) — run locally
2. [Configuration](./config) — `config/mail.json`
3. [Deploy](./deploy) — Cloudflare Worker + domain
4. [FAQ](./faq) — common issues
