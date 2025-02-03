
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
    for name in time stdio mm; do \
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

profile := if enable_release == "1" { 
    "release"
} else { 
    "debug"
}

symbol_link func_name:
    @ln -s $(pwd)/user/{{ func_name }}/target/{{target}}/{{profile}}/lib{{ func_name }}.so target/{{profile}}/

wasm_func func_name:
    cd user/{{ func_name }} \
        && cargo build {{ release_flag }} \
            --target {{target}} {{mpk_feature_flag}}  \
        && cc {{cc_flags_p1}} \
            target/{{target}}/{{profile}}/lib{{ func_name }}.a \
            {{cc_flags_p2}} \
            -o target/{{target}}/{{profile}}/lib{{ func_name }}.so
    
    @-rm target/{{profile}}/lib{{ func_name }}.so
    just symbol_link {{ func_name }}

c_wordcount: 
    just wasm_func wasmtime_mapper
    just wasm_func wasmtime_reducer

c_parallel_sort:
    just wasm_func wasmtime_sorter
    just wasm_func wasmtime_spliter
    just wasm_func wasmtime_merger
    just wasm_func wasmtime_checker

c_long_chain:
    just wasm_func wasmtime_longchain

all_c_wasm: c_wordcount c_parallel_sort c_long_chain

python_wordcount: 
    just wasm_func wasmtime_cpython_wordcount

python_parallel_sort:
    just wasm_func wasmtime_cpython_parallel_sort

all_py_wasm: python_wordcount python_parallel_sort

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

cold_start_latency:
    just all_libos
    just rust_func hello_world
    just rust_func load_all
    @-./scripts/del_tap.sh 2>/dev/null

    @echo '\ncold start with on-demand loading'
    cargo run {{ release_flag }} -- --files isol_config/base_config.json --metrics total-dur 2>&1 | grep 'ms'
    @echo '\ncold start without on-demand loading'
    cargo run {{ release_flag }} -- --files isol_config/load_all.json --metrics total-dur 2>&1 | grep 'ms'

data_transfer_latency: all_libos
    for data_size in '4*1024' '64*1024' '1024*1024' '16*1024*1024' '256*1024*1024'; do \
        echo $data_size > user/data_size.config; \
        just pass_args 1>/dev/null 2>/dev/null; \
        cargo run {{ release_flag }} -- --files isol_config/pass_complex_args.json 2>&1 | grep 'bytes'; \
        echo ''; \
    done

end_to_end_latency: all_libos map_reduce parallel_sort long_chain all_c_wasm
    -sudo mount fs_image 2>/dev/null
    
    sudo -E ./scripts/gen_data.py 3 '100 * 1024 * 1024' 0 0
    @echo 'word count cost: '
    cargo run {{ release_flag }} -- --files isol_config/map_reduce_large_c3.json --metrics total-dur 2>&1 | grep 'total_dur'

    sudo -E ./scripts/gen_data.py 0 0 3 '25 * 1024 * 1024'
    @echo 'parallel sorting cost: '
    cargo run {{ release_flag }} -- --files isol_config/parallel_sort_c3.json --metrics total-dur 2>&1 | grep 'total_dur'

    @echo 'function chain cost: '
    cargo run {{ release_flag }} -- --files isol_config/long_chain_n15.json --metrics total-dur 2>&1 | grep 'total_dur'

    # C applications.
    @echo 'C word count cost: '
    cargo run {{ release_flag }} -- --files isol_config/wasmtime_wordcount_c3.json --metrics total-dur 2>&1 | grep 'total_dur'

    @echo 'C parallel sorting cost: '
    cargo run {{ release_flag }} -- --files isol_config/wasmtime_parallel_sort_c3.json --metrics total-dur 2>&1  | grep 'total_dur'

    @echo 'C function chain cost: '
    cargo run {{ release_flag }} -- --files isol_config/wasmtime_longchain.json --metrics total-dur 2>&1 | grep 'total_dur'

breakdown: all_libos map_reduce parallel_sort long_chain
    @echo 'base'
    