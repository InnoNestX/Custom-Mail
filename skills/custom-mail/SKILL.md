---
name: custom-mail
description: Run Custom Mail (Rust Cloudflare Worker mail console) via Docker Hub or GHCR, or deploy with wrangler.
version: 1.1.0
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
    emoji: "✉️"
    homepage: https://github.com/InnoNestX/Custom-Mail
---

# Custom Mail (Rust / Docker)

[Custom Mail](https://github.com/InnoNestX/Custom-Mail) is a private Brevo webmail console. Runtime is a **Rust** Cloudflare Worker (`workers-rs`).

## Images

- Docker Hub: `xuxuclassmate/custom-mail:latest`
- GHCR: `ghcr.io/innonestx/custom-mail:latest`

## Run (local demo)

```bash
docker run --rm -p 8787:8787 \
  -e ADMIN_PASSWORD='your-password' \
  -e BREVO_API_KEY='xkeysib-...' \
  xuxuclassmate/custom-mail:latest
```

Open http://127.0.0.1:8787 — health check includes `"runtime":"rust"`.

## Deploy (Cloudflare)

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
cargo test --lib
npm install
npm run deploy
```

Requires Rust (`wasm32-unknown-unknown`), `worker-build`, and Wrangler secrets `ADMIN_PASSWORD` / `BREVO_API_KEY`.
