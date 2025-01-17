
set positional-arguments

enable_mpk := "0"
enable_pkey_per_func := "0"
enable_file_buffer := "0"

enable_release := "1"

cmd_flag := if enable_mpk == "1" {
    if enable_pkey_per_func == "1" { 
        "pkey_per_func" 
    } else { 
        "mpk"
    } 
} else { "" }

feature_flag := if enable_mpk == "1" {
    if enable_pkey_per_func == "1" { 
        "--features pkey_per_func" 
    } else { 
        "--features mpk"
    } 
} else { "" }

release_flag := if enable_release == "1" { 
    "--release" 
} else { "" }

rust_func func_name:
    cargo build {{ release_flag }} {{ feature_flag }} {{ if enable_file_buffer == "1" { "--features file-based" } else { "" } }} --manifest-path user/{{ func_name }}/Cargo.toml

libos lib_name:
    cargo build {{ release_flag }} {{ if enable_mpk == "1" { "--features mpk" } else { "" } }} --manifest-path common_service/{{ lib_name }}/Cargo.toml

pass_args:
    just rust_func func_a
    just rust_func func_b

map_reduce:
    for name in time fdtab fatfs stdio mm; do \
    just libos $name; \
    done

    just rust_func mapper
    just rust_func reducer

parallel_sort:
    for name in time fdtab fatfs stdio mm; do \
    just libos $name; \
    done

    for func in file_reader sorter splitter merger; do \
    just rust_func $func; \
    done

long_chain:
    for name in time fdtab stdio mm; do \
    just libos $name; \
    done

    just rust_func array_sum
    

all_libos:
    ./scripts/build_all_common{{ if enable_mpk == "1" { "_mpk" } else { "" } }}.sh {{ release_flag }}

all_rust:
    just all_libos
    ./scripts/build_user.sh {{ release_flag }} {{ cmd_flag }}

run_rust_test:
    just all_libos
    just all_rust
    ./scripts/run_tests.sh {{ cmd_flag }}

cc_flags_p1 := "-Wl,--gc-sections -nostdlib -Wl,--whole-archive"
cc_flags_p2 := "-Wl,--no-whole-archive -shared"
target := "x86_64-unknown-none"

c_mapper_so:
    @echo "c_mapper.so"
    cd user/wasmtime_mapper \
        && cargo build \
            --target {{target}} {{feature_flag}}  \
        && cc {{cc_flags_p1}} \
            target/{{target}}/debug/libwasmtime_mapper.a \
            {{cc_flags_p2}} \
            -o target/{{target}}/debug/libwasmtime_mapper.so

c_reducer_so:
    @echo "c_reducer.so"
    cd user/wasmtime_reducer \
        && cargo build --target {{target}} {{feature_flag}} \
        && cc {{cc_flags_p1}} \
            target/{{target}}/debug/libwasmtime_reducer.a \
            {{cc_flags_p2}} \
            -o target/{{target}}/debug/libwasmtime_reducer.so

wasmtime_wordcount: c_mapper_so c_reducer_so
    @echo "make symbol link: wasmtime_wordcount"
    @-rm target/debug/libwasmtime_mapper.so
    @-rm target/debug/libwasmtime_reducer.so
    @ln -s $(pwd)/user/wasmtime_mapper/target/{{target}}/debug/libwasmtime_mapper.so target/debug/
    @ln -s $(pwd)/user/wasmtime_reducer/target/{{target}}/debug/libwasmtime_reducer.so target/debug/

c_sorter_so:
    @echo "c_sorter.so"
    cd user/wasmtime_sorter \
        && cargo build --target {{target}} {{feature_flag}}\
        && cc {{cc_flags_p1}} \
            target/{{target}}/debug/libwasmtime_sorter.a \
            {{cc_flags_p2}} \
            -o target/{{target}}/debug/libwasmtime_sorter.so

c_spliter_so:
    @echo "c_spliter.so"
    cd user/wasmtime_spliter \
        && cargo build --target {{target}} {{feature_flag}}\
        && cc {{cc_flags_p1}} \
            target/{{target}}/debug/libwasmtime_spliter.a \
            {{cc_flags_p2}} \
            -o target/{{target}}/debug/libwasmtime_spliter.so

