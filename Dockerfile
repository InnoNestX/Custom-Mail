FROM node:22-bookworm-slim

WORKDIR /app

# Minimal install — only wrangler (+ platform workerd)
RUN npm init -y >/dev/null \
  && npm install --no-audit --no-fund wrangler@4.125.0 \
  && npm cache clean --force \
  && rm -rf /tmp/* /root/.npm \
  && test -x node_modules/.bin/wrangler

COPY wrangler.docker.jsonc ./wrangler.jsonc
COPY tsconfig.json ./
COPY src ./src
COPY public ./public
COPY config ./config
COPY docker/entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh \
  && useradd -u 10001 -m cmail \
  && chown -R cmail:cmail /app

USER cmail
ENV PATH="/app/node_modules/.bin:${PATH}"
ENV PORT=8787
EXPOSE 8787

ENTRYPOINT ["/entrypoint.sh"]
