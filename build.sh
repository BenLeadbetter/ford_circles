cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/ford_circles_rs.wasm --out-dir ./wasm --target web