c_merger_so:
    @echo "c_merger.so"
    cd user/wasmtime_merger \
        && cargo build --target {{target}} {{feature_flag}}\
        && cc {{cc_flags_p1}} \
            target/{{target}}/debug/libwasmtime_merger.a \
            {{cc_flags_p2}} \
            -o target/{{target}}/debug/libwasmtime_merger.so

c_checker_so:
    @echo "c_checker.so"
    cd user/wasmtime_checker \
        && cargo build --target {{target}} {{feature_flag}}\
        && cc {{cc_flags_p1}} \
            target/{{target}}/debug/libwasmtime_checker.a \
            {{cc_flags_p2}} \
            -o target/{{target}}/debug/libwasmtime_checker.so

wasmtime_parallel_sort: c_sorter_so c_spliter_so c_merger_so c_checker_so
    @echo "make symbol link: wasmtime_parallel_sort"
    @-rm target/debug/libwasmtime_sorter.so
    @-rm target/debug/libwasmtime_spliter.so
    @-rm target/debug/libwasmtime_merger.so
    @-rm target/debug/libwasmtime_checker.so
    @ln -s $(pwd)/user/wasmtime_sorter/target/{{target}}/debug/libwasmtime_sorter.so target/debug/
    @ln -s $(pwd)/user/wasmtime_spliter/target/{{target}}/debug/libwasmtime_spliter.so target/debug/
    @ln -s $(pwd)/user/wasmtime_merger/target/{{target}}/debug/libwasmtime_merger.so target/debug/
    @ln -s $(pwd)/user/wasmtime_checker/target/{{target}}/debug/libwasmtime_checker.so target/debug/


all_c_wasm: wasmtime_wordcount wasmtime_parallel_sort

cpython_wordcount_so:
    @echo "cpython_wordcount.so"
    cd user/wasmtime_cpython_wordcount \
        && cargo build --target {{target}} {{feature_flag}}\
        && cc {{cc_flags_p1}} \
            target/{{target}}/debug/libwasmtime_cpython_wordcount.a \
            {{cc_flags_p2}} \
            -o target/{{target}}/debug/libwasmtime_cpython_wordcount.so

cpython_wordcount: cpython_wordcount_so
    @echo "make symbol link: cpython_wordcount"
    @-rm target/debug/libwasmtime_cpython_wordcount.so
    @ln -s $(pwd)/user/wasmtime_cpython_wordcount/target/x86_64-unknown-none/debug/libwasmtime_cpython_wordcount.so target/debug

cpython_parallel_sort_so:
    @echo "cpython_parallel_sort.so"
    cd user/wasmtime_cpython_parallel_sort \
        && cargo build --target {{target}} {{feature_flag}}\
        && cc {{cc_flags_p1}} \
            target/{{target}}/debug/libwasmtime_cpython_parallel_sort.a \
            {{cc_flags_p2}} \
            -o target/{{target}}/debug/libwasmtime_cpython_parallel_sort.so

cpython_parallel_sort: cpython_parallel_sort_so
    @echo "make symbol link: cpython_parallel_sort"
    @-rm target/debug/libwasmtime_cpython_parallel_sort.so
    @ln -s $(pwd)/user/wasmtime_cpython_parallel_sort/target/{{target}}/debug/libwasmtime_cpython_parallel_sort.so target/debug/

all_py_wasm: cpython_wordcount cpython_parallel_sort

all_wasm: all_c_wasm all_py_wasm

measure_avg isol_file:
    #!/bin/bash
    for (( i=1; i<=10; i++ )); do \
        output=$(cargo run {{ release_flag }} {{feature_flag}} -- --files {{ isol_file }} --metrics total-dur 2>&1); \
        total_dur=$(echo "$output" | grep -o '"total_dur(ms)": [0-9.]*' | awk -F': ' '{print $2}'); \
        total_dur_rounded=$(printf "%.3f\n" "$total_dur") ;\
        echo "$total_dur_rounded ," ;\
    done ;