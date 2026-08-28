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
  "features": {
    "attachments": true,
    "history": true,
    "addressBook": true,
    "markdown": true,
    "syntaxHelp": true
  },
  "layout": {
    "showHeader": true,
    "showLogo": true,
    "showSubject": true,
    "showFrom": true,
    "showFooterContact": true,
    "showFooterSite": true
  },
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

### `plugins.provider` — outbound API

| Id | Secret(s) | Notes |
|----|-----------|--------|
| `brevo` | `BREVO_API_KEY` | Default. `mail.tag` becomes a Brevo tag. |
| `resend` | `RESEND_API_KEY` | |
| `sendgrid` | `SENDGRID_API_KEY` | |
| `mailgun` | `MAILGUN_API_KEY` + `MAILGUN_DOMAIN` or `mail.providerDomain` | |
| `postmark` | `POSTMARK_SERVER_TOKEN` | |
| `mailersend` | `MAILERSEND_API_KEY` | |
| `smtp2go` | `SMTP2GO_API_KEY` | |
| `sparkpost` | `SPARKPOST_API_KEY` | |

If the provider-specific secret is empty, `MAIL_API_KEY` is used as a fallback.

`fromEmail` must be authorized on the chosen provider.

### `plugins.theme` — console + header colors

| Id | Look |
|----|------|
| `forest` | Green (default, matches the historical palette) |
| `midnight` | Dark indigo |
| `ocean` | Cyan / sky |
| `paper` | Warm stone |
| `rose` | Crimson |
| `slate` | Neutral gray |

Set any `brand.*` color to override a theme token. Leave a field empty to keep the theme default.

Header / “bar before send” colors are `brand.heroFrom`, `brand.heroTo`, and `brand.headerText` (legacy `tile` / `tileEdge` still map onto the header gradient).

### `plugins.layout` — HTML shell for outbound mail

| Id | Look |
|----|------|
| `card` | White card on the paper background |
| `minimal` | Tighter padding, no shadow |
| `banner` | Colored header bar (from/to gradient) |
| `digest` | Same header treatment, newsletter-like spacing |

## `features`

Set a flag to `false` to hide that UI and skip the matching server path (attachments rejected, history not stored, address book cleared, markdown rendered as escaped text, syntax chips omitted).

## `layout` flags (email chrome)

Each flag is independent. If `showFooterContact` is true but `mail.contactEmail` is empty, the contact line is still omitted. Same for logo, site URL, and header.

## `app` — login / title copy

| Field | Used for |
|-------|----------|
| `title` | App name in header, login, and browser tab |
| `subtitle` | Header subtitle (omitted when empty) |
| `locale` | `html lang` and date formatting (`en`, `zh-CN`, …) |
| `loginTagline` | Login hero small line |
| `loginHeadlineBefore` / `loginHeadlineEm` | Login hero headline |
| `loginLead` | Login description (omitted when empty) |
| `loginPoints` | Bullet list (omitted when empty) |
| `loginFormTitle` / `loginFormSub` | Sign-in card |

All strings are plain text; HTML is not interpreted in labels.

## `mail` — sending

| Field | Description |
|-------|-------------|
| `fromEmail` | Locked From address (must be allowed by the provider) |
| `fromNameDefault` | Default display name in compose |
| `contactEmail` | Footer contact; omit or leave empty to hide |
| `tag` | Provider campaign/tag (`brevoTag` still accepted) |
| `providerDomain` | Mailgun sending domain (or set `MAILGUN_DOMAIN`) |

## `site` — branding chrome

| Field | Description |
|-------|-------------|
| `url` / `label` | Footer site link; omit `url` to hide |
| `brandName` | Organization name (falls back to `app.title`) |
| `logoPath` | Path under `public/` (e.g. `/images/logo.svg`). Empty = no image; the console uses a monogram from `brandName`. |
| `logoUrl` | Absolute image URL; takes precedence over `logoPath` |
| `faviconPath` | Browser tab icon. Empty = `logoPath`, then generated `/favicon.svg`. |

Do not leave a stock envelope or third-party logo in `public/` unless it is **your** mark. Replace `public/images/logo.svg` when you fork.

## `brand` — color overrides

Applied on top of `plugins.theme`. Empty strings are ignored.

| Key | Role |
|-----|------|
| `tile` / `tileEdge` | Header gradient (legacy) and logo tile |
| `heroFrom` / `heroTo` / `headerText` | Explicit header bar colors |
| `accent` / `accentDeep` / `accentSoft` | Console accents |
| `cream` / `paper` | Page background |
| `ink` / `muted` / `line` | Text and borders |
| `siteBlue` | Optional extra (kept for older configs) |

## `i18n`

Every console label (tabs, compose fields, errors, history, preview) can be overridden. Omitted keys use English defaults compiled into the Worker.

Example:

```json
"i18n": {
  "tabCompose": "写信",
  "send": "发送",
  "errNeedRecipient": "请至少添加一个收件人"
}
```

## `syntax`

Markdown helper chips. Set `enabled` to `false` or leave `chips` empty to hide (empty chips fall back to a built-in English set **only when** syntax help is enabled).

```json
"syntax": {
  "enabled": true,
  "heading": "CommonMark / GFM · click to copy",
  "chips": [
    { "label": "**bold**", "insert": "**bold**" }
  ]
}
```

## `addressBook`

Array of `{ "address", "note" }` entries shown as quick-pick chips in the To field. Ignored when `features.addressBook` is false.

## Environment secrets (not in mail.json)

| Secret | Purpose |
|--------|---------|
| `ADMIN_PASSWORD` | Login password |
| Provider key (see table above) | Outbound API |
| `MAIL_API_KEY` | Fallback if the provider-specific key is unset |
| `MAILGUN_DOMAIN` | Mailgun domain (optional if `mail.providerDomain` is set) |
| `ALLOW_ANY_HOST=1` | Local dev: skip Host header check |

Never commit secrets or real `wrangler.jsonc` account-specific IDs to git.

## After changing config

```bash
npm run typecheck
npm run deploy
```

No restart needed for KV or secrets — only for `mail.json` and UI code changes.
