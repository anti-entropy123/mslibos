
set positional-arguments

enable_mpk := "0"
enable_pkey_per_func := "0"
enable_file_buffer := "0"

enable_release := "1"

mpk_flag := if enable_mpk == "1" {
    if enable_pkey_per_func == "1" { 
        "pkey_per_func" 
    } else { 
        "mpk"
    } 
} else { "" }

mpk_feature_flag := if mpk_flag == "" { "" } else { "--features " + mpk_flag }

buffer_feature_flag := if enable_file_buffer == "1" { "--features file-based" } else { "" }

release_flag := if enable_release == "1" { 
    "--release" 
} else { "" }

rust_func func_name:
    cargo build {{ release_flag }} {{ mpk_feature_flag }} {{ buffer_feature_flag }} \
        --manifest-path user/{{ func_name }}/Cargo.toml

libos lib_name:
    cargo build {{ release_flag }} {{ if enable_mpk == "1" { "--features mpk" } else { "" } }} \
        --manifest-path common_service/{{ lib_name }}/Cargo.toml

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

simple_file:
    just rust_func simple_file

all_libos:
    ./scripts/build_all_common{{ if enable_mpk == "1" { "_mpk" } else { "" } }}.sh {{ release_flag }}

all_rust:
    just all_libos
    ./scripts/build_user.sh {{ release_flag }} {{ mpk_flag }}

run_rust_test:
    just all_libos
    just all_rust
    ./scripts/run_tests.sh {{ mpk_flag }}

cc_flags_p1 := "-Wl,--gc-sections -nostdlib -Wl,--whole-archive"
cc_flags_p2 := "-Wl,--no-whole-archive -shared"
target := "x86_64-unknown-none"

wasm_func func_name:
    cd user/{{ func_name }} \
        && cargo build \
            --target {{target}} {{mpk_feature_flag}}  \
        && cc {{cc_flags_p1}} \
            target/{{target}}/debug/lib{{ func_name }}.a \
            {{cc_flags_p2}} \
            -o target/{{target}}/debug/lib{{ func_name }}.so

symbol_link func_name:
    @ln -s $(pwd)/user/{{ func_name }}/target/{{target}}/debug/lib{{ func_name }}.so target/debug/


wasmtime_wordcount: 
    just wasm_func wasmtime_mapper
    just wasm_func wasmtime_reducer
    @-rm target/debug/libwasmtime_mapper.so
    @-rm target/debug/libwasmtime_reducer.so
    just symbol_link wasmtime_mapper
    just symbol_link wasmtime_reducer

wasmtime_parallel_sort:
    just wasm_func wasmtime_sorter
    just wasm_func wasmtime_spliter
    just wasm_func wasmtime_merger
    just wasm_func wasmtime_checker

    @-rm target/debug/libwasmtime_sorter.so
    @-rm target/debug/libwasmtime_spliter.so
    @-rm target/debug/libwasmtime_merger.so
    @-rm target/debug/libwasmtime_checker.so
    
    just symbol_link wasmtime_sorter
    just symbol_link wasmtime_spliter
    just symbol_link wasmtime_merger
    just symbol_link wasmtime_checker
    
all_c_wasm: wasmtime_wordcount wasmtime_parallel_sort

cpython_wordcount: 
    just wasm_func wasmtime_cpython_wordcount
    @-rm target/debug/libwasmtime_cpython_wordcount.so
    just symbol_link wasmtime_cpython_wordcount

cpython_parallel_sort:
    just wasm_func wasmtime_cpython_parallel_sort
    @-rm target/debug/libwasmtime_cpython_parallel_sort.so
    just symbol_link wasmtime_cpython_parallel_sort

all_py_wasm: cpython_wordcount cpython_parallel_sort

all_wasm: all_c_wasm all_py_wasm

measure_avg isol_file:
    #!/bin/bash
    for (( i=1; i<=10; i++ )); do \
        output=$(cargo run {{ release_flag }} {{mpk_feature_flag}} -- --files {{ isol_file }} --metrics total-dur 2>&1); \
        total_dur=$(echo "$output" | grep -o '"total_dur(ms)": [0-9.]*' | awk -F': ' '{print $2}'); \
        total_dur_rounded=$(printf "%.3f\n" "$total_dur") ;\
        echo "$total_dur_rounded ," ;\
    done ;

gen_data:
    sudo -E ./scripts/gen_data.py
