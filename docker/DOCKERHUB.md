# custom-mail

<div align="center">

**Private Brevo mail console — compose, preview, attachments, and send history**

[![Version](https://img.shields.io/docker/v/xuxuclassmate/custom-mail?sort=semver&label=version)](https://hub.docker.com/r/xuxuclassmate/custom-mail/tags)
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
- Delivered through **Brevo** — no mail server to maintain
- Runtime: **Rust** Cloudflare Worker (`workers-rs` → WASM) inside a slim image

## Docker Quick Start

### 1. Pull the image

```bash
docker pull xuxuclassmate/custom-mail:latest
```

Also on GHCR: `ghcr.io/innonestx/custom-mail:latest`

### 2. Set your secrets

You need at least a console password. Add a Brevo API key when you want to actually send mail.

```bash
export ADMIN_PASSWORD='choose-a-strong-password'
export BREVO_API_KEY='xkeysib-...'   # optional for UI-only testing
export PORT=8787                        # optional, default 8787
```

Get a Brevo key from [Brevo → SMTP & API](https://app.brevo.com/settings/keys/api).

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
# {"ok":true,"runtime":"rust","service":"mail",...}
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

## Environment Variables

| Variable | Default | Description |
| --- | --- | --- |
| `ADMIN_PASSWORD` | *(required)* | Login password for the mail console |
| `BREVO_API_KEY` | empty | Brevo API key (`xkeysib-…`). UI works without it; send fails until set |
| `PORT` | `8787` | Port Wrangler listens on inside the container |

The image sets `ALLOW_ANY_HOST=1` so local Docker works without tweaking Host headers.

## Branding & config

Product copy, colors, login text, and address book live in `config/mail.json` inside the image. For production on Cloudflare, edit that file in the repo and redeploy — see the [config guide](https://innonestx.github.io/Custom-Mail/config.html).

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
