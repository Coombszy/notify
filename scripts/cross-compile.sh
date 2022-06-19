#!/bin/bash


echo "TARGETPLATFORM: $1"

if [ "$1" = "linux/arm/v7" ] 
then
    rustup target add armv7-unknown-linux-gnueabihf
    cargo build --release --target armv7-unknown-linux-gnueabihf
    exit $?
fi

if [ "$1" = "linux/amd64" ]
then
    cargo build --release
    exit $?
fi

echo "Not supported cross-compile!, Add support in cross-compile.sh"
exit 1