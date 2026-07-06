#!/bin/bash
set -e

cd "$(dirname "$0")/../App"

cargo build --release

mkdir -p ../build

cp -f ./target/release/RustConnect ../build/RustConnect

echo
echo "RustConnect is now in ./build"