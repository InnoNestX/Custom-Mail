# Configuration

Custom Mail is configured through **`config/mail.json`** at the repository root. The Worker reads this file at build/deploy time; change it before `npm run deploy`.

## File overview

```jsonc
{
  "host": "mail.example.com",       // must match wrangler route
  "app": { /* UI strings */ },
  "mail": { /* sender + Brevo */ },
  "site": { /* footer / brand links */ },
  "brand": { /* colors for logo tile */ },
  "addressBook": [ /* preset recipients */ ]
}
```

## `host`

Public hostname users open in the browser. Must equal the custom domain in `wrangler.jsonc` `routes`.

## `app` — UI copy

| Field | Used for |
|-------|----------|
| `title` | App name in header and login |
| `subtitle` | Header subtitle (desktop) |
| `loginTagline` | Login hero small line |
| `loginHeadlineBefore` / `loginHeadlineEm` | Login hero headline |
| `loginLead` | Login description |
| `loginPoints` | Bullet list on login page |
| `loginFormTitle` / `loginFormSub` | Sign-in card |

All strings are plain text; HTML is not interpreted in labels.

## `mail` — sending

| Field | Description |
|-------|-------------|
| `fromEmail` | Locked From address (must be allowed in Brevo) |
| `fromNameDefault` | Default display name in compose |
| `contactEmail` | Contact / support email in metadata |
| `brevoTag` | Tag applied in Brevo for analytics |

## `site` — branding chrome

| Field | Description |
|-------|-------------|
| `url` / `label` | Site link in UI |
| `brandName` | Organization name |
| `logoPath` | Path under `public/` (e.g. `/images/logo.png`) |

## `brand` — colors

Used by the generated logo tile SVG in `src/brand.ts`:

| Key | Role |
|-----|------|
| `tile` / `tileEdge` | Logo background |
| `accent` | Primary green accent |
| `cream` | Page background tone |
| `siteBlue` | Logo mark accent |

## `addressBook`

Array of `{ "address", "note" }` entries shown as quick-pick chips in the To field.

```json
"addressBook": [
  { "address": "ops@example.com", "note": "ops" },
  { "address": "you@gmail.com", "note": "personal" }
]
```

Users can still type any email manually.

## Environment secrets (not in mail.json)

| Secret | Where | Purpose |
|--------|-------|---------|
| `ADMIN_PASSWORD` | `.dev.vars` / Worker secret | Login password |
| `BREVO_API_KEY` | `.dev.vars` / Worker secret | Brevo transactional API |

Never commit secrets into git.

## After changing config

```bash
npm run typecheck
npm run deploy
```
