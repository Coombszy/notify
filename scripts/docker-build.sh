#!/bin/bash

cd $(git rev-parse --show-toplevel)
# Ensure armv7 support is updated
rustup target add armv7-unknown-linux-gnueabihf
rustup target add aarch64-apple-ios-sim

# Create builder
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
docker buildx rm builder || true
docker buildx create --name builder --driver docker-container --use
docker buildx use builder

# Run build
docker buildx build --push \
-f docker/Dockerfile --platform linux/amd64,linux/arm/v7 --tag coombszy/notify:dev .
