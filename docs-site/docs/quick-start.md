# Quick start

## Requirements

- Rust **stable** (with `wasm32-unknown-unknown`) and `worker-build` 0.8.5
- Node.js **22+** (Wrangler)
- Cloudflare account (for production)
- An API key for your chosen provider (Brevo by default; see [Plugins](./plugins) and [Configuration](./config))

## Install and run

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
npm install
cp .dev.vars.example .dev.vars
```

Edit `.dev.vars`:

```bash
ADMIN_PASSWORD=your-strong-password
BREVO_API_KEY=your-brevo-key
# or MAIL_PROVIDER=resend and RESEND_API_KEY=...
# optional: MAIL_THEME=nord  MAIL_LAYOUT=compact  MAIL_LOGO=monogram
ALLOW_ANY_HOST=1
```

Start the local Worker:

```bash
npm run dev
```

Open **http://localhost:8790** and sign in with `ADMIN_PASSWORD`. Then see [Console](./console) for compose, preview, attachments, and history.

## Other ways to run

| Path | When to use |
|------|-------------|
| [Docker](./docker) | No Rust toolchain. `docker run` or Compose on port 8787. |
| [OpenClaw skill](./openclaw) | An agent starts the container: `clawhub install custom-mail-skill`. |
| [Deploy](./deploy) | Production on Cloudflare Workers. |

## Pick plugins

Drop files under `plugins/` (themes, layouts, logos, provider metadata) and set the active ids in `config/mail.json`. Unused sections stay hidden. Details: [Plugins](./plugins).

## Docs site (optional)

```bash
cd docs-site
npm install
npm run docs:dev
```

## Next

- Tune branding in [Configuration](./config)
- Ship to production with [Deploy](./deploy)
