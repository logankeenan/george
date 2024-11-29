#!/bin/bash

IMAGE_NAME="george-daemon"
CONTAINER_NAME="george-daemon-temp"
BINARY_NAME="george-daemon"

cd ./george-daemon

docker build -t "$IMAGE_NAME" .

docker create --name "$CONTAINER_NAME" "$IMAGE_NAME"

docker cp "$CONTAINER_NAME:/usr/src/app/target/release/$BINARY_NAME" ../george-ai

docker rm "$CONTAINER_NAME"