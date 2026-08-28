# OpenClaw skill

Custom Mail ships an [OpenClaw](https://docs.openclaw.ai/tools/skills) skill so an agent can pull the Docker image, set secrets, and start the [console](./console). The skill lives in the repo at `skills/custom-mail/SKILL.md` (ClawHub slug: `custom-mail-skill`, version `1.0.0`).

It requires the **`docker`** binary on the machine where the agent runs. It does not deploy to Cloudflare; that is still [Deploy](./deploy).

## Install

### ClawHub

```bash
clawhub install custom-mail-skill
```

Browse the published skill: [custom-mail-skill on ClawHub](https://clawhub.ai/xuxuclassmate/skills/custom-mail-skill) (version `1.0.0`). CLI notes: [ClawHub](https://docs.openclaw.ai/tools/clawhub).

### From this repository

Copy the skill folder into the OpenClaw workspace (highest priority) or the shared skills directory:

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cp -R Custom-Mail/skills/custom-mail ~/.openclaw/workspace/skills/custom-mail
```

Shared install (all agents on that instance):

```bash
cp -R Custom-Mail/skills/custom-mail ~/.openclaw/skills/custom-mail
```

OpenClaw reloads `SKILL.md` from those directories. See [Skills](https://docs.openclaw.ai/tools/skills).

## What the skill does

When you ask to run Custom Mail locally, the agent should:

1. Confirm Docker is available.
2. Set `ADMIN_PASSWORD` (required) and the provider API key you give it.
3. Optionally set `MAIL_PROVIDER`, `MAIL_THEME`, `MAIL_LAYOUT`, `MAIL_LOGO`.
4. `docker pull` `xuxuclassmate/custom-mail:1.0.0` (or `:latest` / GHCR `ghcr.io/innonestx/custom-mail:1.0.0`).
5. `docker run` on port **8787** (or `PORT`).
6. Tell you to open http://localhost:8787 and sign in.

Health check: `curl -s http://localhost:8787/api/health`.

Full Docker flags and Compose: [Docker](./docker).

## Environment the skill knows

| Variable | Required | Description |
| --- | --- | --- |
| `ADMIN_PASSWORD` | yes | Console login |
| `MAIL_PROVIDER` | no | `brevo`, `resend`, `sendgrid`, `mailgun`, `postmark`, `mailersend`, `smtp2go`, `sparkpost` |
| `MAIL_THEME` | no | `forest`, `midnight`, `ocean`, `paper`, `rose`, `slate`, `aurora`, `sunset`, `nord`, … |
| `MAIL_LAYOUT` | no | `card`, `minimal`, `banner`, `digest`, `compact` |
| `MAIL_LOGO` | no | `auto`, `image`, `monogram`, `none` |
| `BREVO_API_KEY` | no | Default provider; needed to send unless another key is set |
| `MAIL_API_KEY` | no | Fallback API key |
| `RESEND_API_KEY` | no | With `MAIL_PROVIDER=resend` |
| `SENDGRID_API_KEY` | no | With `MAIL_PROVIDER=sendgrid` |
| `PORT` | no | Container port, default `8787` |

The skill metadata lists the same keys under `metadata.openclaw.envVars`. Map them in `~/.openclaw/openclaw.json` if you keep secrets in the OpenClaw config instead of the chat.

## Example prompts

```
Pull and run Custom Mail on port 8787 with ADMIN_PASSWORD=dev-secret and my Brevo key.
```

```
Start the custom-mail Docker container with MAIL_PROVIDER=resend, MAIL_THEME=nord, and RESEND_API_KEY.
```

```
Clone InnoNestX/Custom-Mail and bring it up with docker compose.
```

Chinese triggers in the skill include 「帮我本地跑一下 Custom Mail」 and 「用 Docker 启动发信控制台」.

## After it is up

Use the [console](./console): compose Markdown, preview, attach files, send, browse history.

To change baked-in `mail.json` / `plugins/` files, clone the repo and [rebuild with Compose](./docker#docker-compose). Slot env vars still override the active plugin without a rebuild.

## Production

The skill can remind you of the Cloudflare path, but secrets for production stay on the Worker:

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
npx wrangler secret put ADMIN_PASSWORD
npx wrangler secret put BREVO_API_KEY
npm run deploy
```

See [Deploy](./deploy).
