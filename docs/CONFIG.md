# Configuration

Custom Mail is configured through **`config/mail.json`** plus drop-in files under **`plugins/`**. The Worker compiles those files at build/deploy time; change them before `npm run deploy`.

Empty optional fields (and `false` feature/layout flags) **omit** that chrome. Nothing is required beyond `host`, `app.title`, and `mail.fromEmail` / `fromNameDefault`.

Runtime overrides (no rebuild): `MAIL_PROVIDER`, `MAIL_THEME`, `MAIL_LAYOUT`, `MAIL_LOGO`, and `MAIL_CONFIG_JSON` in `.dev.vars` or Docker.

Full plugin guide: [plugins/README.md](../plugins/README.md).

## File overview

```jsonc
{
  "host": "mail.example.com",
  "plugins": {
    "provider": "brevo",      // ids in plugins/providers/
    "theme": "forest",        // ids in plugins/themes/
    "layout": "banner",       // ids in plugins/layouts/
    "logo": "image"           // auto | image | monogram | none
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

Omit any object you do not need. Overlay JSON in `config/overlays/*.json` is deep-merged at compile time (overlay wins; `null` deletes a key).

## `host`

Public hostname users open in the browser. Must equal the custom domain in `wrangler.jsonc` `routes`.

## `plugins`

Catalog JSON under `plugins/` is compiled into the Worker. You choose which id is **active** in `mail.json` (or via env). `GET /api/health` lists `available.*` from that catalog.

### `plugins.provider` — outbound API

Drop a JSON file in `plugins/providers/` to register metadata. Sending still needs a matching adapter in `src/plugins/provider.rs`.

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

Drop a JSON palette in `plugins/themes/` (no Rust change). Bundled ids:

`forest` · `midnight` · `ocean` · `paper` · `rose` · `slate` · `aurora` · `sunset` · `nord`

Aliases: `dark` → midnight, `blue` → ocean, `light` → paper, `pink` → rose, `gray`/`grey`/`neutral` → slate, `polar` → nord.

Unknown ids fall back to `forest`. Set any `brand.*` color to override a token. Leave a field empty to keep the theme default.

Header / “bar before send” colors are `brand.heroFrom`, `brand.heroTo`, and `brand.headerText` (legacy `tile` / `tileEdge` still map onto the header gradient).

### `plugins.layout` — HTML shell for outbound mail

Drop JSON in `plugins/layouts/` to add a shell that only needs header style, body padding, and card shadow.

| Id | Look |
|----|------|
| `card` | White card on the paper background |
| `minimal` | Tighter padding, no shadow |
| `banner` | Colored header bar (from/to gradient) |
| `digest` | Same header treatment, newsletter-like spacing |
| `compact` | Tight padding, no shadow, plain header |

Unknown ids fall back to `card`.

### `plugins.logo` — brand mark

| Value | Behavior |
|-------|----------|
| `auto` (default) | Image if `site.logoPath` / `logoUrl` / a file in `plugins/logos/` exists; otherwise a monogram from `site.brandName`; otherwise omit |
| `image` | Configured or first bundled image; monogram if the file is missing |
| `monogram` | Letter mark only |
| `none` | No mark (even if a file is configured) |

Drop files in `plugins/logos/`; they are copied to `/plugins/logos/<filename>` at build. `layout.showLogo: false` still hides the mark.

## `features`

Set a flag to `false` to hide that UI and skip the matching server path (attachments rejected, history not stored, address book cleared, markdown rendered as escaped text, syntax chips omitted). Catalog: `plugins/features/*.json`.

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
| `logoPath` | Path under `public/` or `/plugins/logos/…`. Empty uses a bundled plugin logo, then a monogram. |
| `logoUrl` | Absolute image URL; takes precedence over `logoPath` |
| `faviconPath` | Browser tab icon. Empty = `logoPath`, then generated `/favicon.svg`. |

Do not leave a stock envelope or third-party logo in `public/` unless it is **your** mark. Replace `public/images/logo.svg` when you fork, or drop a file in `plugins/logos/`.

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
| `MAIL_PROVIDER` | Runtime override of `plugins.provider` |
| `MAIL_THEME` | Runtime override of `plugins.theme` |
| `MAIL_LAYOUT` | Runtime override of `plugins.layout` |
| `MAIL_LOGO` | Runtime override of `plugins.logo` |
| `MAIL_CONFIG_JSON` | JSON object deep-merged onto `mail.json` at runtime |
| `MAILGUN_DOMAIN` | Mailgun domain (optional if `mail.providerDomain` is set) |
| `ALLOW_ANY_HOST=1` | Local dev: skip Host header check |

Never commit secrets or real `wrangler.jsonc` account-specific IDs to git.

## After changing config

```bash
npm run typecheck
npm run deploy
```

JSON under `plugins/` and `config/overlays/` also requires a rebuild. Slot env vars do not.
