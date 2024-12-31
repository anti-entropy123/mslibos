$CC func.c -o func.wasm -fno-exceptions -fno-rtti -ffast-math -funroll-loops -fomit-frame-pointer -Ofast
# $CC func.c -o func.wasm
wasmtime compile --target x86_64-unknown-none -W threads=n,tail-call=n func.wasm

cargo build --target x86_64-unknown-none --release && cc \
  -Wl,--gc-sections -nostdlib \
  -Wl,--whole-archive \
  target/x86_64-unknown-none/release/libwasmtime_func.a \
  -Wl,--no-whole-archive \
  -shared \
  -o target/x86_64-unknown-none/release/libwasmtime_func.so


ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_longchain/target/x86_64-unknown-none/release/libwasmtime_func.so /home/wyj/alloy_stack/mslibos/target/release/libwasmtime_func.so



# cargo build --target x86_64-unknown-none && cc \
#   -Wl,--gc-sections -nostdlib \
#   -Wl,--whole-archive \
#   target/x86_64-unknown-none/debug/libwasmtime_func.a \
#   -Wl,--no-whole-archive \
#   -shared \
#   -o target/x86_64-unknown-none/debug/libwasmtime_func.so

# ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_func/target/x86_64-unknown-none/debug/libwasmtime_func.so /home/wyj/alloy_stack/mslibos/target/debug/libwasmtime_func.so
