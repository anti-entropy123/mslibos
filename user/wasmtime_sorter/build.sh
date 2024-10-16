$CC sorter.c -o sorter.wasm

wasmtime compile --target x86_64-unknown-none -W threads=n,tail-call=n sorter.wasm

cargo build --target x86_64-unknown-none --release && cc \
  -Wl,--gc-sections -nostdlib \
  -Wl,--whole-archive \
  target/x86_64-unknown-none/release/libwasmtime_sorter.a \
  -Wl,--no-whole-archive \
  -shared \
  -o target/x86_64-unknown-none/release/libwasmtime_sorter.so


# ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_sorter/target/x86_64-unknown-none/debug/libwasmtime_sorter.so /home/wyj/alloy_stack/mslibos/target/debug/libwasmtime_sorter.so
ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_sorter/target/x86_64-unknown-none/release/libwasmtime_sorter.so /home/wyj/alloy_stack/mslibos/target/release/libwasmtime_sorter.so

