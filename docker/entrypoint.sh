#!/bin/sh
set -eu

if [ -z "${ADMIN_PASSWORD:-}" ]; then
  echo "ADMIN_PASSWORD is required" >&2
  exit 1
fi

umask 077
printf 'ADMIN_PASSWORD=%s\nBREVO_API_KEY=%s\nALLOW_ANY_HOST=1\n' \
  "$ADMIN_PASSWORD" \
  "${BREVO_API_KEY:-}" > .dev.vars

exec wrangler dev \
  --config wrangler.jsonc \
  --ip 0.0.0.0 \
  --port "${PORT:-8787}" \
  --local
