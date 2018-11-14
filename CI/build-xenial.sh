#!/bin/bash
set -ex

export PATH="$PATH:$HOME/.cargo/bin"
export LIBOBS_INCLUDE_DIR="/root/obs-studio/libobs"
export LIBOBS_LIB_DIR="/root/obs-studio/build/libobs"
cargo build && cargo test