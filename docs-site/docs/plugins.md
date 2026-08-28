# Plugins

Five slots are drop-in catalogs under `plugins/`. Cloudflare Workers cannot load native `.so` files, so a plugin is a **JSON or image file compiled into the Worker**. You pick the active id in `config/mail.json` (or with env vars). Unused chrome is omitted.

| Slot | Drop files in | Select with | Runtime override |
|------|---------------|-------------|------------------|
| Provider | `plugins/providers/*.json` | `plugins.provider` | `MAIL_PROVIDER` |
| Features | `plugins/features/*.json` | `features.*` (`false` hides it) | — |
| Theme | `plugins/themes/*.json` | `plugins.theme` | `MAIL_THEME` |
| Layout | `plugins/layouts/*.json` | `plugins.layout` | `MAIL_LAYOUT` |
| Logo | `plugins/logos/*` | `plugins.logo` | `MAIL_LOGO` |
| Config | `config/overlays/*.json` | omit unused keys | `MAIL_CONFIG_JSON` |

`GET /api/health` lists the compiled catalog under `available` and the active ids under `plugins`.

## Provider

Bundled: `brevo` · `resend` · `sendgrid` · `mailgun` · `postmark` · `mailersend` · `smtp2go` · `sparkpost`.

JSON registers id, label, secret names, and whether a sending domain is required. **A new HTTP ESP still needs a send adapter in Rust** (`src/plugins/provider.rs`).

## Features

Drop JSON under `plugins/features/`. Each file is a toggle in `features.*` — set it to `false` to hide that capability. Bundled: `markdown` · `history` · `attachments`.

## Theme

Drop a palette JSON — **no Rust change**. Bundled: `forest` · `midnight` · `ocean` · `paper` · `rose` · `slate` · `aurora` · `sunset` · `nord`.

Unknown ids fall back to `forest`. `brand.*` color keys override individual tokens.

Example `plugins/themes/nord.json`:

```json
{
  "id": "nord",
  "label": "Nord",
  "aliases": ["polar"],
  "accent": "#5e81ac",
  "accentDeep": "#2e3440",
  "accentSoft": "#eceff4",
  "ink": "#2e3440",
  "muted": "#4c566a",
  "paper": "#eceff4",
  "line": "#d8dee9",
  "heroFrom": "#5e81ac",
  "heroTo": "#3b4252",
  "headerText": "#eceff4"
}
```

## Layout

JSON sets header style, body padding, and card shadow — **no Rust change** for those knobs. Bundled: `card` · `minimal` · `banner` · `digest` · `compact`.

```json
{
  "id": "compact",
  "label": "Compact",
  "headerStyle": "plain",
  "bodyPadding": "12px 16px 12px",
  "cardShadow": "none"
}
```

`headerStyle` is `plain`, `gradient`, or `none`.

## Logo

| `plugins.logo` | What you see |
|----------------|--------------|
| `auto` | Image if a path/file exists; otherwise a monogram from `site.brandName`; otherwise omit |
| `image` | Configured or first file in `plugins/logos/`; monogram if missing |
| `monogram` | Letter mark only |
| `none` | No mark |

Files in `plugins/logos/` are copied to `/plugins/logos/<filename>` at build. Point `site.logoPath` at that URL, or leave it empty to use the first file.

## Config overlays

JSON in `config/overlays/` is deep-merged onto `mail.json` at **compile** time (overlay wins; `null` deletes a key). For a **runtime** overlay without rebuilding:

```bash
MAIL_CONFIG_JSON='{"plugins":{"theme":"nord","logo":"monogram"}}'
```

Slot env vars (`MAIL_THEME`, …) are applied after that overlay.

## After adding a file

```bash
cargo test --lib
npm run deploy
# or: docker compose build
```

JSON/logo files need a rebuild. Switching the **active** id with `MAIL_*` does not.
