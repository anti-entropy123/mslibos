$CC spliter.c -o spliter.wasm

wasmtime compile --target x86_64-unknown-none -W threads=n,tail-call=n spliter.wasm

cargo build --target x86_64-unknown-none --release && cc \
  -Wl,--gc-sections -nostdlib \
  -Wl,--whole-archive \
  target/x86_64-unknown-none/release/libwasmtime_spliter.a \
  -Wl,--no-whole-archive \
  -shared \
  -o target/x86_64-unknown-none/release/libwasmtime_spliter.so


ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_spliter/target/x86_64-unknown-none/release/libwasmtime_spliter.so /home/wyj/alloy_stack/mslibos/target/release/libwasmtime_spliter.so

