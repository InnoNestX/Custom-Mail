# FAQ

## Login fails with correct password

- **Production:** confirm `wrangler secret put ADMIN_PASSWORD` on Worker `custom-mail`.
- **Local:** check `.dev.vars` and restart `npm run dev`.
- Too many attempts trigger a temporary lockout — wait and retry.

## Send fails / provider errors

- Verify the API secret that matches `plugins.provider` (or `MAIL_PROVIDER`) on the Worker.
- `fromEmail` in `mail.json` must be authorized with that provider.
- Check the provider dashboard for rate limits or domain verification.

## Deploy fails on CI

- `check` job must pass (typecheck).
- Deploy workflow needs `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID`.
- Use Node 22+ locally to match CI.

## History empty

Only successful sends are logged to KV. Failed sends may appear with error status when partially recorded.

## Mobile layout

Send history uses list → full-screen detail on narrow screens. Desktop uses side-by-side list and detail.

## Can I use another ESP instead of Brevo?

Yes. Set `plugins.provider` in `config/mail.json` (or `MAIL_PROVIDER`) to an id from `plugins/providers/` — bundled: `brevo`, `resend`, `sendgrid`, `mailgun`, `postmark`, `mailersend`, `smtp2go`, `sparkpost` — and put the matching API secret on the Worker (see [configuration](./config)). `fromEmail` must be authorized with that provider.

## How do I add a theme, layout, or logo?

Add JSON under `plugins/themes/` or `plugins/layouts/`, or drop a file in `plugins/logos/`. Set `plugins.theme` / `layout` / `logo` (or `MAIL_THEME` / `MAIL_LAYOUT` / `MAIL_LOGO`). See the [configuration](./config) guide.

## Which Markdown is supported?

Mail bodies are rendered with CommonMark plus GitHub Flavored Markdown in Rust (`src/markdown.rs`):

- Headings, paragraphs, emphasis, strong, strikethrough
- Ordered / unordered / task lists (including nesting)
- Links (`http`/`https`/`mailto`) and images (`http`/`https`)
- Fenced and indented code, tables, block quotes, thematic breaks
- GFM alerts (`> [!NOTE]`, `> [!WARNING]`, …)

Raw HTML in the source is ignored. `javascript:` and `data:` URLs are not turned into links.

## How do I run this in Docker?

See the [Docker](./docker) guide: `docker pull xuxuclassmate/custom-mail:1.0.0`, set `ADMIN_PASSWORD`, then `docker run -p 8787:8787`. Compose and env vars are on that page.

## How do I use the OpenClaw / Claw skill?

```bash
clawhub install custom-mail-skill
```

Or copy `skills/custom-mail/` into `~/.openclaw/workspace/skills/`. The skill needs Docker. Details: [OpenClaw skill](./openclaw).

## Security notes

- Do not expose this UI without a strong `ADMIN_PASSWORD`.
- Session cookie is HttpOnly, Secure, SameSite=Strict.
- Do not commit `.dev.vars` or API tokens.

## Get help

- [GitHub Discussions](https://github.com/InnoNestX/Custom-Mail/discussions)
- [InnoNestX Discussions](https://github.com/InnoNestX/.github/discussions)
- Security: [private advisory](https://github.com/InnoNestX/Custom-Mail/security/advisories/new)
