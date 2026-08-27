# FAQ

## Login fails with correct password

- **Production:** confirm `wrangler secret put ADMIN_PASSWORD` on Worker `custom-mail`.
- **Local:** check `.dev.vars` and restart `npm run dev`.
- Too many attempts trigger a temporary lockout — wait and retry.

## Send fails / Brevo errors

- Verify `BREVO_API_KEY` on the Worker.
- `fromEmail` in `mail.json` must be authorized in Brevo.
- Check Brevo for rate limits or domain verification.

## Deploy fails on CI

- `check` job must pass (typecheck).
- Deploy workflow needs `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID`.
- Use Node 22+ locally to match CI.

## History empty

Only successful sends are logged to KV. Failed sends may appear with error status when partially recorded.

## Mobile layout

Send history uses list → full-screen detail on narrow screens. Desktop uses side-by-side list and detail.

## Can I use another ESP instead of Brevo?

The send path is implemented for Brevo in `src/email.ts`. Fork and adapt that module for another provider.

## Security notes

- Do not expose this UI without a strong `ADMIN_PASSWORD`.
- Session cookie is HttpOnly, Secure, SameSite=Strict.
- Do not commit `.dev.vars` or API tokens.

## Get help

- [GitHub Discussions](https://github.com/InnoNestX/Custom-Mail/discussions)
- [InnoNestX Discussions](https://github.com/InnoNestX/.github/discussions)
- Security: [private advisory](https://github.com/InnoNestX/Custom-Mail/security/advisories/new)
