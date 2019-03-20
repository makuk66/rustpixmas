#!/bin/bash

cd "$( dirname "${BASH_SOURCE[0]}" )" || exit 1

cp ../target/arm-unknown-linux-gnueabihf/debug/rustpixmas .
docker build -t makuk66/rustpixmas:latest .
docker push makuk66/rustpixmas:latest
