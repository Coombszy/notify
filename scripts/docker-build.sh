#!/bin/bash

cd ../
# Ensure armv7 support is updated
rustup target add armv7-unknown-linux-gnueabihf

# Create builder
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
docker buildx rm builder || true
docker buildx create --name builder --driver docker-container --use
docker buildx use builder

# Run build
