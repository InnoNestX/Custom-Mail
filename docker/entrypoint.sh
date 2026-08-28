#!/bin/sh
set -eu

if [ -z "${ADMIN_PASSWORD:-}" ]; then
  echo "ADMIN_PASSWORD is required" >&2
  exit 1
fi

umask 077
{
  printf 'ADMIN_PASSWORD=%s\n' "$ADMIN_PASSWORD"
  printf 'ALLOW_ANY_HOST=1\n'
  for name in \
    MAIL_PROVIDER \
    MAIL_API_KEY \
    BREVO_API_KEY \
    RESEND_API_KEY \
    SENDGRID_API_KEY \
    MAILGUN_API_KEY \
    MAILGUN_DOMAIN \
    POSTMARK_SERVER_TOKEN \
    MAILERSEND_API_KEY \
    SMTP2GO_API_KEY \
    SPARKPOST_API_KEY
  do
    eval "val=\${${name}-}"
    if [ -n "$val" ]; then
      printf '%s=%s\n' "$name" "$val"
    fi
  done
} > .dev.vars

exec wrangler dev \
  --config wrangler.jsonc \
  --ip 0.0.0.0 \
  --port "${PORT:-8787}" \
  --local
