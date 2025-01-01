$CPP mapper.cpp -o mapper.wasm -fno-exceptions -fno-rtti -ffast-math -funroll-loops -fomit-frame-pointer -Ofast
# $CC mapper.c -o mapper.wasm -O3

wasmtime compile --target x86_64-unknown-none -W threads=n,tail-call=n mapper.wasm

cargo build --target x86_64-unknown-none --release && cc \
  -Wl,--gc-sections -nostdlib \
  -Wl,--whole-archive \
  target/x86_64-unknown-none/release/libwasmtime_mapper.a \
  -Wl,--no-whole-archive \
  -shared \
  -o target/x86_64-unknown-none/release/libwasmtime_mapper.so

ln -s /home/wyj/alloy_stack/mslibos/user/wasmtime_mapper/target/x86_64-unknown-none/release/libwasmtime_mapper.so /home/wyj/alloy_stack/mslibos/target/release/libwasmtime_mapper.so

