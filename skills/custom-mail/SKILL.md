---
name: custom-mail
description: Run the Custom Mail Brevo console locally with Docker — compose, preview, attachments, and send history.
version: 1.3.0
metadata:
  openclaw:
    requires:
      bins:
        - docker
      env: []
    envVars:
      - name: ADMIN_PASSWORD
        required: true
        description: Login password for the Custom Mail console.
      - name: BREVO_API_KEY
        required: false
        description: Brevo API key (required to actually send mail).
      - name: PORT
        required: false
        description: Container listen port (default 8787).
    emoji: "✉️"
    homepage: https://github.com/InnoNestX/Custom-Mail
---

# Custom Mail

## What this skill does

Spin up a **private Brevo mail console** in Docker. Compose mail, preview Markdown as HTML, attach files, and browse send history — without running a mail server.

Runtime is a **Rust** Cloudflare Worker (`workers-rs` → WASM) packaged with Wrangler for local use.

## When to use this skill

Use it when the user wants to:

- run Custom Mail locally or in Docker
- send mail through Brevo from a self-hosted console
- try the compose / preview / history UI before Cloudflare deploy
- set up a lightweight mail workspace with one password login

Trigger phrases (examples):

- "start custom mail in docker"
- "run the mail console locally"
- "deploy custom-mail container"
- "帮我本地跑一下 Custom Mail"
- "用 Docker 启动发信控制台"

## Docker Quick Start

### 1. Pull

```bash
docker pull xuxuclassmate/custom-mail:latest
```

GHCR mirror: `ghcr.io/innonestx/custom-mail:latest`

### 2. Export secrets

```bash
export ADMIN_PASSWORD='choose-a-strong-password'
export BREVO_API_KEY='xkeysib-...'   # optional until send is needed
export PORT=8787
```

### 3. Run

```bash
docker run -d \
  --name custom-mail \
  -p 8787:8787 \
  -e ADMIN_PASSWORD="$ADMIN_PASSWORD" \
  -e BREVO_API_KEY="$BREVO_API_KEY" \
  xuxuclassmate/custom-mail:latest
```

Open http://localhost:8787 — sign in with `ADMIN_PASSWORD`.

Verify:

```bash
curl -s http://localhost:8787/api/health
```

## Docker Compose

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
export ADMIN_PASSWORD='choose-a-strong-password'
export BREVO_API_KEY='xkeysib-...'
docker compose up -d
```

## Environment

| Variable | Default | Description |
| --- | --- | --- |
| `ADMIN_PASSWORD` | *(required)* | Console login password |
| `BREVO_API_KEY` | empty | Brevo key; UI loads without it, send needs it |
| `PORT` | `8787` | Listen port |

## Example invocations

```
Pull and run Custom Mail on port 8787 with ADMIN_PASSWORD=dev-secret and my Brevo key.
```

```
Start the custom-mail Docker container in the background and tell me the health check URL.
```

```
Clone InnoNestX/Custom-Mail and bring it up with docker compose.
```

## Production (Cloudflare)

For edge deploy instead of Docker:

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
cargo test --lib && npm install
npx wrangler secret put ADMIN_PASSWORD
npx wrangler secret put BREVO_API_KEY
npm run deploy
```

Branding: `config/mail.json` · Docs: https://innonestx.github.io/Custom-Mail/

## Links

- GitHub: https://github.com/InnoNestX/Custom-Mail
- Docker Hub: https://hub.docker.com/r/xuxuclassmate/custom-mail
- Docs: https://innonestx.github.io/Custom-Mail/
- Demo: https://mail.xuxuclassmate.com
