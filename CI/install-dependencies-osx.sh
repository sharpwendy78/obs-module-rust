#!/usr/bin/env bash

# Exit if something fails
set -e
# Echo all commands before executing
set -v

cd ../

brew update
# OBS Studio + obs-websocket deps
brew install ffmpeg libav llvm
brew install ./obs-module-rust/CI/macos/qt.rb

# Build OBS Studio
git clone --recursive https://github.com/obsproject/obs-studio.git
cd ./obs-studio
git checkout 22.0.3
mkdir build && cd build
cmake .. \
    -DCMAKE_PREFIX_PATH=/usr/local/opt/qt/lib/cmake \
&& make -j4
