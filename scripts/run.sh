#!/bin/bash

docker run -p 8000:8000 --name iracing-stats --mount type=volume,src=iracing-stats-dir,target=/iracing-stats-dir iracing-stats