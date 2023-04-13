FROM rust:latest AS rust-builder

WORKDIR /

# https://stackoverflow.com/questions/23391839/clone-private-git-repo-with-dockerfile
# ADD repo-key /
# RUN \
#     chmod 600 /repo-key && \
#     echo "IdentityFile /repo-key" >> /etc/ssh/ssh_config && \  
#     echo "StrictHostKeyChecking no" >> /etc/ssh/ssh_config && \ 
#     git clone git@github.com:r0mai/iracing-stats.git

COPY src /iracing-stats/src
COPY Cargo.toml /iracing-stats/
COPY Cargo.lock /iracing-stats/

WORKDIR /iracing-stats
RUN cargo install --path . --root /app

# -----

FROM node:18.4 AS node-builder

WORKDIR /iracing-stats-site
COPY site .
RUN npm install
RUN npm run build

# -----

FROM debian:bullseye-slim
# no clue what this does, but it doesn't work
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && apt-get install -y ca-certificates curl procps

# TODO it's a bit ugly to copy this to /usr/local
COPY --from=rust-builder /app/bin/iracing-stats /usr/local/bin/iracing-stats
COPY --from=node-builder /iracing-stats-site/build /iracing-stats-site

# no longer needed:
COPY static /static 

COPY static-data /iracing-stats-static/static-data

ARG IRACING_USER
ENV IRACING_USER=${IRACING_USER}
ARG IRACING_TOKEN
ENV IRACING_TOKEN=${IRACING_TOKEN}

ENV IRACING_STATS_BASE_DIR=/iracing-stats-dir
ENV IRACING_STATS_STATIC_DIR=/iracing-stats-static
ENV IRACING_STATS_SITE_DIR=/iracing-stats-site
ENV IRACING_STATS_LOG_FILE=/iracing-stats-dir/server.log

EXPOSE 8000

CMD echo "Starting server" && ROCKET_LOG_LEVEL=normal ROCKET_ADDRESS=0.0.0.0 iracing-stats --server