$CC merger.c -o merger.wasm

wasmtime compile --target x86_64-unknown-none -W threads=n,tail-call=n merger.wasm

cargo build --target x86_64-unknown-none --release && cc \
  -Wl,--gc-sections -nostdlib \
  -Wl,--whole-archive \
  target/x86_64-unknown-none/release/libwasmtime_merger.a \
  -Wl,--no-whole-archive \
  -shared \
  -o target/x86_64-unknown-none/release/libwasmtime_merger.so

ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_merger/target/x86_64-unknown-none/release/libwasmtime_merger.so /home/wyj/alloy_stack/mslibos/target/release/libwasmtime_merger.so

