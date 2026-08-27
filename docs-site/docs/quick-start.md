# Quick start

## Requirements

- Node.js **22+**
- Cloudflare account (for production)
- Brevo API key (for sending)

## Install and run

```bash
git clone https://github.com/InnoNestX/Custom-Mail.git
cd Custom-Mail
npm install
cp .dev.vars.example .dev.vars
```

Edit `.dev.vars`:

```bash
ADMIN_PASSWORD=your-strong-password
BREVO_API_KEY=your-brevo-key
```

Start the local Worker:

```bash
npm run dev
```

Open **http://localhost:8790** and sign in with `ADMIN_PASSWORD`.

## Docs site (optional)

```bash
cd docs-site
npm install
npm run docs:dev
```

## Next

- Tune branding in [Configuration](./config)
- Ship to production with [Deploy](./deploy)
