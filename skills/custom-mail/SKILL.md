---
name: custom-mail
description: Run Custom Mail (Rust Cloudflare Worker mail console) via Docker Hub or GHCR, or deploy with wrangler.
version: 1.2.0
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

[Custom Mail](https://github.com/InnoNestX/Custom-Mail) is a private Brevo webmail console. Runtime is a **Rust** Cloudflare Worker (`workers-rs` → WASM): compose, Markdown preview, attachments, and send history.

| | |
|---|---|
| **Repo** | https://github.com/InnoNestX/Custom-Mail |
| **Docs** | https://innonestx.github.io/Custom-Mail/ |
| **Demo** | https://mail.xuxuclassmate.com |
| **Docker Hub** | `xuxuclassmate/custom-mail` |
| **GHCR** | `ghcr.io/innonestx/custom-mail` |

## Images

```text
xuxuclassmate/custom-mail:latest
xuxuclassmate/custom-mail:0.2.0
ghcr.io/innonestx/custom-mail:latest
ghcr.io/innonestx/custom-mail:0.2.0
```

Prefer Docker Hub for anonymous pulls. GHCR is the same image from GitHub Actions on `main`.

## Run (Docker)

```bash
docker run --rm -p 8787:8787 \
  -e ADMIN_PASSWORD='your-password' \
  -e BREVO_API_KEY='xkeysib-...' \
  xuxuclassmate/custom-mail:latest
```

Open http://127.0.0.1:8787 — sign in with `ADMIN_PASSWORD`.

```bash
curl -s http://127.0.0.1:8787/api/health
# {"ok":true,"runtime":"rust","service":"mail",...}
```

### Compose

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
export ADMIN_PASSWORD='your-password'
export BREVO_API_KEY='xkeysib-...'
docker compose up
```

## Environment

| Variable | Required | Description |
|----------|----------|-------------|
| `ADMIN_PASSWORD` | yes | Console login password |
| `BREVO_API_KEY` | no* | Brevo key to send mail |
| `PORT` | no | Listen port (default `8787`) |

\* UI works without Brevo; send fails until the key is set.

## Deploy (Cloudflare Worker)

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
cargo test --lib
npm install
npx wrangler secret put ADMIN_PASSWORD
npx wrangler secret put BREVO_API_KEY
npm run deploy
```

Needs Rust (`wasm32-unknown-unknown`), `worker-build`, and Wrangler. Branding lives in `config/mail.json`.

## More

- Config: https://innonestx.github.io/Custom-Mail/config.html
- Deploy guide: https://innonestx.github.io/Custom-Mail/deploy.html
- License: MIT
