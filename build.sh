#!/bin/bash
set -euo pipefail
docker run \
    --volume "$PWD:/home/cross/project" \
    --volume "$HOME/.cargo/registry>:/home/cross/.cargo/registry" \
    rust:2018 \
    build

scp ./target/arm-unknown-linux-gnueabihf/debug/rustpixmas pi@${PI_HOST:-pi}:
