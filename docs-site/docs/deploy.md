# Deploy

Custom Mail runs as a single **Cloudflare Worker** with **KV** and **Workers Assets**.

## Prerequisites

- Cloudflare account with Workers enabled
- [Brevo](https://www.brevo.com/) (default) or another supported provider — see [configuration](./config)
- Rust stable (`wasm32-unknown-unknown`) and `worker-build` 0.8.5
- Node.js 22+ and `npm`

## 1. Configure product

Edit `config/mail.json`:

- Set `host` to your mail subdomain
- Set `mail.fromEmail` to an address authorized by your provider
- Customize `plugins`, `app`, `brand`, `site`, `i18n`, `addressBook`

Edit `wrangler.jsonc`:

- `name` — Worker script name (e.g. `custom-mail`)
- `routes` — same host as `config/mail.json`
- `kv_namespaces` — your KV namespace ID

```bash
npx wrangler kv namespace create MAIL_LOG_KV
```

Paste the returned `id` into `wrangler.jsonc`.

## 2. Secrets

**Local** — `.dev.vars`:

```bash
cp .dev.vars.example .dev.vars
```

**Production**:

```bash
npx wrangler secret put ADMIN_PASSWORD
npx wrangler secret put BREVO_API_KEY   # or the key for plugins.provider
```

## 3. Deploy

```bash
npm run typecheck
npm run deploy
```

Visit `https://<host>` and sign in.

## Docker (local)

The published image is for trying the console locally. Pass `ADMIN_PASSWORD` and the API key for the active provider. `MAIL_PROVIDER`, `MAIL_THEME`, `MAIL_LAYOUT`, `MAIL_LOGO`, and `MAIL_CONFIG_JSON` override `mail.json` plugin slots without rebuilding. New theme/layout JSON or logo files still need an image rebuild.

Full commands, Compose, and env vars: [Docker](./docker). Agent install: [OpenClaw skill](./openclaw) (`clawhub install custom-mail`).

## Custom domain

```jsonc
"routes": [
  { "pattern": "mail.example.com", "custom_domain": true }
]
```

## GitHub Actions

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| CI | push / PR to `main` | typecheck |
| CodeQL | push / PR / weekly | security scan |
| Docs | push to `docs-site/**` | GitHub Pages |
| Deploy to Cloudflare Workers | **Manual only** | `wrangler deploy` |

### Required GitHub secrets (Cloudflare deploy)

| Secret | Scope |
|--------|--------|
| `CLOUDFLARE_API_TOKEN` | Workers Scripts write |
| `CLOUDFLARE_ACCOUNT_ID` | Cloudflare account ID |

Worker secrets stay on Cloudflare — not in GitHub.

## Checklist

- `host` in `mail.json` matches the `wrangler.jsonc` route
- KV namespace ID is a real binding, not a placeholder
- `ADMIN_PASSWORD` and the API key for `plugins.provider` are set on the Worker
- The provider has verified `fromEmail`
- CI `check` is green on `main`
