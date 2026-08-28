# Console

How to use the web UI after Custom Mail is running (local Wrangler, Docker, or Cloudflare).

Sign in at the URL you started:

| How you started | URL |
|-----------------|-----|
| `npm run dev` | http://localhost:8790 |
| Docker / Compose / OpenClaw | http://localhost:8787 |
| Production | `https://<host>` from `config/mail.json` |

The login password is `ADMIN_PASSWORD` (`.dev.vars`, Docker `-e`, or `wrangler secret`). Too many failed attempts lock the session for a short time.

## Compose

1. **To** — one or more recipients. Press Enter or comma to add an address.
2. **From name** — display name; the From email is `mail.fromEmail` in `config/mail.json`.
3. **Subject**
4. **Body** — CommonMark + GitHub Flavored Markdown when `features.markdown` is on.

Address-book chips (from `addressBook` in `mail.json`) fill **To** in one click when `features.addressBook` is on.

Empty optional chrome is hidden: if you turn off attachments, history, or the address book in `features`, those blocks are gone.

## Preview and send

Use **Preview** to see the email-safe HTML before you confirm. Raw HTML in the body is ignored; `javascript:` and `data:` URLs are not turned into links.

**Send** calls the active provider (`plugins.provider` or `MAIL_PROVIDER`). The matching API secret must be set. `fromEmail` must be authorized with that provider.

## Attachments

When `features.attachments` is on:

- Up to **8** files
- **8 MB** each
- **15 MB** total

Drop files on the attach area or use the file picker.

## History

When `features.history` is on, successful sends are stored in KV (last 10). Open a row for the detail view. On a narrow screen the detail is full-screen; on desktop it sits beside the list.

Failed sends may appear with an error status when they were partially recorded.

## Health

```bash
curl -s http://localhost:8787/api/health
```

The JSON includes `"ok": true`, `"runtime": "rust"`, active `"plugins"` (provider, theme, layout, logo), and `"available"` catalogs.

## Next

- [Docker](./docker) — run without a Rust toolchain
- [OpenClaw skill](./openclaw) — let an agent start the container
- [Plugins](./plugins) · [Configuration](./config) · [Deploy](./deploy)
