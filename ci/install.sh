#!/usr/bin/env bash

set -ex

export RUST_BACKTRACE=1
cargo install --path cargo-ginit/ --force
