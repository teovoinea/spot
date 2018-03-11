cargo install spot-server
git clone https://github.com/teovoinea/spot-client
cd spot-client
./install_cargo_web.sh
cargo-web deploy --release --target=wasm32-unknown-unknown
cd ..
cp spot-client/target/deploy/* static/
cargo build --release

