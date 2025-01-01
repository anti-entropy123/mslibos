cargo build --target x86_64-unknown-none --release && cc \
  -Wl,--gc-sections -nostdlib \
  -Wl,--whole-archive \
  target/x86_64-unknown-none/release/libwasmtime_mapper.a \
  -Wl,--no-whole-archive \
  -shared \
  -o target/x86_64-unknown-none/release/libwasmtime_mapper.so

cargo run -- --files ./isol_config/wasmtime_wordcount.json

ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_mapper/target/x86_64-unknown-none/debug/libwasmtime_mapper.so target/debug/libwasmtime_mapper.so
ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_mapper/target/x86_64-unknown-none/release/libwasmtime_mapper.so target/release/libwasmtime_mapper.so

wasmtime compile --target x86_64-unknown-none -W threads=n,tail-call=n mapper.wasm




cargo build --target x86_64-unknown-none --release && cc \
  -Wl,--gc-sections -nostdlib \
  -Wl,--whole-archive \
  target/x86_64-unknown-none/release/libwasmtime_reducer.a \
  -Wl,--no-whole-archive \
  -shared \
  -o target/x86_64-unknown-none/release/libwasmtime_reducer.so

cargo run -- --files ./isol_config/wasmtime_wordcount.json

ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_reducer/target/x86_64-unknown-none/debug/libwasmtime_reducer.so target/debug/libwasmtime_reducer.so
ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_reducer/target/x86_64-unknown-none/release/libwasmtime_reducer.so target/release/libwasmtime_reducer.so

wasmtime compile --target x86_64-unknown-none -W threads=n,tail-call=n reducer.wasm




cargo build --target x86_64-unknown-none && cc \
  -Wl,--gc-sections -nostdlib \
  -Wl,--whole-archive \
  target/x86_64-unknown-none/debug/libwasmtime_func.a \
  -Wl,--no-whole-archive \
  -shared \
  -o target/x86_64-unknown-none/debug/libwasmtime_func.so

cargo run -- --files ./isol_config/wasmtime_longchain.json

ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_longchain/target/x86_64-unknown-none/debug/libwasmtime_func.so target/debug/libwasmtime_func.so

wasmtime compile --target x86_64-unknown-none -W threads=n func.wasm
