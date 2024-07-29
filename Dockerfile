FROM rust:latest AS rust-builder

WORKDIR /

COPY server/src /iracing-stats/src
COPY server/Cargo.toml /iracing-stats/
COPY server/Cargo.lock /iracing-stats/

WORKDIR /iracing-stats
RUN cargo install --path . --root /app

# -----

FROM node:20.12 AS node-builder

WORKDIR /iracing-stats-site
COPY site .
RUN npm install
RUN npm run build

# -----

FROM debian:bookworm
# no clue what this does, but it doesn't work
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && apt-get install -y ca-certificates curl procps

# TODO it's a bit ugly to copy this to /usr/local
COPY --from=rust-builder /app/bin/iracing-stats /usr/local/bin/iracing-stats
COPY --from=node-builder /iracing-stats-site/dist /iracing-stats-site

COPY server/static-data /iracing-stats-static/static-data

ARG IRACING_USER
ENV IRACING_USER=${IRACING_USER}
ARG IRACING_TOKEN
ENV IRACING_TOKEN=${IRACING_TOKEN}
ARG DISCORD_HOOK_URL
ENV DISCORD_HOOK_URL=${DISCORD_HOOK_URL}

ENV IRACING_STATS_BASE_DIR=/iracing-stats-dir
ENV IRACING_STATS_STATIC_DIR=/iracing-stats-static
ENV IRACING_STATS_SITE_DIR=/iracing-stats-site
ENV IRACING_STATS_LOG_FILE=/iracing-stats-dir/server.log

EXPOSE 8000

CMD echo "Starting server" && ROCKET_LOG_LEVEL=debug ROCKET_ADDRESS=0.0.0.0 iracing-stats --server --enable-https