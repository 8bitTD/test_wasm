rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name test_wasm --out-dir wasm/target --target web ./target/wasm32-unknown-unknown/release/test_wasm.wasm
cargo install basic-http-server
basic-http-server -a "127.0.0.1:4000" ./
