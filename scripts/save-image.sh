#!/bin/bash

SCRIPT_DIR="$(dirname "$(realpath "$0")")"
docker save -o "${SCRIPT_DIR}/../iracing-stats.img" iracing-stats