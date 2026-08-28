# Deployment

Custom Mail runs as a single **Cloudflare Worker** with **KV** and **Workers Assets**. There is no database server to manage.

## Prerequisites

- Cloudflare account with Workers enabled
- [Brevo](https://www.brevo.com/) account and verified sender domain
- Rust stable (`wasm32-unknown-unknown`) and `worker-build` 0.8.5
- Node.js 22+ and `npm` (Wrangler CLI)

## 1. Configure product

Edit `config/mail.json`:

- Set `host` to your mail subdomain (e.g. `mail.example.com`)
- Set `mail.fromEmail` to a Brevo-authorized address
- Customize `app`, `brand`, `addressBook`

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
# BREVO_API_KEY=...
```

**Production** — Worker secrets (one time per Worker name):

```bash
npx wrangler secret put ADMIN_PASSWORD
npx wrangler secret put BREVO_API_KEY
```

## 3. Deploy

```bash
npm run typecheck
npm run deploy
```

Visit `https://<host>` from `mail.json` and sign in.

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

Worker secrets (`ADMIN_PASSWORD`, `BREVO_API_KEY`) are **not** stored in GitHub.

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
- [ ] `ADMIN_PASSWORD` and `BREVO_API_KEY` on Worker
- [ ] Brevo sender domain verified for `fromEmail`
- [ ] CI `check` passes on `main`
