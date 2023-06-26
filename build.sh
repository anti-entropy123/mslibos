cargo build -p msvisor &&\
cargo build -p gen-file &&\
cargo build -p ms_hostcall &&\

cargo build -p ms_std &&\

cargo build -p fdtab &&\
cargo build -p stdio &&\
cargo build -p socket &&\

cargo build -p hello_world &&\
cargo build -p should_panic &&\
cargo build -p simple_http