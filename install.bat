cargo install spot-server
git clone https://github.com/teovoinea/spot-client
cd spot-client
cargo web deploy --release --target=wasm32-unknown-unknown
cd ..
xcopy /y .\spot-client\target\deploy\* .\static\
cargo build --release