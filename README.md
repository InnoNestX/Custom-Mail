<p align="center">
  <img src="public/images/logo.svg" alt="Custom Mail" width="72" height="72" />
</p>

<h1 align="center">Custom Mail</h1>

<p align="center">
  <strong>Private web mail console on Cloudflare Workers</strong><br />
  Compose · Preview · Attachments · Send history — delivered via Brevo
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-15624f?style=flat-square" alt="MIT" /></a>
  <a href="https://innonestx.github.io/Custom-Mail/"><img src="https://img.shields.io/badge/docs-GitHub%20Pages-15624f?style=flat-square" alt="Docs" /></a>
  <a href="https://mail.xuxuclassmate.com"><img src="https://img.shields.io/badge/demo-mail.xuxuclassmate.com-2f9e7b?style=flat-square" alt="Live demo" /></a>
  <a href="https://workers.cloudflare.com/"><img src="https://img.shields.io/badge/runtime-Cloudflare%20Workers-f38020?style=flat-square" alt="Workers" /></a>
  <a href="https://github.com/InnoNestX/Custom-Mail/releases"><img src="https://img.shields.io/github/v/release/InnoNestX/Custom-Mail?style=flat-square" alt="Release" /></a>
</p>

<p align="center">
  <a href="#features">Features</a> ·
  <a href="#quick-start">Quick start</a> ·
  <a href="https://innonestx.github.io/Custom-Mail/">Documentation</a> ·
  <a href="#deploy">Deploy</a> ·
  <a href="#community">Community</a>
</p>

---

**Custom Mail** is a self-hosted outbound mail workspace. Run it on your own Cloudflare account, brand it with JSON config, and send through **Brevo** without maintaining a mail server.

| | |
|---|---|
| **Live example** | [mail.xuxuclassmate.com](https://mail.xuxuclassmate.com) |
| **Org** | Part of [InnoNestX](https://github.com/InnoNestX) |
| **Docs** | [Documentation site](https://innonestx.github.io/Custom-Mail/) · [repo docs/](docs/README.md) |

## Why use it

- **No VPS** — Worker + KV + static assets on the edge
- **One password** — session login for a private compose UI
- **Brandable** — title, colors, login copy, address book in `config/mail.json`
- **Markdown body** — preview before send; optional attachments
- **Audit trail** — last 10 sends stored in KV with detail view (desktop + mobile)

## Features

| Area | Details |
|------|---------|
| Compose | To / from name, subject, Markdown body, address book chips |
| Attachments | Up to 8 files · 8 MB each · 15 MB total |
| Preview | HTML preview modal before confirm send |
| History | List + detail; mobile full-screen detail layout |
| Security | HttpOnly session cookie, login rate limit, secrets on Worker |
| CI | Typecheck on every push; CodeQL security scanning |

## Quick start

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
npm install
cp .dev.vars.example .dev.vars   # ADMIN_PASSWORD, BREVO_API_KEY
npm run dev
```

Open **http://localhost:8790** and sign in with `ADMIN_PASSWORD`.

## Configuration

All product copy and mail defaults live in **`config/mail.json`**. See the full reference:

- [Configuration guide](docs/CONFIG.md) — `host`, `app`, `mail`, `brand`, `addressBook`
- [Deployment guide](docs/DEPLOY.md) — Cloudflare, secrets, CI, custom domain

`host` in `mail.json` must match the route in `wrangler.jsonc`.

## Deploy

### Local / manual

```bash
npx wrangler secret put ADMIN_PASSWORD
npx wrangler secret put BREVO_API_KEY
npm run deploy
```

### GitHub Actions

| Workflow | When |
|----------|------|
| **CI** | Every push / PR — `npm run typecheck` |
| **Deploy to Cloudflare Workers** | Manual **Run workflow** only |

Org/repo secrets: `CLOUDFLARE_API_TOKEN`, `CLOUDFLARE_ACCOUNT_ID`. Worker secrets stay on Cloudflare (`ADMIN_PASSWORD`, `BREVO_API_KEY`).

Details: [docs/DEPLOY.md](docs/DEPLOY.md)

## Architecture

```text
Browser ──► Cloudflare Worker (custom-mail)
              ├── Workers Assets (UI)
              ├── KV MAIL_LOG_KV (sessions + send log)
              └── Brevo API (transactional send)
```

## Community

- [Contributing](.github/CONTRIBUTING.md) · [Security](SECURITY.md) · [Code of Conduct](CODE_OF_CONDUCT.md)
- [Report a vulnerability](https://github.com/InnoNestX/Custom-Mail/security/advisories/new)
- [Discussions](https://github.com/InnoNestX/Custom-Mail/discussions)
- [InnoNestX projects](https://github.com/InnoNestX)

## License

MIT · © 2026 InnoNestX
