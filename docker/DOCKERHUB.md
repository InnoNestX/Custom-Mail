# custom-mail

<div align="center">

**Private web mail console — compose, preview, attachments, and send history**

[![Version](https://img.shields.io/github/v/release/InnoNestX/Custom-Mail?label=version)](https://github.com/InnoNestX/Custom-Mail/releases)
[![Docker Pulls](https://img.shields.io/docker/pulls/xuxuclassmate/custom-mail)](https://hub.docker.com/r/xuxuclassmate/custom-mail)

[GitHub](https://github.com/InnoNestX/Custom-Mail) • [Docker Hub](https://hub.docker.com/r/xuxuclassmate/custom-mail) • [Docs](https://innonestx.github.io/Custom-Mail/) • [Live demo](https://mail.xuxuclassmate.com)

</div>

---

## What It Does

Custom Mail is a self-hosted outbound mail workspace. Run it locally in Docker to try the UI, or deploy the same Rust Worker to Cloudflare for production.

- Compose to multiple recipients with an address book
- Markdown body with HTML preview before send
- Attachments up to 8 files · 8 MB each · 15 MB total
- Send history with detail view (desktop + mobile)
- Session login, login rate limit, HttpOnly cookies
- Pluggable ESP (Brevo, Resend, SendGrid, Mailgun, Postmark, MailerSend, SMTP2GO, SparkPost)
- Drop-in themes, layouts, logo, features, and config overlays (`plugins/` + `config/mail.json`)
- Runtime: **Rust** Cloudflare Worker (`workers-rs` → WASM) inside a slim image

## Docker Quick Start

### 1. Pull the image

```bash
docker pull xuxuclassmate/custom-mail:1.0.0
```

Or `:latest`. GHCR: `ghcr.io/innonestx/custom-mail:1.0.0`

### 2. Set your secrets

You need a console password. Add the API key for the provider selected in `config/mail.json` (`plugins.provider`, default `brevo`) when you want to send.

```bash
export ADMIN_PASSWORD='choose-a-strong-password'
export BREVO_API_KEY='xkeysib-...'   # or RESEND_API_KEY / SENDGRID_API_KEY / MAIL_API_KEY / …
export PORT=8787                      # optional, default 8787
```

To use another provider, theme, layout, or logo **without rebuilding** the image:

```bash
export MAIL_PROVIDER=resend
export MAIL_THEME=nord
export MAIL_LAYOUT=compact
export MAIL_LOGO=monogram
export RESEND_API_KEY='re_...'
```

These override the matching `plugins.*` slots baked into the image. `MAIL_CONFIG_JSON='{"site":{"brandName":"Desk"}}'` deep-merges extra JSON. Adding a new theme JSON file still needs `docker compose build`.

### 3. Run the container

Foreground (good for a quick try):

```bash
docker run --rm -p 8787:8787 \
  -e ADMIN_PASSWORD="$ADMIN_PASSWORD" \
  -e BREVO_API_KEY="$BREVO_API_KEY" \
  xuxuclassmate/custom-mail:1.0.0
```

Background:

```bash
docker run -d \
  --name custom-mail \
  -p 8787:8787 \
  -e ADMIN_PASSWORD="$ADMIN_PASSWORD" \
  -e BREVO_API_KEY="$BREVO_API_KEY" \
  xuxuclassmate/custom-mail:1.0.0
```

Open [http://localhost:8787](http://localhost:8787) and sign in with `ADMIN_PASSWORD`.

Quick health check:

```bash
curl -s http://localhost:8787/api/health
# {"ok":true,"runtime":"rust","plugins":{"provider":"brevo","theme":"forest","layout":"banner","logo":"image"},...}
```

## Docker Compose

Clone the repo if you want the bundled `docker-compose.yml`:

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
export ADMIN_PASSWORD='choose-a-strong-password'
export BREVO_API_KEY='xkeysib-...'
docker compose up -d
```

Stop the service:

```bash
docker compose down
```

To change **branding or add plugin JSON/logo files**, edit `config/mail.json` / `plugins/` and rebuild (`docker compose build`). Slot env vars above switch the active plugin without a rebuild.

## Environment Variables

| Variable | Default | Description |
| --- | --- | --- |
| `ADMIN_PASSWORD` | *(required)* | Login password for the mail console |
| `MAIL_PROVIDER` | empty | Overrides `plugins.provider` (`brevo`, `resend`, `sendgrid`, `mailgun`, `postmark`, `mailersend`, `smtp2go`, `sparkpost`) |
| `MAIL_THEME` | empty | Overrides `plugins.theme` (`forest`, `midnight`, `ocean`, `paper`, `rose`, `slate`, `aurora`, `sunset`, `nord`, …) |
| `MAIL_LAYOUT` | empty | Overrides `plugins.layout` (`card`, `minimal`, `banner`, `digest`, `compact`) |
| `MAIL_LOGO` | empty | Overrides `plugins.logo` (`auto`, `image`, `monogram`, `none`) |
| `MAIL_CONFIG_JSON` | empty | JSON object deep-merged onto `mail.json` at runtime |
| `BREVO_API_KEY` | empty | Brevo key. UI works without a key; send needs the key for the active provider |
| `RESEND_API_KEY` | empty | Resend |
| `SENDGRID_API_KEY` | empty | SendGrid |
| `MAILGUN_API_KEY` / `MAILGUN_DOMAIN` | empty | Mailgun |
| `POSTMARK_SERVER_TOKEN` | empty | Postmark |
| `MAILERSEND_API_KEY` | empty | MailerSend |
| `SMTP2GO_API_KEY` | empty | SMTP2GO |
| `SPARKPOST_API_KEY` | empty | SparkPost |
| `MAIL_API_KEY` | empty | Fallback if the provider-specific key is unset |
| `PORT` | `8787` | Port Wrangler listens on inside the container |

The image sets `ALLOW_ANY_HOST=1` so local Docker works without tweaking Host headers.

## Branding & config

Product copy, colors, footer, and the default plugin ids live in `config/mail.json`. Drop-in catalogs live in `plugins/` (themes, layouts, providers, logos). For a custom brand, edit those files then rebuild. On Cloudflare, edit the repo and redeploy — see the [config guide](https://innonestx.github.io/Custom-Mail/config.html).

## Image Tags

- `xuxuclassmate/custom-mail:1.0.0`
- `xuxuclassmate/custom-mail:latest` (same release)
- `ghcr.io/innonestx/custom-mail:1.0.0`
- `ghcr.io/innonestx/custom-mail:latest`

## Architectures

Multi-arch manifest (pick the right one automatically with `docker pull`):

| Platform | Typical hosts |
| --- | --- |
| `linux/amd64` | Intel/AMD PCs, most cloud VMs, Windows Docker Desktop (x86_64) |
| `linux/arm64` | Apple Silicon Macs, Windows/ARM Docker Desktop, Pi 4/5 (64-bit), AWS Graviton |
| `linux/arm/v7` | Older 32-bit ARM boards (e.g. Raspberry Pi 2/3) |

macOS and Windows run these **Linux** images via Docker Desktop — there is no separate Windows/macOS container image.

## OpenClaw

```bash
clawhub install custom-mail-skill
```

Skill page: https://clawhub.ai/xuxuclassmate/skills/custom-mail-skill (version `1.0.0`).

Usage on the docs site: [OpenClaw skill](https://innonestx.github.io/Custom-Mail/openclaw.html).

## Links

- GitHub: https://github.com/InnoNestX/Custom-Mail
- Docker Hub: https://hub.docker.com/r/xuxuclassmate/custom-mail
- GHCR: https://github.com/InnoNestX/Custom-Mail/pkgs/container/custom-mail
- Docs: https://innonestx.github.io/Custom-Mail/
- Demo: https://mail.xuxuclassmate.com
