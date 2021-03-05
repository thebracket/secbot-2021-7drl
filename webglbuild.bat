@ECHO OFF
cargo build --target wasm32-unknown-unknown --release

wasm-bindgen .\target\wasm32-unknown-unknown\release\secbot.wasm --out-dir .\wasm_help\staging --no-modules --no-typescript
copy .\wasm_help\index.html .\wasm_help\staging\index.html

REM Send to server. Not included on Github so I'm not giving you server details. Sorry.
./webglbuild2.bat
