#!/bin/bash

ssh -i $AWS_PERM_FILE $AWS_NODE << EOF
    docker ps -aq --filter="name=iracing-stats" | xargs --no-run-if-empty docker stop | xargs --no-run-if-empty docker rm
    docker load -i ~/iracing-stats.img
    docker run --name iracing-stats --detach -p 443:8000 --mount type=volume,src=iracing-stats-dir,target=/iracing-stats-dir iracing-stats
EOF