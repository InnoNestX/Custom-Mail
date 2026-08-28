# Configuration

Custom Mail is configured through **`config/mail.json`** at the repository root. The Worker reads this file at build/deploy time; change it before `npm run deploy`.

Empty optional fields (and `false` feature/layout flags) **omit** that chrome. Nothing is required beyond `host`, `app.title`, and `mail.fromEmail` / `fromNameDefault`.

## File overview

```jsonc
{
  "host": "mail.example.com",
  "plugins": {
    "provider": "brevo",      // brevo | resend | sendgrid | mailgun | postmark | mailersend | smtp2go | sparkpost
    "theme": "forest",        // forest | midnight | ocean | paper | rose | slate
    "layout": "banner"        // card | minimal | banner | digest
  },
  "features": { "attachments": true, "history": true, "addressBook": true, "markdown": true, "syntaxHelp": true },
  "layout": { "showHeader": true, "showLogo": true, "showSubject": true, "showFrom": true, "showFooterContact": true, "showFooterSite": true },
  "app": { /* console copy */ },
  "mail": { /* sender */ },
  "site": { /* logo, favicon, footer link */ },
  "brand": { /* color overrides on top of the theme */ },
  "i18n": { /* console labels; omitted keys use English defaults */ },
  "syntax": { /* markdown helper chips */ },
  "addressBook": [ /* preset recipients */ ]
}
```

## `host`

Public hostname users open in the browser. Must equal the custom domain in `wrangler.jsonc` `routes`.

## `plugins`

All plugins are compiled into the Worker. You choose which one is active at deploy time.

### `plugins.provider`

| Id | Secret(s) |
|----|-----------|
| `brevo` | `BREVO_API_KEY` |
| `resend` | `RESEND_API_KEY` |
| `sendgrid` | `SENDGRID_API_KEY` |
| `mailgun` | `MAILGUN_API_KEY` + `MAILGUN_DOMAIN` or `mail.providerDomain` |
| `postmark` | `POSTMARK_SERVER_TOKEN` |
| `mailersend` | `MAILERSEND_API_KEY` |
| `smtp2go` | `SMTP2GO_API_KEY` |
| `sparkpost` | `SPARKPOST_API_KEY` |

`MAIL_API_KEY` is used when the provider-specific secret is empty. `fromEmail` must be authorized on the chosen provider. `mail.tag` (`brevoTag` still accepted) is sent as a campaign/tag when the API supports it.

### `plugins.theme`

`forest` · `midnight` · `ocean` · `paper` · `rose` · `slate`

Set any `brand.*` color to override a theme token. Header / top-bar colors are `brand.heroFrom`, `brand.heroTo`, and `brand.headerText` (legacy `tile` / `tileEdge` still map onto the header gradient).

### `plugins.layout`

`card` · `minimal` · `banner` · `digest`

## `features` / `layout` flags

`false` or empty config **hides** that block (attachments UI, history, address book, markdown helper, email header, logo, footer contact, footer site).

## `app` — login / title copy

| Field | Used for |
|-------|----------|
| `title` | App name in header, login, and browser tab |
| `subtitle` | Header subtitle (omitted when empty) |
| `locale` | `html lang` and date formatting |
| `loginTagline` / `loginHeadlineBefore` / `loginHeadlineEm` / `loginLead` / `loginPoints` | Login hero |
| `loginFormTitle` / `loginFormSub` | Sign-in card |

## `mail` — sending

| Field | Description |
|-------|-------------|
| `fromEmail` | Locked From address |
| `fromNameDefault` | Default display name |
| `contactEmail` | Footer contact; empty hides it |
| `tag` | Provider campaign/tag |
| `providerDomain` | Mailgun sending domain |

## `site` — branding chrome

| Field | Description |
|-------|-------------|
| `url` / `label` | Footer site link; empty `url` hides it |
| `brandName` | Organization name (falls back to `app.title`) |
| `logoPath` / `logoUrl` | Logo image. Empty = monogram from the brand name, no stock icon. |
| `faviconPath` | Browser tab icon. Empty = `logoPath`, then generated `/favicon.svg`. |

Replace `public/images/logo.svg` with your own mark when you fork.

## `brand` — color overrides

`tile`, `tileEdge`, `heroFrom`, `heroTo`, `headerText`, `accent`, `accentDeep`, `accentSoft`, `cream`, `paper`, `ink`, `muted`, `line`, `siteBlue`. Empty strings keep the theme default.

## `i18n` / `syntax` / `addressBook`

Console labels default to English. Override any key under `i18n`. Syntax chips live under `syntax.chips`. Address-book entries are `{ "address", "note" }`.

## Environment secrets (not in mail.json)

| Secret | Purpose |
|--------|---------|
| `ADMIN_PASSWORD` | Login password |
| Provider key (see table) | Outbound API |
| `MAIL_API_KEY` | Fallback API key |
| `MAILGUN_DOMAIN` | Mailgun domain |
| `ALLOW_ANY_HOST=1` | Local dev: skip Host header check |

Never commit secrets into git.

## After changing config

```bash
npm run typecheck
npm run deploy
```
