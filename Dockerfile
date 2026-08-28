# syntax=docker/dockerfile:1

FROM rust:1-bookworm AS build
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown \
  && cargo install worker-build --locked --version 0.8.5
COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY rust-toolchain.toml ./
COPY build.rs ./
COPY plugins ./plugins
COPY src ./src
COPY config ./config
COPY templates ./templates
COPY public ./public
RUN worker-build --release --no-panic-recovery

FROM node:22-bookworm-slim
WORKDIR /app
RUN npm install -g wrangler@4.125.0 && npm cache clean --force
COPY --from=build /app/build ./build
COPY wrangler.docker.jsonc ./wrangler.jsonc
COPY config ./config
COPY public ./public
COPY --from=build /app/public/plugins ./public/plugins
COPY docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh \
  && useradd -u 10001 -m cmail \
  && chown -R cmail:cmail /app
USER cmail
ENV PORT=8787
EXPOSE 8787
ENTRYPOINT ["/entrypoint.sh"]
