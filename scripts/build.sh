#!/bin/bash

docker build --build-arg IRACING_USER --build-arg IRACING_TOKEN --build-arg DISCORD_HOOK_URL -t iracing-stats . "$@"