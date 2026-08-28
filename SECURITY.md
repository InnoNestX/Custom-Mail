# Security Policy

## Supported Versions

Custom Mail runs as a Cloudflare Worker. Security fixes are deployed by updating the Worker on your account.

| Deployment | Supported |
| ---------- | --------- |
| Latest `main` | Yes |
| Older Worker versions | Update via `npm run deploy` |

## Reporting a Vulnerability

**Do not report security issues in public GitHub Issues.**

| Method | Link |
| ------ | ---- |
| **Private vulnerability reporting** | [GitHub Security Advisories](https://github.com/InnoNestX/Custom-Mail/security/advisories/new) |
| **Discussions (non-sensitive)** | [Custom-Mail Discussions](https://github.com/InnoNestX/Custom-Mail/discussions) |

Include steps to reproduce, impact, and affected routes (`mail.*` host, Worker name).

## Response Targets

| Stage | Target |
| ----- | ------ |
| Acknowledgement | 48 hours |
| Severity assessment | 7 days |
| Fix | Critical issues as soon as possible |

## Scope

In scope: authentication bypass, secret leakage, unauthorized send, XSS in compose UI, attachment handling.

Out of scope: misconfigured `ADMIN_PASSWORD` on your deployment, mail-provider account compromise, DNS misrouting outside this codebase.
