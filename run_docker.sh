#!/bin/bash

git pull
docker stop rssserver
docker rm rssserver
docker build -t rssserver .
docker run -d \
    --name=rssserver \
    -p 3030:3030 \
    --restart=unless-stopped \
    rssserver
