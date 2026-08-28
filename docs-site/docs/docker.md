# Docker

Run the same Rust Worker locally in a container. You do not need Rust or `wasm32-unknown-unknown` on the host. The published image is for trying the [console](./console) — production still goes to [Cloudflare](./deploy).

Images:

- Docker Hub / GHCR: `xuxuclassmate/custom-mail:1.0.0` and `ghcr.io/innonestx/custom-mail:1.0.0`
- Moving tag: `:latest` (same release)
- Architectures: `linux/amd64`, `linux/arm64`, `linux/arm/v7` (Docker Desktop on macOS/Windows uses the matching Linux image)

The image sets `ALLOW_ANY_HOST=1` so Host-header checks do not block `localhost`.

## Quick start

### 1. Pull

```bash
docker pull xuxuclassmate/custom-mail:1.0.0
```

GHCR mirror:

```bash
docker pull ghcr.io/innonestx/custom-mail:1.0.0
```

### 2. Secrets

`ADMIN_PASSWORD` is required. Add the API key for the provider baked into the image (`plugins.provider` in `config/mail.json`, default `brevo`) when you want to send. The UI loads without a key.

```bash
export ADMIN_PASSWORD='choose-a-strong-password'
export BREVO_API_KEY='xkeysib-...'   # or another provider key
export PORT=8787                      # optional, default 8787
```

Switch provider / theme / layout / logo **without rebuilding**:

```bash
export MAIL_PROVIDER=resend
export MAIL_THEME=nord
export MAIL_LAYOUT=compact
export MAIL_LOGO=monogram
export RESEND_API_KEY='re_...'
```

`MAIL_CONFIG_JSON='{"site":{"brandName":"Desk"}}'` deep-merges extra JSON at runtime. Adding a **new** theme JSON or logo file still needs `docker compose build`.

### 3. Run

Foreground:

```bash
docker run --rm -p 8787:8787 \
  -e ADMIN_PASSWORD="$ADMIN_PASSWORD" \
  -e BREVO_API_KEY="$BREVO_API_KEY" \
  xuxuclassmate/custom-mail:1.0.0
```

Background:

```bash
docker run -d \
  --name custom-mail \
  -p 8787:8787 \
  -e ADMIN_PASSWORD="$ADMIN_PASSWORD" \
  -e BREVO_API_KEY="$BREVO_API_KEY" \
  xuxuclassmate/custom-mail:1.0.0
```

Open [http://localhost:8787](http://localhost:8787) and sign in with `ADMIN_PASSWORD`. See [Console](./console) for compose, preview, attachments, and history.

```bash
curl -s http://localhost:8787/api/health
```

Stop a named container with `docker stop custom-mail` and remove it with `docker rm custom-mail`.

## Docker Compose

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
export ADMIN_PASSWORD='choose-a-strong-password'
export BREVO_API_KEY='xkeysib-...'
docker compose up -d
```

Compose maps port **8787**, passes through the same env vars, and keeps Wrangler state in the `mail-data` volume.

```bash
docker compose down
```

To change branding or add plugin JSON / logo files, edit `config/mail.json` / `plugins/` then:

```bash
docker compose build
docker compose up -d
```

Slot env vars (`MAIL_PROVIDER`, `MAIL_THEME`, …) still switch the **active** plugin without a rebuild.

## Environment

| Variable | Default | Description |
| --- | --- | --- |
| `ADMIN_PASSWORD` | *(required)* | Console login password |
| `MAIL_PROVIDER` | empty | Overrides `plugins.provider` |
| `MAIL_THEME` | empty | Overrides `plugins.theme` |
| `MAIL_LAYOUT` | empty | Overrides `plugins.layout` |
| `MAIL_LOGO` | empty | Overrides `plugins.logo` (`auto`, `image`, `monogram`, `none`) |
| `MAIL_CONFIG_JSON` | empty | JSON object deep-merged onto `mail.json` at runtime |
| `BREVO_API_KEY` | empty | Default provider; UI works without it, send needs a key |
| `RESEND_API_KEY` | empty | Resend |
| `SENDGRID_API_KEY` | empty | SendGrid |
| `MAILGUN_API_KEY` / `MAILGUN_DOMAIN` | empty | Mailgun |
| `POSTMARK_SERVER_TOKEN` | empty | Postmark |
| `MAILERSEND_API_KEY` | empty | MailerSend |
| `SMTP2GO_API_KEY` | empty | SMTP2GO |
| `SPARKPOST_API_KEY` | empty | SparkPost |
| `MAIL_API_KEY` | empty | Used when the provider-specific secret is empty |
| `PORT` | `8787` | Listen port inside the container |

Provider ids: `brevo`, `resend`, `sendgrid`, `mailgun`, `postmark`, `mailersend`, `smtp2go`, `sparkpost`.  
Theme ids: `forest`, `midnight`, `ocean`, `paper`, `rose`, `slate`, `aurora`, `sunset`, `nord`, …  
Layout ids: `card`, `minimal`, `banner`, `digest`, `compact`.

## What the entrypoint does

`docker/entrypoint.sh` writes `.dev.vars` from the environment (`ALLOW_ANY_HOST=1` plus any non-empty keys above) and starts `wrangler dev --ip 0.0.0.0 --port $PORT --local`.

## Troubleshooting

- **Container exits immediately** — `ADMIN_PASSWORD` was empty. The entrypoint refuses to start without it.
- **Cannot send** — set the secret that matches `MAIL_PROVIDER` / `plugins.provider`, and authorize `fromEmail`.
- **New theme file not listed** — catalogs are compiled at **image build**. Rebuild after adding files under `plugins/`.
- **Port already in use** — change the host mapping, e.g. `-p 8788:8787`.

## Next

- [Console](./console) — use the UI
- [OpenClaw skill](./openclaw) — `clawhub install custom-mail-skill`
- [Plugins](./plugins) · [Deploy](./deploy)
