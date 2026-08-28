# Custom Mail

Private Brevo webmail console as a **Rust** Cloudflare Worker (`workers-rs` → WASM). Compose, Markdown preview, attachments, and send history — no mail server to maintain.

| | |
|---|---|
| **Source** | https://github.com/InnoNestX/Custom-Mail |
| **Docs** | https://innonestx.github.io/Custom-Mail/ |
| **Demo** | https://mail.xuxuclassmate.com |
| **GHCR** | `ghcr.io/innonestx/custom-mail` |

## Quick start

```bash
docker run --rm -p 8787:8787 \
  -e ADMIN_PASSWORD='your-password' \
  -e BREVO_API_KEY='xkeysib-...' \
  xuxuclassmate/custom-mail:latest
```

Open http://127.0.0.1:8787 and sign in with `ADMIN_PASSWORD`.

Health check:

```bash
curl -s http://127.0.0.1:8787/api/health
# {"ok":true,"runtime":"rust","service":"mail",...}
```

## Tags

| Tag | Meaning |
|-----|---------|
| `latest` | Current stable on `main` |
| `0.2.0` | Rust Worker runtime |

Same image is published to GHCR: `ghcr.io/innonestx/custom-mail:latest`.

## Environment

| Variable | Required | Description |
|----------|----------|-------------|
| `ADMIN_PASSWORD` | **yes** | Console login password |
| `BREVO_API_KEY` | no* | Brevo API key (`xkeysib-…`). Needed to send mail |
| `PORT` | no | Listen port (default `8787`) |

\* Without `BREVO_API_KEY` the UI still loads; send will fail until the key is set.

## Compose

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
export ADMIN_PASSWORD='your-password'
export BREVO_API_KEY='xkeysib-...'
docker compose up
```

## What’s inside

- Prebuilt Worker WASM + Wrangler in a slim Node image
- `ALLOW_ANY_HOST=1` so local Docker does not need a custom Host header
- Config from the image’s `config/mail.json` (brand, address book, copy)

For Cloudflare production deploy (not Docker), see the [docs](https://innonestx.github.io/Custom-Mail/) and `npm run deploy` in the repo.

## OpenClaw

```bash
clawhub install custom-mail
```

## License

MIT — https://github.com/InnoNestX/Custom-Mail/blob/main/LICENSE
