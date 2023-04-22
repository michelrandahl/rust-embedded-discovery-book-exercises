#!/usr/bin/env bash
cargo readobj --features v2 --target thumbv7em-none-eabihf --bin led-roulette -- --file-headers
