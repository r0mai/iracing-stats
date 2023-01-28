#!/bin/bash

SCRIPT_DIR="$(dirname "$(realpath "$0")")"
scp -i $AWS_PERM_FILE "${SCRIPT_DIR}/../iracing-stats.img"  ${AWS_NODE}:~