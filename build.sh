#!/bin/bash
set -euo pipefail
docker run \
    --volume "$PWD:/home/cross/project" \
    --volume "$HOME/.cargo/registry>:/home/cross/.cargo/registry" \
    ragnaroek/rust-raspberry:1.30.0 \
    build

scp ./target/arm-unknown-linux-gnueabihf/debug/rustpixmas pi@${PI_HOST:-pi}:
