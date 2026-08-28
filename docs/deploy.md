# Deployment

Custom Mail runs as a single **Cloudflare Worker** with **KV** and **Workers Assets**. There is no database server to manage.

## Prerequisites

- Cloudflare account with Workers enabled
- [Brevo](https://www.brevo.com/) (default) or another supported provider — see [CONFIG.md](CONFIG.md)
- Rust stable (`wasm32-unknown-unknown`) and `worker-build` 0.8.5
- Node.js 22+ and `npm` (Wrangler CLI)

## 1. Configure product

Edit `config/mail.json`:

- Set `host` to your mail subdomain (e.g. `mail.example.com`)
- Set `mail.fromEmail` to an address authorized by your provider
- Customize `plugins`, `app`, `brand`, `site`, `i18n`, `addressBook`

Edit `wrangler.jsonc`:

- `name` — Worker script name (e.g. `custom-mail`)
- `routes` — same host as `config/mail.json`
- `kv_namespaces` — your KV namespace ID

Create KV if needed:

```bash
npx wrangler kv namespace create MAIL_LOG_KV
```

Paste the returned `id` into `wrangler.jsonc`.

## 2. Secrets

**Local development** — `.dev.vars`:

```bash
cp .dev.vars.example .dev.vars
# ADMIN_PASSWORD=...
# Provider API key matching plugins.provider (BREVO_API_KEY, RESEND_API_KEY, …)
```

**Production** — Worker secrets (one time per Worker name):

```bash
npx wrangler secret put ADMIN_PASSWORD
npx wrangler secret put BREVO_API_KEY   # or RESEND_API_KEY / SENDGRID_API_KEY / …
```

## 3. Deploy

```bash
npm run typecheck
npm run deploy
```

Visit `https://<host>` from `mail.json` and sign in.

## Docker (local)

The published image is for trying the console locally. Pass `ADMIN_PASSWORD` and the API key for the active provider. `MAIL_PROVIDER`, `MAIL_THEME`, `MAIL_LAYOUT`, `MAIL_LOGO`, and `MAIL_CONFIG_JSON` override the matching `mail.json` slots without rebuilding. Adding a new theme/layout JSON or logo file still needs an image rebuild.

See [docker/DOCKERHUB.md](../docker/DOCKERHUB.md), the docs [Docker](https://innonestx.github.io/Custom-Mail/docker.html) and [OpenClaw](https://innonestx.github.io/Custom-Mail/openclaw.html) guides, `skills/custom-mail/SKILL.md`, and ClawHub [`custom-mail-skill`@1.0.0](https://clawhub.ai/xuxuclassmate/skills/custom-mail-skill) (`clawhub install custom-mail-skill`).

## Custom domain

In `wrangler.jsonc`:

```jsonc
"routes": [
  { "pattern": "mail.example.com", "custom_domain": true }
]
```

Cloudflare will attach the domain when deploy succeeds. Ensure DNS is on Cloudflare or follow Workers custom domain docs.

## GitHub Actions

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| CI | push / PR to `main` | `npm ci` + typecheck |
| CodeQL | push / PR / weekly | Security scan |
| Deploy to Cloudflare Workers | **Manual only** | `wrangler deploy` |

### Required GitHub secrets

| Secret | Scope |
|--------|--------|
| `CLOUDFLARE_API_TOKEN` | Org or repo — Workers Scripts write |
| `CLOUDFLARE_ACCOUNT_ID` | Your Cloudflare account ID |

Worker secrets (`ADMIN_PASSWORD` and the provider API key) are **not** stored in GitHub.

### Manual deploy from Actions

1. GitHub → **Actions** → **Deploy to Cloudflare Workers**
2. **Run workflow** → branch `main`

## Upgrades

```bash
git pull
npm ci
npm run typecheck
npm run deploy
```

If you change the Worker `name` in `wrangler.jsonc`, set secrets again on the new Worker script.

## Checklist

- [ ] `host` in `mail.json` matches `wrangler.jsonc` route
- [ ] KV namespace ID is real (not placeholder)
- [ ] `ADMIN_PASSWORD` and the API key for `plugins.provider` on the Worker
- [ ] Provider has verified `fromEmail`
- [ ] CI `check` passes on `main`
