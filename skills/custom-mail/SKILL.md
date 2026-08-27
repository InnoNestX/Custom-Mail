---
name: custom-mail
description: Run Custom Mail (private Brevo webmail console) via the lightweight Docker image from Docker Hub or GHCR.
version: 1.0.0
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

# Custom Mail (Docker)

Lightweight local run of [Custom Mail](https://github.com/InnoNestX/Custom-Mail) — Cloudflare Workers mail console, packaged for Docker.

## Images

- Docker Hub: `xuxuclassmate/custom-mail:latest`
- GHCR: `ghcr.io/innonestx/custom-mail:latest`

## Run

```bash
docker run --rm -p 8787:8787 \
  -e ADMIN_PASSWORD='your-password' \
  -e BREVO_API_KEY='xkeysib-...' \
  xuxuclassmate/custom-mail:latest
```

Open http://127.0.0.1:8787

Or compose (repo root):

```bash
ADMIN_PASSWORD=your-password BREVO_API_KEY=xkeysib-... docker compose up
```

## Notes

- Image runs `wrangler dev --local` (no Cloudflare account needed for UI).
- Persist KV/session data with a volume on `/app/.wrangler`.
- Production deploy remains `npm run deploy` to Cloudflare Workers — Docker is for local/demo.
- Branding/config lives in `config/mail.json` inside the image; mount a file over it if you need a custom brand.
