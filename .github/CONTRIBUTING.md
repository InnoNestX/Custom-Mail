# Contributing to Custom Mail

Thank you for helping improve Custom Mail.

## Development

```bash
git clone git@github.com:InnoNestX/Custom-Mail.git
cd Custom-Mail
# Rust stable + wasm32-unknown-unknown + worker-build
npm install
cp .dev.vars.example .dev.vars
npm run dev
```

Open <http://localhost:8790> and set `ADMIN_PASSWORD` and `BREVO_API_KEY` in `.dev.vars`. Localhost also needs `ALLOW_ANY_HOST=1`.

Published docs: https://innonestx.github.io/Custom-Mail/  
Full guides: [docs/README.md](docs/README.md)

## Before You Submit

```bash
npm run typecheck
```

CI runs the same check on every push and pull request to `main`.

## Pull Requests

1. Fork and branch from `main`
2. Keep changes focused; prefer Rust (`src/`) over new JavaScript
3. Do not commit secrets, `wrangler.jsonc` account IDs, or `.dev.vars`
4. Open a PR — `check` must pass before merge

## Issues

- [Bug report](.github/ISSUE_TEMPLATE/bug_report.yml)
- [Feature request](.github/ISSUE_TEMPLATE/feature_request.yml)
- Search existing issues first

## Deploy

Production Worker deploy is **manual** (local `npm run deploy` or the **Deploy to Cloudflare Workers** workflow). Do not put Cloudflare or Brevo secrets in the repository.

## Code of Conduct

This project follows the [Contributor Covenant](CODE_OF_CONDUCT.md).
