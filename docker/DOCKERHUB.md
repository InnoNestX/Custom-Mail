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
- Themes, layouts, logo, favicon, and copy from `config/mail.json`
- Runtime: **Rust** Cloudflare Worker (`workers-rs` → WASM) inside a slim image

## Docker Quick Start

### 1. Pull the image

```bash
docker pull xuxuclassmate/custom-mail:latest
```

Also on GHCR: `ghcr.io/innonestx/custom-mail:latest`

### 2. Set your secrets

You need a console password. Add the API key for the provider selected in `config/mail.json` (`plugins.provider`, default `brevo`) when you want to send.

```bash
export ADMIN_PASSWORD='choose-a-strong-password'
export BREVO_API_KEY='xkeysib-...'   # or RESEND_API_KEY / SENDGRID_API_KEY / MAIL_API_KEY / …
export PORT=8787                      # optional, default 8787
```

To use another provider **without rebuilding** the image:

```bash
export MAIL_PROVIDER=resend
export RESEND_API_KEY='re_...'
```

`MAIL_PROVIDER` overrides `plugins.provider` baked into the image.

### 3. Run the container

Foreground (good for a quick try):

```bash
docker run --rm -p 8787:8787 \
  -e ADMIN_PASSWORD="$ADMIN_PASSWORD" \
  -e BREVO_API_KEY="$BREVO_API_KEY" \
  xuxuclassmate/custom-mail:latest
```

Background:

```bash
docker run -d \
  --name custom-mail \
  -p 8787:8787 \
  -e ADMIN_PASSWORD="$ADMIN_PASSWORD" \
  -e BREVO_API_KEY="$BREVO_API_KEY" \
  xuxuclassmate/custom-mail:latest
```

Open [http://localhost:8787](http://localhost:8787) and sign in with `ADMIN_PASSWORD`.

Quick health check:

```bash
curl -s http://localhost:8787/api/health
# {"ok":true,"runtime":"rust","provider":"brevo","theme":"forest",...}
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

To change **branding, theme, layout, or the default provider**, edit `config/mail.json` and rebuild (`docker compose build`). Logo/favicon files go under `public/`.

## Environment Variables

| Variable | Default | Description |
| --- | --- | --- |
| `ADMIN_PASSWORD` | *(required)* | Login password for the mail console |
| `MAIL_PROVIDER` | empty | Overrides `plugins.provider` (`brevo`, `resend`, `sendgrid`, `mailgun`, `postmark`, `mailersend`, `smtp2go`, `sparkpost`) |
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

Product copy, colors, logo, favicon, footer, plugins, and address book live in `config/mail.json` (compiled into the Worker at image build). For a custom brand, edit that file (and `public/images/`) then rebuild. On Cloudflare, edit the repo and redeploy — see the [config guide](https://innonestx.github.io/Custom-Mail/config.html).

## Image Tags

- `xuxuclassmate/custom-mail:latest`
- `xuxuclassmate/custom-mail:0.2.0`
- `ghcr.io/innonestx/custom-mail:latest`
- `ghcr.io/innonestx/custom-mail:0.2.0`

## OpenClaw

```bash
clawhub install custom-mail
```

## Links

- GitHub: https://github.com/InnoNestX/Custom-Mail
- Docker Hub: https://hub.docker.com/r/xuxuclassmate/custom-mail
- GHCR: https://github.com/InnoNestX/Custom-Mail/pkgs/container/custom-mail
- Docs: https://innonestx.github.io/Custom-Mail/
- Demo: https://mail.xuxuclassmate.com
