# Quick start

## Requirements

- Rust **stable** (with `wasm32-unknown-unknown`) and `worker-build` 0.8.5
- Node.js **22+** (Wrangler)
- Cloudflare account (for production)
- An API key for your chosen provider (Brevo by default; see [Configuration](./config))

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
ALLOW_ANY_HOST=1
```

Start the local Worker:

```bash
npm run dev
```

Open **http://localhost:8790** and sign in with `ADMIN_PASSWORD`.

## Docs site (optional)

```bash
cd docs-site
npm install
npm run docs:dev
```

## Next

- Tune branding in [Configuration](./config)
- Ship to production with [Deploy](./deploy)
