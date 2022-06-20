#!/bin/bash


echo "TARGETPLATFORM: $1"

apt-get update && apt-get upgrade -qq
apt-get install --install-recommends -y perl openssl libssl-dev librust-openssl-dev \
	build-essential ca-certificates

if [ "$1" = "linux/arm/v7" ] 
then
    rm -f target/armv7-unknown-linux-gnueabihf/release/deps/notify-*
    apt-get install --install-recommends -y gcc-arm-linux-gnueabihf pkg-config
    rustup target add armv7-unknown-linux-gnueabihf
    cargo build --release --target armv7-unknown-linux-gnueabihf; code=$?
    mkdir -p /target/release
    cp target/armv7-unknown-linux-gnueabihf/release/notify target/release/
    exit $code
fi

if [ "$1" = "linux/amd64" ]
then
    # Assuming you are running this on a amd64 machine
    rm -f target/release/deps/notify-*
    cargo build --release
    exit $?
fi

echo "Not supported cross-compile!, Add support in cross-compile.sh and update .cargo/config"
exit 1
