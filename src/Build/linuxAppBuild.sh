#!/bin/bash
set -e

cd "$(dirname "$0")/../App"

rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

mkdir -p ../build

cp -f ./target/x86_64-unknown-linux-musl/release/RustConnect

echo
echo "RustConnect is now in ./build"