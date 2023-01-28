FROM rust:latest AS builder

WORKDIR /

# https://stackoverflow.com/questions/23391839/clone-private-git-repo-with-dockerfile
ADD repo-key /
RUN \
    chmod 600 /repo-key && \
    echo "IdentityFile /repo-key" >> /etc/ssh/ssh_config && \  
    echo "StrictHostKeyChecking no" >> /etc/ssh/ssh_config && \ 
    git clone git@github.com:r0mai/iracing-stats.git

WORKDIR /iracing-stats
RUN cargo install --path . --root /app

# -----

FROM debian:bullseye-slim
# no clue what this does, but it doesn't work
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/bin/iracing-stats /usr/local/bin/iracing-stats

CMD iracing-stats --help