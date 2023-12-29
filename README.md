

# Commands:
 ```
 // Quick updates
 cargo run --features bevy/dynamic_linking
 ```
 ```
// Web build
cargo build --release --target wasm32-unknown-unknown
// Bridge to js
wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "blob-eater" ./target/wasm32-unknown-unknown/release/blob-eater.wasm
 ```