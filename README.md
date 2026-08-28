<p align="center">
  <img src="public/images/logo.svg" alt="Custom Mail" width="72" height="72" />
</p>

<h1 align="center">Custom Mail</h1>

<p align="center">
  <strong>Private web mail console on Cloudflare Workers</strong><br />
  Compose · Preview · Attachments · Send history — pluggable ESP (Brevo, Resend, SendGrid, …)
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-15624f?style=flat-square" alt="MIT" /></a>
  <a href="https://innonestx.github.io/Custom-Mail/"><img src="https://img.shields.io/badge/docs-GitHub%20Pages-15624f?style=flat-square" alt="Docs" /></a>
  <a href="https://mail.xuxuclassmate.com"><img src="https://img.shields.io/badge/demo-mail.xuxuclassmate.com-2f9e7b?style=flat-square" alt="Live demo" /></a>
  <a href="https://workers.cloudflare.com/"><img src="https://img.shields.io/badge/runtime-Cloudflare%20Workers-f38020?style=flat-square" alt="Workers" /></a>
  <a href="https://github.com/InnoNestX/Custom-Mail/releases"><img src="https://img.shields.io/github/v/release/InnoNestX/Custom-Mail?label=version&style=flat-square" alt="Version" /></a>
  <a href="https://hub.docker.com/r/xuxuclassmate/custom-mail"><img src="https://img.shields.io/docker/pulls/xuxuclassmate/custom-mail?style=flat-square&label=docker%20pulls" alt="Docker pulls" /></a>
  <a href="https://github.com/InnoNestX/Custom-Mail/pkgs/container/custom-mail"><img src="https://img.shields.io/static/v1?label=GHCR&amp;message=innonestx/custom-mail&amp;color=15624f&amp;style=flat-square&amp;logo=github&amp;logoColor=white" alt="GHCR" /></a>
</p>

<p align="center">
  <a href="#features">Features</a> ·
  <a href="#quick-start">Quick start</a> ·
  <a href="https://innonestx.github.io/Custom-Mail/">Documentation</a> ·
  <a href="#deploy">Deploy</a> ·
  <a href="#community">Community</a>
</p>

---

**Custom Mail** is a self-hosted outbound mail workspace written in **Rust** (`workers-rs`). Run it on your own Cloudflare account, brand it with JSON config (name, domain, colors, logo, favicon, footer, copy), pick a theme / layout / mail-provider plugin, and send without maintaining a mail server.

**Docs:** [English](https://innonestx.github.io/Custom-Mail/) · [中文](https://innonestx.github.io/Custom-Mail/zh/)  
**Docs URL:** https://innonestx.github.io/Custom-Mail/

| | |
|---|---|
| **Live demo** | https://mail.xuxuclassmate.com |
| **Documentation** | https://innonestx.github.io/Custom-Mail/ |

## Why use it

- **No VPS** — Worker + KV + static assets on the edge
- **One password** — session login for a private compose UI
- **Brandable** — title, domain, header colors, logo, favicon, footer, and every label in `config/mail.json`; unused sections stay hidden
- **Plugins** — pick a visual theme, HTML layout, and mail provider at deploy time
- **Markdown body** — CommonMark + GitHub Flavored Markdown preview before send; optional attachments
- **Audit trail** — last 10 sends stored in KV with detail view (desktop + mobile)

## Features

| Area | Details |
|------|---------|
| Compose | To / from name, subject, CommonMark + GFM body, address book chips |
| Attachments | Up to 8 files · 8 MB each · 15 MB total |
| Preview | Email-safe HTML preview (headings, lists, tables, code, …) before confirm send |
| History | List + detail; mobile full-screen detail layout |
| Security | HttpOnly session cookie, login rate limit, secrets on Worker |
| CI | Rust tests + wasm check on every push; CodeQL scanning |
| Runtime | Cloudflare Workers via `workers-rs` (Rust → WASM) |

## Markdown

The compose box is **CommonMark + GitHub Flavored Markdown**, rendered in Rust (`pulldown-cmark`) to email-safe HTML:

- Headings, paragraphs, **strong**, *emphasis*, ~~strikethrough~~
- Nested ordered / unordered lists and `- [ ]` task lists
- Tables, block quotes (including `> [!NOTE]` alerts), images, `http`/`https`/`mailto` links
- Fenced and indented code — preview has a Copy control; sent mail links to a snippet page

Raw HTML in the body is ignored. `javascript:` and `data:` URLs are not turned into links.

## Quick start

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
# Rust + wasm32-unknown-unknown + worker-build required
cargo test --lib
npm install
cp .dev.vars.example .dev.vars   # ADMIN_PASSWORD, provider API key; ALLOW_ANY_HOST=1 for localhost
npm run dev
```

Open **http://localhost:8790** and sign in with `ADMIN_PASSWORD`. Health reports `"runtime":"rust"`.

## Docker (lightweight local)

Images:

- Docker Hub: `xuxuclassmate/custom-mail:latest`
- GHCR: `ghcr.io/innonestx/custom-mail:latest`

```bash
docker run --rm -p 8787:8787 \
  -e ADMIN_PASSWORD='your-password' \
  -e BREVO_API_KEY='xkeysib-...' \
  xuxuclassmate/custom-mail:latest
```

Open **http://127.0.0.1:8787**. Or: `docker compose up`.

OpenClaw skill: `clawhub install custom-mail`
## Configuration

All product copy and mail defaults live in **`config/mail.json`**. Full reference on the docs site:

- https://innonestx.github.io/Custom-Mail/config.html

Also in-repo: [docs/CONFIG.md](docs/CONFIG.md) · [docs/DEPLOY.md](docs/DEPLOY.md) · [docs/FAQ.md](docs/FAQ.md)

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

Details: https://innonestx.github.io/Custom-Mail/deploy.html · [docs/DEPLOY.md](docs/DEPLOY.md)

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
