# Plugins

Drop-in catalogs compiled into the Worker. Cloudflare Workers cannot load native `.so` files at runtime, so a plugin is a **JSON (or logo) file in this tree** plus, for a new mail API, a Rust send adapter.

Pick the active plugin in `config/mail.json` (`plugins.*`). Optional runtime overrides (Docker / `.dev.vars`): `MAIL_PROVIDER`, `MAIL_THEME`, `MAIL_LAYOUT`, `MAIL_LOGO`, and `MAIL_CONFIG_JSON`.

Unused chrome is omitted: empty logo, empty footer fields, and `false` feature flags do not render.

## Layout

| Directory | What you add | Rebuild needed |
|-----------|----------------|----------------|
| `providers/*.json` | ESP metadata (id, secrets). Send logic stays in `src/plugins/provider.rs`. | Yes |
| `themes/*.json` | A full color palette. **No Rust change.** | Yes (`worker-build`) |
| `layouts/*.json` | Header style, padding, shadow. **No Rust change** for those knobs. | Yes |
| `features/*.json` | Feature catalog shown in `/api/health`. Enable/disable in `features`. | Yes |
| `logos/*` | Image files copied to `/plugins/logos/…` at build. Point `site.logoPath` at one, or leave path empty to use the first file. | Yes |

After adding a file: `cargo test --lib` and `npm run deploy` (or `docker compose build`).

## `plugins` in mail.json

```json
"plugins": {
  "provider": "brevo",
  "theme": "forest",
  "layout": "banner",
  "logo": "image"
}
```

| Field | Values | Notes |
|-------|--------|--------|
| `provider` | ids in `providers/` | Secret names come from the JSON. Unknown id falls back to `brevo`. |
| `theme` | ids (and aliases) in `themes/` | Unknown id falls back to `forest`. `brand.*` overrides individual colors. |
| `layout` | ids in `layouts/` | Unknown id falls back to `card`. |
| `logo` | `auto` · `image` · `monogram` · `none` | `auto` (default): image if a path/file exists, otherwise a letter mark, otherwise omit. `none` hides the mark even if a file is configured. |

## Config overlays

JSON files in `config/overlays/` are deep-merged onto `config/mail.json` at **compile** time (overlay wins; `null` removes a key). For a **runtime** overlay without rebuilding, set `MAIL_CONFIG_JSON` to a JSON object.

See [config/overlays/README.md](../config/overlays/README.md).

## Health

`GET /api/health` reports the active slots and the compiled catalog (`available.providers`, `themes`, `layouts`, `logos`, `features`).
