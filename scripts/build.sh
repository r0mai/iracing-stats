#!/bin/bash

docker build --build-arg IRACING_USER --build-arg IRACING_TOKEN -t iracing-stats . "$@"