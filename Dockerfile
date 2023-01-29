FROM rust:latest AS builder

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

FROM debian:bullseye-slim
# no clue what this does, but it doesn't work
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && apt-get install -y ca-certificates curl

# TODO it's a bit ugly to copy this to /usr/local
COPY --from=builder /app/bin/iracing-stats /usr/local/bin/iracing-stats
COPY static /static

ARG IRACING_USER
ENV IRACING_USER=${IRACING_USER}
ARG IRACING_TOKEN
ENV IRACING_TOKEN=${IRACING_TOKEN}

ENV IRACING_STATS_BASE_DIR=/iracing-stats-dir

EXPOSE 8000

CMD echo "Starting server" && ROCKET_ADDRESS=0.0.0.0 iracing-stats --server