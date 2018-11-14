@echo off
SETLOCAL EnableDelayedExpansion

curl -sSf -o rustup-init.exe https://win.rustup.rs
rustup-init.exe --default-host x86_64-pc-windows-msvc --default-toolchain stable -y
set PATH=%PATH%;C:\Users\appveyor\.cargo\bin

rustc -Vv
cargo -V