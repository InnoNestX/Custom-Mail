# FAQ

## Login fails with correct password

- **Production:** confirm `wrangler secret put ADMIN_PASSWORD` on the active Worker name (`custom-mail` in `wrangler.jsonc`).
- **Local:** check `.dev.vars` and restart `npm run dev`.
- Too many attempts trigger a temporary lockout — wait and retry.

## Send fails / Brevo errors

- Verify `BREVO_API_KEY` on the Worker.
- `fromEmail` in `mail.json` must be authorized in Brevo.
- Check Brevo dashboard for rate limits or domain verification.

## Deploy fails on CI

- `check` job must pass (typecheck).
- Deploy workflow needs `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID`.
- Use Node 22+ locally to match CI.

## History empty

Only **successful** sends are logged to KV. Failed sends may show in the list with error status when partially recorded.

## Mobile layout

Send history uses a list → full-screen detail flow on narrow screens. Desktop uses side-by-side list and detail.

## Can I use another ESP instead of Brevo?

The send path is implemented for Brevo in `src/email.rs`. Fork and adapt that module for another provider.

## Which Markdown is supported?

Mail bodies are rendered with CommonMark plus GitHub Flavored Markdown in Rust (`src/markdown.rs`):

- Headings, paragraphs, emphasis, strong, strikethrough
- Ordered / unordered / task lists (including nesting)
- Links (`http`/`https`/`mailto`) and images (`http`/`https`)
- Fenced and indented code, tables, block quotes, thematic breaks
- GFM alerts (`> [!NOTE]`, `> [!WARNING]`, …)

Raw HTML in the source is ignored. `javascript:` and `data:` URLs are not turned into links.

## Security notes

- Do not expose this UI on a public host without a strong `ADMIN_PASSWORD`.
- Session cookie is HttpOnly, Secure, SameSite=Strict.
- Do not commit `.dev.vars`, API tokens, or production KV IDs if your fork is public.

## Get help

- [GitHub Discussions](https://github.com/InnoNestX/Custom-Mail/discussions)
- [InnoNestX org Discussions](https://github.com/InnoNestX/.github/discussions)
- Security issues: [private advisory](https://github.com/InnoNestX/Custom-Mail/security/advisories/new)
