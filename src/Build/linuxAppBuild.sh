
cd "$(dirname "$0")/../App" || exit 1

cargo build --release

mkdir -p ../build

cp -f ./target/release/rustconnect ../build/rustconnect

echo
echo "RustConnect is now in ./build"