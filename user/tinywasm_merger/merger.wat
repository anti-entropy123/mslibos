(module $merger.wasm
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i64 i32) (result i64)))
  (type (;2;) (func (param i32 i32 i32 i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32 i64 i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32)))
  (type (;8;) (func))
  (type (;9;) (func (param i32 i32 i32)))
  (type (;10;) (func (param i32 i32)))
  (type (;11;) (func (result i32)))
  (type (;12;) (func (param f64 i32) (result f64)))
  (type (;13;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;14;) (func (param i32 i32 i32 i32 i32)))
  (type (;15;) (func (param i32 i64)))
  (type (;16;) (func (param i32 i32 i32 i64) (result i64)))
  (type (;17;) (func (param f64 f64) (result f64)))
  (type (;18;) (func (param i32 i32 i32) (result f64)))
  (type (;19;) (func (param i32 i32 i32 i32 i32) (result f64)))
  (type (;20;) (func (param i32 i32) (result i64)))
  (type (;21;) (func (param i32 i64 i64 i64 i64)))
  (import "env" "access_buffer" (func $access_buffer (type 2)))
  (import "wasi_snapshot_preview1" "args_get" (func $__imported_wasi_snapshot_preview1_args_get (type 3)))
  (import "wasi_snapshot_preview1" "args_sizes_get" (func $__imported_wasi_snapshot_preview1_args_sizes_get (type 3)))
  (import "wasi_snapshot_preview1" "fd_close" (func $__imported_wasi_snapshot_preview1_fd_close (type 4)))
  (import "wasi_snapshot_preview1" "fd_fdstat_get" (func $__imported_wasi_snapshot_preview1_fd_fdstat_get (type 3)))
  (import "wasi_snapshot_preview1" "fd_seek" (func $__imported_wasi_snapshot_preview1_fd_seek (type 5)))
  (import "wasi_snapshot_preview1" "fd_write" (func $__imported_wasi_snapshot_preview1_fd_write (type 6)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (type 7)))
  (func $__wasm_call_ctors (type 8))
  (func $_start (type 8)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        global.get $GOT.data.internal.__memory_base
        i32.const 4224
        i32.add
        i32.load
        br_if 0 (;@2;)
        global.get $GOT.data.internal.__memory_base
        i32.const 4224
        i32.add
        i32.const 1
        i32.store
        call $__wasm_call_ctors
        call $__main_void
        local.set 0
        call $__wasm_call_dtors
        local.get 0
        br_if 1 (;@1;)
        return
      end
      unreachable
      unreachable
    end
    local.get 0
    call $__wasi_proc_exit
    unreachable)
  (func $heapifyDown (type 9) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 3
    i32.const 48
    local.set 4
    local.get 3
    local.get 4
    i32.sub
    local.set 5
    local.get 5
    global.set $__stack_pointer
    local.get 5
    local.get 0
    i32.store offset=44
    local.get 5
    local.get 1
    i32.store offset=40
    local.get 5
    local.get 2
    i32.store offset=36
    local.get 5
    i32.load offset=36
    local.set 6
    local.get 5
    local.get 6
    i32.store offset=32
    local.get 5
    i32.load offset=36
    local.set 7
    i32.const 1
    local.set 8
    local.get 7
    local.get 8
    i32.shl
    local.set 9
    i32.const 1
    local.set 10
    local.get 9
    local.get 10
    i32.add
    local.set 11
    local.get 5
    local.get 11
    i32.store offset=28
    local.get 5
    i32.load offset=36
    local.set 12
    i32.const 1
    local.set 13
    local.get 12
    local.get 13
    i32.shl
    local.set 14
    i32.const 2
    local.set 15
    local.get 14
    local.get 15
    i32.add
    local.set 16
    local.get 5
    local.get 16
    i32.store offset=24
    local.get 5
    i32.load offset=28
    local.set 17
    local.get 5
    i32.load offset=40
    local.set 18
    local.get 17
    local.set 19
    local.get 18
    local.set 20
    local.get 19
    local.get 20
    i32.lt_s
    local.set 21
    i32.const 1
    local.set 22
    local.get 21
    local.get 22
    i32.and
    local.set 23
    block  ;; label = @1
      local.get 23
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=44
      local.set 24
      local.get 5
      i32.load offset=28
      local.set 25
      i32.const 12
      local.set 26
      local.get 25
      local.get 26
      i32.mul
      local.set 27
      local.get 24
      local.get 27
      i32.add
      local.set 28
      local.get 28
      i32.load
      local.set 29
      local.get 5
      i32.load offset=44
      local.set 30
      local.get 5
      i32.load offset=32
      local.set 31
      i32.const 12
      local.set 32
      local.get 31
      local.get 32
      i32.mul
      local.set 33
      local.get 30
      local.get 33
      i32.add
      local.set 34
      local.get 34
      i32.load
      local.set 35
      local.get 29
      local.set 36
      local.get 35
      local.set 37
      local.get 36
      local.get 37
      i32.lt_s
      local.set 38
      i32.const 1
      local.set 39
      local.get 38
      local.get 39
      i32.and
      local.set 40
      local.get 40
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=28
      local.set 41
      local.get 5
      local.get 41
      i32.store offset=32
    end
    local.get 5
    i32.load offset=24
    local.set 42
    local.get 5
    i32.load offset=40
    local.set 43
    local.get 42
    local.set 44
    local.get 43
    local.set 45
    local.get 44
    local.get 45
    i32.lt_s
    local.set 46
    i32.const 1
    local.set 47
    local.get 46
    local.get 47
    i32.and
    local.set 48
    block  ;; label = @1
      local.get 48
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=44
      local.set 49
      local.get 5
      i32.load offset=24
      local.set 50
      i32.const 12
      local.set 51
      local.get 50
      local.get 51
      i32.mul
      local.set 52
      local.get 49
      local.get 52
      i32.add
      local.set 53
      local.get 53
      i32.load
      local.set 54
      local.get 5
      i32.load offset=44
      local.set 55
      local.get 5
      i32.load offset=32
      local.set 56
      i32.const 12
      local.set 57
      local.get 56
      local.get 57
      i32.mul
      local.set 58
      local.get 55
      local.get 58
      i32.add
      local.set 59
      local.get 59
      i32.load
      local.set 60
      local.get 54
      local.set 61
      local.get 60
      local.set 62
      local.get 61
      local.get 62
      i32.lt_s
      local.set 63
      i32.const 1
      local.set 64
      local.get 63
      local.get 64
      i32.and
      local.set 65
      local.get 65
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=24
      local.set 66
      local.get 5
      local.get 66
      i32.store offset=32
    end
    local.get 5
    i32.load offset=32
    local.set 67
    local.get 5
    i32.load offset=36
    local.set 68
    local.get 67
    local.set 69
    local.get 68
    local.set 70
    local.get 69
    local.get 70
    i32.ne
    local.set 71
    i32.const 1
    local.set 72
    local.get 71
    local.get 72
    i32.and
    local.set 73
    block  ;; label = @1
      local.get 73
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=44
      local.set 74
      local.get 5
      i32.load offset=36
      local.set 75
      i32.const 12
      local.set 76
      local.get 75
      local.get 76
      i32.mul
      local.set 77
      local.get 74
      local.get 77
      i32.add
      local.set 78
      i32.const 8
      local.set 79
      local.get 78
      local.get 79
      i32.add
      local.set 80
      local.get 80
      i32.load
      local.set 81
      i32.const 8
      local.set 82
      local.get 5
      local.get 82
      i32.add
      local.set 83
      local.get 83
      local.get 79
      i32.add
      local.set 84
      local.get 84
      local.get 81
      i32.store
      local.get 78
      i64.load align=4
      local.set 85
      local.get 5
      local.get 85
      i64.store offset=8
      local.get 5
      i32.load offset=44
      local.set 86
      local.get 5
      i32.load offset=36
      local.set 87
      i32.const 12
      local.set 88
      local.get 87
      local.get 88
      i32.mul
      local.set 89
      local.get 86
      local.get 89
      i32.add
      local.set 90
      local.get 5
      i32.load offset=44
      local.set 91
      local.get 5
      i32.load offset=32
      local.set 92
      i32.const 12
      local.set 93
      local.get 92
      local.get 93
      i32.mul
      local.set 94
      local.get 91
      local.get 94
      i32.add
      local.set 95
      local.get 95
      i64.load align=4
      local.set 96
      local.get 90
      local.get 96
      i64.store align=4
      i32.const 8
      local.set 97
      local.get 90
      local.get 97
      i32.add
      local.set 98
      local.get 95
      local.get 97
      i32.add
      local.set 99
      local.get 99
      i32.load
      local.set 100
      local.get 98
      local.get 100
      i32.store
      local.get 5
      i32.load offset=44
      local.set 101
      local.get 5
      i32.load offset=32
      local.set 102
      i32.const 12
      local.set 103
      local.get 102
      local.get 103
      i32.mul
      local.set 104
      local.get 101
      local.get 104
      i32.add
      local.set 105
      local.get 5
      i64.load offset=8 align=4
      local.set 106
      local.get 105
      local.get 106
      i64.store align=4
      i32.const 8
      local.set 107
      local.get 105
      local.get 107
      i32.add
      local.set 108
      i32.const 8
      local.set 109
      local.get 5
      local.get 109
      i32.add
      local.set 110
      local.get 110
      local.get 107
      i32.add
      local.set 111
      local.get 111
      i32.load
      local.set 112
      local.get 108
      local.get 112
      i32.store
      local.get 5
      i32.load offset=44
      local.set 113
      local.get 5
      i32.load offset=40
      local.set 114
      local.get 5
      i32.load offset=32
      local.set 115
      local.get 113
      local.get 114
      local.get 115
      call $heapifyDown
    end
    i32.const 48
    local.set 116
    local.get 5
    local.get 116
    i32.add
    local.set 117
    local.get 117
    global.set $__stack_pointer
    return)
  (func $main (type 3) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 2
    i32.const 592
    local.set 3
    local.get 2
    local.get 3
    i32.sub
    local.set 4
    local.get 4
    local.set 5
    local.get 4
    global.set $__stack_pointer
    i32.const 0
    local.set 6
    local.get 5
    local.get 6
    i32.store offset=588
    local.get 5
    local.get 0
    i32.store offset=584
    local.get 5
    local.get 1
    i32.store offset=580
    local.get 5
    i32.load offset=580
    local.set 7
    local.get 7
    i32.load offset=4
    local.set 8
    local.get 8
    call $atoi
    local.set 9
    local.get 5
    local.get 9
    i32.store offset=576
    local.get 5
    i32.load offset=580
    local.set 10
    local.get 10
    i32.load offset=8
    local.set 11
    local.get 11
    call $atoi
    local.set 12
    local.get 5
    local.get 12
    i32.store offset=572
    local.get 5
    i32.load offset=580
    local.set 13
    local.get 13
    i32.load offset=12
    local.set 14
    local.get 14
    call $atoi
    local.set 15
    local.get 5
    local.get 15
    i32.store offset=568
    local.get 5
    i32.load offset=576
    local.set 16
    local.get 5
    local.get 16
    i32.store offset=48
    i32.const 1235
    local.set 17
    i32.const 48
    local.set 18
    local.get 5
    local.get 18
    i32.add
    local.set 19
    local.get 17
    local.get 19
    call $printf
    drop
    local.get 5
    i32.load offset=572
    local.set 20
    local.get 4
    local.set 21
    local.get 5
    local.get 21
    i32.store offset=564
    i32.const 400
    local.set 22
    local.get 20
    local.get 22
    i32.mul
    local.set 23
    local.get 4
    local.set 24
    local.get 24
    local.get 23
    i32.sub
    local.set 25
    local.get 25
    local.set 4
    local.get 4
    global.set $__stack_pointer
    local.get 5
    local.get 20
    i32.store offset=560
    local.get 5
    i32.load offset=572
    local.set 26
    i32.const 2
    local.set 27
    local.get 26
    local.get 27
    i32.shl
    local.set 28
    i32.const 15
    local.set 29
    local.get 28
    local.get 29
    i32.add
    local.set 30
    i32.const -16
    local.set 31
    local.get 30
    local.get 31
    i32.and
    local.set 32
    local.get 4
    local.set 33
    local.get 33
    local.get 32
    i32.sub
    local.set 34
    local.get 34
    local.set 4
    local.get 4
    global.set $__stack_pointer
    local.get 5
    local.get 26
    i32.store offset=556
    i32.const 2
    local.set 35
    local.get 26
    local.get 35
    i32.shl
    local.set 36
    i32.const 0
    local.set 37
    local.get 34
    local.get 37
    local.get 36
    call $memset
    drop
    i32.const 0
    local.set 38
    local.get 5
    local.get 38
    i32.store offset=552
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          local.get 5
          i32.load offset=552
          local.set 39
          local.get 5
          i32.load offset=572
          local.set 40
          local.get 39
          local.set 41
          local.get 40
          local.set 42
          local.get 41
          local.get 42
          i32.lt_s
          local.set 43
          i32.const 1
          local.set 44
          local.get 43
          local.get 44
          i32.and
          local.set 45
          local.get 45
          i32.eqz
          br_if 1 (;@2;)
          i32.const 528
          local.set 46
          local.get 5
          local.get 46
          i32.add
          local.set 47
          local.get 47
          local.set 48
          local.get 5
          i32.load offset=552
          local.set 49
          local.get 5
          i32.load offset=576
          local.set 50
          local.get 5
          local.get 50
          i32.store offset=20
          local.get 5
          local.get 49
          i32.store offset=16
          i32.const 1074
          local.set 51
          i32.const 16
          local.set 52
          local.get 5
          local.get 52
          i32.add
          local.set 53
          local.get 48
          local.get 51
          local.get 53
          call $sprintf
          drop
          i32.const 1000
          local.set 54
          local.get 5
          local.get 54
          i32.store offset=520
          local.get 5
          i32.load offset=520
          local.set 55
          i32.const 0
          local.set 56
          local.get 55
          local.get 56
          i32.shl
          local.set 57
          local.get 57
          call $malloc
          local.set 58
          local.get 5
          local.get 58
          i32.store offset=524
          local.get 5
          i32.load offset=524
          local.set 59
          i32.const 0
          local.set 60
          local.get 59
          local.set 61
          local.get 60
          local.set 62
          local.get 61
          local.get 62
          i32.eq
          local.set 63
          i32.const 1
          local.set 64
          local.get 63
          local.get 64
          i32.and
          local.set 65
          block  ;; label = @4
            local.get 65
            i32.eqz
            br_if 0 (;@4;)
            i32.const 1053
            local.set 66
            local.get 66
            call $perror
            i32.const 1
            local.set 67
            local.get 5
            local.get 67
            i32.store offset=588
            i32.const 1
            local.set 68
            local.get 5
            local.get 68
            i32.store offset=516
            br 3 (;@1;)
          end
          local.get 5
          i32.load offset=524
          local.set 69
          local.get 5
          i32.load offset=520
          local.set 70
          i32.const 0
          local.set 71
          local.get 70
          local.get 71
          i32.shl
          local.set 72
          i32.const 0
          local.set 73
          local.get 69
          local.get 73
          local.get 72
          call $memset
          drop
          local.get 5
          i32.load offset=524
          local.set 74
          i32.const 0
          local.set 75
          local.get 74
          local.get 75
          i32.store8
          i32.const 528
          local.set 76
          local.get 5
          local.get 76
          i32.add
          local.set 77
          local.get 77
          local.set 78
          i32.const 528
          local.set 79
          local.get 5
          local.get 79
          i32.add
          local.set 80
          local.get 80
          local.set 81
          local.get 81
          call $strlen
          local.set 82
          local.get 5
          i32.load offset=524
          local.set 83
          local.get 5
          i32.load offset=520
          local.set 84
          local.get 78
          local.get 82
          local.get 83
          local.get 84
          call $access_buffer
          local.get 5
          i32.load offset=524
          local.set 85
          local.get 5
          local.get 85
          i32.store offset=512
          block  ;; label = @4
            loop  ;; label = @5
              local.get 5
              i32.load offset=512
              local.set 86
              i32.const 508
              local.set 87
              local.get 5
              local.get 87
              i32.add
              local.set 88
              local.get 5
              local.get 88
              i32.store
              i32.const 1084
              local.set 89
              local.get 86
              local.get 89
              local.get 5
              call $sscanf
              local.set 90
              i32.const 1
              local.set 91
              local.get 90
              local.set 92
              local.get 91
              local.set 93
              local.get 92
              local.get 93
              i32.eq
              local.set 94
              i32.const 1
              local.set 95
              local.get 94
              local.get 95
              i32.and
              local.set 96
              local.get 96
              i32.eqz
              br_if 1 (;@4;)
              local.get 5
              i32.load offset=508
              local.set 97
              local.get 5
              i32.load offset=552
              local.set 98
              i32.const 400
              local.set 99
              local.get 98
              local.get 99
              i32.mul
              local.set 100
              local.get 25
              local.get 100
              i32.add
              local.set 101
              local.get 5
              i32.load offset=552
              local.set 102
              i32.const 2
              local.set 103
              local.get 102
              local.get 103
              i32.shl
              local.set 104
              local.get 34
              local.get 104
              i32.add
              local.set 105
              local.get 105
              i32.load
              local.set 106
              i32.const 2
              local.set 107
              local.get 106
              local.get 107
              i32.shl
              local.set 108
              local.get 101
              local.get 108
              i32.add
              local.set 109
              local.get 109
              local.get 97
              i32.store
              local.get 5
              i32.load offset=552
              local.set 110
              i32.const 2
              local.set 111
              local.get 110
              local.get 111
              i32.shl
              local.set 112
              local.get 34
              local.get 112
              i32.add
              local.set 113
              local.get 113
              i32.load
              local.set 114
              i32.const 1
              local.set 115
              local.get 114
              local.get 115
              i32.add
              local.set 116
              local.get 113
              local.get 116
              i32.store
              loop  ;; label = @6
                local.get 5
                i32.load offset=512
                local.set 117
                local.get 117
                i32.load8_u
                local.set 118
                i32.const 24
                local.set 119
                local.get 118
                local.get 119
                i32.shl
                local.set 120
                local.get 120
                local.get 119
                i32.shr_s
                local.set 121
                i32.const 0
                local.set 122
                local.get 122
                local.set 123
                block  ;; label = @7
                  local.get 121
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  i32.load offset=512
                  local.set 124
                  local.get 124
                  i32.load8_u
                  local.set 125
                  i32.const 24
                  local.set 126
                  local.get 125
                  local.get 126
                  i32.shl
                  local.set 127
                  local.get 127
                  local.get 126
                  i32.shr_s
                  local.set 128
                  i32.const 32
                  local.set 129
                  local.get 128
                  local.set 130
                  local.get 129
                  local.set 131
                  local.get 130
                  local.get 131
                  i32.ne
                  local.set 132
                  local.get 132
                  local.set 123
                end
                local.get 123
                local.set 133
                i32.const 1
                local.set 134
                local.get 133
                local.get 134
                i32.and
                local.set 135
                block  ;; label = @7
                  local.get 135
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  i32.load offset=512
                  local.set 136
                  i32.const 1
                  local.set 137
                  local.get 136
                  local.get 137
                  i32.add
                  local.set 138
                  local.get 5
                  local.get 138
                  i32.store offset=512
                  br 1 (;@6;)
                end
              end
              local.get 5
              i32.load offset=512
              local.set 139
              local.get 139
              i32.load8_u
              local.set 140
              i32.const 24
              local.set 141
              local.get 140
              local.get 141
              i32.shl
              local.set 142
              local.get 142
              local.get 141
              i32.shr_s
              local.set 143
              i32.const 32
              local.set 144
              local.get 143
              local.set 145
              local.get 144
              local.set 146
              local.get 145
              local.get 146
              i32.eq
              local.set 147
              i32.const 1
              local.set 148
              local.get 147
              local.get 148
              i32.and
              local.set 149
              block  ;; label = @6
                local.get 149
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                i32.load offset=512
                local.set 150
                i32.const 1
                local.set 151
                local.get 150
                local.get 151
                i32.add
                local.set 152
                local.get 5
                local.get 152
                i32.store offset=512
              end
              br 0 (;@5;)
            end
          end
          local.get 5
          i32.load offset=524
          local.set 153
          local.get 153
          call $free
          local.get 5
          i32.load offset=552
          local.set 154
          i32.const 1
          local.set 155
          local.get 154
          local.get 155
          i32.add
          local.set 156
          local.get 5
          local.get 156
          i32.store offset=552
          br 0 (;@3;)
        end
      end
      i32.const 0
      local.set 157
      local.get 5
      local.get 157
      i32.store offset=92
      local.get 5
      i32.load offset=572
      local.set 158
      i32.const 12
      local.set 159
      local.get 158
      local.get 159
      i32.mul
      local.set 160
      local.get 160
      call $malloc
      local.set 161
      local.get 5
      local.get 161
      i32.store offset=88
      i32.const 0
      local.set 162
      local.get 5
      local.get 162
      i32.store offset=84
      i32.const 0
      local.set 163
      local.get 5
      local.get 163
      i32.store offset=80
      block  ;; label = @2
        loop  ;; label = @3
          local.get 5
          i32.load offset=80
          local.set 164
          local.get 5
          i32.load offset=572
          local.set 165
          local.get 164
          local.set 166
          local.get 165
          local.set 167
          local.get 166
          local.get 167
          i32.lt_s
          local.set 168
          i32.const 1
          local.set 169
          local.get 168
          local.get 169
          i32.and
          local.set 170
          local.get 170
          i32.eqz
          br_if 1 (;@2;)
          local.get 5
          i32.load offset=80
          local.set 171
          i32.const 2
          local.set 172
          local.get 171
          local.get 172
          i32.shl
          local.set 173
          local.get 34
          local.get 173
          i32.add
          local.set 174
          local.get 174
          i32.load
          local.set 175
          i32.const 0
          local.set 176
          local.get 175
          local.set 177
          local.get 176
          local.set 178
          local.get 177
          local.get 178
          i32.gt_s
          local.set 179
          i32.const 1
          local.set 180
          local.get 179
          local.get 180
          i32.and
          local.set 181
          block  ;; label = @4
            local.get 181
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.load offset=80
            local.set 182
            i32.const 400
            local.set 183
            local.get 182
            local.get 183
            i32.mul
            local.set 184
            local.get 25
            local.get 184
            i32.add
            local.set 185
            local.get 185
            i32.load
            local.set 186
            local.get 5
            i32.load offset=88
            local.set 187
            local.get 5
            i32.load offset=84
            local.set 188
            i32.const 12
            local.set 189
            local.get 188
            local.get 189
            i32.mul
            local.set 190
            local.get 187
            local.get 190
            i32.add
            local.set 191
            local.get 191
            local.get 186
            i32.store
            local.get 5
            i32.load offset=80
            local.set 192
            local.get 5
            i32.load offset=88
            local.set 193
            local.get 5
            i32.load offset=84
            local.set 194
            i32.const 12
            local.set 195
            local.get 194
            local.get 195
            i32.mul
            local.set 196
            local.get 193
            local.get 196
            i32.add
            local.set 197
            local.get 197
            local.get 192
            i32.store offset=4
            local.get 5
            i32.load offset=88
            local.set 198
            local.get 5
            i32.load offset=84
            local.set 199
            i32.const 12
            local.set 200
            local.get 199
            local.get 200
            i32.mul
            local.set 201
            local.get 198
            local.get 201
            i32.add
            local.set 202
            i32.const 0
            local.set 203
            local.get 202
            local.get 203
            i32.store offset=8
            local.get 5
            i32.load offset=84
            local.set 204
            i32.const 1
            local.set 205
            local.get 204
            local.get 205
            i32.add
            local.set 206
            local.get 5
            local.get 206
            i32.store offset=84
          end
          local.get 5
          i32.load offset=80
          local.set 207
          i32.const 1
          local.set 208
          local.get 207
          local.get 208
          i32.add
          local.set 209
          local.get 5
          local.get 209
          i32.store offset=80
          br 0 (;@3;)
        end
      end
      local.get 5
      i32.load offset=84
      local.set 210
      i32.const 2
      local.set 211
      local.get 210
      local.get 211
      i32.sub
      local.set 212
      i32.const 2
      local.set 213
      local.get 212
      local.get 213
      i32.div_s
      local.set 214
      local.get 5
      local.get 214
      i32.store offset=76
      block  ;; label = @2
        loop  ;; label = @3
          local.get 5
          i32.load offset=76
          local.set 215
          i32.const 0
          local.set 216
          local.get 215
          local.set 217
          local.get 216
          local.set 218
          local.get 217
          local.get 218
          i32.ge_s
          local.set 219
          i32.const 1
          local.set 220
          local.get 219
          local.get 220
          i32.and
          local.set 221
          local.get 221
          i32.eqz
          br_if 1 (;@2;)
          local.get 5
          i32.load offset=88
          local.set 222
          local.get 5
          i32.load offset=84
          local.set 223
          local.get 5
          i32.load offset=76
          local.set 224
          local.get 222
          local.get 223
          local.get 224
          call $heapifyDown
          local.get 5
          i32.load offset=76
          local.set 225
          i32.const -1
          local.set 226
          local.get 225
          local.get 226
          i32.add
          local.set 227
          local.get 5
          local.get 227
          i32.store offset=76
          br 0 (;@3;)
        end
      end
      block  ;; label = @2
        loop  ;; label = @3
          local.get 5
          i32.load offset=84
          local.set 228
          i32.const 0
          local.set 229
          local.get 228
          local.set 230
          local.get 229
          local.set 231
          local.get 230
          local.get 231
          i32.gt_s
          local.set 232
          i32.const 1
          local.set 233
          local.get 232
          local.get 233
          i32.and
          local.set 234
          local.get 234
          i32.eqz
          br_if 1 (;@2;)
          local.get 5
          i32.load offset=88
          local.set 235
          i32.const 8
          local.set 236
          local.get 235
          local.get 236
          i32.add
          local.set 237
          local.get 237
          i32.load
          local.set 238
          i32.const 64
          local.set 239
          local.get 5
          local.get 239
          i32.add
          local.set 240
          local.get 240
          local.get 236
          i32.add
          local.set 241
          local.get 241
          local.get 238
          i32.store
          local.get 235
          i64.load align=4
          local.set 242
          local.get 5
          local.get 242
          i64.store offset=64
          local.get 5
          i32.load offset=64
          local.set 243
          local.get 5
          i32.load offset=92
          local.set 244
          i32.const 1
          local.set 245
          local.get 244
          local.get 245
          i32.add
          local.set 246
          local.get 5
          local.get 246
          i32.store offset=92
          i32.const 96
          local.set 247
          local.get 5
          local.get 247
          i32.add
          local.set 248
          local.get 248
          local.set 249
          i32.const 2
          local.set 250
          local.get 244
          local.get 250
          i32.shl
          local.set 251
          local.get 249
          local.get 251
          i32.add
          local.set 252
          local.get 252
          local.get 243
          i32.store
          local.get 5
          i32.load offset=72
          local.set 253
          i32.const 1
          local.set 254
          local.get 253
          local.get 254
          i32.add
          local.set 255
          local.get 5
          i32.load offset=68
          local.set 256
          i32.const 2
          local.set 257
          local.get 256
          local.get 257
          i32.shl
          local.set 258
          local.get 34
          local.get 258
          i32.add
          local.set 259
          local.get 259
          i32.load
          local.set 260
          local.get 255
          local.set 261
          local.get 260
          local.set 262
          local.get 261
          local.get 262
          i32.lt_s
          local.set 263
          i32.const 1
          local.set 264
          local.get 263
          local.get 264
          i32.and
          local.set 265
          block  ;; label = @4
            block  ;; label = @5
              local.get 265
              i32.eqz
              br_if 0 (;@5;)
              local.get 5
              i32.load offset=72
              local.set 266
              i32.const 1
              local.set 267
              local.get 266
              local.get 267
              i32.add
              local.set 268
              local.get 5
              local.get 268
              i32.store offset=72
              local.get 5
              i32.load offset=68
              local.set 269
              i32.const 400
              local.set 270
              local.get 269
              local.get 270
              i32.mul
              local.set 271
              local.get 25
              local.get 271
              i32.add
              local.set 272
              local.get 5
              i32.load offset=72
              local.set 273
              i32.const 2
              local.set 274
              local.get 273
              local.get 274
              i32.shl
              local.set 275
              local.get 272
              local.get 275
              i32.add
              local.set 276
              local.get 276
              i32.load
              local.set 277
              local.get 5
              local.get 277
              i32.store offset=64
              local.get 5
              i32.load offset=88
              local.set 278
              local.get 5
              i64.load offset=64
              local.set 279
              local.get 278
              local.get 279
              i64.store align=4
              i32.const 8
              local.set 280
              local.get 278
              local.get 280
              i32.add
              local.set 281
              i32.const 64
              local.set 282
              local.get 5
              local.get 282
              i32.add
              local.set 283
              local.get 283
              local.get 280
              i32.add
              local.set 284
              local.get 284
              i32.load
              local.set 285
              local.get 281
              local.get 285
              i32.store
              br 1 (;@4;)
            end
            local.get 5
            i32.load offset=88
            local.set 286
            local.get 5
            i32.load offset=88
            local.set 287
            local.get 5
            i32.load offset=84
            local.set 288
            i32.const -1
            local.set 289
            local.get 288
            local.get 289
            i32.add
            local.set 290
            local.get 5
            local.get 290
            i32.store offset=84
            i32.const 12
            local.set 291
            local.get 290
            local.get 291
            i32.mul
            local.set 292
            local.get 287
            local.get 292
            i32.add
            local.set 293
            local.get 293
            i64.load align=4
            local.set 294
            local.get 286
            local.get 294
            i64.store align=4
            i32.const 8
            local.set 295
            local.get 286
            local.get 295
            i32.add
            local.set 296
            local.get 293
            local.get 295
            i32.add
            local.set 297
            local.get 297
            i32.load
            local.set 298
            local.get 296
            local.get 298
            i32.store
          end
          local.get 5
          i32.load offset=88
          local.set 299
          local.get 5
          i32.load offset=84
          local.set 300
          i32.const 0
          local.set 301
          local.get 299
          local.get 300
          local.get 301
          call $heapifyDown
          br 0 (;@3;)
        end
      end
      local.get 5
      i32.load offset=88
      local.set 302
      local.get 302
      call $free
      local.get 5
      i32.load offset=576
      local.set 303
      local.get 5
      local.get 303
      i32.store offset=32
      i32.const 1253
      local.set 304
      i32.const 32
      local.set 305
      local.get 5
      local.get 305
      i32.add
      local.set 306
      local.get 304
      local.get 306
      call $printf
      drop
      i32.const 0
      local.set 307
      local.get 5
      local.get 307
      i32.store offset=588
      i32.const 1
      local.set 308
      local.get 5
      local.get 308
      i32.store offset=516
    end
    local.get 5
    i32.load offset=564
    local.set 309
    local.get 309
    local.set 4
    local.get 5
    i32.load offset=588
    local.set 310
    i32.const 592
    local.set 311
    local.get 5
    local.get 311
    i32.add
    local.set 312
    local.get 312
    global.set $__stack_pointer
    local.get 310
    return)
  (func $malloc (type 4) (param i32) (result i32)
    local.get 0
    call $dlmalloc)
  (func $dlmalloc (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=4252
                            local.tee 2
                            br_if 0 (;@12;)
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=4700
                              local.tee 3
                              br_if 0 (;@13;)
                              i32.const 0
                              i64.const -1
                              i64.store offset=4712 align=4
                              i32.const 0
                              i64.const 281474976776192
                              i64.store offset=4704 align=4
                              i32.const 0
                              local.get 1
                              i32.const 8
                              i32.add
                              i32.const -16
                              i32.and
                              i32.const 1431655768
                              i32.xor
                              local.tee 3
                              i32.store offset=4700
                              i32.const 0
                              i32.const 0
                              i32.store offset=4720
                              i32.const 0
                              i32.const 0
                              i32.store offset=4672
                            end
                            i32.const 131072
                            i32.const 71360
                            i32.lt_u
                            br_if 1 (;@11;)
                            i32.const 0
                            local.set 2
                            i32.const 131072
                            i32.const 71360
                            i32.sub
                            i32.const 89
                            i32.lt_u
                            br_if 0 (;@12;)
                            i32.const 0
                            local.set 4
                            i32.const 0
                            i32.const 71360
                            i32.store offset=4676
                            i32.const 0
                            i32.const 71360
                            i32.store offset=4244
                            i32.const 0
                            local.get 3
                            i32.store offset=4264
                            i32.const 0
                            i32.const -1
                            i32.store offset=4260
                            i32.const 0
                            i32.const 131072
                            i32.const 71360
                            i32.sub
                            local.tee 3
                            i32.store offset=4680
                            i32.const 0
                            local.get 3
                            i32.store offset=4664
                            i32.const 0
                            local.get 3
                            i32.store offset=4660
                            loop  ;; label = @13
                              local.get 4
                              i32.const 4288
                              i32.add
                              local.get 4
                              i32.const 4276
                              i32.add
                              local.tee 3
                              i32.store
                              local.get 3
                              local.get 4
                              i32.const 4268
                              i32.add
                              local.tee 5
                              i32.store
                              local.get 4
                              i32.const 4280
                              i32.add
                              local.get 5
                              i32.store
                              local.get 4
                              i32.const 4296
                              i32.add
                              local.get 4
                              i32.const 4284
                              i32.add
                              local.tee 5
                              i32.store
                              local.get 5
                              local.get 3
                              i32.store
                              local.get 4
                              i32.const 4304
                              i32.add
                              local.get 4
                              i32.const 4292
                              i32.add
                              local.tee 3
                              i32.store
                              local.get 3
                              local.get 5
                              i32.store
                              local.get 4
                              i32.const 4300
                              i32.add
                              local.get 3
                              i32.store
                              local.get 4
                              i32.const 32
                              i32.add
                              local.tee 4
                              i32.const 256
                              i32.ne
                              br_if 0 (;@13;)
                            end
                            i32.const 71360
                            i32.const -8
                            i32.const 71360
                            i32.sub
                            i32.const 15
                            i32.and
                            local.tee 4
                            i32.add
                            local.tee 2
                            i32.const 131072
                            i32.const 71360
                            i32.sub
                            i32.const -56
                            i32.add
                            local.tee 3
                            local.get 4
                            i32.sub
                            local.tee 4
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 0
                            i32.const 0
                            i32.load offset=4716
                            i32.store offset=4256
                            i32.const 0
                            local.get 4
                            i32.store offset=4240
                            i32.const 0
                            local.get 2
                            i32.store offset=4252
                            local.get 3
                            i32.const 71360
                            i32.add
                            i32.const 4
                            i32.add
                            i32.const 56
                            i32.store
                          end
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 0
                              i32.const 236
                              i32.gt_u
                              br_if 0 (;@13;)
                              block  ;; label = @14
                                i32.const 0
                                i32.load offset=4228
                                local.tee 6
                                i32.const 16
                                local.get 0
                                i32.const 19
                                i32.add
                                i32.const 496
                                i32.and
                                local.get 0
                                i32.const 11
                                i32.lt_u
                                select
                                local.tee 7
                                i32.const 3
                                i32.shr_u
                                local.tee 3
                                i32.shr_u
                                local.tee 4
                                i32.const 3
                                i32.and
                                i32.eqz
                                br_if 0 (;@14;)
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 4
                                    i32.const 1
                                    i32.and
                                    local.get 3
                                    i32.or
                                    i32.const 1
                                    i32.xor
                                    local.tee 5
                                    i32.const 3
                                    i32.shl
                                    local.tee 3
                                    i32.const 4268
                                    i32.add
                                    local.tee 4
                                    local.get 3
                                    i32.const 4276
                                    i32.add
                                    i32.load
                                    local.tee 3
                                    i32.load offset=8
                                    local.tee 7
                                    i32.ne
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.get 6
                                    i32.const -2
                                    local.get 5
                                    i32.rotl
                                    i32.and
                                    i32.store offset=4228
                                    br 1 (;@15;)
                                  end
                                  local.get 4
                                  local.get 7
                                  i32.store offset=8
                                  local.get 7
                                  local.get 4
                                  i32.store offset=12
                                end
                                local.get 3
                                i32.const 8
                                i32.add
                                local.set 4
                                local.get 3
                                local.get 5
                                i32.const 3
                                i32.shl
                                local.tee 5
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 3
                                local.get 5
                                i32.add
                                local.tee 3
                                local.get 3
                                i32.load offset=4
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                br 13 (;@1;)
                              end
                              local.get 7
                              i32.const 0
                              i32.load offset=4236
                              local.tee 8
                              i32.le_u
                              br_if 1 (;@12;)
                              block  ;; label = @14
                                local.get 4
                                i32.eqz
                                br_if 0 (;@14;)
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 4
                                    local.get 3
                                    i32.shl
                                    i32.const 2
                                    local.get 3
                                    i32.shl
                                    local.tee 4
                                    i32.const 0
                                    local.get 4
                                    i32.sub
                                    i32.or
                                    i32.and
                                    i32.ctz
                                    local.tee 3
                                    i32.const 3
                                    i32.shl
                                    local.tee 4
                                    i32.const 4268
                                    i32.add
                                    local.tee 5
                                    local.get 4
                                    i32.const 4276
                                    i32.add
                                    i32.load
                                    local.tee 4
                                    i32.load offset=8
                                    local.tee 0
                                    i32.ne
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.get 6
                                    i32.const -2
                                    local.get 3
                                    i32.rotl
                                    i32.and
                                    local.tee 6
                                    i32.store offset=4228
                                    br 1 (;@15;)
                                  end
                                  local.get 5
                                  local.get 0
                                  i32.store offset=8
                                  local.get 0
                                  local.get 5
                                  i32.store offset=12
                                end
                                local.get 4
                                local.get 7
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 4
                                local.get 3
                                i32.const 3
                                i32.shl
                                local.tee 3
                                i32.add
                                local.get 3
                                local.get 7
                                i32.sub
                                local.tee 5
                                i32.store
                                local.get 4
                                local.get 7
                                i32.add
                                local.tee 0
                                local.get 5
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                block  ;; label = @15
                                  local.get 8
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 8
                                  i32.const -8
                                  i32.and
                                  i32.const 4268
                                  i32.add
                                  local.set 7
                                  i32.const 0
                                  i32.load offset=4248
                                  local.set 3
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      local.get 6
                                      i32.const 1
                                      local.get 8
                                      i32.const 3
                                      i32.shr_u
                                      i32.shl
                                      local.tee 9
                                      i32.and
                                      br_if 0 (;@17;)
                                      i32.const 0
                                      local.get 6
                                      local.get 9
                                      i32.or
                                      i32.store offset=4228
                                      local.get 7
                                      local.set 9
                                      br 1 (;@16;)
                                    end
                                    local.get 7
                                    i32.load offset=8
                                    local.set 9
                                  end
                                  local.get 9
                                  local.get 3
                                  i32.store offset=12
                                  local.get 7
                                  local.get 3
                                  i32.store offset=8
                                  local.get 3
                                  local.get 7
                                  i32.store offset=12
                                  local.get 3
                                  local.get 9
                                  i32.store offset=8
                                end
                                local.get 4
                                i32.const 8
                                i32.add
                                local.set 4
                                i32.const 0
                                local.get 0
                                i32.store offset=4248
                                i32.const 0
                                local.get 5
                                i32.store offset=4236
                                br 13 (;@1;)
                              end
                              i32.const 0
                              i32.load offset=4232
                              local.tee 10
                              i32.eqz
                              br_if 1 (;@12;)
                              local.get 10
                              i32.ctz
                              i32.const 2
                              i32.shl
                              i32.const 4532
                              i32.add
                              i32.load
                              local.tee 0
                              i32.load offset=4
                              i32.const -8
                              i32.and
                              local.get 7
                              i32.sub
                              local.set 3
                              local.get 0
                              local.set 5
                              block  ;; label = @14
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    local.get 5
                                    i32.load offset=16
                                    local.tee 4
                                    br_if 0 (;@16;)
                                    local.get 5
                                    i32.const 20
                                    i32.add
                                    i32.load
                                    local.tee 4
                                    i32.eqz
                                    br_if 2 (;@14;)
                                  end
                                  local.get 4
                                  i32.load offset=4
                                  i32.const -8
                                  i32.and
                                  local.get 7
                                  i32.sub
                                  local.tee 5
                                  local.get 3
                                  local.get 5
                                  local.get 3
                                  i32.lt_u
                                  local.tee 5
                                  select
                                  local.set 3
                                  local.get 4
                                  local.get 0
                                  local.get 5
                                  select
                                  local.set 0
                                  local.get 4
                                  local.set 5
                                  br 0 (;@15;)
                                end
                              end
                              local.get 0
                              i32.load offset=24
                              local.set 11
                              block  ;; label = @14
                                local.get 0
                                i32.load offset=12
                                local.tee 9
                                local.get 0
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 0
                                i32.load offset=8
                                local.tee 4
                                i32.const 0
                                i32.load offset=4244
                                i32.lt_u
                                drop
                                local.get 9
                                local.get 4
                                i32.store offset=8
                                local.get 4
                                local.get 9
                                i32.store offset=12
                                br 12 (;@2;)
                              end
                              block  ;; label = @14
                                local.get 0
                                i32.const 20
                                i32.add
                                local.tee 5
                                i32.load
                                local.tee 4
                                br_if 0 (;@14;)
                                local.get 0
                                i32.load offset=16
                                local.tee 4
                                i32.eqz
                                br_if 4 (;@10;)
                                local.get 0
                                i32.const 16
                                i32.add
                                local.set 5
                              end
                              loop  ;; label = @14
                                local.get 5
                                local.set 2
                                local.get 4
                                local.tee 9
                                i32.const 20
                                i32.add
                                local.tee 5
                                i32.load
                                local.tee 4
                                br_if 0 (;@14;)
                                local.get 9
                                i32.const 16
                                i32.add
                                local.set 5
                                local.get 9
                                i32.load offset=16
                                local.tee 4
                                br_if 0 (;@14;)
                              end
                              local.get 2
                              i32.const 0
                              i32.store
                              br 11 (;@2;)
                            end
                            i32.const -1
                            local.set 7
                            local.get 0
                            i32.const -65
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 0
                            i32.const 19
                            i32.add
                            local.tee 4
                            i32.const -16
                            i32.and
                            local.set 7
                            i32.const 0
                            i32.load offset=4232
                            local.tee 11
                            i32.eqz
                            br_if 0 (;@12;)
                            i32.const 0
                            local.set 8
                            block  ;; label = @13
                              local.get 7
                              i32.const 256
                              i32.lt_u
                              br_if 0 (;@13;)
                              i32.const 31
                              local.set 8
                              local.get 7
                              i32.const 16777215
                              i32.gt_u
                              br_if 0 (;@13;)
                              local.get 7
                              i32.const 38
                              local.get 4
                              i32.const 8
                              i32.shr_u
                              i32.clz
                              local.tee 4
                              i32.sub
                              i32.shr_u
                              i32.const 1
                              i32.and
                              local.get 4
                              i32.const 1
                              i32.shl
                              i32.sub
                              i32.const 62
                              i32.add
                              local.set 8
                            end
                            i32.const 0
                            local.get 7
                            i32.sub
                            local.set 3
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 8
                                    i32.const 2
                                    i32.shl
                                    i32.const 4532
                                    i32.add
                                    i32.load
                                    local.tee 5
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.set 4
                                    i32.const 0
                                    local.set 9
                                    br 1 (;@15;)
                                  end
                                  i32.const 0
                                  local.set 4
                                  local.get 7
                                  i32.const 0
                                  i32.const 25
                                  local.get 8
                                  i32.const 1
                                  i32.shr_u
                                  i32.sub
                                  local.get 8
                                  i32.const 31
                                  i32.eq
                                  select
                                  i32.shl
                                  local.set 0
                                  i32.const 0
                                  local.set 9
                                  loop  ;; label = @16
                                    block  ;; label = @17
                                      local.get 5
                                      i32.load offset=4
                                      i32.const -8
                                      i32.and
                                      local.get 7
                                      i32.sub
                                      local.tee 6
                                      local.get 3
                                      i32.ge_u
                                      br_if 0 (;@17;)
                                      local.get 6
                                      local.set 3
                                      local.get 5
                                      local.set 9
                                      local.get 6
                                      br_if 0 (;@17;)
                                      i32.const 0
                                      local.set 3
                                      local.get 5
                                      local.set 9
                                      local.get 5
                                      local.set 4
                                      br 3 (;@14;)
                                    end
                                    local.get 4
                                    local.get 5
                                    i32.const 20
                                    i32.add
                                    i32.load
                                    local.tee 6
                                    local.get 6
                                    local.get 5
                                    local.get 0
                                    i32.const 29
                                    i32.shr_u
                                    i32.const 4
                                    i32.and
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    i32.load
                                    local.tee 5
                                    i32.eq
                                    select
                                    local.get 4
                                    local.get 6
                                    select
                                    local.set 4
                                    local.get 0
                                    i32.const 1
                                    i32.shl
                                    local.set 0
                                    local.get 5
                                    br_if 0 (;@16;)
                                  end
                                end
                                block  ;; label = @15
                                  local.get 4
                                  local.get 9
                                  i32.or
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.set 9
                                  i32.const 2
                                  local.get 8
                                  i32.shl
                                  local.tee 4
                                  i32.const 0
                                  local.get 4
                                  i32.sub
                                  i32.or
                                  local.get 11
                                  i32.and
                                  local.tee 4
                                  i32.eqz
                                  br_if 3 (;@12;)
                                  local.get 4
                                  i32.ctz
                                  i32.const 2
                                  i32.shl
                                  i32.const 4532
                                  i32.add
                                  i32.load
                                  local.set 4
                                end
                                local.get 4
                                i32.eqz
                                br_if 1 (;@13;)
                              end
                              loop  ;; label = @14
                                local.get 4
                                i32.load offset=4
                                i32.const -8
                                i32.and
                                local.get 7
                                i32.sub
                                local.tee 6
                                local.get 3
                                i32.lt_u
                                local.set 0
                                block  ;; label = @15
                                  local.get 4
                                  i32.load offset=16
                                  local.tee 5
                                  br_if 0 (;@15;)
                                  local.get 4
                                  i32.const 20
                                  i32.add
                                  i32.load
                                  local.set 5
                                end
                                local.get 6
                                local.get 3
                                local.get 0
                                select
                                local.set 3
                                local.get 4
                                local.get 9
                                local.get 0
                                select
                                local.set 9
                                local.get 5
                                local.set 4
                                local.get 5
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 9
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 3
                            i32.const 0
                            i32.load offset=4236
                            local.get 7
                            i32.sub
                            i32.ge_u
                            br_if 0 (;@12;)
                            local.get 9
                            i32.load offset=24
                            local.set 2
                            block  ;; label = @13
                              local.get 9
                              i32.load offset=12
                              local.tee 0
                              local.get 9
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 9
                              i32.load offset=8
                              local.tee 4
                              i32.const 0
                              i32.load offset=4244
                              i32.lt_u
                              drop
                              local.get 0
                              local.get 4
                              i32.store offset=8
                              local.get 4
                              local.get 0
                              i32.store offset=12
                              br 10 (;@3;)
                            end
                            block  ;; label = @13
                              local.get 9
                              i32.const 20
                              i32.add
                              local.tee 5
                              i32.load
                              local.tee 4
                              br_if 0 (;@13;)
                              local.get 9
                              i32.load offset=16
                              local.tee 4
                              i32.eqz
                              br_if 4 (;@9;)
                              local.get 9
                              i32.const 16
                              i32.add
                              local.set 5
                            end
                            loop  ;; label = @13
                              local.get 5
                              local.set 6
                              local.get 4
                              local.tee 0
                              i32.const 20
                              i32.add
                              local.tee 5
                              i32.load
                              local.tee 4
                              br_if 0 (;@13;)
                              local.get 0
                              i32.const 16
                              i32.add
                              local.set 5
                              local.get 0
                              i32.load offset=16
                              local.tee 4
                              br_if 0 (;@13;)
                            end
                            local.get 6
                            i32.const 0
                            i32.store
                            br 9 (;@3;)
                          end
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=4236
                            local.tee 4
                            local.get 7
                            i32.lt_u
                            br_if 0 (;@12;)
                            i32.const 0
                            i32.load offset=4248
                            local.set 3
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 4
                                local.get 7
                                i32.sub
                                local.tee 5
                                i32.const 16
                                i32.lt_u
                                br_if 0 (;@14;)
                                local.get 3
                                local.get 7
                                i32.add
                                local.tee 0
                                local.get 5
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                local.get 3
                                local.get 4
                                i32.add
                                local.get 5
                                i32.store
                                local.get 3
                                local.get 7
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                br 1 (;@13;)
                              end
                              local.get 3
                              local.get 4
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 3
                              local.get 4
                              i32.add
                              local.tee 4
                              local.get 4
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              i32.const 0
                              local.set 0
                              i32.const 0
                              local.set 5
                            end
                            i32.const 0
                            local.get 5
                            i32.store offset=4236
                            i32.const 0
                            local.get 0
                            i32.store offset=4248
                            local.get 3
                            i32.const 8
                            i32.add
                            local.set 4
                            br 11 (;@1;)
                          end
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=4240
                            local.tee 5
                            local.get 7
                            i32.le_u
                            br_if 0 (;@12;)
                            local.get 2
                            local.get 7
                            i32.add
                            local.tee 4
                            local.get 5
                            local.get 7
                            i32.sub
                            local.tee 3
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 0
                            local.get 4
                            i32.store offset=4252
                            i32.const 0
                            local.get 3
                            i32.store offset=4240
                            local.get 2
                            local.get 7
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            local.get 2
                            i32.const 8
                            i32.add
                            local.set 4
                            br 11 (;@1;)
                          end
                          block  ;; label = @12
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=4700
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.load offset=4708
                              local.set 3
                              br 1 (;@12;)
                            end
                            i32.const 0
                            i64.const -1
                            i64.store offset=4712 align=4
                            i32.const 0
                            i64.const 281474976776192
                            i64.store offset=4704 align=4
                            i32.const 0
                            local.get 1
                            i32.const 12
                            i32.add
                            i32.const -16
                            i32.and
                            i32.const 1431655768
                            i32.xor
                            i32.store offset=4700
                            i32.const 0
                            i32.const 0
                            i32.store offset=4720
                            i32.const 0
                            i32.const 0
                            i32.store offset=4672
                            i32.const 65536
                            local.set 3
                          end
                          i32.const 0
                          local.set 4
                          block  ;; label = @12
                            local.get 3
                            local.get 7
                            i32.const 71
                            i32.add
                            local.tee 8
                            i32.add
                            local.tee 0
                            i32.const 0
                            local.get 3
                            i32.sub
                            local.tee 6
                            i32.and
                            local.tee 9
                            local.get 7
                            i32.gt_u
                            br_if 0 (;@12;)
                            i32.const 0
                            i32.const 48
                            i32.store offset=4724
                            br 11 (;@1;)
                          end
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=4668
                            local.tee 4
                            i32.eqz
                            br_if 0 (;@12;)
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=4660
                              local.tee 3
                              local.get 9
                              i32.add
                              local.tee 11
                              local.get 3
                              i32.le_u
                              br_if 0 (;@13;)
                              local.get 11
                              local.get 4
                              i32.le_u
                              br_if 1 (;@12;)
                            end
                            i32.const 0
                            local.set 4
                            i32.const 0
                            i32.const 48
                            i32.store offset=4724
                            br 11 (;@1;)
                          end
                          i32.const 0
                          i32.load8_u offset=4672
                          i32.const 4
                          i32.and
                          br_if 5 (;@6;)
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 2
                                i32.eqz
                                br_if 0 (;@14;)
                                i32.const 4676
                                local.set 4
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    local.get 4
                                    i32.load
                                    local.tee 3
                                    local.get 2
                                    i32.gt_u
                                    br_if 0 (;@16;)
                                    local.get 3
                                    local.get 4
                                    i32.load offset=4
                                    i32.add
                                    local.get 2
                                    i32.gt_u
                                    br_if 3 (;@13;)
                                  end
                                  local.get 4
                                  i32.load offset=8
                                  local.tee 4
                                  br_if 0 (;@15;)
                                end
                              end
                              i32.const 0
                              call $sbrk
                              local.tee 0
                              i32.const -1
                              i32.eq
                              br_if 6 (;@7;)
                              local.get 9
                              local.set 6
                              block  ;; label = @14
                                i32.const 0
                                i32.load offset=4704
                                local.tee 4
                                i32.const -1
                                i32.add
                                local.tee 3
                                local.get 0
                                i32.and
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 9
                                local.get 0
                                i32.sub
                                local.get 3
                                local.get 0
                                i32.add
                                i32.const 0
                                local.get 4
                                i32.sub
                                i32.and
                                i32.add
                                local.set 6
                              end
                              local.get 6
                              local.get 7
                              i32.le_u
                              br_if 6 (;@7;)
                              local.get 6
                              i32.const 2147483646
                              i32.gt_u
                              br_if 6 (;@7;)
                              block  ;; label = @14
                                i32.const 0
                                i32.load offset=4668
                                local.tee 4
                                i32.eqz
                                br_if 0 (;@14;)
                                i32.const 0
                                i32.load offset=4660
                                local.tee 3
                                local.get 6
                                i32.add
                                local.tee 5
                                local.get 3
                                i32.le_u
                                br_if 7 (;@7;)
                                local.get 5
                                local.get 4
                                i32.gt_u
                                br_if 7 (;@7;)
                              end
                              local.get 6
                              call $sbrk
                              local.tee 4
                              local.get 0
                              i32.ne
                              br_if 1 (;@12;)
                              br 8 (;@5;)
                            end
                            local.get 0
                            local.get 5
                            i32.sub
                            local.get 6
                            i32.and
                            local.tee 6
                            i32.const 2147483646
                            i32.gt_u
                            br_if 5 (;@7;)
                            local.get 6
                            call $sbrk
                            local.tee 0
                            local.get 4
                            i32.load
                            local.get 4
                            i32.load offset=4
                            i32.add
                            i32.eq
                            br_if 4 (;@8;)
                            local.get 0
                            local.set 4
                          end
                          block  ;; label = @12
                            local.get 6
                            local.get 7
                            i32.const 72
                            i32.add
                            i32.ge_u
                            br_if 0 (;@12;)
                            local.get 4
                            i32.const -1
                            i32.eq
                            br_if 0 (;@12;)
                            block  ;; label = @13
                              local.get 8
                              local.get 6
                              i32.sub
                              i32.const 0
                              i32.load offset=4708
                              local.tee 3
                              i32.add
                              i32.const 0
                              local.get 3
                              i32.sub
                              i32.and
                              local.tee 3
                              i32.const 2147483646
                              i32.le_u
                              br_if 0 (;@13;)
                              local.get 4
                              local.set 0
                              br 8 (;@5;)
                            end
                            block  ;; label = @13
                              local.get 3
                              call $sbrk
                              i32.const -1
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 3
                              local.get 6
                              i32.add
                              local.set 6
                              local.get 4
                              local.set 0
                              br 8 (;@5;)
                            end
                            i32.const 0
                            local.get 6
                            i32.sub
                            call $sbrk
                            drop
                            br 5 (;@7;)
                          end
                          local.get 4
                          local.set 0
                          local.get 4
                          i32.const -1
                          i32.ne
                          br_if 6 (;@5;)
                          br 4 (;@7;)
                        end
                        unreachable
                        unreachable
                      end
                      i32.const 0
                      local.set 9
                      br 7 (;@2;)
                    end
                    i32.const 0
                    local.set 0
                    br 5 (;@3;)
                  end
                  local.get 0
                  i32.const -1
                  i32.ne
                  br_if 2 (;@5;)
                end
                i32.const 0
                i32.const 0
                i32.load offset=4672
                i32.const 4
                i32.or
                i32.store offset=4672
              end
              local.get 9
              i32.const 2147483646
              i32.gt_u
              br_if 1 (;@4;)
              local.get 9
              call $sbrk
              local.set 0
              i32.const 0
              call $sbrk
              local.set 4
              local.get 0
              i32.const -1
              i32.eq
              br_if 1 (;@4;)
              local.get 4
              i32.const -1
              i32.eq
              br_if 1 (;@4;)
              local.get 0
              local.get 4
              i32.ge_u
              br_if 1 (;@4;)
              local.get 4
              local.get 0
              i32.sub
              local.tee 6
              local.get 7
              i32.const 56
              i32.add
              i32.le_u
              br_if 1 (;@4;)
            end
            i32.const 0
            i32.const 0
            i32.load offset=4660
            local.get 6
            i32.add
            local.tee 4
            i32.store offset=4660
            block  ;; label = @5
              local.get 4
              i32.const 0
              i32.load offset=4664
              i32.le_u
              br_if 0 (;@5;)
              i32.const 0
              local.get 4
              i32.store offset=4664
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=4252
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 4676
                    local.set 4
                    loop  ;; label = @9
                      local.get 0
                      local.get 4
                      i32.load
                      local.tee 5
                      local.get 4
                      i32.load offset=4
                      local.tee 9
                      i32.add
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 4
                      i32.load offset=8
                      local.tee 4
                      br_if 0 (;@9;)
                      br 3 (;@6;)
                    end
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      i32.const 0
                      i32.load offset=4244
                      local.tee 4
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 4
                      i32.ge_u
                      br_if 1 (;@8;)
                    end
                    i32.const 0
                    local.get 0
                    i32.store offset=4244
                  end
                  i32.const 0
                  local.set 4
                  i32.const 0
                  local.get 6
                  i32.store offset=4680
                  i32.const 0
                  local.get 0
                  i32.store offset=4676
                  i32.const 0
                  i32.const -1
                  i32.store offset=4260
                  i32.const 0
                  i32.const 0
                  i32.load offset=4700
                  i32.store offset=4264
                  i32.const 0
                  i32.const 0
                  i32.store offset=4688
                  loop  ;; label = @8
                    local.get 4
                    i32.const 4288
                    i32.add
                    local.get 4
                    i32.const 4276
                    i32.add
                    local.tee 3
                    i32.store
                    local.get 3
                    local.get 4
                    i32.const 4268
                    i32.add
                    local.tee 5
                    i32.store
                    local.get 4
                    i32.const 4280
                    i32.add
                    local.get 5
                    i32.store
                    local.get 4
                    i32.const 4296
                    i32.add
                    local.get 4
                    i32.const 4284
                    i32.add
                    local.tee 5
                    i32.store
                    local.get 5
                    local.get 3
                    i32.store
                    local.get 4
                    i32.const 4304
                    i32.add
                    local.get 4
                    i32.const 4292
                    i32.add
                    local.tee 3
                    i32.store
                    local.get 3
                    local.get 5
                    i32.store
                    local.get 4
                    i32.const 4300
                    i32.add
                    local.get 3
                    i32.store
                    local.get 4
                    i32.const 32
                    i32.add
                    local.tee 4
                    i32.const 256
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 0
                  i32.const -8
                  local.get 0
                  i32.sub
                  i32.const 15
                  i32.and
                  local.tee 4
                  i32.add
                  local.tee 3
                  local.get 6
                  i32.const -56
                  i32.add
                  local.tee 5
                  local.get 4
                  i32.sub
                  local.tee 4
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=4716
                  i32.store offset=4256
                  i32.const 0
                  local.get 4
                  i32.store offset=4240
                  i32.const 0
                  local.get 3
                  i32.store offset=4252
                  local.get 0
                  local.get 5
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  br 2 (;@5;)
                end
                local.get 3
                local.get 0
                i32.ge_u
                br_if 0 (;@6;)
                local.get 3
                local.get 5
                i32.lt_u
                br_if 0 (;@6;)
                local.get 4
                i32.load offset=12
                i32.const 8
                i32.and
                br_if 0 (;@6;)
                local.get 3
                i32.const -8
                local.get 3
                i32.sub
                i32.const 15
                i32.and
                local.tee 5
                i32.add
                local.tee 0
                i32.const 0
                i32.load offset=4240
                local.get 6
                i32.add
                local.tee 2
                local.get 5
                i32.sub
                local.tee 5
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 4
                local.get 9
                local.get 6
                i32.add
                i32.store offset=4
                i32.const 0
                i32.const 0
                i32.load offset=4716
                i32.store offset=4256
                i32.const 0
                local.get 5
                i32.store offset=4240
                i32.const 0
                local.get 0
                i32.store offset=4252
                local.get 3
                local.get 2
                i32.add
                i32.const 56
                i32.store offset=4
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 0
                i32.const 0
                i32.load offset=4244
                i32.ge_u
                br_if 0 (;@6;)
                i32.const 0
                local.get 0
                i32.store offset=4244
              end
              local.get 0
              local.get 6
              i32.add
              local.set 5
              i32.const 4676
              local.set 4
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      loop  ;; label = @10
                        local.get 4
                        i32.load
                        local.get 5
                        i32.eq
                        br_if 1 (;@9;)
                        local.get 4
                        i32.load offset=8
                        local.tee 4
                        br_if 0 (;@10;)
                        br 2 (;@8;)
                      end
                    end
                    local.get 4
                    i32.load8_u offset=12
                    i32.const 8
                    i32.and
                    i32.eqz
                    br_if 1 (;@7;)
                  end
                  i32.const 4676
                  local.set 4
                  block  ;; label = @8
                    loop  ;; label = @9
                      block  ;; label = @10
                        local.get 4
                        i32.load
                        local.tee 5
                        local.get 3
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 5
                        local.get 4
                        i32.load offset=4
                        i32.add
                        local.tee 5
                        local.get 3
                        i32.gt_u
                        br_if 2 (;@8;)
                      end
                      local.get 4
                      i32.load offset=8
                      local.set 4
                      br 0 (;@9;)
                    end
                  end
                  local.get 0
                  i32.const -8
                  local.get 0
                  i32.sub
                  i32.const 15
                  i32.and
                  local.tee 4
                  i32.add
                  local.tee 2
                  local.get 6
                  i32.const -56
                  i32.add
                  local.tee 9
                  local.get 4
                  i32.sub
                  local.tee 4
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 9
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  local.get 3
                  local.get 5
                  i32.const 55
                  local.get 5
                  i32.sub
                  i32.const 15
                  i32.and
                  i32.add
                  i32.const -63
                  i32.add
                  local.tee 9
                  local.get 9
                  local.get 3
                  i32.const 16
                  i32.add
                  i32.lt_u
                  select
                  local.tee 9
                  i32.const 35
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=4716
                  i32.store offset=4256
                  i32.const 0
                  local.get 4
                  i32.store offset=4240
                  i32.const 0
                  local.get 2
                  i32.store offset=4252
                  local.get 9
                  i32.const 16
                  i32.add
                  i32.const 0
                  i64.load offset=4684 align=4
                  i64.store align=4
                  local.get 9
                  i32.const 0
                  i64.load offset=4676 align=4
                  i64.store offset=8 align=4
                  i32.const 0
                  local.get 9
                  i32.const 8
                  i32.add
                  i32.store offset=4684
                  i32.const 0
                  local.get 6
                  i32.store offset=4680
                  i32.const 0
                  local.get 0
                  i32.store offset=4676
                  i32.const 0
                  i32.const 0
                  i32.store offset=4688
                  local.get 9
                  i32.const 36
                  i32.add
                  local.set 4
                  loop  ;; label = @8
                    local.get 4
                    i32.const 7
                    i32.store
                    local.get 4
                    i32.const 4
                    i32.add
                    local.tee 4
                    local.get 5
                    i32.lt_u
                    br_if 0 (;@8;)
                  end
                  local.get 9
                  local.get 3
                  i32.eq
                  br_if 2 (;@5;)
                  local.get 9
                  local.get 9
                  i32.load offset=4
                  i32.const -2
                  i32.and
                  i32.store offset=4
                  local.get 9
                  local.get 9
                  local.get 3
                  i32.sub
                  local.tee 0
                  i32.store
                  local.get 3
                  local.get 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  block  ;; label = @8
                    local.get 0
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const -8
                    i32.and
                    i32.const 4268
                    i32.add
                    local.set 4
                    block  ;; label = @9
                      block  ;; label = @10
                        i32.const 0
                        i32.load offset=4228
                        local.tee 5
                        i32.const 1
                        local.get 0
                        i32.const 3
                        i32.shr_u
                        i32.shl
                        local.tee 0
                        i32.and
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 5
                        local.get 0
                        i32.or
                        i32.store offset=4228
                        local.get 4
                        local.set 5
                        br 1 (;@9;)
                      end
                      local.get 4
                      i32.load offset=8
                      local.set 5
                    end
                    local.get 5
                    local.get 3
                    i32.store offset=12
                    local.get 4
                    local.get 3
                    i32.store offset=8
                    local.get 3
                    local.get 4
                    i32.store offset=12
                    local.get 3
                    local.get 5
                    i32.store offset=8
                    br 3 (;@5;)
                  end
                  i32.const 31
                  local.set 4
                  block  ;; label = @8
                    local.get 0
                    i32.const 16777215
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 38
                    local.get 0
                    i32.const 8
                    i32.shr_u
                    i32.clz
                    local.tee 4
                    i32.sub
                    i32.shr_u
                    i32.const 1
                    i32.and
                    local.get 4
                    i32.const 1
                    i32.shl
                    i32.sub
                    i32.const 62
                    i32.add
                    local.set 4
                  end
                  local.get 3
                  local.get 4
                  i32.store offset=28
                  local.get 3
                  i64.const 0
                  i64.store offset=16 align=4
                  local.get 4
                  i32.const 2
                  i32.shl
                  i32.const 4532
                  i32.add
                  local.set 5
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=4232
                    local.tee 9
                    i32.const 1
                    local.get 4
                    i32.shl
                    local.tee 6
                    i32.and
                    br_if 0 (;@8;)
                    local.get 5
                    local.get 3
                    i32.store
                    i32.const 0
                    local.get 9
                    local.get 6
                    i32.or
                    i32.store offset=4232
                    local.get 3
                    local.get 5
                    i32.store offset=24
                    local.get 3
                    local.get 3
                    i32.store offset=8
                    local.get 3
                    local.get 3
                    i32.store offset=12
                    br 3 (;@5;)
                  end
                  local.get 0
                  i32.const 0
                  i32.const 25
                  local.get 4
                  i32.const 1
                  i32.shr_u
                  i32.sub
                  local.get 4
                  i32.const 31
                  i32.eq
                  select
                  i32.shl
                  local.set 4
                  local.get 5
                  i32.load
                  local.set 9
                  loop  ;; label = @8
                    local.get 9
                    local.tee 5
                    i32.load offset=4
                    i32.const -8
                    i32.and
                    local.get 0
                    i32.eq
                    br_if 2 (;@6;)
                    local.get 4
                    i32.const 29
                    i32.shr_u
                    local.set 9
                    local.get 4
                    i32.const 1
                    i32.shl
                    local.set 4
                    local.get 5
                    local.get 9
                    i32.const 4
                    i32.and
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 6
                    i32.load
                    local.tee 9
                    br_if 0 (;@8;)
                  end
                  local.get 6
                  local.get 3
                  i32.store
                  local.get 3
                  local.get 5
                  i32.store offset=24
                  local.get 3
                  local.get 3
                  i32.store offset=12
                  local.get 3
                  local.get 3
                  i32.store offset=8
                  br 2 (;@5;)
                end
                local.get 4
                local.get 0
                i32.store
                local.get 4
                local.get 4
                i32.load offset=4
                local.get 6
                i32.add
                i32.store offset=4
                local.get 0
                local.get 5
                local.get 7
                call $prepend_alloc
                local.set 4
                br 5 (;@1;)
              end
              local.get 5
              i32.load offset=8
              local.tee 4
              local.get 3
              i32.store offset=12
              local.get 5
              local.get 3
              i32.store offset=8
              local.get 3
              i32.const 0
              i32.store offset=24
              local.get 3
              local.get 5
              i32.store offset=12
              local.get 3
              local.get 4
              i32.store offset=8
            end
            i32.const 0
            i32.load offset=4240
            local.tee 4
            local.get 7
            i32.le_u
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=4252
            local.tee 3
            local.get 7
            i32.add
            local.tee 5
            local.get 4
            local.get 7
            i32.sub
            local.tee 4
            i32.const 1
            i32.or
            i32.store offset=4
            i32.const 0
            local.get 4
            i32.store offset=4240
            i32.const 0
            local.get 5
            i32.store offset=4252
            local.get 3
            local.get 7
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 3
            i32.const 8
            i32.add
            local.set 4
            br 3 (;@1;)
          end
          i32.const 0
          local.set 4
          i32.const 0
          i32.const 48
          i32.store offset=4724
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 9
              local.get 9
              i32.load offset=28
              local.tee 5
              i32.const 2
              i32.shl
              i32.const 4532
              i32.add
              local.tee 4
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 4
              local.get 0
              i32.store
              local.get 0
              br_if 1 (;@4;)
              i32.const 0
              local.get 11
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              local.tee 11
              i32.store offset=4232
              br 2 (;@3;)
            end
            local.get 2
            i32.const 16
            i32.const 20
            local.get 2
            i32.load offset=16
            local.get 9
            i32.eq
            select
            i32.add
            local.get 0
            i32.store
            local.get 0
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 2
          i32.store offset=24
          block  ;; label = @4
            local.get 9
            i32.load offset=16
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 4
            i32.store offset=16
            local.get 4
            local.get 0
            i32.store offset=24
          end
          local.get 9
          i32.const 20
          i32.add
          i32.load
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.const 20
          i32.add
          local.get 4
          i32.store
          local.get 4
          local.get 0
          i32.store offset=24
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
            local.get 9
            local.get 3
            local.get 7
            i32.or
            local.tee 4
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 9
            local.get 4
            i32.add
            local.tee 4
            local.get 4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br 1 (;@3;)
          end
          local.get 9
          local.get 7
          i32.add
          local.tee 0
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 9
          local.get 7
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          i32.add
          local.get 3
          i32.store
          block  ;; label = @4
            local.get 3
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const -8
            i32.and
            i32.const 4268
            i32.add
            local.set 4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i32.load offset=4228
                local.tee 5
                i32.const 1
                local.get 3
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 3
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                local.get 5
                local.get 3
                i32.or
                i32.store offset=4228
                local.get 4
                local.set 3
                br 1 (;@5;)
              end
              local.get 4
              i32.load offset=8
              local.set 3
            end
            local.get 3
            local.get 0
            i32.store offset=12
            local.get 4
            local.get 0
            i32.store offset=8
            local.get 0
            local.get 4
            i32.store offset=12
            local.get 0
            local.get 3
            i32.store offset=8
            br 1 (;@3;)
          end
          i32.const 31
          local.set 4
          block  ;; label = @4
            local.get 3
            i32.const 16777215
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const 38
            local.get 3
            i32.const 8
            i32.shr_u
            i32.clz
            local.tee 4
            i32.sub
            i32.shr_u
            i32.const 1
            i32.and
            local.get 4
            i32.const 1
            i32.shl
            i32.sub
            i32.const 62
            i32.add
            local.set 4
          end
          local.get 0
          local.get 4
          i32.store offset=28
          local.get 0
          i64.const 0
          i64.store offset=16 align=4
          local.get 4
          i32.const 2
          i32.shl
          i32.const 4532
          i32.add
          local.set 5
          block  ;; label = @4
            local.get 11
            i32.const 1
            local.get 4
            i32.shl
            local.tee 7
            i32.and
            br_if 0 (;@4;)
            local.get 5
            local.get 0
            i32.store
            i32.const 0
            local.get 11
            local.get 7
            i32.or
            i32.store offset=4232
            local.get 0
            local.get 5
            i32.store offset=24
            local.get 0
            local.get 0
            i32.store offset=8
            local.get 0
            local.get 0
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 3
          i32.const 0
          i32.const 25
          local.get 4
          i32.const 1
          i32.shr_u
          i32.sub
          local.get 4
          i32.const 31
          i32.eq
          select
          i32.shl
          local.set 4
          local.get 5
          i32.load
          local.set 7
          block  ;; label = @4
            loop  ;; label = @5
              local.get 7
              local.tee 5
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 3
              i32.eq
              br_if 1 (;@4;)
              local.get 4
              i32.const 29
              i32.shr_u
              local.set 7
              local.get 4
              i32.const 1
              i32.shl
              local.set 4
              local.get 5
              local.get 7
              i32.const 4
              i32.and
              i32.add
              i32.const 16
              i32.add
              local.tee 6
              i32.load
              local.tee 7
              br_if 0 (;@5;)
            end
            local.get 6
            local.get 0
            i32.store
            local.get 0
            local.get 5
            i32.store offset=24
            local.get 0
            local.get 0
            i32.store offset=12
            local.get 0
            local.get 0
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 5
          i32.load offset=8
          local.tee 4
          local.get 0
          i32.store offset=12
          local.get 5
          local.get 0
          i32.store offset=8
          local.get 0
          i32.const 0
          i32.store offset=24
          local.get 0
          local.get 5
          i32.store offset=12
          local.get 0
          local.get 4
          i32.store offset=8
        end
        local.get 9
        i32.const 8
        i32.add
        local.set 4
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 11
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            local.get 0
            i32.load offset=28
            local.tee 5
            i32.const 2
            i32.shl
            i32.const 4532
            i32.add
            local.tee 4
            i32.load
            i32.ne
            br_if 0 (;@4;)
            local.get 4
            local.get 9
            i32.store
            local.get 9
            br_if 1 (;@3;)
            i32.const 0
            local.get 10
            i32.const -2
            local.get 5
            i32.rotl
            i32.and
            i32.store offset=4232
            br 2 (;@2;)
          end
          local.get 11
          i32.const 16
          i32.const 20
          local.get 11
          i32.load offset=16
          local.get 0
          i32.eq
          select
          i32.add
          local.get 9
          i32.store
          local.get 9
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 9
        local.get 11
        i32.store offset=24
        block  ;; label = @3
          local.get 0
          i32.load offset=16
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 9
          local.get 4
          i32.store offset=16
          local.get 4
          local.get 9
          i32.store offset=24
        end
        local.get 0
        i32.const 20
        i32.add
        i32.load
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 9
        i32.const 20
        i32.add
        local.get 4
        i32.store
        local.get 4
        local.get 9
        i32.store offset=24
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const 15
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          local.get 7
          i32.or
          local.tee 4
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 4
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 0
        local.get 7
        i32.add
        local.tee 5
        local.get 3
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        local.get 7
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 5
        local.get 3
        i32.add
        local.get 3
        i32.store
        block  ;; label = @3
          local.get 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const -8
          i32.and
          i32.const 4268
          i32.add
          local.set 7
          i32.const 0
          i32.load offset=4248
          local.set 4
          block  ;; label = @4
            block  ;; label = @5
              i32.const 1
              local.get 8
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 9
              local.get 6
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 9
              local.get 6
              i32.or
              i32.store offset=4228
              local.get 7
              local.set 9
              br 1 (;@4;)
            end
            local.get 7
            i32.load offset=8
            local.set 9
          end
          local.get 9
          local.get 4
          i32.store offset=12
          local.get 7
          local.get 4
          i32.store offset=8
          local.get 4
          local.get 7
          i32.store offset=12
          local.get 4
          local.get 9
          i32.store offset=8
        end
        i32.const 0
        local.get 5
        i32.store offset=4248
        i32.const 0
        local.get 3
        i32.store offset=4236
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 4
    end
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4)
  (func $prepend_alloc (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.const -8
    local.get 0
    i32.sub
    i32.const 15
    i32.and
    i32.add
    local.tee 3
    local.get 2
    i32.const 3
    i32.or
    i32.store offset=4
    local.get 1
    i32.const -8
    local.get 1
    i32.sub
    i32.const 15
    i32.and
    i32.add
    local.tee 4
    local.get 3
    local.get 2
    i32.add
    local.tee 5
    i32.sub
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.const 0
        i32.load offset=4252
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.get 5
        i32.store offset=4252
        i32.const 0
        i32.const 0
        i32.load offset=4240
        local.get 2
        i32.add
        local.tee 2
        i32.store offset=4240
        local.get 5
        local.get 2
        i32.const 1
        i32.or
        i32.store offset=4
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 4
        i32.const 0
        i32.load offset=4248
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.get 5
        i32.store offset=4248
        i32.const 0
        i32.const 0
        i32.load offset=4236
        local.get 2
        i32.add
        local.tee 2
        i32.store offset=4236
        local.get 5
        local.get 2
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 5
        local.get 2
        i32.add
        local.get 2
        i32.store
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 4
        i32.load offset=4
        local.tee 0
        i32.const 3
        i32.and
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.const -8
        i32.and
        local.set 6
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 4
            i32.load offset=8
            local.tee 1
            local.get 0
            i32.const 3
            i32.shr_u
            local.tee 7
            i32.const 3
            i32.shl
            i32.const 4268
            i32.add
            local.tee 8
            i32.eq
            drop
            block  ;; label = @5
              local.get 4
              i32.load offset=12
              local.tee 0
              local.get 1
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=4228
              i32.const -2
              local.get 7
              i32.rotl
              i32.and
              i32.store offset=4228
              br 2 (;@3;)
            end
            local.get 0
            local.get 8
            i32.eq
            drop
            local.get 0
            local.get 1
            i32.store offset=8
            local.get 1
            local.get 0
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 4
          i32.load offset=24
          local.set 9
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.load offset=12
              local.tee 8
              local.get 4
              i32.eq
              br_if 0 (;@5;)
              local.get 4
              i32.load offset=8
              local.tee 0
              i32.const 0
              i32.load offset=4244
              i32.lt_u
              drop
              local.get 8
              local.get 0
              i32.store offset=8
              local.get 0
              local.get 8
              i32.store offset=12
              br 1 (;@4;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                i32.const 20
                i32.add
                local.tee 1
                i32.load
                local.tee 0
                br_if 0 (;@6;)
                local.get 4
                i32.load offset=16
                local.tee 0
                i32.eqz
                br_if 1 (;@5;)
                local.get 4
                i32.const 16
                i32.add
                local.set 1
              end
              loop  ;; label = @6
                local.get 1
                local.set 7
                local.get 0
                local.tee 8
                i32.const 20
                i32.add
                local.tee 1
                i32.load
                local.tee 0
                br_if 0 (;@6;)
                local.get 8
                i32.const 16
                i32.add
                local.set 1
                local.get 8
                i32.load offset=16
                local.tee 0
                br_if 0 (;@6;)
              end
              local.get 7
              i32.const 0
              i32.store
              br 1 (;@4;)
            end
            i32.const 0
            local.set 8
          end
          local.get 9
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              local.get 4
              i32.load offset=28
              local.tee 1
              i32.const 2
              i32.shl
              i32.const 4532
              i32.add
              local.tee 0
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 0
              local.get 8
              i32.store
              local.get 8
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=4232
              i32.const -2
              local.get 1
              i32.rotl
              i32.and
              i32.store offset=4232
              br 2 (;@3;)
            end
            local.get 9
            i32.const 16
            i32.const 20
            local.get 9
            i32.load offset=16
            local.get 4
            i32.eq
            select
            i32.add
            local.get 8
            i32.store
            local.get 8
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 8
          local.get 9
          i32.store offset=24
          block  ;; label = @4
            local.get 4
            i32.load offset=16
            local.tee 0
            i32.eqz
            br_if 0 (;@4;)
            local.get 8
            local.get 0
            i32.store offset=16
            local.get 0
            local.get 8
            i32.store offset=24
          end
          local.get 4
          i32.const 20
          i32.add
          i32.load
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const 20
          i32.add
          local.get 0
          i32.store
          local.get 0
          local.get 8
          i32.store offset=24
        end
        local.get 6
        local.get 2
        i32.add
        local.set 2
        local.get 4
        local.get 6
        i32.add
        local.tee 4
        i32.load offset=4
        local.set 0
      end
      local.get 4
      local.get 0
      i32.const -2
      i32.and
      i32.store offset=4
      local.get 5
      local.get 2
      i32.add
      local.get 2
      i32.store
      local.get 5
      local.get 2
      i32.const 1
      i32.or
      i32.store offset=4
      block  ;; label = @2
        local.get 2
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 2
        i32.const -8
        i32.and
        i32.const 4268
        i32.add
        local.set 0
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=4228
            local.tee 1
            i32.const 1
            local.get 2
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 2
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            local.get 2
            i32.or
            i32.store offset=4228
            local.get 0
            local.set 2
            br 1 (;@3;)
          end
          local.get 0
          i32.load offset=8
          local.set 2
        end
        local.get 2
        local.get 5
        i32.store offset=12
        local.get 0
        local.get 5
        i32.store offset=8
        local.get 5
        local.get 0
        i32.store offset=12
        local.get 5
        local.get 2
        i32.store offset=8
        br 1 (;@1;)
      end
      i32.const 31
      local.set 0
      block  ;; label = @2
        local.get 2
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 2
        i32.const 38
        local.get 2
        i32.const 8
        i32.shr_u
        i32.clz
        local.tee 0
        i32.sub
        i32.shr_u
        i32.const 1
        i32.and
        local.get 0
        i32.const 1
        i32.shl
        i32.sub
        i32.const 62
        i32.add
        local.set 0
      end
      local.get 5
      local.get 0
      i32.store offset=28
      local.get 5
      i64.const 0
      i64.store offset=16 align=4
      local.get 0
      i32.const 2
      i32.shl
      i32.const 4532
      i32.add
      local.set 1
      block  ;; label = @2
        i32.const 0
        i32.load offset=4232
        local.tee 8
        i32.const 1
        local.get 0
        i32.shl
        local.tee 4
        i32.and
        br_if 0 (;@2;)
        local.get 1
        local.get 5
        i32.store
        i32.const 0
        local.get 8
        local.get 4
        i32.or
        i32.store offset=4232
        local.get 5
        local.get 1
        i32.store offset=24
        local.get 5
        local.get 5
        i32.store offset=8
        local.get 5
        local.get 5
        i32.store offset=12
        br 1 (;@1;)
      end
      local.get 2
      i32.const 0
      i32.const 25
      local.get 0
      i32.const 1
      i32.shr_u
      i32.sub
      local.get 0
      i32.const 31
      i32.eq
      select
      i32.shl
      local.set 0
      local.get 1
      i32.load
      local.set 8
      block  ;; label = @2
        loop  ;; label = @3
          local.get 8
          local.tee 1
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 2
          i32.eq
          br_if 1 (;@2;)
          local.get 0
          i32.const 29
          i32.shr_u
          local.set 8
          local.get 0
          i32.const 1
          i32.shl
          local.set 0
          local.get 1
          local.get 8
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          local.tee 4
          i32.load
          local.tee 8
          br_if 0 (;@3;)
        end
        local.get 4
        local.get 5
        i32.store
        local.get 5
        local.get 1
        i32.store offset=24
        local.get 5
        local.get 5
        i32.store offset=12
        local.get 5
        local.get 5
        i32.store offset=8
        br 1 (;@1;)
      end
      local.get 1
      i32.load offset=8
      local.tee 2
      local.get 5
      i32.store offset=12
      local.get 1
      local.get 5
      i32.store offset=8
      local.get 5
      i32.const 0
      i32.store offset=24
      local.get 5
      local.get 1
      i32.store offset=12
      local.get 5
      local.get 2
      i32.store offset=8
    end
    local.get 3
    i32.const 8
    i32.add)
  (func $free (type 7) (param i32)
    local.get 0
    call $dlfree)
  (func $dlfree (type 7) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const -8
      i32.add
      local.tee 1
      local.get 0
      i32.const -4
      i32.add
      i32.load
      local.tee 2
      i32.const -8
      i32.and
      local.tee 0
      i32.add
      local.set 3
      block  ;; label = @2
        local.get 2
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 1
        i32.load
        local.tee 2
        i32.sub
        local.tee 1
        i32.const 0
        i32.load offset=4244
        local.tee 4
        i32.lt_u
        br_if 1 (;@1;)
        local.get 2
        local.get 0
        i32.add
        local.set 0
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.const 0
              i32.load offset=4248
              i32.eq
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 2
                i32.const 255
                i32.gt_u
                br_if 0 (;@6;)
                local.get 1
                i32.load offset=8
                local.tee 4
                local.get 2
                i32.const 3
                i32.shr_u
                local.tee 5
                i32.const 3
                i32.shl
                i32.const 4268
                i32.add
                local.tee 6
                i32.eq
                drop
                block  ;; label = @7
                  local.get 1
                  i32.load offset=12
                  local.tee 2
                  local.get 4
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=4228
                  i32.const -2
                  local.get 5
                  i32.rotl
                  i32.and
                  i32.store offset=4228
                  br 5 (;@2;)
                end
                local.get 2
                local.get 6
                i32.eq
                drop
                local.get 2
                local.get 4
                i32.store offset=8
                local.get 4
                local.get 2
                i32.store offset=12
                br 4 (;@2;)
              end
              local.get 1
              i32.load offset=24
              local.set 7
              block  ;; label = @6
                local.get 1
                i32.load offset=12
                local.tee 6
                local.get 1
                i32.eq
                br_if 0 (;@6;)
                local.get 1
                i32.load offset=8
                local.tee 2
                local.get 4
                i32.lt_u
                drop
                local.get 6
                local.get 2
                i32.store offset=8
                local.get 2
                local.get 6
                i32.store offset=12
                br 3 (;@3;)
              end
              block  ;; label = @6
                local.get 1
                i32.const 20
                i32.add
                local.tee 4
                i32.load
                local.tee 2
                br_if 0 (;@6;)
                local.get 1
                i32.load offset=16
                local.tee 2
                i32.eqz
                br_if 2 (;@4;)
                local.get 1
                i32.const 16
                i32.add
                local.set 4
              end
              loop  ;; label = @6
                local.get 4
                local.set 5
                local.get 2
                local.tee 6
                i32.const 20
                i32.add
                local.tee 4
                i32.load
                local.tee 2
                br_if 0 (;@6;)
                local.get 6
                i32.const 16
                i32.add
                local.set 4
                local.get 6
                i32.load offset=16
                local.tee 2
                br_if 0 (;@6;)
              end
              local.get 5
              i32.const 0
              i32.store
              br 2 (;@3;)
            end
            local.get 3
            i32.load offset=4
            local.tee 2
            i32.const 3
            i32.and
            i32.const 3
            i32.ne
            br_if 2 (;@2;)
            local.get 3
            local.get 2
            i32.const -2
            i32.and
            i32.store offset=4
            i32.const 0
            local.get 0
            i32.store offset=4236
            local.get 3
            local.get 0
            i32.store
            local.get 1
            local.get 0
            i32.const 1
            i32.or
            i32.store offset=4
            return
          end
          i32.const 0
          local.set 6
        end
        local.get 7
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            local.get 1
            i32.load offset=28
            local.tee 4
            i32.const 2
            i32.shl
            i32.const 4532
            i32.add
            local.tee 2
            i32.load
            i32.ne
            br_if 0 (;@4;)
            local.get 2
            local.get 6
            i32.store
            local.get 6
            br_if 1 (;@3;)
            i32.const 0
            i32.const 0
            i32.load offset=4232
            i32.const -2
            local.get 4
            i32.rotl
            i32.and
            i32.store offset=4232
            br 2 (;@2;)
          end
          local.get 7
          i32.const 16
          i32.const 20
          local.get 7
          i32.load offset=16
          local.get 1
          i32.eq
          select
          i32.add
          local.get 6
          i32.store
          local.get 6
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 6
        local.get 7
        i32.store offset=24
        block  ;; label = @3
          local.get 1
          i32.load offset=16
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          local.get 2
          i32.store offset=16
          local.get 2
          local.get 6
          i32.store offset=24
        end
        local.get 1
        i32.const 20
        i32.add
        i32.load
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 6
        i32.const 20
        i32.add
        local.get 2
        i32.store
        local.get 2
        local.get 6
        i32.store offset=24
      end
      local.get 1
      local.get 3
      i32.ge_u
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=4
      local.tee 2
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.const 2
                i32.and
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 3
                  i32.const 0
                  i32.load offset=4252
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 1
                  i32.store offset=4252
                  i32.const 0
                  i32.const 0
                  i32.load offset=4240
                  local.get 0
                  i32.add
                  local.tee 0
                  i32.store offset=4240
                  local.get 1
                  local.get 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  i32.const 0
                  i32.load offset=4248
                  i32.ne
                  br_if 6 (;@1;)
                  i32.const 0
                  i32.const 0
                  i32.store offset=4236
                  i32.const 0
                  i32.const 0
                  i32.store offset=4248
                  return
                end
                block  ;; label = @7
                  local.get 3
                  i32.const 0
                  i32.load offset=4248
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 1
                  i32.store offset=4248
                  i32.const 0
                  i32.const 0
                  i32.load offset=4236
                  local.get 0
                  i32.add
                  local.tee 0
                  i32.store offset=4236
                  local.get 1
                  local.get 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  local.get 0
                  i32.add
                  local.get 0
                  i32.store
                  return
                end
                local.get 2
                i32.const -8
                i32.and
                local.get 0
                i32.add
                local.set 0
                block  ;; label = @7
                  local.get 2
                  i32.const 255
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=8
                  local.tee 4
                  local.get 2
                  i32.const 3
                  i32.shr_u
                  local.tee 5
                  i32.const 3
                  i32.shl
                  i32.const 4268
                  i32.add
                  local.tee 6
                  i32.eq
                  drop
                  block  ;; label = @8
                    local.get 3
                    i32.load offset=12
                    local.tee 2
                    local.get 4
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.const 0
                    i32.load offset=4228
                    i32.const -2
                    local.get 5
                    i32.rotl
                    i32.and
                    i32.store offset=4228
                    br 5 (;@3;)
                  end
                  local.get 2
                  local.get 6
                  i32.eq
                  drop
                  local.get 2
                  local.get 4
                  i32.store offset=8
                  local.get 4
                  local.get 2
                  i32.store offset=12
                  br 4 (;@3;)
                end
                local.get 3
                i32.load offset=24
                local.set 7
                block  ;; label = @7
                  local.get 3
                  i32.load offset=12
                  local.tee 6
                  local.get 3
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=8
                  local.tee 2
                  i32.const 0
                  i32.load offset=4244
                  i32.lt_u
                  drop
                  local.get 6
                  local.get 2
                  i32.store offset=8
                  local.get 2
                  local.get 6
                  i32.store offset=12
                  br 3 (;@4;)
                end
                block  ;; label = @7
                  local.get 3
                  i32.const 20
                  i32.add
                  local.tee 4
                  i32.load
                  local.tee 2
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=16
                  local.tee 2
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 3
                  i32.const 16
                  i32.add
                  local.set 4
                end
                loop  ;; label = @7
                  local.get 4
                  local.set 5
                  local.get 2
                  local.tee 6
                  i32.const 20
                  i32.add
                  local.tee 4
                  i32.load
                  local.tee 2
                  br_if 0 (;@7;)
                  local.get 6
                  i32.const 16
                  i32.add
                  local.set 4
                  local.get 6
                  i32.load offset=16
                  local.tee 2
                  br_if 0 (;@7;)
                end
                local.get 5
                i32.const 0
                i32.store
                br 2 (;@4;)
              end
              local.get 3
              local.get 2
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 1
              local.get 0
              i32.add
              local.get 0
              i32.store
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              br 3 (;@2;)
            end
            i32.const 0
            local.set 6
          end
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              local.get 3
              i32.load offset=28
              local.tee 4
              i32.const 2
              i32.shl
              i32.const 4532
              i32.add
              local.tee 2
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 2
              local.get 6
              i32.store
              local.get 6
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=4232
              i32.const -2
              local.get 4
              i32.rotl
              i32.and
              i32.store offset=4232
              br 2 (;@3;)
            end
            local.get 7
            i32.const 16
            i32.const 20
            local.get 7
            i32.load offset=16
            local.get 3
            i32.eq
            select
            i32.add
            local.get 6
            i32.store
            local.get 6
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 6
          local.get 7
          i32.store offset=24
          block  ;; label = @4
            local.get 3
            i32.load offset=16
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.get 2
            i32.store offset=16
            local.get 2
            local.get 6
            i32.store offset=24
          end
          local.get 3
          i32.const 20
          i32.add
          i32.load
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.const 20
          i32.add
          local.get 2
          i32.store
          local.get 2
          local.get 6
          i32.store offset=24
        end
        local.get 1
        local.get 0
        i32.add
        local.get 0
        i32.store
        local.get 1
        local.get 0
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        i32.const 0
        i32.load offset=4248
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.get 0
        i32.store offset=4236
        return
      end
      block  ;; label = @2
        local.get 0
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -8
        i32.and
        i32.const 4268
        i32.add
        local.set 2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=4228
            local.tee 4
            i32.const 1
            local.get 0
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 0
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 4
            local.get 0
            i32.or
            i32.store offset=4228
            local.get 2
            local.set 0
            br 1 (;@3;)
          end
          local.get 2
          i32.load offset=8
          local.set 0
        end
        local.get 0
        local.get 1
        i32.store offset=12
        local.get 2
        local.get 1
        i32.store offset=8
        local.get 1
        local.get 2
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
        return
      end
      i32.const 31
      local.set 2
      block  ;; label = @2
        local.get 0
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 38
        local.get 0
        i32.const 8
        i32.shr_u
        i32.clz
        local.tee 2
        i32.sub
        i32.shr_u
        i32.const 1
        i32.and
        local.get 2
        i32.const 1
        i32.shl
        i32.sub
        i32.const 62
        i32.add
        local.set 2
      end
      local.get 1
      local.get 2
      i32.store offset=28
      local.get 1
      i64.const 0
      i64.store offset=16 align=4
      local.get 2
      i32.const 2
      i32.shl
      i32.const 4532
      i32.add
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=4232
          local.tee 6
          i32.const 1
          local.get 2
          i32.shl
          local.tee 3
          i32.and
          br_if 0 (;@3;)
          local.get 4
          local.get 1
          i32.store
          i32.const 0
          local.get 6
          local.get 3
          i32.or
          i32.store offset=4232
          local.get 1
          local.get 4
          i32.store offset=24
          local.get 1
          local.get 1
          i32.store offset=8
          local.get 1
          local.get 1
          i32.store offset=12
          br 1 (;@2;)
        end
        local.get 0
        i32.const 0
        i32.const 25
        local.get 2
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 2
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 2
        local.get 4
        i32.load
        local.set 6
        block  ;; label = @3
          loop  ;; label = @4
            local.get 6
            local.tee 4
            i32.load offset=4
            i32.const -8
            i32.and
            local.get 0
            i32.eq
            br_if 1 (;@3;)
            local.get 2
            i32.const 29
            i32.shr_u
            local.set 6
            local.get 2
            i32.const 1
            i32.shl
            local.set 2
            local.get 4
            local.get 6
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee 3
            i32.load
            local.tee 6
            br_if 0 (;@4;)
          end
          local.get 3
          local.get 1
          i32.store
          local.get 1
          local.get 4
          i32.store offset=24
          local.get 1
          local.get 1
          i32.store offset=12
          local.get 1
          local.get 1
          i32.store offset=8
          br 1 (;@2;)
        end
        local.get 4
        i32.load offset=8
        local.tee 0
        local.get 1
        i32.store offset=12
        local.get 4
        local.get 1
        i32.store offset=8
        local.get 1
        i32.const 0
        i32.store offset=24
        local.get 1
        local.get 4
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
      end
      i32.const 0
      i32.const 0
      i32.load offset=4260
      i32.const -1
      i32.add
      local.tee 1
      i32.const -1
      local.get 1
      select
      i32.store offset=4260
    end)
  (func $calloc (type 3) (param i32 i32) (result i32)
    (local i32 i64)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 0
      i64.extend_i32_u
      local.get 1
      i64.extend_i32_u
      i64.mul
      local.tee 3
      i32.wrap_i64
      local.set 2
      local.get 1
      local.get 0
      i32.or
      i32.const 65536
      i32.lt_u
      br_if 0 (;@1;)
      i32.const -1
      local.get 2
      local.get 3
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      i32.const 0
      i32.ne
      select
      local.set 2
    end
    block  ;; label = @1
      local.get 2
      call $dlmalloc
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const -4
      i32.add
      i32.load8_u
      i32.const 3
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      local.get 2
      call $memset
      drop
    end
    local.get 0)
  (func $realloc (type 3) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      local.get 1
      call $dlmalloc
      return
    end
    block  ;; label = @1
      local.get 1
      i32.const -64
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 48
      i32.store offset=4724
      i32.const 0
      return
    end
    i32.const 16
    local.get 1
    i32.const 19
    i32.add
    i32.const -16
    i32.and
    local.get 1
    i32.const 11
    i32.lt_u
    select
    local.set 2
    local.get 0
    i32.const -4
    i32.add
    local.tee 3
    i32.load
    local.tee 4
    i32.const -8
    i32.and
    local.set 5
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.const 3
          i32.and
          br_if 0 (;@3;)
          local.get 2
          i32.const 256
          i32.lt_u
          br_if 1 (;@2;)
          local.get 5
          local.get 2
          i32.const 4
          i32.or
          i32.lt_u
          br_if 1 (;@2;)
          local.get 5
          local.get 2
          i32.sub
          i32.const 0
          i32.load offset=4708
          i32.const 1
          i32.shl
          i32.le_u
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        local.get 0
        i32.const -8
        i32.add
        local.tee 6
        local.get 5
        i32.add
        local.set 7
        block  ;; label = @3
          local.get 5
          local.get 2
          i32.lt_u
          br_if 0 (;@3;)
          local.get 5
          local.get 2
          i32.sub
          local.tee 1
          i32.const 16
          i32.lt_u
          br_if 2 (;@1;)
          local.get 3
          local.get 2
          local.get 4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 6
          local.get 2
          i32.add
          local.tee 2
          local.get 1
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 7
          local.get 7
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          call $dispose_chunk
          local.get 0
          return
        end
        block  ;; label = @3
          local.get 7
          i32.const 0
          i32.load offset=4252
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=4240
          local.get 5
          i32.add
          local.tee 5
          local.get 2
          i32.le_u
          br_if 1 (;@2;)
          local.get 3
          local.get 2
          local.get 4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          i32.const 0
          local.get 6
          local.get 2
          i32.add
          local.tee 1
          i32.store offset=4252
          i32.const 0
          local.get 5
          local.get 2
          i32.sub
          local.tee 2
          i32.store offset=4240
          local.get 1
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          return
        end
        block  ;; label = @3
          local.get 7
          i32.const 0
          i32.load offset=4248
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=4236
          local.get 5
          i32.add
          local.tee 5
          local.get 2
          i32.lt_u
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 5
              local.get 2
              i32.sub
              local.tee 1
              i32.const 16
              i32.lt_u
              br_if 0 (;@5;)
              local.get 3
              local.get 2
              local.get 4
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 6
              local.get 2
              i32.add
              local.tee 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 6
              local.get 5
              i32.add
              local.tee 5
              local.get 1
              i32.store
              local.get 5
              local.get 5
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
              br 1 (;@4;)
            end
            local.get 3
            local.get 4
            i32.const 1
            i32.and
            local.get 5
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 6
            local.get 5
            i32.add
            local.tee 1
            local.get 1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            i32.const 0
            local.set 1
            i32.const 0
            local.set 2
          end
          i32.const 0
          local.get 2
          i32.store offset=4248
          i32.const 0
          local.get 1
          i32.store offset=4236
          local.get 0
          return
        end
        local.get 7
        i32.load offset=4
        local.tee 8
        i32.const 2
        i32.and
        br_if 0 (;@2;)
        local.get 8
        i32.const -8
        i32.and
        local.get 5
        i32.add
        local.tee 9
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
        local.get 9
        local.get 2
        i32.sub
        local.set 10
        block  ;; label = @3
          block  ;; label = @4
            local.get 8
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 7
            i32.load offset=8
            local.tee 1
            local.get 8
            i32.const 3
            i32.shr_u
            local.tee 11
            i32.const 3
            i32.shl
            i32.const 4268
            i32.add
            local.tee 8
            i32.eq
            drop
            block  ;; label = @5
              local.get 7
              i32.load offset=12
              local.tee 5
              local.get 1
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=4228
              i32.const -2
              local.get 11
              i32.rotl
              i32.and
              i32.store offset=4228
              br 2 (;@3;)
            end
            local.get 5
            local.get 8
            i32.eq
            drop
            local.get 5
            local.get 1
            i32.store offset=8
            local.get 1
            local.get 5
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 7
          i32.load offset=24
          local.set 12
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              i32.load offset=12
              local.tee 8
              local.get 7
              i32.eq
              br_if 0 (;@5;)
              local.get 7
              i32.load offset=8
              local.tee 1
              i32.const 0
              i32.load offset=4244
              i32.lt_u
              drop
              local.get 8
              local.get 1
              i32.store offset=8
              local.get 1
              local.get 8
              i32.store offset=12
              br 1 (;@4;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 7
                i32.const 20
                i32.add
                local.tee 5
                i32.load
                local.tee 1
                br_if 0 (;@6;)
                local.get 7
                i32.load offset=16
                local.tee 1
                i32.eqz
                br_if 1 (;@5;)
                local.get 7
                i32.const 16
                i32.add
                local.set 5
              end
              loop  ;; label = @6
                local.get 5
                local.set 11
                local.get 1
                local.tee 8
                i32.const 20
                i32.add
                local.tee 5
                i32.load
                local.tee 1
                br_if 0 (;@6;)
                local.get 8
                i32.const 16
                i32.add
                local.set 5
                local.get 8
                i32.load offset=16
                local.tee 1
                br_if 0 (;@6;)
              end
              local.get 11
              i32.const 0
              i32.store
              br 1 (;@4;)
            end
            i32.const 0
            local.set 8
          end
          local.get 12
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              local.get 7
              i32.load offset=28
              local.tee 5
              i32.const 2
              i32.shl
              i32.const 4532
              i32.add
              local.tee 1
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 1
              local.get 8
              i32.store
              local.get 8
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=4232
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=4232
              br 2 (;@3;)
            end
            local.get 12
            i32.const 16
            i32.const 20
            local.get 12
            i32.load offset=16
            local.get 7
            i32.eq
            select
            i32.add
            local.get 8
            i32.store
            local.get 8
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 8
          local.get 12
          i32.store offset=24
          block  ;; label = @4
            local.get 7
            i32.load offset=16
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 8
            local.get 1
            i32.store offset=16
            local.get 1
            local.get 8
            i32.store offset=24
          end
          local.get 7
          i32.const 20
          i32.add
          i32.load
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const 20
          i32.add
          local.get 1
          i32.store
          local.get 1
          local.get 8
          i32.store offset=24
        end
        block  ;; label = @3
          local.get 10
          i32.const 15
          i32.gt_u
          br_if 0 (;@3;)
          local.get 3
          local.get 4
          i32.const 1
          i32.and
          local.get 9
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 6
          local.get 9
          i32.add
          local.tee 1
          local.get 1
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          return
        end
        local.get 3
        local.get 2
        local.get 4
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store
        local.get 6
        local.get 2
        i32.add
        local.tee 1
        local.get 10
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 6
        local.get 9
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 10
        call $dispose_chunk
        local.get 0
        return
      end
      block  ;; label = @2
        local.get 1
        call $dlmalloc
        local.tee 2
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 2
      local.get 0
      i32.const -4
      i32.const -8
      local.get 3
      i32.load
      local.tee 5
      i32.const 3
      i32.and
      select
      local.get 5
      i32.const -8
      i32.and
      i32.add
      local.tee 5
      local.get 1
      local.get 5
      local.get 1
      i32.lt_u
      select
      call $memcpy
      local.set 1
      local.get 0
      call $dlfree
      local.get 1
      local.set 0
    end
    local.get 0)
  (func $dispose_chunk (type 10) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 0
    local.get 1
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 3
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.tee 3
        local.get 1
        i32.add
        local.set 1
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                local.get 3
                i32.sub
                local.tee 0
                i32.const 0
                i32.load offset=4248
                i32.eq
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 3
                  i32.const 255
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load offset=8
                  local.tee 4
                  local.get 3
                  i32.const 3
                  i32.shr_u
                  local.tee 5
                  i32.const 3
                  i32.shl
                  i32.const 4268
                  i32.add
                  local.tee 6
                  i32.eq
                  drop
                  local.get 0
                  i32.load offset=12
                  local.tee 3
                  local.get 4
                  i32.ne
                  br_if 2 (;@5;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=4228
                  i32.const -2
                  local.get 5
                  i32.rotl
                  i32.and
                  i32.store offset=4228
                  br 5 (;@2;)
                end
                local.get 0
                i32.load offset=24
                local.set 7
                block  ;; label = @7
                  local.get 0
                  i32.load offset=12
                  local.tee 6
                  local.get 0
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load offset=8
                  local.tee 3
                  i32.const 0
                  i32.load offset=4244
                  i32.lt_u
                  drop
                  local.get 6
                  local.get 3
                  i32.store offset=8
                  local.get 3
                  local.get 6
                  i32.store offset=12
                  br 4 (;@3;)
                end
                block  ;; label = @7
                  local.get 0
                  i32.const 20
                  i32.add
                  local.tee 4
                  i32.load
                  local.tee 3
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load offset=16
                  local.tee 3
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 0
                  i32.const 16
                  i32.add
                  local.set 4
                end
                loop  ;; label = @7
                  local.get 4
                  local.set 5
                  local.get 3
                  local.tee 6
                  i32.const 20
                  i32.add
                  local.tee 4
                  i32.load
                  local.tee 3
                  br_if 0 (;@7;)
                  local.get 6
                  i32.const 16
                  i32.add
                  local.set 4
                  local.get 6
                  i32.load offset=16
                  local.tee 3
                  br_if 0 (;@7;)
                end
                local.get 5
                i32.const 0
                i32.store
                br 3 (;@3;)
              end
              local.get 2
              i32.load offset=4
              local.tee 3
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if 3 (;@2;)
              local.get 2
              local.get 3
              i32.const -2
              i32.and
              i32.store offset=4
              i32.const 0
              local.get 1
              i32.store offset=4236
              local.get 2
              local.get 1
              i32.store
              local.get 0
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              return
            end
            local.get 3
            local.get 6
            i32.eq
            drop
            local.get 3
            local.get 4
            i32.store offset=8
            local.get 4
            local.get 3
            i32.store offset=12
            br 2 (;@2;)
          end
          i32.const 0
          local.set 6
        end
        local.get 7
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            local.get 0
            i32.load offset=28
            local.tee 4
            i32.const 2
            i32.shl
            i32.const 4532
            i32.add
            local.tee 3
            i32.load
            i32.ne
            br_if 0 (;@4;)
            local.get 3
            local.get 6
            i32.store
            local.get 6
            br_if 1 (;@3;)
            i32.const 0
            i32.const 0
            i32.load offset=4232
            i32.const -2
            local.get 4
            i32.rotl
            i32.and
            i32.store offset=4232
            br 2 (;@2;)
          end
          local.get 7
          i32.const 16
          i32.const 20
          local.get 7
          i32.load offset=16
          local.get 0
          i32.eq
          select
          i32.add
          local.get 6
          i32.store
          local.get 6
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 6
        local.get 7
        i32.store offset=24
        block  ;; label = @3
          local.get 0
          i32.load offset=16
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          local.get 3
          i32.store offset=16
          local.get 3
          local.get 6
          i32.store offset=24
        end
        local.get 0
        i32.const 20
        i32.add
        i32.load
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 6
        i32.const 20
        i32.add
        local.get 3
        i32.store
        local.get 3
        local.get 6
        i32.store offset=24
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.load offset=4
                local.tee 3
                i32.const 2
                i32.and
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 2
                  i32.const 0
                  i32.load offset=4252
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 0
                  i32.store offset=4252
                  i32.const 0
                  i32.const 0
                  i32.load offset=4240
                  local.get 1
                  i32.add
                  local.tee 1
                  i32.store offset=4240
                  local.get 0
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  i32.const 0
                  i32.load offset=4248
                  i32.ne
                  br_if 6 (;@1;)
                  i32.const 0
                  i32.const 0
                  i32.store offset=4236
                  i32.const 0
                  i32.const 0
                  i32.store offset=4248
                  return
                end
                block  ;; label = @7
                  local.get 2
                  i32.const 0
                  i32.load offset=4248
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 0
                  i32.store offset=4248
                  i32.const 0
                  i32.const 0
                  i32.load offset=4236
                  local.get 1
                  i32.add
                  local.tee 1
                  i32.store offset=4236
                  local.get 0
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 1
                  i32.add
                  local.get 1
                  i32.store
                  return
                end
                local.get 3
                i32.const -8
                i32.and
                local.get 1
                i32.add
                local.set 1
                block  ;; label = @7
                  local.get 3
                  i32.const 255
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 2
                  i32.load offset=8
                  local.tee 4
                  local.get 3
                  i32.const 3
                  i32.shr_u
                  local.tee 5
                  i32.const 3
                  i32.shl
                  i32.const 4268
                  i32.add
                  local.tee 6
                  i32.eq
                  drop
                  block  ;; label = @8
                    local.get 2
                    i32.load offset=12
                    local.tee 3
                    local.get 4
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.const 0
                    i32.load offset=4228
                    i32.const -2
                    local.get 5
                    i32.rotl
                    i32.and
                    i32.store offset=4228
                    br 5 (;@3;)
                  end
                  local.get 3
                  local.get 6
                  i32.eq
                  drop
                  local.get 3
                  local.get 4
                  i32.store offset=8
                  local.get 4
                  local.get 3
                  i32.store offset=12
                  br 4 (;@3;)
                end
                local.get 2
                i32.load offset=24
                local.set 7
                block  ;; label = @7
                  local.get 2
                  i32.load offset=12
                  local.tee 6
                  local.get 2
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 2
                  i32.load offset=8
                  local.tee 3
                  i32.const 0
                  i32.load offset=4244
                  i32.lt_u
                  drop
                  local.get 6
                  local.get 3
                  i32.store offset=8
                  local.get 3
                  local.get 6
                  i32.store offset=12
                  br 3 (;@4;)
                end
                block  ;; label = @7
                  local.get 2
                  i32.const 20
                  i32.add
                  local.tee 4
                  i32.load
                  local.tee 3
                  br_if 0 (;@7;)
                  local.get 2
                  i32.load offset=16
                  local.tee 3
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 4
                end
                loop  ;; label = @7
                  local.get 4
                  local.set 5
                  local.get 3
                  local.tee 6
                  i32.const 20
                  i32.add
                  local.tee 4
                  i32.load
                  local.tee 3
                  br_if 0 (;@7;)
                  local.get 6
                  i32.const 16
                  i32.add
                  local.set 4
                  local.get 6
                  i32.load offset=16
                  local.tee 3
                  br_if 0 (;@7;)
                end
                local.get 5
                i32.const 0
                i32.store
                br 2 (;@4;)
              end
              local.get 2
              local.get 3
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 1
              i32.store
              local.get 0
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              br 3 (;@2;)
            end
            i32.const 0
            local.set 6
          end
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              local.get 2
              i32.load offset=28
              local.tee 4
              i32.const 2
              i32.shl
              i32.const 4532
              i32.add
              local.tee 3
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 3
              local.get 6
              i32.store
              local.get 6
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=4232
              i32.const -2
              local.get 4
              i32.rotl
              i32.and
              i32.store offset=4232
              br 2 (;@3;)
            end
            local.get 7
            i32.const 16
            i32.const 20
            local.get 7
            i32.load offset=16
            local.get 2
            i32.eq
            select
            i32.add
            local.get 6
            i32.store
            local.get 6
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 6
          local.get 7
          i32.store offset=24
          block  ;; label = @4
            local.get 2
            i32.load offset=16
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.get 3
            i32.store offset=16
            local.get 3
            local.get 6
            i32.store offset=24
          end
          local.get 2
          i32.const 20
          i32.add
          i32.load
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.const 20
          i32.add
          local.get 3
          i32.store
          local.get 3
          local.get 6
          i32.store offset=24
        end
        local.get 0
        local.get 1
        i32.add
        local.get 1
        i32.store
        local.get 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.load offset=4248
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.get 1
        i32.store offset=4236
        return
      end
      block  ;; label = @2
        local.get 1
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const -8
        i32.and
        i32.const 4268
        i32.add
        local.set 3
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=4228
            local.tee 4
            i32.const 1
            local.get 1
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 1
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 4
            local.get 1
            i32.or
            i32.store offset=4228
            local.get 3
            local.set 1
            br 1 (;@3;)
          end
          local.get 3
          i32.load offset=8
          local.set 1
        end
        local.get 1
        local.get 0
        i32.store offset=12
        local.get 3
        local.get 0
        i32.store offset=8
        local.get 0
        local.get 3
        i32.store offset=12
        local.get 0
        local.get 1
        i32.store offset=8
        return
      end
      i32.const 31
      local.set 3
      block  ;; label = @2
        local.get 1
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 38
        local.get 1
        i32.const 8
        i32.shr_u
        i32.clz
        local.tee 3
        i32.sub
        i32.shr_u
        i32.const 1
        i32.and
        local.get 3
        i32.const 1
        i32.shl
        i32.sub
        i32.const 62
        i32.add
        local.set 3
      end
      local.get 0
      local.get 3
      i32.store offset=28
      local.get 0
      i64.const 0
      i64.store offset=16 align=4
      local.get 3
      i32.const 2
      i32.shl
      i32.const 4532
      i32.add
      local.set 4
      block  ;; label = @2
        i32.const 0
        i32.load offset=4232
        local.tee 6
        i32.const 1
        local.get 3
        i32.shl
        local.tee 2
        i32.and
        br_if 0 (;@2;)
        local.get 4
        local.get 0
        i32.store
        i32.const 0
        local.get 6
        local.get 2
        i32.or
        i32.store offset=4232
        local.get 0
        local.get 4
        i32.store offset=24
        local.get 0
        local.get 0
        i32.store offset=8
        local.get 0
        local.get 0
        i32.store offset=12
        return
      end
      local.get 1
      i32.const 0
      i32.const 25
      local.get 3
      i32.const 1
      i32.shr_u
      i32.sub
      local.get 3
      i32.const 31
      i32.eq
      select
      i32.shl
      local.set 3
      local.get 4
      i32.load
      local.set 6
      block  ;; label = @2
        loop  ;; label = @3
          local.get 6
          local.tee 4
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.eq
          br_if 1 (;@2;)
          local.get 3
          i32.const 29
          i32.shr_u
          local.set 6
          local.get 3
          i32.const 1
          i32.shl
          local.set 3
          local.get 4
          local.get 6
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          local.tee 2
          i32.load
          local.tee 6
          br_if 0 (;@3;)
        end
        local.get 2
        local.get 0
        i32.store
        local.get 0
        local.get 4
        i32.store offset=24
        local.get 0
        local.get 0
        i32.store offset=12
        local.get 0
        local.get 0
        i32.store offset=8
        return
      end
      local.get 4
      i32.load offset=8
      local.tee 1
      local.get 0
      i32.store offset=12
      local.get 4
      local.get 0
      i32.store offset=8
      local.get 0
      i32.const 0
      i32.store offset=24
      local.get 0
      local.get 4
      i32.store offset=12
      local.get 0
      local.get 1
      i32.store offset=8
    end)
  (func $_Exit (type 7) (param i32)
    local.get 0
    call $__wasi_proc_exit
    unreachable)
  (func $__main_void (type 11) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const 8
              i32.add
              local.get 0
              i32.const 12
              i32.add
              call $__wasi_args_sizes_get
              br_if 0 (;@5;)
              local.get 0
              i32.load offset=8
              i32.const 1
              i32.add
              local.tee 1
              i32.eqz
              br_if 1 (;@4;)
              local.get 0
              i32.load offset=12
              call $malloc
              local.tee 2
              i32.eqz
              br_if 2 (;@3;)
              local.get 1
              i32.const 4
              call $calloc
              local.tee 1
              i32.eqz
              br_if 3 (;@2;)
              local.get 1
              local.get 2
              call $__wasi_args_get
              br_if 4 (;@1;)
              local.get 0
              i32.load offset=8
              local.get 1
              call $main
              local.set 1
              local.get 0
              i32.const 16
              i32.add
              global.set $__stack_pointer
              local.get 1
              return
            end
            i32.const 71
            call $_Exit
            unreachable
          end
          i32.const 70
          call $_Exit
          unreachable
        end
        i32.const 70
        call $_Exit
        unreachable
      end
      local.get 2
      call $free
      i32.const 70
      call $_Exit
      unreachable
    end
    local.get 2
    call $free
    local.get 1
    call $free
    i32.const 71
    call $_Exit
    unreachable)
  (func $__wasi_args_get (type 3) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_args_get
    i32.const 65535
    i32.and)
  (func $__wasi_args_sizes_get (type 3) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_args_sizes_get
    i32.const 65535
    i32.and)
  (func $__wasi_fd_close (type 4) (param i32) (result i32)
    local.get 0
    call $__imported_wasi_snapshot_preview1_fd_close
    i32.const 65535
    i32.and)
  (func $__wasi_fd_fdstat_get (type 3) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_fd_fdstat_get
    i32.const 65535
    i32.and)
  (func $__wasi_fd_seek (type 5) (param i32 i64 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $__imported_wasi_snapshot_preview1_fd_seek
    i32.const 65535
    i32.and)
  (func $__wasi_fd_write (type 6) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $__imported_wasi_snapshot_preview1_fd_write
    i32.const 65535
    i32.and)
  (func $__wasi_proc_exit (type 7) (param i32)
    local.get 0
    call $__imported_wasi_snapshot_preview1_proc_exit
    unreachable)
  (func $abort (type 8)
    unreachable
    unreachable)
  (func $sbrk (type 4) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      memory.size
      i32.const 16
      i32.shl
      return
    end
    block  ;; label = @1
      local.get 0
      i32.const 65535
      i32.and
      br_if 0 (;@1;)
      local.get 0
      i32.const -1
      i32.le_s
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.const 16
        i32.shr_u
        memory.grow
        local.tee 0
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.const 48
        i32.store offset=4724
        i32.const -1
        return
      end
      local.get 0
      i32.const 16
      i32.shl
      return
    end
    call $abort
    unreachable)
  (func $dummy (type 8))
  (func $__wasm_call_dtors (type 8)
    call $dummy
    call $__stdio_exit)
  (func $dummy.1 (type 3) (param i32 i32) (result i32)
    local.get 0)
  (func $__lctrans (type 3) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $dummy.1)
  (func $strerror (type 4) (param i32) (result i32)
    (local i32)
    block  ;; label = @1
      i32.const 0
      i32.load offset=4752
      local.tee 1
      br_if 0 (;@1;)
      i32.const 4728
      local.set 1
      i32.const 0
      i32.const 4728
      i32.store offset=4752
    end
    i32.const 0
    local.get 0
    local.get 0
    i32.const 76
    i32.gt_u
    select
    i32.const 1
    i32.shl
    i32.const 2832
    i32.add
    i32.load16_u
    i32.const 1278
    i32.add
    local.get 1
    i32.load offset=20
    call $__lctrans)
  (func $__ofl_lock (type 11) (result i32)
    i32.const 4756)
  (func $__stdio_exit (type 8)
    (local i32 i32 i32)
    block  ;; label = @1
      call $__ofl_lock
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=20
          local.get 0
          i32.load offset=24
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          i32.const 0
          i32.const 0
          local.get 0
          i32.load offset=32
          call_indirect (type 0)
          drop
        end
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 1
          local.get 0
          i32.load offset=8
          local.tee 2
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          local.get 2
          i32.sub
          i64.extend_i32_s
          i32.const 1
          local.get 0
          i32.load offset=36
          call_indirect (type 1)
          drop
        end
        local.get 0
        i32.load offset=52
        local.tee 0
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      i32.const 0
      i32.load offset=4760
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.load offset=20
        local.get 0
        i32.load offset=24
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.const 0
        local.get 0
        i32.load offset=32
        call_indirect (type 0)
        drop
      end
      local.get 0
      i32.load offset=4
      local.tee 1
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 1)
      drop
    end
    block  ;; label = @1
      i32.const 0
      i32.load offset=4216
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.load offset=20
        local.get 0
        i32.load offset=24
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.const 0
        local.get 0
        i32.load offset=32
        call_indirect (type 0)
        drop
      end
      local.get 0
      i32.load offset=4
      local.tee 1
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 1)
      drop
    end
    block  ;; label = @1
      i32.const 0
      i32.load offset=4096
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.load offset=20
        local.get 0
        i32.load offset=24
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.const 0
        local.get 0
        i32.load offset=32
        call_indirect (type 0)
        drop
      end
      local.get 0
      i32.load offset=4
      local.tee 1
      local.get 0
      i32.load offset=8
      local.tee 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      i32.sub
      i64.extend_i32_s
      i32.const 1
      local.get 0
      i32.load offset=36
      call_indirect (type 1)
      drop
    end)
  (func $__towrite (type 4) (param i32) (result i32)
    (local i32)
    local.get 0
    local.get 0
    i32.load offset=60
    local.tee 1
    i32.const -1
    i32.add
    local.get 1
    i32.or
    i32.store offset=60
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.const 8
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.const 32
      i32.or
      i32.store
      i32.const -1
      return
    end
    local.get 0
    i64.const 0
    i64.store offset=4 align=4
    local.get 0
    local.get 0
    i32.load offset=40
    local.tee 1
    i32.store offset=24
    local.get 0
    local.get 1
    i32.store offset=20
    local.get 0
    local.get 1
    local.get 0
    i32.load offset=44
    i32.add
    i32.store offset=16
    i32.const 0)
  (func $__fwritex (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load offset=16
        local.tee 3
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        local.get 2
        call $__towrite
        br_if 1 (;@1;)
        local.get 2
        i32.load offset=16
        local.set 3
      end
      block  ;; label = @2
        local.get 3
        local.get 2
        i32.load offset=20
        local.tee 5
        i32.sub
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        local.get 1
        local.get 2
        i32.load offset=32
        call_indirect (type 0)
        return
      end
      i32.const 0
      local.set 6
      block  ;; label = @2
        local.get 2
        i32.load offset=64
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        i32.add
        local.set 4
        i32.const 0
        local.set 3
        block  ;; label = @3
          loop  ;; label = @4
            local.get 4
            local.get 3
            i32.add
            i32.const -1
            i32.add
            i32.load8_u
            i32.const 10
            i32.eq
            br_if 1 (;@3;)
            local.get 1
            local.get 3
            i32.const -1
            i32.add
            local.tee 3
            i32.add
            br_if 0 (;@4;)
          end
          i32.const 0
          local.set 6
          br 1 (;@2;)
        end
        local.get 2
        local.get 0
        local.get 1
        local.get 3
        i32.add
        local.tee 6
        local.get 2
        i32.load offset=32
        call_indirect (type 0)
        local.tee 4
        local.get 6
        i32.lt_u
        br_if 1 (;@1;)
        local.get 6
        local.get 0
        i32.add
        local.set 0
        i32.const 0
        local.get 3
        i32.sub
        local.set 1
        local.get 2
        i32.load offset=20
        local.set 5
      end
      local.get 5
      local.get 0
      local.get 1
      call $memcpy
      drop
      local.get 2
      local.get 2
      i32.load offset=20
      local.get 1
      i32.add
      i32.store offset=20
      local.get 6
      local.get 1
      i32.add
      local.set 4
    end
    local.get 4)
  (func $fwrite (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    local.get 2
    local.get 1
    i32.mul
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.load offset=16
        local.tee 5
        br_if 0 (;@2;)
        i32.const 0
        local.set 6
        local.get 3
        call $__towrite
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=16
        local.set 5
      end
      block  ;; label = @2
        local.get 5
        local.get 3
        i32.load offset=20
        local.tee 7
        i32.sub
        local.get 4
        i32.ge_u
        br_if 0 (;@2;)
        local.get 3
        local.get 0
        local.get 4
        local.get 3
        i32.load offset=32
        call_indirect (type 0)
        local.set 6
        br 1 (;@1;)
      end
      i32.const 0
      local.set 8
      block  ;; label = @2
        block  ;; label = @3
          local.get 4
          br_if 0 (;@3;)
          local.get 4
          local.set 5
          br 1 (;@2;)
        end
        i32.const 0
        local.set 5
        block  ;; label = @3
          local.get 3
          i32.load offset=64
          i32.const 0
          i32.ge_s
          br_if 0 (;@3;)
          local.get 4
          local.set 5
          br 1 (;@2;)
        end
        local.get 0
        local.get 4
        i32.add
        local.set 6
        block  ;; label = @3
          loop  ;; label = @4
            local.get 6
            local.get 5
            i32.add
            i32.const -1
            i32.add
            i32.load8_u
            i32.const 10
            i32.eq
            br_if 1 (;@3;)
            local.get 4
            local.get 5
            i32.const -1
            i32.add
            local.tee 5
            i32.add
            br_if 0 (;@4;)
          end
          i32.const 0
          local.set 8
          local.get 4
          local.set 5
          br 1 (;@2;)
        end
        local.get 3
        local.get 0
        local.get 4
        local.get 5
        i32.add
        local.tee 8
        local.get 3
        i32.load offset=32
        call_indirect (type 0)
        local.tee 6
        local.get 8
        i32.lt_u
        br_if 1 (;@1;)
        local.get 8
        local.get 0
        i32.add
        local.set 0
        i32.const 0
        local.get 5
        i32.sub
        local.set 5
        local.get 3
        i32.load offset=20
        local.set 7
      end
      local.get 7
      local.get 0
      local.get 5
      call $memcpy
      drop
      local.get 3
      local.get 3
      i32.load offset=20
      local.get 5
      i32.add
      i32.store offset=20
      local.get 8
      local.get 5
      i32.add
      local.set 6
    end
    block  ;; label = @1
      local.get 6
      local.get 4
      i32.ne
      br_if 0 (;@1;)
      local.get 2
      i32.const 0
      local.get 1
      select
      return
    end
    local.get 6
    local.get 1
    i32.div_u)
  (func $__overflow (type 3) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    i32.store8 offset=15
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=16
        local.tee 3
        br_if 0 (;@2;)
        i32.const -1
        local.set 3
        local.get 0
        call $__towrite
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=16
        local.set 3
      end
      block  ;; label = @2
        local.get 0
        i32.load offset=20
        local.tee 4
        local.get 3
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=64
        local.get 1
        i32.const 255
        i32.and
        local.tee 3
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        local.get 4
        i32.const 1
        i32.add
        i32.store offset=20
        local.get 4
        local.get 1
        i32.store8
        br 1 (;@1;)
      end
      i32.const -1
      local.set 3
      local.get 0
      local.get 2
      i32.const 15
      i32.add
      i32.const 1
      local.get 0
      i32.load offset=32
      call_indirect (type 0)
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 2
      i32.load8_u offset=15
      local.set 3
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3)
  (func $fputc (type 3) (param i32 i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.const 255
      i32.and
      local.tee 2
      local.get 1
      i32.load offset=64
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=20
      local.tee 3
      local.get 1
      i32.load offset=16
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      local.get 3
      i32.const 1
      i32.add
      i32.store offset=20
      local.get 3
      local.get 0
      i32.store8
      local.get 2
      return
    end
    local.get 1
    local.get 2
    call $__overflow)
  (func $perror (type 7) (param i32)
    (local i32 i32 i32)
    i32.const 0
    i32.load offset=4724
    call $strerror
    local.set 1
    i32.const 0
    i32.load offset=4044
    local.set 2
    i32.const 0
    i32.load offset=4088
    local.set 3
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load8_u
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 0
      call $strlen
      i32.const 1
      i32.const 3984
      call $fwrite
      drop
      i32.const 58
      i32.const 3984
      call $fputc
      drop
      i32.const 32
      i32.const 3984
      call $fputc
      drop
    end
    local.get 1
    local.get 1
    call $strlen
    i32.const 1
    i32.const 3984
    call $fwrite
    drop
    i32.const 10
    i32.const 3984
    call $fputc
    drop
    i32.const 0
    local.get 3
    i32.store offset=4088
    i32.const 0
    local.get 2
    i32.store offset=4044)
  (func $printf (type 3) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    i32.store offset=12
    i32.const 4104
    local.get 0
    local.get 1
    call $vfprintf
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $sprintf (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 2
    i32.store offset=12
    local.get 0
    local.get 1
    local.get 2
    call $vsprintf
    local.set 2
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2)
  (func $sscanf (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 2
    i32.store offset=12
    local.get 0
    local.get 1
    local.get 2
    call $vsscanf
    local.set 2
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2)
  (func $__wasilibc_populate_preopens (type 8))
  (func $close (type 4) (param i32) (result i32)
    call $__wasilibc_populate_preopens
    block  ;; label = @1
      local.get 0
      call $__wasi_fd_close
      local.tee 0
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 0
    local.get 0
    i32.store offset=4724
    i32.const -1)
  (func $__stdio_close (type 4) (param i32) (result i32)
    local.get 0
    i32.load offset=56
    call $close)
  (func $writev (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const -1
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const -1
        i32.gt_s
        br_if 0 (;@2;)
        i32.const 0
        i32.const 28
        i32.store offset=4724
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 0
        local.get 1
        local.get 2
        local.get 3
        i32.const 12
        i32.add
        call $__wasi_fd_write
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.get 2
        i32.store offset=4724
        i32.const -1
        local.set 4
        br 1 (;@1;)
      end
      local.get 3
      i32.load offset=12
      local.set 4
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4)
  (func $__stdio_write (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 2
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 3
    local.get 0
    i32.load offset=24
    local.tee 1
    i32.store
    local.get 3
    local.get 0
    i32.load offset=20
    local.get 1
    i32.sub
    local.tee 4
    i32.store offset=4
    i32.const 2
    local.set 5
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=56
        local.get 3
        i32.const 2
        call $writev
        local.tee 1
        local.get 4
        local.get 2
        i32.add
        local.tee 6
        i32.eq
        br_if 0 (;@2;)
        local.get 3
        local.set 4
        loop  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const -1
            i32.gt_s
            br_if 0 (;@4;)
            i32.const 0
            local.set 1
            local.get 0
            i32.const 0
            i32.store offset=24
            local.get 0
            i64.const 0
            i64.store offset=16
            local.get 0
            local.get 0
            i32.load
            i32.const 32
            i32.or
            i32.store
            local.get 5
            i32.const 2
            i32.eq
            br_if 3 (;@1;)
            local.get 2
            local.get 4
            i32.load offset=4
            i32.sub
            local.set 1
            br 3 (;@1;)
          end
          local.get 4
          local.get 1
          local.get 4
          i32.load offset=4
          local.tee 7
          i32.gt_u
          local.tee 8
          i32.const 3
          i32.shl
          i32.add
          local.tee 9
          local.get 9
          i32.load
          local.get 1
          local.get 7
          i32.const 0
          local.get 8
          select
          i32.sub
          local.tee 7
          i32.add
          i32.store
          local.get 4
          i32.const 12
          i32.const 4
          local.get 8
          select
          i32.add
          local.tee 4
          local.get 4
          i32.load
          local.get 7
          i32.sub
          i32.store
          local.get 9
          local.set 4
          local.get 6
          local.get 1
          i32.sub
          local.tee 6
          local.get 0
          i32.load offset=56
          local.get 9
          local.get 5
          local.get 8
          i32.sub
          local.tee 5
          call $writev
          local.tee 1
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 0
      local.get 0
      i32.load offset=40
      local.tee 1
      i32.store offset=24
      local.get 0
      local.get 1
      i32.store offset=20
      local.get 0
      local.get 1
      local.get 0
      i32.load offset=44
      i32.add
      i32.store offset=16
      local.get 2
      local.set 1
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $__lseek (type 1) (param i32 i64 i32) (result i64)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        local.get 1
        local.get 2
        i32.const 255
        i32.and
        local.get 3
        i32.const 8
        i32.add
        call $__wasi_fd_seek
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.const 70
        local.get 2
        local.get 2
        i32.const 76
        i32.eq
        select
        i32.store offset=4724
        i64.const -1
        local.set 1
        br 1 (;@1;)
      end
      local.get 3
      i64.load offset=8
      local.set 1
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $__stdio_seek (type 1) (param i32 i64 i32) (result i64)
    local.get 0
    i32.load offset=56
    local.get 1
    local.get 2
    call $__lseek)
  (func $__isatty (type 4) (param i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        local.get 1
        i32.const 8
        i32.add
        call $__wasi_fd_fdstat_get
        local.tee 0
        br_if 0 (;@2;)
        i32.const 59
        local.set 0
        local.get 1
        i32.load8_u offset=8
        i32.const 2
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u offset=16
        i32.const 36
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 2
        br 1 (;@1;)
      end
      i32.const 0
      local.set 2
      i32.const 0
      local.get 0
      i32.store offset=4724
    end
    local.get 1
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 2)
  (func $__stdout_write (type 0) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 2
    i32.store offset=32
    block  ;; label = @1
      local.get 0
      i32.load8_u
      i32.const 64
      i32.and
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=56
      call $__isatty
      br_if 0 (;@1;)
      local.get 0
      i32.const -1
      i32.store offset=64
    end
    local.get 0
    local.get 1
    local.get 2
    call $__stdio_write)
  (func $wcrtomb (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    i32.const 1
    local.set 3
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 1
        i32.const 127
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        i32.store8
        i32.const 1
        return
      end
      block  ;; label = @2
        i32.const 0
        i32.load offset=4752
        local.tee 3
        br_if 0 (;@2;)
        i32.const 4728
        local.set 3
        i32.const 0
        i32.const 4728
        i32.store offset=4752
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.load
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 1
            i32.const -128
            i32.and
            i32.const 57216
            i32.eq
            br_if 0 (;@4;)
            i32.const 0
            i32.const 25
            i32.store offset=4724
            br 2 (;@2;)
          end
          local.get 0
          local.get 1
          i32.store8
          i32.const 1
          return
        end
        block  ;; label = @3
          local.get 1
          i32.const 2047
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=1
          local.get 0
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 192
          i32.or
          i32.store8
          i32.const 2
          return
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 55296
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const -8192
            i32.and
            i32.const 57344
            i32.ne
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=2
          local.get 0
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 224
          i32.or
          i32.store8
          local.get 0
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=1
          i32.const 3
          return
        end
        block  ;; label = @3
          local.get 1
          i32.const -65536
          i32.add
          i32.const 1048575
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=3
          local.get 0
          local.get 1
          i32.const 18
          i32.shr_u
          i32.const 240
          i32.or
          i32.store8
          local.get 0
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=2
          local.get 0
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=1
          i32.const 4
          return
        end
        i32.const 0
        i32.const 25
        i32.store offset=4724
      end
      i32.const -1
      local.set 3
    end
    local.get 3)
  (func $wctomb (type 3) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    local.get 0
    local.get 1
    i32.const 0
    call $wcrtomb)
  (func $frexp (type 12) (param f64 i32) (result f64)
    (local i64 i32)
    block  ;; label = @1
      local.get 0
      i64.reinterpret_f64
      local.tee 2
      i64.const 52
      i64.shr_u
      i32.wrap_i64
      i32.const 2047
      i32.and
      local.tee 3
      i32.const 2047
      i32.eq
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 0
          f64.const 0x0p+0 (;=0;)
          f64.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 0
          i32.store
          local.get 0
          return
        end
        local.get 0
        f64.const 0x1p+64 (;=1.84467e+19;)
        f64.mul
        local.get 1
        call $frexp
        local.set 0
        local.get 1
        local.get 1
        i32.load
        i32.const -64
        i32.add
        i32.store
        local.get 0
        return
      end
      local.get 1
      local.get 3
      i32.const -1022
      i32.add
      i32.store
      local.get 2
      i64.const -9218868437227405313
      i64.and
      i64.const 4602678819172646912
      i64.or
      f64.reinterpret_i64
      local.set 0
    end
    local.get 0)
  (func $fputs (type 3) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    call $strlen
    local.set 2
    i32.const -1
    i32.const 0
    local.get 2
    local.get 0
    i32.const 1
    local.get 2
    local.get 1
    call $fwrite
    i32.ne
    select)
  (func $vfprintf (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 208
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 2
    i32.store offset=204
    local.get 3
    i32.const 160
    i32.add
    i32.const 32
    i32.add
    i64.const 0
    i64.store
    local.get 3
    i32.const 184
    i32.add
    i64.const 0
    i64.store
    local.get 3
    i32.const 176
    i32.add
    i64.const 0
    i64.store
    local.get 3
    i64.const 0
    i64.store offset=168
    local.get 3
    i64.const 0
    i64.store offset=160
    local.get 3
    local.get 2
    i32.store offset=200
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        local.get 1
        local.get 3
        i32.const 200
        i32.add
        local.get 3
        i32.const 80
        i32.add
        local.get 3
        i32.const 160
        i32.add
        call $printf_core
        i32.const 0
        i32.ge_s
        br_if 0 (;@2;)
        i32.const -1
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      i32.load
      local.set 4
      block  ;; label = @2
        local.get 0
        i32.load offset=60
        i32.const 0
        i32.gt_s
        br_if 0 (;@2;)
        local.get 0
        local.get 4
        i32.const -33
        i32.and
        i32.store
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load offset=44
              br_if 0 (;@5;)
              local.get 0
              i32.const 80
              i32.store offset=44
              local.get 0
              i32.const 0
              i32.store offset=24
              local.get 0
              i64.const 0
              i64.store offset=16
              local.get 0
              i32.load offset=40
              local.set 5
              local.get 0
              local.get 3
              i32.store offset=40
              br 1 (;@4;)
            end
            i32.const 0
            local.set 5
            local.get 0
            i32.load offset=16
            br_if 1 (;@3;)
          end
          i32.const -1
          local.set 2
          local.get 0
          call $__towrite
          br_if 1 (;@2;)
        end
        local.get 0
        local.get 1
        local.get 3
        i32.const 200
        i32.add
        local.get 3
        i32.const 80
        i32.add
        local.get 3
        i32.const 160
        i32.add
        call $printf_core
        local.set 2
      end
      local.get 4
      i32.const 32
      i32.and
      local.set 1
      block  ;; label = @2
        local.get 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.const 0
        local.get 0
        i32.load offset=32
        call_indirect (type 0)
        drop
        local.get 0
        i32.const 0
        i32.store offset=44
        local.get 0
        local.get 5
        i32.store offset=40
        local.get 0
        i32.const 0
        i32.store offset=24
        local.get 0
        i32.load offset=20
        local.set 5
        local.get 0
        i64.const 0
        i64.store offset=16
        local.get 2
        i32.const -1
        local.get 5
        select
        local.set 2
      end
      local.get 0
      local.get 0
      i32.load
      local.tee 5
      local.get 1
      i32.or
      i32.store
      i32.const -1
      local.get 2
      local.get 5
      i32.const 32
      i32.and
      select
      local.set 0
    end
    local.get 3
    i32.const 208
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $printf_core (type 13) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 f64 i32 i32 i32 i32 i32 i32 i32 i32 i32 f64)
    global.get $__stack_pointer
    i32.const 880
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 3
    i32.const -384
    i32.add
    local.set 6
    local.get 5
    i32.const 68
    i32.add
    i32.const 12
    i32.add
    local.set 7
    i32.const 0
    local.get 5
    i32.const 112
    i32.add
    i32.sub
    local.set 8
    local.get 5
    i32.const 112
    i32.add
    i32.const -4100
    i32.add
    local.set 9
    local.get 5
    i32.const 55
    i32.add
    local.set 10
    local.get 5
    i32.const 68
    i32.add
    i32.const 11
    i32.add
    local.set 11
    local.get 5
    i32.const 80
    i32.add
    i32.const -1
    i32.add
    local.set 12
    local.get 5
    i32.const 80
    i32.add
    i32.const 8
    i32.or
    local.set 13
    local.get 5
    i32.const 80
    i32.add
    i32.const 9
    i32.or
    local.set 14
    local.get 5
    i32.const 68
    i32.add
    i32.const 10
    i32.add
    local.set 15
    local.get 5
    i32.const 56
    i32.add
    local.set 16
    i32.const 0
    local.set 17
    i32.const 0
    local.set 18
    i32.const 0
    local.set 19
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          loop  ;; label = @4
            local.get 1
            local.set 20
            local.get 19
            local.get 18
            i32.const 2147483647
            i32.xor
            i32.gt_s
            br_if 1 (;@3;)
            local.get 19
            local.get 18
            i32.add
            local.set 18
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 20
                              i32.load8_u
                              local.tee 19
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 20
                              local.set 1
                              loop  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      local.get 19
                                      i32.const 255
                                      i32.and
                                      local.tee 19
                                      i32.eqz
                                      br_if 0 (;@17;)
                                      local.get 19
                                      i32.const 37
                                      i32.ne
                                      br_if 2 (;@15;)
                                      local.get 1
                                      local.set 21
                                      local.get 1
                                      local.set 19
                                      loop  ;; label = @18
                                        block  ;; label = @19
                                          local.get 19
                                          i32.load8_u offset=1
                                          i32.const 37
                                          i32.eq
                                          br_if 0 (;@19;)
                                          local.get 19
                                          local.set 1
                                          br 3 (;@16;)
                                        end
                                        local.get 21
                                        i32.const 1
                                        i32.add
                                        local.set 21
                                        local.get 19
                                        i32.load8_u offset=2
                                        local.set 22
                                        local.get 19
                                        i32.const 2
                                        i32.add
                                        local.tee 1
                                        local.set 19
                                        local.get 22
                                        i32.const 37
                                        i32.eq
                                        br_if 0 (;@18;)
                                        br 2 (;@16;)
                                      end
                                    end
                                    local.get 1
                                    local.set 21
                                  end
                                  local.get 21
                                  local.get 20
                                  i32.sub
                                  local.tee 19
                                  local.get 18
                                  i32.const 2147483647
                                  i32.xor
                                  local.tee 21
                                  i32.gt_s
                                  br_if 12 (;@3;)
                                  block  ;; label = @16
                                    local.get 0
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    local.get 0
                                    i32.load8_u
                                    i32.const 32
                                    i32.and
                                    br_if 0 (;@16;)
                                    local.get 20
                                    local.get 19
                                    local.get 0
                                    call $__fwritex
                                    drop
                                  end
                                  local.get 19
                                  br_if 11 (;@4;)
                                  local.get 1
                                  i32.const 1
                                  i32.add
                                  local.set 19
                                  i32.const -1
                                  local.set 23
                                  block  ;; label = @16
                                    local.get 1
                                    i32.load8_s offset=1
                                    local.tee 24
                                    i32.const -48
                                    i32.add
                                    local.tee 22
                                    i32.const 9
                                    i32.gt_u
                                    br_if 0 (;@16;)
                                    local.get 1
                                    i32.load8_u offset=2
                                    i32.const 36
                                    i32.ne
                                    br_if 0 (;@16;)
                                    local.get 1
                                    i32.const 3
                                    i32.add
                                    local.set 19
                                    local.get 1
                                    i32.load8_s offset=3
                                    local.set 24
                                    i32.const 1
                                    local.set 17
                                    local.get 22
                                    local.set 23
                                  end
                                  i32.const 0
                                  local.set 25
                                  block  ;; label = @16
                                    local.get 24
                                    i32.const -32
                                    i32.add
                                    local.tee 1
                                    i32.const 31
                                    i32.gt_u
                                    br_if 0 (;@16;)
                                    i32.const 1
                                    local.get 1
                                    i32.shl
                                    local.tee 1
                                    i32.const 75913
                                    i32.and
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    local.get 19
                                    i32.const 1
                                    i32.add
                                    local.set 22
                                    i32.const 0
                                    local.set 25
                                    loop  ;; label = @17
                                      local.get 1
                                      local.get 25
                                      i32.or
                                      local.set 25
                                      local.get 22
                                      local.tee 19
                                      i32.load8_s
                                      local.tee 24
                                      i32.const -32
                                      i32.add
                                      local.tee 1
                                      i32.const 32
                                      i32.ge_u
                                      br_if 1 (;@16;)
                                      local.get 19
                                      i32.const 1
                                      i32.add
                                      local.set 22
                                      i32.const 1
                                      local.get 1
                                      i32.shl
                                      local.tee 1
                                      i32.const 75913
                                      i32.and
                                      br_if 0 (;@17;)
                                    end
                                  end
                                  block  ;; label = @16
                                    local.get 24
                                    i32.const 42
                                    i32.ne
                                    br_if 0 (;@16;)
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        local.get 19
                                        i32.load8_s offset=1
                                        i32.const -48
                                        i32.add
                                        local.tee 1
                                        i32.const 9
                                        i32.gt_u
                                        br_if 0 (;@18;)
                                        local.get 19
                                        i32.load8_u offset=2
                                        i32.const 36
                                        i32.ne
                                        br_if 0 (;@18;)
                                        local.get 4
                                        local.get 1
                                        i32.const 2
                                        i32.shl
                                        i32.add
                                        i32.const 10
                                        i32.store
                                        local.get 19
                                        i32.const 3
                                        i32.add
                                        local.set 22
                                        local.get 6
                                        local.get 19
                                        i32.load8_s offset=1
                                        i32.const 3
                                        i32.shl
                                        i32.add
                                        i32.load
                                        local.set 26
                                        i32.const 1
                                        local.set 17
                                        br 1 (;@17;)
                                      end
                                      local.get 17
                                      br_if 6 (;@11;)
                                      local.get 19
                                      i32.const 1
                                      i32.add
                                      local.set 22
                                      block  ;; label = @18
                                        local.get 0
                                        br_if 0 (;@18;)
                                        i32.const 0
                                        local.set 17
                                        i32.const 0
                                        local.set 26
                                        br 6 (;@12;)
                                      end
                                      local.get 2
                                      local.get 2
                                      i32.load
                                      local.tee 1
                                      i32.const 4
                                      i32.add
                                      i32.store
                                      local.get 1
                                      i32.load
                                      local.set 26
                                      i32.const 0
                                      local.set 17
                                    end
                                    local.get 26
                                    i32.const -1
                                    i32.gt_s
                                    br_if 4 (;@12;)
                                    i32.const 0
                                    local.get 26
                                    i32.sub
                                    local.set 26
                                    local.get 25
                                    i32.const 8192
                                    i32.or
                                    local.set 25
                                    br 4 (;@12;)
                                  end
                                  i32.const 0
                                  local.set 26
                                  block  ;; label = @16
                                    local.get 24
                                    i32.const -48
                                    i32.add
                                    local.tee 1
                                    i32.const 9
                                    i32.le_u
                                    br_if 0 (;@16;)
                                    local.get 19
                                    local.set 22
                                    br 4 (;@12;)
                                  end
                                  i32.const 0
                                  local.set 26
                                  loop  ;; label = @16
                                    block  ;; label = @17
                                      local.get 26
                                      i32.const 214748364
                                      i32.gt_u
                                      br_if 0 (;@17;)
                                      i32.const -1
                                      local.get 26
                                      i32.const 10
                                      i32.mul
                                      local.tee 22
                                      local.get 1
                                      i32.add
                                      local.get 1
                                      local.get 22
                                      i32.const 2147483647
                                      i32.xor
                                      i32.gt_u
                                      local.tee 24
                                      select
                                      local.set 26
                                      local.get 19
                                      i32.load8_s offset=1
                                      local.set 1
                                      local.get 19
                                      i32.const 1
                                      i32.add
                                      local.tee 22
                                      local.set 19
                                      local.get 1
                                      i32.const -48
                                      i32.add
                                      local.tee 1
                                      i32.const 10
                                      i32.lt_u
                                      br_if 1 (;@16;)
                                      local.get 24
                                      br_if 14 (;@3;)
                                      br 5 (;@12;)
                                    end
                                    local.get 19
                                    i32.load8_s offset=1
                                    local.set 1
                                    i32.const -1
                                    local.set 26
                                    local.get 19
                                    i32.const 1
                                    i32.add
                                    local.set 19
                                    local.get 1
                                    i32.const -48
                                    i32.add
                                    local.tee 1
                                    i32.const 10
                                    i32.lt_u
                                    br_if 0 (;@16;)
                                    br 13 (;@3;)
                                  end
                                end
                                local.get 1
                                i32.load8_u offset=1
                                local.set 19
                                local.get 1
                                i32.const 1
                                i32.add
                                local.set 1
                                br 0 (;@14;)
                              end
                            end
                            local.get 0
                            br_if 11 (;@1;)
                            block  ;; label = @13
                              local.get 17
                              br_if 0 (;@13;)
                              i32.const 0
                              local.set 18
                              br 12 (;@1;)
                            end
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 4
                                  i32.load offset=4
                                  local.tee 1
                                  br_if 0 (;@15;)
                                  i32.const 1
                                  local.set 1
                                  br 1 (;@14;)
                                end
                                local.get 3
                                i32.const 8
                                i32.add
                                local.get 1
                                local.get 2
                                call $pop_arg
                                block  ;; label = @15
                                  local.get 4
                                  i32.load offset=8
                                  local.tee 1
                                  br_if 0 (;@15;)
                                  i32.const 2
                                  local.set 1
                                  br 1 (;@14;)
                                end
                                local.get 3
                                i32.const 16
                                i32.add
                                local.get 1
                                local.get 2
                                call $pop_arg
                                block  ;; label = @15
                                  local.get 4
                                  i32.load offset=12
                                  local.tee 1
                                  br_if 0 (;@15;)
                                  i32.const 3
                                  local.set 1
                                  br 1 (;@14;)
                                end
                                local.get 3
                                i32.const 24
                                i32.add
                                local.get 1
                                local.get 2
                                call $pop_arg
                                block  ;; label = @15
                                  local.get 4
                                  i32.load offset=16
                                  local.tee 1
                                  br_if 0 (;@15;)
                                  i32.const 4
                                  local.set 1
                                  br 1 (;@14;)
                                end
                                local.get 3
                                i32.const 32
                                i32.add
                                local.get 1
                                local.get 2
                                call $pop_arg
                                block  ;; label = @15
                                  local.get 4
                                  i32.load offset=20
                                  local.tee 1
                                  br_if 0 (;@15;)
                                  i32.const 5
                                  local.set 1
                                  br 1 (;@14;)
                                end
                                local.get 3
                                i32.const 40
                                i32.add
                                local.get 1
                                local.get 2
                                call $pop_arg
                                block  ;; label = @15
                                  local.get 4
                                  i32.load offset=24
                                  local.tee 1
                                  br_if 0 (;@15;)
                                  i32.const 6
                                  local.set 1
                                  br 1 (;@14;)
                                end
                                local.get 3
                                i32.const 48
                                i32.add
                                local.get 1
                                local.get 2
                                call $pop_arg
                                block  ;; label = @15
                                  local.get 4
                                  i32.load offset=28
                                  local.tee 1
                                  br_if 0 (;@15;)
                                  i32.const 7
                                  local.set 1
                                  br 1 (;@14;)
                                end
                                local.get 3
                                i32.const 56
                                i32.add
                                local.get 1
                                local.get 2
                                call $pop_arg
                                block  ;; label = @15
                                  local.get 4
                                  i32.load offset=32
                                  local.tee 1
                                  br_if 0 (;@15;)
                                  i32.const 8
                                  local.set 1
                                  br 1 (;@14;)
                                end
                                local.get 3
                                i32.const 64
                                i32.add
                                local.get 1
                                local.get 2
                                call $pop_arg
                                local.get 4
                                i32.load offset=36
                                local.tee 1
                                br_if 1 (;@13;)
                                i32.const 9
                                local.set 1
                              end
                              local.get 1
                              i32.const 2
                              i32.shl
                              local.set 1
                              loop  ;; label = @14
                                local.get 4
                                local.get 1
                                i32.add
                                i32.load
                                br_if 3 (;@11;)
                                local.get 1
                                i32.const 4
                                i32.add
                                local.tee 1
                                i32.const 40
                                i32.ne
                                br_if 0 (;@14;)
                              end
                              i32.const 1
                              local.set 18
                              br 12 (;@1;)
                            end
                            local.get 3
                            i32.const 72
                            i32.add
                            local.get 1
                            local.get 2
                            call $pop_arg
                            i32.const 1
                            local.set 18
                            br 11 (;@1;)
                          end
                          i32.const 0
                          local.set 19
                          i32.const -1
                          local.set 24
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 22
                              i32.load8_u
                              i32.const 46
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 22
                              local.set 1
                              i32.const 0
                              local.set 27
                              br 1 (;@12;)
                            end
                            block  ;; label = @13
                              local.get 22
                              i32.load8_s offset=1
                              local.tee 24
                              i32.const 42
                              i32.ne
                              br_if 0 (;@13;)
                              block  ;; label = @14
                                local.get 22
                                i32.load8_s offset=2
                                i32.const -48
                                i32.add
                                local.tee 1
                                i32.const 9
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 22
                                i32.load8_u offset=3
                                i32.const 36
                                i32.ne
                                br_if 0 (;@14;)
                                local.get 4
                                local.get 1
                                i32.const 2
                                i32.shl
                                i32.add
                                i32.const 10
                                i32.store
                                local.get 22
                                i32.const 4
                                i32.add
                                local.set 1
                                local.get 6
                                local.get 22
                                i32.load8_s offset=2
                                i32.const 3
                                i32.shl
                                i32.add
                                i32.load
                                local.tee 24
                                i32.const -1
                                i32.gt_s
                                local.set 27
                                br 2 (;@12;)
                              end
                              local.get 17
                              br_if 2 (;@11;)
                              local.get 22
                              i32.const 2
                              i32.add
                              local.set 1
                              block  ;; label = @14
                                local.get 0
                                br_if 0 (;@14;)
                                i32.const 0
                                local.set 24
                                i32.const 0
                                i32.const -1
                                i32.gt_s
                                local.set 27
                                br 2 (;@12;)
                              end
                              local.get 2
                              local.get 2
                              i32.load
                              local.tee 22
                              i32.const 4
                              i32.add
                              i32.store
                              local.get 22
                              i32.load
                              local.tee 24
                              i32.const -1
                              i32.gt_s
                              local.set 27
                              br 1 (;@12;)
                            end
                            local.get 22
                            i32.const 1
                            i32.add
                            local.set 1
                            block  ;; label = @13
                              local.get 24
                              i32.const -48
                              i32.add
                              local.tee 28
                              i32.const 9
                              i32.le_u
                              br_if 0 (;@13;)
                              i32.const 1
                              local.set 27
                              i32.const 0
                              local.set 24
                              br 1 (;@12;)
                            end
                            i32.const 0
                            local.set 29
                            local.get 1
                            local.set 22
                            loop  ;; label = @13
                              i32.const -1
                              local.set 24
                              block  ;; label = @14
                                local.get 29
                                i32.const 214748364
                                i32.gt_u
                                br_if 0 (;@14;)
                                i32.const -1
                                local.get 29
                                i32.const 10
                                i32.mul
                                local.tee 1
                                local.get 28
                                i32.add
                                local.get 28
                                local.get 1
                                i32.const 2147483647
                                i32.xor
                                i32.gt_u
                                select
                                local.set 24
                              end
                              i32.const 1
                              local.set 27
                              local.get 22
                              i32.load8_s offset=1
                              local.set 28
                              local.get 24
                              local.set 29
                              local.get 22
                              i32.const 1
                              i32.add
                              local.tee 1
                              local.set 22
                              local.get 28
                              i32.const -48
                              i32.add
                              local.tee 28
                              i32.const 10
                              i32.lt_u
                              br_if 0 (;@13;)
                            end
                          end
                          loop  ;; label = @12
                            local.get 19
                            local.set 22
                            local.get 1
                            i32.load8_s
                            local.tee 19
                            i32.const -123
                            i32.add
                            i32.const -58
                            i32.lt_u
                            br_if 1 (;@11;)
                            local.get 1
                            i32.const 1
                            i32.add
                            local.set 1
                            local.get 19
                            local.get 22
                            i32.const 58
                            i32.mul
                            i32.add
                            i32.const 2927
                            i32.add
                            i32.load8_u
                            local.tee 19
                            i32.const -1
                            i32.add
                            i32.const 8
                            i32.lt_u
                            br_if 0 (;@12;)
                          end
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 19
                                i32.const 27
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 19
                                i32.eqz
                                br_if 3 (;@11;)
                                block  ;; label = @15
                                  local.get 23
                                  i32.const 0
                                  i32.lt_s
                                  br_if 0 (;@15;)
                                  local.get 4
                                  local.get 23
                                  i32.const 2
                                  i32.shl
                                  i32.add
                                  local.get 19
                                  i32.store
                                  local.get 5
                                  local.get 3
                                  local.get 23
                                  i32.const 3
                                  i32.shl
                                  i32.add
                                  i64.load
                                  i64.store offset=56
                                  br 2 (;@13;)
                                end
                                block  ;; label = @15
                                  local.get 0
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.set 18
                                  br 14 (;@1;)
                                end
                                local.get 5
                                i32.const 56
                                i32.add
                                local.get 19
                                local.get 2
                                call $pop_arg
                                br 2 (;@12;)
                              end
                              local.get 23
                              i32.const -1
                              i32.gt_s
                              br_if 2 (;@11;)
                            end
                            i32.const 0
                            local.set 19
                            local.get 0
                            i32.eqz
                            br_if 8 (;@4;)
                          end
                          local.get 25
                          i32.const -65537
                          i32.and
                          local.tee 29
                          local.get 25
                          local.get 25
                          i32.const 8192
                          i32.and
                          select
                          local.set 30
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              block  ;; label = @22
                                                block  ;; label = @23
                                                  block  ;; label = @24
                                                    block  ;; label = @25
                                                      block  ;; label = @26
                                                        block  ;; label = @27
                                                          block  ;; label = @28
                                                            local.get 1
                                                            i32.const -1
                                                            i32.add
                                                            i32.load8_s
                                                            local.tee 19
                                                            i32.const -45
                                                            i32.and
                                                            local.get 19
                                                            local.get 19
                                                            i32.const 15
                                                            i32.and
                                                            i32.const 3
                                                            i32.eq
                                                            select
                                                            local.get 19
                                                            local.get 22
                                                            select
                                                            local.tee 31
                                                            i32.const -65
                                                            i32.add
                                                            br_table 16 (;@12;) 18 (;@10;) 13 (;@15;) 18 (;@10;) 16 (;@12;) 16 (;@12;) 16 (;@12;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 12 (;@16;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 3 (;@25;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 16 (;@12;) 18 (;@10;) 8 (;@20;) 5 (;@23;) 16 (;@12;) 16 (;@12;) 16 (;@12;) 18 (;@10;) 5 (;@23;) 18 (;@10;) 18 (;@10;) 18 (;@10;) 9 (;@19;) 1 (;@27;) 4 (;@24;) 2 (;@26;) 18 (;@10;) 18 (;@10;) 10 (;@18;) 18 (;@10;) 0 (;@28;) 18 (;@10;) 18 (;@10;) 3 (;@25;) 18 (;@10;)
                                                          end
                                                          i32.const 0
                                                          local.set 28
                                                          i32.const 1024
                                                          local.set 23
                                                          local.get 5
                                                          i64.load offset=56
                                                          local.set 32
                                                          br 5 (;@22;)
                                                        end
                                                        i32.const 0
                                                        local.set 19
                                                        block  ;; label = @27
                                                          block  ;; label = @28
                                                            block  ;; label = @29
                                                              block  ;; label = @30
                                                                block  ;; label = @31
                                                                  block  ;; label = @32
                                                                    block  ;; label = @33
                                                                      local.get 22
                                                                      i32.const 255
                                                                      i32.and
                                                                      br_table 0 (;@33;) 1 (;@32;) 2 (;@31;) 3 (;@30;) 4 (;@29;) 29 (;@4;) 5 (;@28;) 6 (;@27;) 29 (;@4;)
                                                                    end
                                                                    local.get 5
                                                                    i32.load offset=56
                                                                    local.get 18
                                                                    i32.store
                                                                    br 28 (;@4;)
                                                                  end
                                                                  local.get 5
                                                                  i32.load offset=56
                                                                  local.get 18
                                                                  i32.store
                                                                  br 27 (;@4;)
                                                                end
                                                                local.get 5
                                                                i32.load offset=56
                                                                local.get 18
                                                                i64.extend_i32_s
                                                                i64.store
                                                                br 26 (;@4;)
                                                              end
                                                              local.get 5
                                                              i32.load offset=56
                                                              local.get 18
                                                              i32.store16
                                                              br 25 (;@4;)
                                                            end
                                                            local.get 5
                                                            i32.load offset=56
                                                            local.get 18
                                                            i32.store8
                                                            br 24 (;@4;)
                                                          end
                                                          local.get 5
                                                          i32.load offset=56
                                                          local.get 18
                                                          i32.store
                                                          br 23 (;@4;)
                                                        end
                                                        local.get 5
                                                        i32.load offset=56
                                                        local.get 18
                                                        i64.extend_i32_s
                                                        i64.store
                                                        br 22 (;@4;)
                                                      end
                                                      local.get 24
                                                      i32.const 8
                                                      local.get 24
                                                      i32.const 8
                                                      i32.gt_u
                                                      select
                                                      local.set 24
                                                      local.get 30
                                                      i32.const 8
                                                      i32.or
                                                      local.set 30
                                                      i32.const 120
                                                      local.set 31
                                                    end
                                                    i32.const 0
                                                    local.set 28
                                                    i32.const 1024
                                                    local.set 23
                                                    block  ;; label = @25
                                                      local.get 5
                                                      i64.load offset=56
                                                      local.tee 32
                                                      i64.eqz
                                                      i32.eqz
                                                      br_if 0 (;@25;)
                                                      local.get 16
                                                      local.set 20
                                                      br 4 (;@21;)
                                                    end
                                                    local.get 31
                                                    i32.const 32
                                                    i32.and
                                                    local.set 22
                                                    local.get 16
                                                    local.set 20
                                                    loop  ;; label = @25
                                                      local.get 20
                                                      i32.const -1
                                                      i32.add
                                                      local.tee 20
                                                      local.get 32
                                                      i32.wrap_i64
                                                      i32.const 15
                                                      i32.and
                                                      i32.const 3456
                                                      i32.add
                                                      i32.load8_u
                                                      local.get 22
                                                      i32.or
                                                      i32.store8
                                                      local.get 32
                                                      i64.const 15
                                                      i64.gt_u
                                                      local.set 19
                                                      local.get 32
                                                      i64.const 4
                                                      i64.shr_u
                                                      local.set 32
                                                      local.get 19
                                                      br_if 0 (;@25;)
                                                    end
                                                    local.get 30
                                                    i32.const 8
                                                    i32.and
                                                    i32.eqz
                                                    br_if 3 (;@21;)
                                                    local.get 31
                                                    i32.const 4
                                                    i32.shr_s
                                                    i32.const 1024
                                                    i32.add
                                                    local.set 23
                                                    i32.const 2
                                                    local.set 28
                                                    br 3 (;@21;)
                                                  end
                                                  local.get 16
                                                  local.set 20
                                                  block  ;; label = @24
                                                    local.get 5
                                                    i64.load offset=56
                                                    local.tee 32
                                                    i64.eqz
                                                    br_if 0 (;@24;)
                                                    local.get 16
                                                    local.set 20
                                                    loop  ;; label = @25
                                                      local.get 20
                                                      i32.const -1
                                                      i32.add
                                                      local.tee 20
                                                      local.get 32
                                                      i32.wrap_i64
                                                      i32.const 7
                                                      i32.and
                                                      i32.const 48
                                                      i32.or
                                                      i32.store8
                                                      local.get 32
                                                      i64.const 7
                                                      i64.gt_u
                                                      local.set 19
                                                      local.get 32
                                                      i64.const 3
                                                      i64.shr_u
                                                      local.set 32
                                                      local.get 19
                                                      br_if 0 (;@25;)
                                                    end
                                                  end
                                                  i32.const 0
                                                  local.set 28
                                                  i32.const 1024
                                                  local.set 23
                                                  local.get 30
                                                  i32.const 8
                                                  i32.and
                                                  i32.eqz
                                                  br_if 2 (;@21;)
                                                  local.get 24
                                                  local.get 16
                                                  local.get 20
                                                  i32.sub
                                                  local.tee 19
                                                  i32.const 1
                                                  i32.add
                                                  local.get 24
                                                  local.get 19
                                                  i32.gt_s
                                                  select
                                                  local.set 24
                                                  br 2 (;@21;)
                                                end
                                                block  ;; label = @23
                                                  local.get 5
                                                  i64.load offset=56
                                                  local.tee 32
                                                  i64.const -1
                                                  i64.gt_s
                                                  br_if 0 (;@23;)
                                                  local.get 5
                                                  i64.const 0
                                                  local.get 32
                                                  i64.sub
                                                  local.tee 32
                                                  i64.store offset=56
                                                  i32.const 1
                                                  local.set 28
                                                  i32.const 1024
                                                  local.set 23
                                                  br 1 (;@22;)
                                                end
                                                block  ;; label = @23
                                                  local.get 30
                                                  i32.const 2048
                                                  i32.and
                                                  i32.eqz
                                                  br_if 0 (;@23;)
                                                  i32.const 1
                                                  local.set 28
                                                  i32.const 1025
                                                  local.set 23
                                                  br 1 (;@22;)
                                                end
                                                i32.const 1026
                                                i32.const 1024
                                                local.get 30
                                                i32.const 1
                                                i32.and
                                                local.tee 28
                                                select
                                                local.set 23
                                              end
                                              block  ;; label = @22
                                                block  ;; label = @23
                                                  local.get 32
                                                  i64.const 4294967296
                                                  i64.ge_u
                                                  br_if 0 (;@23;)
                                                  local.get 32
                                                  local.set 33
                                                  local.get 16
                                                  local.set 20
                                                  br 1 (;@22;)
                                                end
                                                local.get 16
                                                local.set 20
                                                loop  ;; label = @23
                                                  local.get 20
                                                  i32.const -1
                                                  i32.add
                                                  local.tee 20
                                                  local.get 32
                                                  local.get 32
                                                  i64.const 10
                                                  i64.div_u
                                                  local.tee 33
                                                  i64.const 10
                                                  i64.mul
                                                  i64.sub
                                                  i32.wrap_i64
                                                  i32.const 48
                                                  i32.or
                                                  i32.store8
                                                  local.get 32
                                                  i64.const 42949672959
                                                  i64.gt_u
                                                  local.set 19
                                                  local.get 33
                                                  local.set 32
                                                  local.get 19
                                                  br_if 0 (;@23;)
                                                end
                                              end
                                              local.get 33
                                              i32.wrap_i64
                                              local.tee 19
                                              i32.eqz
                                              br_if 0 (;@21;)
                                              loop  ;; label = @22
                                                local.get 20
                                                i32.const -1
                                                i32.add
                                                local.tee 20
                                                local.get 19
                                                local.get 19
                                                i32.const 10
                                                i32.div_u
                                                local.tee 22
                                                i32.const 10
                                                i32.mul
                                                i32.sub
                                                i32.const 48
                                                i32.or
                                                i32.store8
                                                local.get 19
                                                i32.const 9
                                                i32.gt_u
                                                local.set 25
                                                local.get 22
                                                local.set 19
                                                local.get 25
                                                br_if 0 (;@22;)
                                              end
                                            end
                                            local.get 27
                                            local.get 24
                                            i32.const 0
                                            i32.lt_s
                                            i32.and
                                            br_if 17 (;@3;)
                                            local.get 30
                                            i32.const -65537
                                            i32.and
                                            local.get 30
                                            local.get 27
                                            select
                                            local.set 29
                                            block  ;; label = @21
                                              local.get 5
                                              i64.load offset=56
                                              local.tee 32
                                              i64.const 0
                                              i64.ne
                                              br_if 0 (;@21;)
                                              i32.const 0
                                              local.set 25
                                              local.get 24
                                              br_if 0 (;@21;)
                                              local.get 16
                                              local.set 20
                                              local.get 16
                                              local.set 19
                                              br 12 (;@9;)
                                            end
                                            local.get 24
                                            local.get 16
                                            local.get 20
                                            i32.sub
                                            local.get 32
                                            i64.eqz
                                            i32.add
                                            local.tee 19
                                            local.get 24
                                            local.get 19
                                            i32.gt_s
                                            select
                                            local.set 25
                                            local.get 16
                                            local.set 19
                                            br 11 (;@9;)
                                          end
                                          local.get 5
                                          local.get 5
                                          i64.load offset=56
                                          i64.store8 offset=55
                                          i32.const 0
                                          local.set 28
                                          i32.const 1024
                                          local.set 23
                                          i32.const 1
                                          local.set 25
                                          local.get 10
                                          local.set 20
                                          local.get 16
                                          local.set 19
                                          br 10 (;@9;)
                                        end
                                        i32.const 4724
                                        i32.load
                                        call $strerror
                                        local.set 20
                                        br 1 (;@17;)
                                      end
                                      local.get 5
                                      i32.load offset=56
                                      local.tee 19
                                      i32.const 1097
                                      local.get 19
                                      select
                                      local.set 20
                                    end
                                    local.get 20
                                    local.get 20
                                    local.get 24
                                    i32.const 2147483647
                                    local.get 24
                                    i32.const 2147483647
                                    i32.lt_u
                                    select
                                    call $strnlen
                                    local.tee 25
                                    i32.add
                                    local.set 19
                                    i32.const 0
                                    local.set 28
                                    i32.const 1024
                                    local.set 23
                                    local.get 24
                                    i32.const -1
                                    i32.gt_s
                                    br_if 7 (;@9;)
                                    local.get 19
                                    i32.load8_u
                                    i32.eqz
                                    br_if 7 (;@9;)
                                    br 13 (;@3;)
                                  end
                                  local.get 5
                                  i32.load offset=56
                                  local.set 20
                                  local.get 24
                                  br_if 1 (;@14;)
                                  i32.const 0
                                  local.set 19
                                  br 2 (;@13;)
                                end
                                local.get 5
                                i32.const 0
                                i32.store offset=12
                                local.get 5
                                local.get 5
                                i64.load offset=56
                                i64.store32 offset=8
                                local.get 5
                                local.get 5
                                i32.const 8
                                i32.add
                                i32.store offset=56
                                local.get 5
                                i32.const 8
                                i32.add
                                local.set 20
                                i32.const -1
                                local.set 24
                              end
                              i32.const 0
                              local.set 19
                              local.get 20
                              local.set 21
                              block  ;; label = @14
                                loop  ;; label = @15
                                  local.get 21
                                  i32.load
                                  local.tee 22
                                  i32.eqz
                                  br_if 1 (;@14;)
                                  local.get 5
                                  i32.const 4
                                  i32.add
                                  local.get 22
                                  call $wctomb
                                  local.tee 22
                                  i32.const 0
                                  i32.lt_s
                                  br_if 13 (;@2;)
                                  local.get 22
                                  local.get 24
                                  local.get 19
                                  i32.sub
                                  i32.gt_u
                                  br_if 1 (;@14;)
                                  local.get 21
                                  i32.const 4
                                  i32.add
                                  local.set 21
                                  local.get 22
                                  local.get 19
                                  i32.add
                                  local.tee 19
                                  local.get 24
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 19
                              i32.const 0
                              i32.lt_s
                              br_if 10 (;@3;)
                            end
                            block  ;; label = @13
                              local.get 30
                              i32.const 73728
                              i32.and
                              local.tee 25
                              br_if 0 (;@13;)
                              local.get 26
                              local.get 19
                              i32.le_s
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 112
                              i32.add
                              i32.const 32
                              local.get 26
                              local.get 19
                              i32.sub
                              local.tee 21
                              i32.const 256
                              local.get 21
                              i32.const 256
                              i32.lt_u
                              local.tee 22
                              select
                              call $memset
                              drop
                              block  ;; label = @14
                                local.get 22
                                br_if 0 (;@14;)
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    local.get 0
                                    i32.load8_u
                                    i32.const 32
                                    i32.and
                                    br_if 0 (;@16;)
                                    local.get 5
                                    i32.const 112
                                    i32.add
                                    i32.const 256
                                    local.get 0
                                    call $__fwritex
                                    drop
                                  end
                                  local.get 21
                                  i32.const -256
                                  i32.add
                                  local.tee 21
                                  i32.const 255
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 0
                              i32.load8_u
                              i32.const 32
                              i32.and
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 112
                              i32.add
                              local.get 21
                              local.get 0
                              call $__fwritex
                              drop
                            end
                            block  ;; label = @13
                              local.get 19
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 0
                              local.set 21
                              loop  ;; label = @14
                                local.get 20
                                i32.load
                                local.tee 22
                                i32.eqz
                                br_if 1 (;@13;)
                                local.get 5
                                i32.const 4
                                i32.add
                                local.get 22
                                call $wctomb
                                local.tee 22
                                local.get 21
                                i32.add
                                local.tee 21
                                local.get 19
                                i32.gt_u
                                br_if 1 (;@13;)
                                block  ;; label = @15
                                  local.get 0
                                  i32.load8_u
                                  i32.const 32
                                  i32.and
                                  br_if 0 (;@15;)
                                  local.get 5
                                  i32.const 4
                                  i32.add
                                  local.get 22
                                  local.get 0
                                  call $__fwritex
                                  drop
                                end
                                local.get 20
                                i32.const 4
                                i32.add
                                local.set 20
                                local.get 21
                                local.get 19
                                i32.lt_u
                                br_if 0 (;@14;)
                              end
                            end
                            block  ;; label = @13
                              local.get 25
                              i32.const 8192
                              i32.ne
                              br_if 0 (;@13;)
                              local.get 26
                              local.get 19
                              i32.le_s
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 112
                              i32.add
                              i32.const 32
                              local.get 26
                              local.get 19
                              i32.sub
                              local.tee 21
                              i32.const 256
                              local.get 21
                              i32.const 256
                              i32.lt_u
                              local.tee 22
                              select
                              call $memset
                              drop
                              block  ;; label = @14
                                local.get 22
                                br_if 0 (;@14;)
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    local.get 0
                                    i32.load8_u
                                    i32.const 32
                                    i32.and
                                    br_if 0 (;@16;)
                                    local.get 5
                                    i32.const 112
                                    i32.add
                                    i32.const 256
                                    local.get 0
                                    call $__fwritex
                                    drop
                                  end
                                  local.get 21
                                  i32.const -256
                                  i32.add
                                  local.tee 21
                                  i32.const 255
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 0
                              i32.load8_u
                              i32.const 32
                              i32.and
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 112
                              i32.add
                              local.get 21
                              local.get 0
                              call $__fwritex
                              drop
                            end
                            local.get 26
                            local.get 19
                            local.get 26
                            local.get 19
                            i32.gt_s
                            select
                            local.set 19
                            br 8 (;@4;)
                          end
                          local.get 27
                          local.get 24
                          i32.const 0
                          i32.lt_s
                          local.tee 19
                          i32.and
                          br_if 8 (;@3;)
                          local.get 5
                          f64.load offset=56
                          local.set 34
                          local.get 5
                          i32.const 0
                          i32.store offset=108
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 34
                              i64.reinterpret_f64
                              i64.const -1
                              i64.gt_s
                              br_if 0 (;@13;)
                              local.get 34
                              f64.neg
                              local.set 34
                              i32.const 1
                              local.set 35
                              i32.const 0
                              local.set 36
                              i32.const 1034
                              local.set 37
                              br 1 (;@12;)
                            end
                            block  ;; label = @13
                              local.get 30
                              i32.const 2048
                              i32.and
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 1
                              local.set 35
                              i32.const 0
                              local.set 36
                              i32.const 1037
                              local.set 37
                              br 1 (;@12;)
                            end
                            i32.const 1040
                            i32.const 1035
                            local.get 30
                            i32.const 1
                            i32.and
                            local.tee 35
                            select
                            local.set 37
                            local.get 35
                            i32.eqz
                            local.set 36
                          end
                          block  ;; label = @12
                            local.get 34
                            f64.abs
                            f64.const inf (;=inf;)
                            f64.lt
                            br_if 0 (;@12;)
                            local.get 35
                            i32.const 3
                            i32.add
                            local.set 21
                            block  ;; label = @13
                              local.get 30
                              i32.const 8192
                              i32.and
                              br_if 0 (;@13;)
                              local.get 26
                              local.get 21
                              i32.le_s
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 624
                              i32.add
                              i32.const 32
                              local.get 26
                              local.get 21
                              i32.sub
                              local.tee 19
                              i32.const 256
                              local.get 19
                              i32.const 256
                              i32.lt_u
                              local.tee 22
                              select
                              call $memset
                              drop
                              block  ;; label = @14
                                local.get 22
                                br_if 0 (;@14;)
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    local.get 0
                                    i32.load8_u
                                    i32.const 32
                                    i32.and
                                    br_if 0 (;@16;)
                                    local.get 5
                                    i32.const 624
                                    i32.add
                                    i32.const 256
                                    local.get 0
                                    call $__fwritex
                                    drop
                                  end
                                  local.get 19
                                  i32.const -256
                                  i32.add
                                  local.tee 19
                                  i32.const 255
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 0
                              i32.load8_u
                              i32.const 32
                              i32.and
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 624
                              i32.add
                              local.get 19
                              local.get 0
                              call $__fwritex
                              drop
                            end
                            block  ;; label = @13
                              local.get 0
                              i32.load
                              local.tee 19
                              i32.const 32
                              i32.and
                              br_if 0 (;@13;)
                              local.get 37
                              local.get 35
                              local.get 0
                              call $__fwritex
                              drop
                              local.get 0
                              i32.load
                              local.set 19
                            end
                            block  ;; label = @13
                              local.get 19
                              i32.const 32
                              i32.and
                              br_if 0 (;@13;)
                              i32.const 1066
                              i32.const 1087
                              local.get 31
                              i32.const 32
                              i32.and
                              local.tee 19
                              select
                              i32.const 1070
                              i32.const 1091
                              local.get 19
                              select
                              local.get 34
                              local.get 34
                              f64.ne
                              select
                              i32.const 3
                              local.get 0
                              call $__fwritex
                              drop
                            end
                            block  ;; label = @13
                              local.get 30
                              i32.const 73728
                              i32.and
                              i32.const 8192
                              i32.ne
                              br_if 0 (;@13;)
                              local.get 26
                              local.get 21
                              i32.le_s
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 624
                              i32.add
                              i32.const 32
                              local.get 26
                              local.get 21
                              i32.sub
                              local.tee 19
                              i32.const 256
                              local.get 19
                              i32.const 256
                              i32.lt_u
                              local.tee 22
                              select
                              call $memset
                              drop
                              block  ;; label = @14
                                local.get 22
                                br_if 0 (;@14;)
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    local.get 0
                                    i32.load8_u
                                    i32.const 32
                                    i32.and
                                    br_if 0 (;@16;)
                                    local.get 5
                                    i32.const 624
                                    i32.add
                                    i32.const 256
                                    local.get 0
                                    call $__fwritex
                                    drop
                                  end
                                  local.get 19
                                  i32.const -256
                                  i32.add
                                  local.tee 19
                                  i32.const 255
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 0
                              i32.load8_u
                              i32.const 32
                              i32.and
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 624
                              i32.add
                              local.get 19
                              local.get 0
                              call $__fwritex
                              drop
                            end
                            local.get 21
                            local.get 26
                            local.get 21
                            local.get 26
                            i32.gt_s
                            select
                            local.set 19
                            br 8 (;@4;)
                          end
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 34
                                local.get 5
                                i32.const 108
                                i32.add
                                call $frexp
                                local.tee 34
                                local.get 34
                                f64.add
                                local.tee 34
                                f64.const 0x0p+0 (;=0;)
                                f64.eq
                                br_if 0 (;@14;)
                                local.get 5
                                local.get 5
                                i32.load offset=108
                                local.tee 21
                                i32.const -1
                                i32.add
                                i32.store offset=108
                                local.get 31
                                i32.const 32
                                i32.or
                                local.tee 38
                                i32.const 97
                                i32.ne
                                br_if 1 (;@13;)
                                br 8 (;@6;)
                              end
                              local.get 31
                              i32.const 32
                              i32.or
                              local.tee 38
                              i32.const 97
                              i32.eq
                              br_if 7 (;@6;)
                              i32.const 6
                              local.get 24
                              local.get 19
                              select
                              local.set 27
                              local.get 5
                              i32.load offset=108
                              local.set 20
                              br 1 (;@12;)
                            end
                            local.get 5
                            local.get 21
                            i32.const -29
                            i32.add
                            local.tee 20
                            i32.store offset=108
                            i32.const 6
                            local.get 24
                            local.get 19
                            select
                            local.set 27
                            local.get 34
                            f64.const 0x1p+28 (;=2.68435e+08;)
                            f64.mul
                            local.set 34
                          end
                          local.get 5
                          i32.const 112
                          i32.add
                          i32.const 0
                          i32.const 72
                          local.get 20
                          i32.const 0
                          i32.lt_s
                          local.tee 39
                          select
                          i32.const 2
                          i32.shl
                          local.tee 40
                          i32.add
                          local.tee 23
                          local.set 21
                          loop  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 34
                                f64.const 0x1p+32 (;=4.29497e+09;)
                                f64.lt
                                local.get 34
                                f64.const 0x0p+0 (;=0;)
                                f64.ge
                                i32.and
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 34
                                i32.trunc_f64_u
                                local.set 19
                                br 1 (;@13;)
                              end
                              i32.const 0
                              local.set 19
                            end
                            local.get 21
                            local.get 19
                            i32.store
                            local.get 21
                            i32.const 4
                            i32.add
                            local.set 21
                            local.get 34
                            local.get 19
                            f64.convert_i32_u
                            f64.sub
                            f64.const 0x1.dcd65p+29 (;=1e+09;)
                            f64.mul
                            local.tee 34
                            f64.const 0x0p+0 (;=0;)
                            f64.ne
                            br_if 0 (;@12;)
                          end
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 20
                              i32.const 1
                              i32.ge_s
                              br_if 0 (;@13;)
                              local.get 21
                              local.set 19
                              local.get 23
                              local.set 22
                              br 1 (;@12;)
                            end
                            local.get 23
                            local.set 22
                            loop  ;; label = @13
                              local.get 20
                              i32.const 29
                              local.get 20
                              i32.const 29
                              i32.lt_s
                              select
                              local.set 20
                              block  ;; label = @14
                                local.get 21
                                i32.const -4
                                i32.add
                                local.tee 19
                                local.get 22
                                i32.lt_u
                                br_if 0 (;@14;)
                                local.get 20
                                i64.extend_i32_u
                                local.set 33
                                i64.const 0
                                local.set 32
                                loop  ;; label = @15
                                  local.get 19
                                  local.get 19
                                  i64.load32_u
                                  local.get 33
                                  i64.shl
                                  local.get 32
                                  i64.const 4294967295
                                  i64.and
                                  i64.add
                                  local.tee 32
                                  local.get 32
                                  i64.const 1000000000
                                  i64.div_u
                                  local.tee 32
                                  i64.const 1000000000
                                  i64.mul
                                  i64.sub
                                  i64.store32
                                  local.get 19
                                  i32.const -4
                                  i32.add
                                  local.tee 19
                                  local.get 22
                                  i32.ge_u
                                  br_if 0 (;@15;)
                                end
                                local.get 32
                                i32.wrap_i64
                                local.tee 19
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 22
                                i32.const -4
                                i32.add
                                local.tee 22
                                local.get 19
                                i32.store
                              end
                              block  ;; label = @14
                                loop  ;; label = @15
                                  local.get 21
                                  local.tee 19
                                  local.get 22
                                  i32.le_u
                                  br_if 1 (;@14;)
                                  local.get 19
                                  i32.const -4
                                  i32.add
                                  local.tee 21
                                  i32.load
                                  i32.eqz
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 5
                              local.get 5
                              i32.load offset=108
                              local.get 20
                              i32.sub
                              local.tee 20
                              i32.store offset=108
                              local.get 19
                              local.set 21
                              local.get 20
                              i32.const 0
                              i32.gt_s
                              br_if 0 (;@13;)
                            end
                          end
                          block  ;; label = @12
                            local.get 20
                            i32.const -1
                            i32.gt_s
                            br_if 0 (;@12;)
                            local.get 27
                            i32.const 25
                            i32.add
                            i32.const 9
                            i32.div_u
                            i32.const 1
                            i32.add
                            local.set 41
                            local.get 38
                            i32.const 102
                            i32.eq
                            local.set 42
                            loop  ;; label = @13
                              i32.const 0
                              local.get 20
                              i32.sub
                              local.tee 21
                              i32.const 9
                              local.get 21
                              i32.const 9
                              i32.lt_s
                              select
                              local.set 24
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 22
                                  local.get 19
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                  local.get 22
                                  i32.load
                                  local.set 21
                                  br 1 (;@14;)
                                end
                                i32.const 1000000000
                                local.get 24
                                i32.shr_u
                                local.set 29
                                i32.const -1
                                local.get 24
                                i32.shl
                                i32.const -1
                                i32.xor
                                local.set 28
                                i32.const 0
                                local.set 20
                                local.get 22
                                local.set 21
                                loop  ;; label = @15
                                  local.get 21
                                  local.get 21
                                  i32.load
                                  local.tee 25
                                  local.get 24
                                  i32.shr_u
                                  local.get 20
                                  i32.add
                                  i32.store
                                  local.get 25
                                  local.get 28
                                  i32.and
                                  local.get 29
                                  i32.mul
                                  local.set 20
                                  local.get 21
                                  i32.const 4
                                  i32.add
                                  local.tee 21
                                  local.get 19
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                end
                                local.get 22
                                i32.load
                                local.set 21
                                local.get 20
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 19
                                local.get 20
                                i32.store
                                local.get 19
                                i32.const 4
                                i32.add
                                local.set 19
                              end
                              local.get 5
                              local.get 5
                              i32.load offset=108
                              local.get 24
                              i32.add
                              local.tee 20
                              i32.store offset=108
                              local.get 23
                              local.get 22
                              local.get 21
                              i32.eqz
                              i32.const 2
                              i32.shl
                              i32.add
                              local.tee 22
                              local.get 42
                              select
                              local.tee 21
                              local.get 41
                              i32.const 2
                              i32.shl
                              i32.add
                              local.get 19
                              local.get 19
                              local.get 21
                              i32.sub
                              i32.const 2
                              i32.shr_s
                              local.get 41
                              i32.gt_s
                              select
                              local.set 19
                              local.get 20
                              i32.const 0
                              i32.lt_s
                              br_if 0 (;@13;)
                            end
                          end
                          i32.const 0
                          local.set 25
                          block  ;; label = @12
                            local.get 22
                            local.get 19
                            i32.ge_u
                            br_if 0 (;@12;)
                            local.get 23
                            local.get 22
                            i32.sub
                            i32.const 2
                            i32.shr_s
                            i32.const 9
                            i32.mul
                            local.set 25
                            local.get 22
                            i32.load
                            local.tee 20
                            i32.const 10
                            i32.lt_u
                            br_if 0 (;@12;)
                            i32.const 10
                            local.set 21
                            loop  ;; label = @13
                              local.get 25
                              i32.const 1
                              i32.add
                              local.set 25
                              local.get 20
                              local.get 21
                              i32.const 10
                              i32.mul
                              local.tee 21
                              i32.ge_u
                              br_if 0 (;@13;)
                            end
                          end
                          block  ;; label = @12
                            local.get 27
                            i32.const 0
                            local.get 25
                            local.get 38
                            i32.const 102
                            i32.eq
                            select
                            i32.sub
                            local.get 27
                            i32.const 0
                            i32.ne
                            local.get 38
                            i32.const 103
                            i32.eq
                            local.tee 28
                            i32.and
                            i32.sub
                            local.tee 21
                            local.get 19
                            local.get 23
                            i32.sub
                            i32.const 2
                            i32.shr_s
                            i32.const 9
                            i32.mul
                            i32.const -9
                            i32.add
                            i32.ge_s
                            br_if 0 (;@12;)
                            local.get 5
                            i32.const 112
                            i32.add
                            i32.const 1
                            i32.const 73
                            local.get 39
                            select
                            i32.const 2
                            i32.shl
                            local.tee 39
                            i32.add
                            local.get 21
                            i32.const 9216
                            i32.add
                            local.tee 20
                            i32.const 9
                            i32.div_s
                            local.tee 24
                            i32.const 2
                            i32.shl
                            local.tee 43
                            i32.add
                            local.tee 42
                            i32.const -4096
                            i32.add
                            local.set 29
                            i32.const 10
                            local.set 21
                            block  ;; label = @13
                              local.get 20
                              local.get 24
                              i32.const 9
                              i32.mul
                              i32.sub
                              local.tee 24
                              i32.const 7
                              i32.gt_s
                              br_if 0 (;@13;)
                              i32.const 8
                              local.get 24
                              i32.sub
                              local.tee 41
                              i32.const 7
                              i32.and
                              local.set 20
                              i32.const 10
                              local.set 21
                              block  ;; label = @14
                                local.get 24
                                i32.const -1
                                i32.add
                                i32.const 7
                                i32.lt_u
                                br_if 0 (;@14;)
                                local.get 41
                                i32.const -8
                                i32.and
                                local.set 24
                                i32.const 10
                                local.set 21
                                loop  ;; label = @15
                                  local.get 21
                                  i32.const 100000000
                                  i32.mul
                                  local.set 21
                                  local.get 24
                                  i32.const -8
                                  i32.add
                                  local.tee 24
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 20
                              i32.eqz
                              br_if 0 (;@13;)
                              loop  ;; label = @14
                                local.get 21
                                i32.const 10
                                i32.mul
                                local.set 21
                                local.get 20
                                i32.const -1
                                i32.add
                                local.tee 20
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 42
                            i32.const -4092
                            i32.add
                            local.set 41
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 29
                                i32.load
                                local.tee 20
                                local.get 20
                                local.get 21
                                i32.div_u
                                local.tee 38
                                local.get 21
                                i32.mul
                                i32.sub
                                local.tee 24
                                br_if 0 (;@14;)
                                local.get 41
                                local.get 19
                                i32.eq
                                br_if 1 (;@13;)
                              end
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 38
                                  i32.const 1
                                  i32.and
                                  br_if 0 (;@15;)
                                  f64.const 0x1p+53 (;=9.0072e+15;)
                                  local.set 34
                                  local.get 21
                                  i32.const 1000000000
                                  i32.ne
                                  br_if 1 (;@14;)
                                  local.get 29
                                  local.get 22
                                  i32.le_u
                                  br_if 1 (;@14;)
                                  local.get 42
                                  i32.const -4100
                                  i32.add
                                  i32.load8_u
                                  i32.const 1
                                  i32.and
                                  i32.eqz
                                  br_if 1 (;@14;)
                                end
                                f64.const 0x1.0000000000001p+53 (;=9.0072e+15;)
                                local.set 34
                              end
                              f64.const 0x1p-1 (;=0.5;)
                              f64.const 0x1p+0 (;=1;)
                              f64.const 0x1.8p+0 (;=1.5;)
                              local.get 41
                              local.get 19
                              i32.eq
                              select
                              f64.const 0x1.8p+0 (;=1.5;)
                              local.get 24
                              local.get 21
                              i32.const 1
                              i32.shr_u
                              local.tee 41
                              i32.eq
                              select
                              local.get 24
                              local.get 41
                              i32.lt_u
                              select
                              local.set 44
                              block  ;; label = @14
                                local.get 36
                                br_if 0 (;@14;)
                                local.get 37
                                i32.load8_u
                                i32.const 45
                                i32.ne
                                br_if 0 (;@14;)
                                local.get 44
                                f64.neg
                                local.set 44
                                local.get 34
                                f64.neg
                                local.set 34
                              end
                              local.get 29
                              local.get 20
                              local.get 24
                              i32.sub
                              local.tee 20
                              i32.store
                              local.get 34
                              local.get 44
                              f64.add
                              local.get 34
                              f64.eq
                              br_if 0 (;@13;)
                              local.get 29
                              local.get 20
                              local.get 21
                              i32.add
                              local.tee 21
                              i32.store
                              block  ;; label = @14
                                local.get 21
                                i32.const 1000000000
                                i32.lt_u
                                br_if 0 (;@14;)
                                local.get 9
                                local.get 39
                                local.get 43
                                i32.add
                                i32.add
                                local.set 21
                                loop  ;; label = @15
                                  local.get 21
                                  i32.const 4
                                  i32.add
                                  i32.const 0
                                  i32.store
                                  block  ;; label = @16
                                    local.get 21
                                    local.get 22
                                    i32.ge_u
                                    br_if 0 (;@16;)
                                    local.get 22
                                    i32.const -4
                                    i32.add
                                    local.tee 22
                                    i32.const 0
                                    i32.store
                                  end
                                  local.get 21
                                  local.get 21
                                  i32.load
                                  i32.const 1
                                  i32.add
                                  local.tee 20
                                  i32.store
                                  local.get 21
                                  i32.const -4
                                  i32.add
                                  local.set 21
                                  local.get 20
                                  i32.const 999999999
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                end
                                local.get 21
                                i32.const 4
                                i32.add
                                local.set 29
                              end
                              local.get 23
                              local.get 22
                              i32.sub
                              i32.const 2
                              i32.shr_s
                              i32.const 9
                              i32.mul
                              local.set 25
                              local.get 22
                              i32.load
                              local.tee 20
                              i32.const 10
                              i32.lt_u
                              br_if 0 (;@13;)
                              i32.const 10
                              local.set 21
                              loop  ;; label = @14
                                local.get 25
                                i32.const 1
                                i32.add
                                local.set 25
                                local.get 20
                                local.get 21
                                i32.const 10
                                i32.mul
                                local.tee 21
                                i32.ge_u
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 29
                            i32.const 4
                            i32.add
                            local.tee 21
                            local.get 19
                            local.get 19
                            local.get 21
                            i32.gt_u
                            select
                            local.set 19
                          end
                          local.get 8
                          local.get 19
                          i32.add
                          local.get 40
                          i32.sub
                          local.set 21
                          block  ;; label = @12
                            loop  ;; label = @13
                              local.get 21
                              local.set 20
                              local.get 19
                              local.tee 29
                              local.get 22
                              i32.le_u
                              local.tee 24
                              br_if 1 (;@12;)
                              local.get 20
                              i32.const -4
                              i32.add
                              local.set 21
                              local.get 29
                              i32.const -4
                              i32.add
                              local.tee 19
                              i32.load
                              i32.eqz
                              br_if 0 (;@13;)
                            end
                          end
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 28
                              br_if 0 (;@13;)
                              local.get 30
                              i32.const 8
                              i32.and
                              local.set 41
                              br 1 (;@12;)
                            end
                            local.get 25
                            i32.const -1
                            i32.xor
                            i32.const -1
                            local.get 27
                            i32.const 1
                            local.get 27
                            select
                            local.tee 19
                            local.get 25
                            i32.gt_s
                            local.get 25
                            i32.const -5
                            i32.gt_s
                            i32.and
                            local.tee 21
                            select
                            local.get 19
                            i32.add
                            local.set 27
                            i32.const -1
                            i32.const -2
                            local.get 21
                            select
                            local.get 31
                            i32.add
                            local.set 31
                            local.get 30
                            i32.const 8
                            i32.and
                            local.tee 41
                            br_if 0 (;@12;)
                            i32.const -9
                            local.set 19
                            block  ;; label = @13
                              local.get 24
                              br_if 0 (;@13;)
                              local.get 29
                              i32.const -4
                              i32.add
                              i32.load
                              local.tee 24
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 0
                              local.set 19
                              local.get 24
                              i32.const 10
                              i32.rem_u
                              br_if 0 (;@13;)
                              i32.const 10
                              local.set 21
                              i32.const 0
                              local.set 19
                              loop  ;; label = @14
                                local.get 19
                                i32.const -1
                                i32.add
                                local.set 19
                                local.get 24
                                local.get 21
                                i32.const 10
                                i32.mul
                                local.tee 21
                                i32.rem_u
                                i32.eqz
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 20
                            i32.const 2
                            i32.shr_s
                            i32.const 9
                            i32.mul
                            local.set 21
                            block  ;; label = @13
                              local.get 31
                              i32.const -33
                              i32.and
                              i32.const 70
                              i32.ne
                              br_if 0 (;@13;)
                              i32.const 0
                              local.set 41
                              local.get 27
                              local.get 21
                              local.get 19
                              i32.add
                              i32.const -9
                              i32.add
                              local.tee 19
                              i32.const 0
                              local.get 19
                              i32.const 0
                              i32.gt_s
                              select
                              local.tee 19
                              local.get 27
                              local.get 19
                              i32.lt_s
                              select
                              local.set 27
                              br 1 (;@12;)
                            end
                            i32.const 0
                            local.set 41
                            local.get 27
                            local.get 25
                            local.get 21
                            i32.add
                            local.get 19
                            i32.add
                            i32.const -9
                            i32.add
                            local.tee 19
                            i32.const 0
                            local.get 19
                            i32.const 0
                            i32.gt_s
                            select
                            local.tee 19
                            local.get 27
                            local.get 19
                            i32.lt_s
                            select
                            local.set 27
                          end
                          local.get 27
                          i32.const 2147483645
                          i32.const 2147483646
                          local.get 27
                          local.get 41
                          i32.or
                          local.tee 38
                          select
                          i32.gt_s
                          br_if 8 (;@3;)
                          local.get 27
                          local.get 38
                          i32.const 0
                          i32.ne
                          i32.add
                          i32.const 1
                          i32.add
                          local.set 42
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 31
                              i32.const -33
                              i32.and
                              i32.const 70
                              i32.ne
                              local.tee 40
                              br_if 0 (;@13;)
                              local.get 25
                              local.get 42
                              i32.const 2147483647
                              i32.xor
                              i32.gt_s
                              br_if 10 (;@3;)
                              local.get 25
                              i32.const 0
                              local.get 25
                              i32.const 0
                              i32.gt_s
                              select
                              local.set 19
                              br 1 (;@12;)
                            end
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 25
                                br_if 0 (;@14;)
                                local.get 7
                                local.set 20
                                local.get 7
                                local.set 21
                                br 1 (;@13;)
                              end
                              local.get 25
                              local.get 25
                              i32.const 31
                              i32.shr_s
                              local.tee 19
                              i32.xor
                              local.get 19
                              i32.sub
                              local.set 19
                              local.get 7
                              local.set 20
                              local.get 7
                              local.set 21
                              loop  ;; label = @14
                                local.get 21
                                i32.const -1
                                i32.add
                                local.tee 21
                                local.get 19
                                local.get 19
                                i32.const 10
                                i32.div_u
                                local.tee 24
                                i32.const 10
                                i32.mul
                                i32.sub
                                i32.const 48
                                i32.or
                                i32.store8
                                local.get 20
                                i32.const -1
                                i32.add
                                local.set 20
                                local.get 19
                                i32.const 9
                                i32.gt_u
                                local.set 28
                                local.get 24
                                local.set 19
                                local.get 28
                                br_if 0 (;@14;)
                              end
                            end
                            block  ;; label = @13
                              local.get 7
                              local.get 20
                              i32.sub
                              i32.const 1
                              i32.gt_s
                              br_if 0 (;@13;)
                              local.get 21
                              local.get 15
                              local.get 20
                              i32.sub
                              i32.add
                              local.tee 21
                              i32.const 48
                              local.get 20
                              local.get 5
                              i32.const 68
                              i32.add
                              i32.sub
                              i32.const -10
                              i32.add
                              call $memset
                              drop
                            end
                            local.get 21
                            i32.const -2
                            i32.add
                            local.tee 36
                            local.get 31
                            i32.store8
                            local.get 21
                            i32.const -1
                            i32.add
                            i32.const 45
                            i32.const 43
                            local.get 25
                            i32.const 0
                            i32.lt_s
                            select
                            i32.store8
                            local.get 7
                            local.get 36
                            i32.sub
                            local.tee 19
                            local.get 42
                            i32.const 2147483647
                            i32.xor
                            i32.gt_s
                            br_if 9 (;@3;)
                          end
                          local.get 19
                          local.get 42
                          i32.add
                          local.tee 19
                          local.get 35
                          i32.const 2147483647
                          i32.xor
                          i32.gt_s
                          br_if 8 (;@3;)
                          local.get 19
                          local.get 35
                          i32.add
                          local.set 28
                          block  ;; label = @12
                            local.get 30
                            i32.const 73728
                            i32.and
                            local.tee 30
                            br_if 0 (;@12;)
                            local.get 26
                            local.get 28
                            i32.le_s
                            br_if 0 (;@12;)
                            local.get 5
                            i32.const 624
                            i32.add
                            i32.const 32
                            local.get 26
                            local.get 28
                            i32.sub
                            local.tee 19
                            i32.const 256
                            local.get 19
                            i32.const 256
                            i32.lt_u
                            local.tee 21
                            select
                            call $memset
                            drop
                            block  ;; label = @13
                              local.get 21
                              br_if 0 (;@13;)
                              loop  ;; label = @14
                                block  ;; label = @15
                                  local.get 0
                                  i32.load8_u
                                  i32.const 32
                                  i32.and
                                  br_if 0 (;@15;)
                                  local.get 5
                                  i32.const 624
                                  i32.add
                                  i32.const 256
                                  local.get 0
                                  call $__fwritex
                                  drop
                                end
                                local.get 19
                                i32.const -256
                                i32.add
                                local.tee 19
                                i32.const 255
                                i32.gt_u
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 0
                            i32.load8_u
                            i32.const 32
                            i32.and
                            br_if 0 (;@12;)
                            local.get 5
                            i32.const 624
                            i32.add
                            local.get 19
                            local.get 0
                            call $__fwritex
                            drop
                          end
                          block  ;; label = @12
                            local.get 0
                            i32.load8_u
                            i32.const 32
                            i32.and
                            br_if 0 (;@12;)
                            local.get 37
                            local.get 35
                            local.get 0
                            call $__fwritex
                            drop
                          end
                          block  ;; label = @12
                            local.get 30
                            i32.const 65536
                            i32.ne
                            br_if 0 (;@12;)
                            local.get 26
                            local.get 28
                            i32.le_s
                            br_if 0 (;@12;)
                            local.get 5
                            i32.const 624
                            i32.add
                            i32.const 48
                            local.get 26
                            local.get 28
                            i32.sub
                            local.tee 19
                            i32.const 256
                            local.get 19
                            i32.const 256
                            i32.lt_u
                            local.tee 21
                            select
                            call $memset
                            drop
                            block  ;; label = @13
                              local.get 21
                              br_if 0 (;@13;)
                              loop  ;; label = @14
                                block  ;; label = @15
                                  local.get 0
                                  i32.load8_u
                                  i32.const 32
                                  i32.and
                                  br_if 0 (;@15;)
                                  local.get 5
                                  i32.const 624
                                  i32.add
                                  i32.const 256
                                  local.get 0
                                  call $__fwritex
                                  drop
                                end
                                local.get 19
                                i32.const -256
                                i32.add
                                local.tee 19
                                i32.const 255
                                i32.gt_u
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 0
                            i32.load8_u
                            i32.const 32
                            i32.and
                            br_if 0 (;@12;)
                            local.get 5
                            i32.const 624
                            i32.add
                            local.get 19
                            local.get 0
                            call $__fwritex
                            drop
                          end
                          local.get 40
                          br_if 3 (;@8;)
                          local.get 23
                          local.get 22
                          local.get 22
                          local.get 23
                          i32.gt_u
                          select
                          local.tee 25
                          local.set 24
                          loop  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 24
                                    i32.load
                                    local.tee 19
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    i32.const 8
                                    local.set 21
                                    loop  ;; label = @17
                                      local.get 5
                                      i32.const 80
                                      i32.add
                                      local.get 21
                                      i32.add
                                      local.get 19
                                      local.get 19
                                      i32.const 10
                                      i32.div_u
                                      local.tee 22
                                      i32.const 10
                                      i32.mul
                                      i32.sub
                                      i32.const 48
                                      i32.or
                                      i32.store8
                                      local.get 21
                                      i32.const -1
                                      i32.add
                                      local.set 21
                                      local.get 19
                                      i32.const 9
                                      i32.gt_u
                                      local.set 20
                                      local.get 22
                                      local.set 19
                                      local.get 20
                                      br_if 0 (;@17;)
                                    end
                                    local.get 21
                                    i32.const 1
                                    i32.add
                                    local.tee 22
                                    local.get 5
                                    i32.const 80
                                    i32.add
                                    i32.add
                                    local.set 19
                                    block  ;; label = @17
                                      local.get 24
                                      local.get 25
                                      i32.eq
                                      br_if 0 (;@17;)
                                      local.get 21
                                      i32.const 2
                                      i32.add
                                      i32.const 2
                                      i32.lt_s
                                      br_if 4 (;@13;)
                                      br 3 (;@14;)
                                    end
                                    local.get 21
                                    i32.const 8
                                    i32.ne
                                    br_if 3 (;@13;)
                                    br 1 (;@15;)
                                  end
                                  i32.const 9
                                  local.set 22
                                  local.get 24
                                  local.get 25
                                  i32.ne
                                  br_if 1 (;@14;)
                                end
                                local.get 5
                                i32.const 48
                                i32.store8 offset=88
                                local.get 13
                                local.set 19
                                br 1 (;@13;)
                              end
                              local.get 5
                              i32.const 80
                              i32.add
                              local.get 12
                              local.get 22
                              i32.add
                              local.tee 19
                              local.get 5
                              i32.const 80
                              i32.add
                              local.get 19
                              i32.lt_u
                              select
                              local.tee 19
                              i32.const 48
                              local.get 22
                              local.get 5
                              i32.const 80
                              i32.add
                              i32.add
                              local.get 19
                              i32.sub
                              call $memset
                              drop
                            end
                            block  ;; label = @13
                              local.get 0
                              i32.load8_u
                              i32.const 32
                              i32.and
                              br_if 0 (;@13;)
                              local.get 19
                              local.get 14
                              local.get 19
                              i32.sub
                              local.get 0
                              call $__fwritex
                              drop
                            end
                            local.get 24
                            i32.const 4
                            i32.add
                            local.tee 24
                            local.get 23
                            i32.le_u
                            br_if 0 (;@12;)
                          end
                          block  ;; label = @12
                            local.get 38
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 0
                            i32.load8_u
                            i32.const 32
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 1095
                            i32.const 1
                            local.get 0
                            call $__fwritex
                            drop
                          end
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 27
                              i32.const 1
                              i32.ge_s
                              br_if 0 (;@13;)
                              local.get 27
                              local.set 19
                              br 1 (;@12;)
                            end
                            block  ;; label = @13
                              local.get 24
                              local.get 29
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 27
                              local.set 19
                              br 1 (;@12;)
                            end
                            loop  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 24
                                    i32.load
                                    local.tee 19
                                    br_if 0 (;@16;)
                                    local.get 14
                                    local.set 21
                                    local.get 14
                                    local.set 22
                                    br 1 (;@15;)
                                  end
                                  local.get 14
                                  local.set 22
                                  local.get 14
                                  local.set 21
                                  loop  ;; label = @16
                                    local.get 21
                                    i32.const -1
                                    i32.add
                                    local.tee 21
                                    local.get 19
                                    local.get 19
                                    i32.const 10
                                    i32.div_u
                                    local.tee 20
                                    i32.const 10
                                    i32.mul
                                    i32.sub
                                    i32.const 48
                                    i32.or
                                    i32.store8
                                    local.get 22
                                    i32.const -1
                                    i32.add
                                    local.set 22
                                    local.get 19
                                    i32.const 9
                                    i32.gt_u
                                    local.set 25
                                    local.get 20
                                    local.set 19
                                    local.get 25
                                    br_if 0 (;@16;)
                                  end
                                  local.get 21
                                  local.get 5
                                  i32.const 80
                                  i32.add
                                  i32.le_u
                                  br_if 1 (;@14;)
                                end
                                local.get 21
                                local.get 5
                                i32.const 80
                                i32.add
                                i32.add
                                local.get 22
                                i32.sub
                                local.tee 21
                                i32.const 48
                                local.get 22
                                local.get 5
                                i32.const 80
                                i32.add
                                i32.sub
                                call $memset
                                drop
                              end
                              block  ;; label = @14
                                local.get 0
                                i32.load8_u
                                i32.const 32
                                i32.and
                                br_if 0 (;@14;)
                                local.get 21
                                local.get 27
                                i32.const 9
                                local.get 27
                                i32.const 9
                                i32.lt_s
                                select
                                local.get 0
                                call $__fwritex
                                drop
                              end
                              local.get 27
                              i32.const -9
                              i32.add
                              local.set 19
                              local.get 24
                              i32.const 4
                              i32.add
                              local.tee 24
                              local.get 29
                              i32.ge_u
                              br_if 1 (;@12;)
                              local.get 27
                              i32.const 9
                              i32.gt_s
                              local.set 21
                              local.get 19
                              local.set 27
                              local.get 21
                              br_if 0 (;@13;)
                            end
                          end
                          local.get 0
                          i32.const 48
                          local.get 19
                          i32.const 9
                          i32.add
                          i32.const 9
                          i32.const 0
                          call $pad
                          br 4 (;@7;)
                        end
                        i32.const 4724
                        i32.const 28
                        i32.store
                        br 8 (;@2;)
                      end
                      i32.const 0
                      local.set 28
                      i32.const 1024
                      local.set 23
                      local.get 16
                      local.set 19
                      local.get 30
                      local.set 29
                      local.get 24
                      local.set 25
                    end
                    local.get 25
                    local.get 19
                    local.get 20
                    i32.sub
                    local.tee 24
                    local.get 25
                    local.get 24
                    i32.gt_s
                    select
                    local.tee 27
                    local.get 28
                    i32.const 2147483647
                    i32.xor
                    i32.gt_s
                    br_if 5 (;@3;)
                    local.get 26
                    local.get 28
                    local.get 27
                    i32.add
                    local.tee 22
                    local.get 26
                    local.get 22
                    i32.gt_s
                    select
                    local.tee 19
                    local.get 21
                    i32.gt_s
                    br_if 5 (;@3;)
                    block  ;; label = @9
                      local.get 29
                      i32.const 73728
                      i32.and
                      local.tee 29
                      br_if 0 (;@9;)
                      local.get 26
                      local.get 22
                      i32.le_s
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 112
                      i32.add
                      i32.const 32
                      local.get 19
                      local.get 22
                      i32.sub
                      local.tee 21
                      i32.const 256
                      local.get 21
                      i32.const 256
                      i32.lt_u
                      local.tee 30
                      select
                      call $memset
                      drop
                      block  ;; label = @10
                        local.get 30
                        br_if 0 (;@10;)
                        loop  ;; label = @11
                          block  ;; label = @12
                            local.get 0
                            i32.load8_u
                            i32.const 32
                            i32.and
                            br_if 0 (;@12;)
                            local.get 5
                            i32.const 112
                            i32.add
                            i32.const 256
                            local.get 0
                            call $__fwritex
                            drop
                          end
                          local.get 21
                          i32.const -256
                          i32.add
                          local.tee 21
                          i32.const 255
                          i32.gt_u
                          br_if 0 (;@11;)
                        end
                      end
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 112
                      i32.add
                      local.get 21
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    block  ;; label = @9
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 23
                      local.get 28
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    block  ;; label = @9
                      local.get 29
                      i32.const 65536
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 26
                      local.get 22
                      i32.le_s
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 112
                      i32.add
                      i32.const 48
                      local.get 19
                      local.get 22
                      i32.sub
                      local.tee 21
                      i32.const 256
                      local.get 21
                      i32.const 256
                      i32.lt_u
                      local.tee 28
                      select
                      call $memset
                      drop
                      block  ;; label = @10
                        local.get 28
                        br_if 0 (;@10;)
                        loop  ;; label = @11
                          block  ;; label = @12
                            local.get 0
                            i32.load8_u
                            i32.const 32
                            i32.and
                            br_if 0 (;@12;)
                            local.get 5
                            i32.const 112
                            i32.add
                            i32.const 256
                            local.get 0
                            call $__fwritex
                            drop
                          end
                          local.get 21
                          i32.const -256
                          i32.add
                          local.tee 21
                          i32.const 255
                          i32.gt_u
                          br_if 0 (;@11;)
                        end
                      end
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 112
                      i32.add
                      local.get 21
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    block  ;; label = @9
                      local.get 25
                      local.get 24
                      i32.le_s
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 112
                      i32.add
                      i32.const 48
                      local.get 27
                      local.get 24
                      i32.sub
                      local.tee 21
                      i32.const 256
                      local.get 21
                      i32.const 256
                      i32.lt_u
                      local.tee 25
                      select
                      call $memset
                      drop
                      block  ;; label = @10
                        local.get 25
                        br_if 0 (;@10;)
                        loop  ;; label = @11
                          block  ;; label = @12
                            local.get 0
                            i32.load8_u
                            i32.const 32
                            i32.and
                            br_if 0 (;@12;)
                            local.get 5
                            i32.const 112
                            i32.add
                            i32.const 256
                            local.get 0
                            call $__fwritex
                            drop
                          end
                          local.get 21
                          i32.const -256
                          i32.add
                          local.tee 21
                          i32.const 255
                          i32.gt_u
                          br_if 0 (;@11;)
                        end
                      end
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 112
                      i32.add
                      local.get 21
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    block  ;; label = @9
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 20
                      local.get 24
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    local.get 29
                    i32.const 8192
                    i32.ne
                    br_if 4 (;@4;)
                    local.get 26
                    local.get 22
                    i32.le_s
                    br_if 4 (;@4;)
                    local.get 5
                    i32.const 112
                    i32.add
                    i32.const 32
                    local.get 19
                    local.get 22
                    i32.sub
                    local.tee 21
                    i32.const 256
                    local.get 21
                    i32.const 256
                    i32.lt_u
                    local.tee 22
                    select
                    call $memset
                    drop
                    block  ;; label = @9
                      local.get 22
                      br_if 0 (;@9;)
                      loop  ;; label = @10
                        block  ;; label = @11
                          local.get 0
                          i32.load8_u
                          i32.const 32
                          i32.and
                          br_if 0 (;@11;)
                          local.get 5
                          i32.const 112
                          i32.add
                          i32.const 256
                          local.get 0
                          call $__fwritex
                          drop
                        end
                        local.get 21
                        i32.const -256
                        i32.add
                        local.tee 21
                        i32.const 255
                        i32.gt_u
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 0
                    i32.load8_u
                    i32.const 32
                    i32.and
                    br_if 4 (;@4;)
                    local.get 5
                    i32.const 112
                    i32.add
                    local.get 21
                    local.get 0
                    call $__fwritex
                    drop
                    br 4 (;@4;)
                  end
                  block  ;; label = @8
                    local.get 27
                    i32.const 0
                    i32.lt_s
                    br_if 0 (;@8;)
                    local.get 29
                    local.get 22
                    i32.const 4
                    i32.add
                    local.get 29
                    local.get 22
                    i32.gt_u
                    select
                    local.set 29
                    local.get 22
                    local.set 24
                    loop  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 24
                          i32.load
                          local.tee 19
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 14
                          local.set 21
                          loop  ;; label = @12
                            local.get 21
                            i32.const -1
                            i32.add
                            local.tee 21
                            local.get 19
                            local.get 19
                            i32.const 10
                            i32.div_u
                            local.tee 20
                            i32.const 10
                            i32.mul
                            i32.sub
                            i32.const 48
                            i32.or
                            i32.store8
                            local.get 19
                            i32.const 10
                            i32.lt_u
                            local.set 25
                            local.get 20
                            local.set 19
                            local.get 25
                            i32.eqz
                            br_if 0 (;@12;)
                            br 2 (;@10;)
                          end
                        end
                        local.get 5
                        i32.const 48
                        i32.store8 offset=88
                        local.get 13
                        local.set 21
                      end
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 24
                          local.get 22
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 21
                          local.get 5
                          i32.const 80
                          i32.add
                          i32.le_u
                          br_if 1 (;@10;)
                          local.get 5
                          i32.const 80
                          i32.add
                          i32.const 48
                          local.get 21
                          local.get 5
                          i32.const 80
                          i32.add
                          i32.sub
                          call $memset
                          drop
                          local.get 5
                          i32.const 80
                          i32.add
                          local.set 21
                          br 1 (;@10;)
                        end
                        block  ;; label = @11
                          local.get 0
                          i32.load8_u
                          i32.const 32
                          i32.and
                          br_if 0 (;@11;)
                          local.get 21
                          i32.const 1
                          local.get 0
                          call $__fwritex
                          drop
                        end
                        local.get 21
                        i32.const 1
                        i32.add
                        local.set 21
                        block  ;; label = @11
                          local.get 41
                          br_if 0 (;@11;)
                          local.get 27
                          i32.const 1
                          i32.lt_s
                          br_if 1 (;@10;)
                        end
                        local.get 0
                        i32.load8_u
                        i32.const 32
                        i32.and
                        br_if 0 (;@10;)
                        i32.const 1095
                        i32.const 1
                        local.get 0
                        call $__fwritex
                        drop
                      end
                      local.get 14
                      local.get 21
                      i32.sub
                      local.set 19
                      block  ;; label = @10
                        local.get 0
                        i32.load8_u
                        i32.const 32
                        i32.and
                        br_if 0 (;@10;)
                        local.get 21
                        local.get 19
                        local.get 27
                        local.get 19
                        local.get 27
                        i32.lt_s
                        select
                        local.get 0
                        call $__fwritex
                        drop
                      end
                      local.get 27
                      local.get 19
                      i32.sub
                      local.set 27
                      local.get 24
                      i32.const 4
                      i32.add
                      local.tee 24
                      local.get 29
                      i32.ge_u
                      br_if 1 (;@8;)
                      local.get 27
                      i32.const -1
                      i32.gt_s
                      br_if 0 (;@9;)
                    end
                  end
                  local.get 0
                  i32.const 48
                  local.get 27
                  i32.const 18
                  i32.add
                  i32.const 18
                  i32.const 0
                  call $pad
                  local.get 0
                  i32.load8_u
                  i32.const 32
                  i32.and
                  br_if 0 (;@7;)
                  local.get 36
                  local.get 7
                  local.get 36
                  i32.sub
                  local.get 0
                  call $__fwritex
                  drop
                end
                local.get 30
                i32.const 8192
                i32.ne
                br_if 1 (;@5;)
                local.get 26
                local.get 28
                i32.le_s
                br_if 1 (;@5;)
                local.get 5
                i32.const 624
                i32.add
                i32.const 32
                local.get 26
                local.get 28
                i32.sub
                local.tee 19
                i32.const 256
                local.get 19
                i32.const 256
                i32.lt_u
                local.tee 21
                select
                call $memset
                drop
                block  ;; label = @7
                  local.get 21
                  br_if 0 (;@7;)
                  loop  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 624
                      i32.add
                      i32.const 256
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    local.get 19
                    i32.const -256
                    i32.add
                    local.tee 19
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@8;)
                  end
                end
                local.get 0
                i32.load8_u
                i32.const 32
                i32.and
                br_if 1 (;@5;)
                local.get 5
                i32.const 624
                i32.add
                local.get 19
                local.get 0
                call $__fwritex
                drop
                br 1 (;@5;)
              end
              local.get 37
              local.get 31
              i32.const 26
              i32.shl
              i32.const 31
              i32.shr_s
              i32.const 9
              i32.and
              i32.add
              local.set 23
              block  ;; label = @6
                local.get 24
                i32.const 11
                i32.gt_u
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 12
                    local.get 24
                    i32.sub
                    local.tee 19
                    i32.const 7
                    i32.and
                    local.tee 21
                    br_if 0 (;@8;)
                    f64.const 0x1p+4 (;=16;)
                    local.set 44
                    br 1 (;@7;)
                  end
                  local.get 24
                  i32.const -12
                  i32.add
                  local.set 19
                  f64.const 0x1p+4 (;=16;)
                  local.set 44
                  loop  ;; label = @8
                    local.get 19
                    i32.const 1
                    i32.add
                    local.set 19
                    local.get 44
                    f64.const 0x1p+4 (;=16;)
                    f64.mul
                    local.set 44
                    local.get 21
                    i32.const -1
                    i32.add
                    local.tee 21
                    br_if 0 (;@8;)
                  end
                  i32.const 0
                  local.get 19
                  i32.sub
                  local.set 19
                end
                block  ;; label = @7
                  local.get 24
                  i32.const -5
                  i32.add
                  i32.const 7
                  i32.lt_u
                  br_if 0 (;@7;)
                  loop  ;; label = @8
                    local.get 44
                    f64.const 0x1p+4 (;=16;)
                    f64.mul
                    f64.const 0x1p+4 (;=16;)
                    f64.mul
                    f64.const 0x1p+4 (;=16;)
                    f64.mul
                    f64.const 0x1p+4 (;=16;)
                    f64.mul
                    f64.const 0x1p+4 (;=16;)
                    f64.mul
                    f64.const 0x1p+4 (;=16;)
                    f64.mul
                    f64.const 0x1p+4 (;=16;)
                    f64.mul
                    f64.const 0x1p+4 (;=16;)
                    f64.mul
                    local.set 44
                    local.get 19
                    i32.const -8
                    i32.add
                    local.tee 19
                    br_if 0 (;@8;)
                  end
                end
                block  ;; label = @7
                  local.get 23
                  i32.load8_u
                  i32.const 45
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 44
                  local.get 34
                  f64.neg
                  local.get 44
                  f64.sub
                  f64.add
                  f64.neg
                  local.set 34
                  br 1 (;@6;)
                end
                local.get 34
                local.get 44
                f64.add
                local.get 44
                f64.sub
                local.set 34
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 5
                  i32.load offset=108
                  local.tee 25
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 25
                  local.get 25
                  i32.const 31
                  i32.shr_s
                  local.tee 19
                  i32.xor
                  local.get 19
                  i32.sub
                  local.set 19
                  local.get 7
                  local.set 21
                  loop  ;; label = @8
                    local.get 21
                    i32.const -1
                    i32.add
                    local.tee 21
                    local.get 19
                    local.get 19
                    i32.const 10
                    i32.div_u
                    local.tee 22
                    i32.const 10
                    i32.mul
                    i32.sub
                    i32.const 48
                    i32.or
                    i32.store8
                    local.get 19
                    i32.const 10
                    i32.lt_u
                    local.set 20
                    local.get 22
                    local.set 19
                    local.get 20
                    i32.eqz
                    br_if 0 (;@8;)
                    br 2 (;@6;)
                  end
                end
                local.get 5
                i32.const 48
                i32.store8 offset=79
                local.get 11
                local.set 21
              end
              local.get 35
              i32.const 2
              i32.or
              local.set 29
              local.get 31
              i32.const 32
              i32.and
              local.set 22
              local.get 21
              i32.const -2
              i32.add
              local.tee 27
              local.get 31
              i32.const 15
              i32.add
              i32.store8
              local.get 21
              i32.const -1
              i32.add
              i32.const 45
              i32.const 43
              local.get 25
              i32.const 0
              i32.lt_s
              select
              i32.store8
              local.get 30
              i32.const 8
              i32.and
              local.set 20
              local.get 5
              i32.const 80
              i32.add
              local.set 21
              loop  ;; label = @6
                local.get 21
                local.set 19
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 34
                    f64.abs
                    f64.const 0x1p+31 (;=2.14748e+09;)
                    f64.lt
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 34
                    i32.trunc_f64_s
                    local.set 21
                    br 1 (;@7;)
                  end
                  i32.const -2147483648
                  local.set 21
                end
                local.get 19
                local.get 21
                i32.const 3456
                i32.add
                i32.load8_u
                local.get 22
                i32.or
                i32.store8
                local.get 34
                local.get 21
                f64.convert_i32_s
                f64.sub
                f64.const 0x1p+4 (;=16;)
                f64.mul
                local.set 34
                block  ;; label = @7
                  local.get 19
                  i32.const 1
                  i32.add
                  local.tee 21
                  local.get 5
                  i32.const 80
                  i32.add
                  i32.sub
                  i32.const 1
                  i32.ne
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 20
                    br_if 0 (;@8;)
                    local.get 24
                    i32.const 0
                    i32.gt_s
                    br_if 0 (;@8;)
                    local.get 34
                    f64.const 0x0p+0 (;=0;)
                    f64.eq
                    br_if 1 (;@7;)
                  end
                  local.get 19
                  i32.const 46
                  i32.store8 offset=1
                  local.get 19
                  i32.const 2
                  i32.add
                  local.set 21
                end
                local.get 34
                f64.const 0x0p+0 (;=0;)
                f64.ne
                br_if 0 (;@6;)
              end
              i32.const 2147483645
              local.get 7
              local.get 27
              i32.sub
              local.tee 25
              local.get 29
              i32.add
              local.tee 19
              i32.sub
              local.get 24
              i32.lt_s
              br_if 2 (;@3;)
              local.get 24
              i32.const 2
              i32.add
              local.get 21
              local.get 5
              i32.const 80
              i32.add
              i32.sub
              local.tee 21
              local.get 21
              i32.const -2
              i32.add
              local.get 24
              i32.lt_s
              select
              local.get 21
              local.get 24
              select
              local.tee 20
              local.get 19
              i32.add
              local.set 28
              block  ;; label = @6
                local.get 30
                i32.const 73728
                i32.and
                local.tee 22
                br_if 0 (;@6;)
                local.get 26
                local.get 28
                i32.le_s
                br_if 0 (;@6;)
                local.get 5
                i32.const 624
                i32.add
                i32.const 32
                local.get 26
                local.get 28
                i32.sub
                local.tee 19
                i32.const 256
                local.get 19
                i32.const 256
                i32.lt_u
                local.tee 24
                select
                call $memset
                drop
                block  ;; label = @7
                  local.get 24
                  br_if 0 (;@7;)
                  loop  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 624
                      i32.add
                      i32.const 256
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    local.get 19
                    i32.const -256
                    i32.add
                    local.tee 19
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@8;)
                  end
                end
                local.get 0
                i32.load8_u
                i32.const 32
                i32.and
                br_if 0 (;@6;)
                local.get 5
                i32.const 624
                i32.add
                local.get 19
                local.get 0
                call $__fwritex
                drop
              end
              block  ;; label = @6
                local.get 0
                i32.load8_u
                i32.const 32
                i32.and
                br_if 0 (;@6;)
                local.get 23
                local.get 29
                local.get 0
                call $__fwritex
                drop
              end
              block  ;; label = @6
                local.get 22
                i32.const 65536
                i32.ne
                br_if 0 (;@6;)
                local.get 26
                local.get 28
                i32.le_s
                br_if 0 (;@6;)
                local.get 5
                i32.const 624
                i32.add
                i32.const 48
                local.get 26
                local.get 28
                i32.sub
                local.tee 19
                i32.const 256
                local.get 19
                i32.const 256
                i32.lt_u
                local.tee 24
                select
                call $memset
                drop
                block  ;; label = @7
                  local.get 24
                  br_if 0 (;@7;)
                  loop  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 624
                      i32.add
                      i32.const 256
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    local.get 19
                    i32.const -256
                    i32.add
                    local.tee 19
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@8;)
                  end
                end
                local.get 0
                i32.load8_u
                i32.const 32
                i32.and
                br_if 0 (;@6;)
                local.get 5
                i32.const 624
                i32.add
                local.get 19
                local.get 0
                call $__fwritex
                drop
              end
              block  ;; label = @6
                local.get 0
                i32.load8_u
                i32.const 32
                i32.and
                br_if 0 (;@6;)
                local.get 5
                i32.const 80
                i32.add
                local.get 21
                local.get 0
                call $__fwritex
                drop
              end
              block  ;; label = @6
                local.get 20
                local.get 21
                i32.sub
                local.tee 19
                i32.const 1
                i32.lt_s
                br_if 0 (;@6;)
                local.get 5
                i32.const 624
                i32.add
                i32.const 48
                local.get 19
                i32.const 256
                local.get 19
                i32.const 256
                i32.lt_u
                local.tee 21
                select
                call $memset
                drop
                block  ;; label = @7
                  local.get 21
                  br_if 0 (;@7;)
                  loop  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load8_u
                      i32.const 32
                      i32.and
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 624
                      i32.add
                      i32.const 256
                      local.get 0
                      call $__fwritex
                      drop
                    end
                    local.get 19
                    i32.const -256
                    i32.add
                    local.tee 19
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@8;)
                  end
                end
                local.get 0
                i32.load8_u
                i32.const 32
                i32.and
                br_if 0 (;@6;)
                local.get 5
                i32.const 624
                i32.add
                local.get 19
                local.get 0
                call $__fwritex
                drop
              end
              block  ;; label = @6
                local.get 0
                i32.load8_u
                i32.const 32
                i32.and
                br_if 0 (;@6;)
                local.get 27
                local.get 25
                local.get 0
                call $__fwritex
                drop
              end
              local.get 22
              i32.const 8192
              i32.ne
              br_if 0 (;@5;)
              local.get 26
              local.get 28
              i32.le_s
              br_if 0 (;@5;)
              local.get 5
              i32.const 624
              i32.add
              i32.const 32
              local.get 26
              local.get 28
              i32.sub
              local.tee 19
              i32.const 256
              local.get 19
              i32.const 256
              i32.lt_u
              local.tee 21
              select
              call $memset
              drop
              block  ;; label = @6
                local.get 21
                br_if 0 (;@6;)
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.load8_u
                    i32.const 32
                    i32.and
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 624
                    i32.add
                    i32.const 256
                    local.get 0
                    call $__fwritex
                    drop
                  end
                  local.get 19
                  i32.const -256
                  i32.add
                  local.tee 19
                  i32.const 255
                  i32.gt_u
                  br_if 0 (;@7;)
                end
              end
              local.get 0
              i32.load8_u
              i32.const 32
              i32.and
              br_if 0 (;@5;)
              local.get 5
              i32.const 624
              i32.add
              local.get 19
              local.get 0
              call $__fwritex
              drop
            end
            local.get 28
            local.get 26
            local.get 28
            local.get 26
            i32.gt_s
            select
            local.tee 19
            i32.const 0
            i32.ge_s
            br_if 0 (;@4;)
          end
        end
        i32.const 4724
        i32.const 61
        i32.store
      end
      i32.const -1
      local.set 18
    end
    local.get 5
    i32.const 880
    i32.add
    global.set $__stack_pointer
    local.get 18)
  (func $pop_arg (type 9) (param i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          local.get 1
                                          i32.const -9
                                          i32.add
                                          br_table 17 (;@2;) 0 (;@19;) 1 (;@18;) 4 (;@15;) 2 (;@17;) 3 (;@16;) 5 (;@14;) 6 (;@13;) 7 (;@12;) 8 (;@11;) 9 (;@10;) 10 (;@9;) 11 (;@8;) 12 (;@7;) 13 (;@6;) 14 (;@5;) 15 (;@4;) 16 (;@3;) 18 (;@1;)
                                        end
                                        local.get 2
                                        local.get 2
                                        i32.load
                                        local.tee 1
                                        i32.const 4
                                        i32.add
                                        i32.store
                                        local.get 0
                                        local.get 1
                                        i64.load32_s
                                        i64.store
                                        return
                                      end
                                      local.get 2
                                      local.get 2
                                      i32.load
                                      local.tee 1
                                      i32.const 4
                                      i32.add
                                      i32.store
                                      local.get 0
                                      local.get 1
                                      i64.load32_u
                                      i64.store
                                      return
                                    end
                                    local.get 2
                                    local.get 2
                                    i32.load
                                    local.tee 1
                                    i32.const 4
                                    i32.add
                                    i32.store
                                    local.get 0
                                    local.get 1
                                    i64.load32_s
                                    i64.store
                                    return
                                  end
                                  local.get 2
                                  local.get 2
                                  i32.load
                                  local.tee 1
                                  i32.const 4
                                  i32.add
                                  i32.store
                                  local.get 0
                                  local.get 1
                                  i64.load32_u
                                  i64.store
                                  return
                                end
                                local.get 2
                                local.get 2
                                i32.load
                                i32.const 7
                                i32.add
                                i32.const -8
                                i32.and
                                local.tee 1
                                i32.const 8
                                i32.add
                                i32.store
                                local.get 0
                                local.get 1
                                i64.load
                                i64.store
                                return
                              end
                              local.get 2
                              local.get 2
                              i32.load
                              local.tee 1
                              i32.const 4
                              i32.add
                              i32.store
                              local.get 0
                              local.get 1
                              i64.load16_s
                              i64.store
                              return
                            end
                            local.get 2
                            local.get 2
                            i32.load
                            local.tee 1
                            i32.const 4
                            i32.add
                            i32.store
                            local.get 0
                            local.get 1
                            i64.load16_u
                            i64.store
                            return
                          end
                          local.get 2
                          local.get 2
                          i32.load
                          local.tee 1
                          i32.const 4
                          i32.add
                          i32.store
                          local.get 0
                          local.get 1
                          i64.load8_s
                          i64.store
                          return
                        end
                        local.get 2
                        local.get 2
                        i32.load
                        local.tee 1
                        i32.const 4
                        i32.add
                        i32.store
                        local.get 0
                        local.get 1
                        i64.load8_u
                        i64.store
                        return
                      end
                      local.get 2
                      local.get 2
                      i32.load
                      i32.const 7
                      i32.add
                      i32.const -8
                      i32.and
                      local.tee 1
                      i32.const 8
                      i32.add
                      i32.store
                      local.get 0
                      local.get 1
                      i64.load
                      i64.store
                      return
                    end
                    local.get 2
                    local.get 2
                    i32.load
                    local.tee 1
                    i32.const 4
                    i32.add
                    i32.store
                    local.get 0
                    local.get 1
                    i64.load32_u
                    i64.store
                    return
                  end
                  local.get 2
                  local.get 2
                  i32.load
                  i32.const 7
                  i32.add
                  i32.const -8
                  i32.and
                  local.tee 1
                  i32.const 8
                  i32.add
                  i32.store
                  local.get 0
                  local.get 1
                  i64.load
                  i64.store
                  return
                end
                local.get 2
                local.get 2
                i32.load
                i32.const 7
                i32.add
                i32.const -8
                i32.and
                local.tee 1
                i32.const 8
                i32.add
                i32.store
                local.get 0
                local.get 1
                i64.load
                i64.store
                return
              end
              local.get 2
              local.get 2
              i32.load
              local.tee 1
              i32.const 4
              i32.add
              i32.store
              local.get 0
              local.get 1
              i64.load32_s
              i64.store
              return
            end
            local.get 2
            local.get 2
            i32.load
            local.tee 1
            i32.const 4
            i32.add
            i32.store
            local.get 0
            local.get 1
            i64.load32_u
            i64.store
            return
          end
          local.get 2
          local.get 2
          i32.load
          i32.const 7
          i32.add
          i32.const -8
          i32.and
          local.tee 1
          i32.const 8
          i32.add
          i32.store
          local.get 0
          local.get 1
          f64.load
          f64.store
          return
        end
        call $long_double_not_supported
        unreachable
      end
      local.get 2
      local.get 2
      i32.load
      local.tee 1
      i32.const 4
      i32.add
      i32.store
      local.get 0
      local.get 1
      i32.load
      i32.store
    end)
  (func $pad (type 14) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 256
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 2
      local.get 3
      i32.le_s
      br_if 0 (;@1;)
      local.get 4
      i32.const 73728
      i32.and
      br_if 0 (;@1;)
      local.get 5
      local.get 1
      local.get 2
      local.get 3
      i32.sub
      local.tee 3
      i32.const 256
      local.get 3
      i32.const 256
      i32.lt_u
      local.tee 4
      select
      call $memset
      local.set 2
      block  ;; label = @2
        local.get 4
        br_if 0 (;@2;)
        loop  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load8_u
            i32.const 32
            i32.and
            br_if 0 (;@4;)
            local.get 2
            i32.const 256
            local.get 0
            call $__fwritex
            drop
          end
          local.get 3
          i32.const -256
          i32.add
          local.tee 3
          i32.const 255
          i32.gt_u
          br_if 0 (;@3;)
        end
      end
      local.get 0
      i32.load8_u
      i32.const 32
      i32.and
      br_if 0 (;@1;)
      local.get 2
      local.get 3
      local.get 0
      call $__fwritex
      drop
    end
    local.get 5
    i32.const 256
    i32.add
    global.set $__stack_pointer)
  (func $long_double_not_supported (type 8)
    i32.const 1104
    i32.const 3984
    call $fputs
    drop
    call $abort
    unreachable)
  (func $vsnprintf (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    local.get 0
    local.get 4
    i32.const 126
    i32.add
    local.get 1
    select
    local.tee 5
    i32.store offset=116
    i32.const -1
    local.set 0
    local.get 4
    i32.const 0
    local.get 1
    i32.const -1
    i32.add
    local.tee 6
    local.get 6
    local.get 1
    i32.gt_u
    select
    i32.store offset=120
    local.get 4
    i32.const 0
    i32.const 112
    call $memset
    local.tee 4
    i32.const -1
    i32.store offset=64
    local.get 4
    i32.const 5
    i32.store offset=32
    local.get 4
    local.get 4
    i32.const 116
    i32.add
    i32.store offset=68
    local.get 4
    local.get 4
    i32.const 127
    i32.add
    i32.store offset=40
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const -1
        i32.gt_s
        br_if 0 (;@2;)
        i32.const 0
        i32.const 61
        i32.store offset=4724
        br 1 (;@1;)
      end
      local.get 5
      i32.const 0
      i32.store8
      local.get 4
      local.get 2
      local.get 3
      call $vfprintf
      local.set 0
    end
    local.get 4
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $sn_write (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=68
    local.tee 3
    i32.load
    local.set 4
    block  ;; label = @1
      local.get 3
      i32.load offset=4
      local.tee 5
      local.get 0
      i32.load offset=20
      local.get 0
      i32.load offset=24
      local.tee 6
      i32.sub
      local.tee 7
      local.get 5
      local.get 7
      i32.lt_u
      select
      local.tee 7
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      local.get 6
      local.get 7
      call $memcpy
      drop
      local.get 3
      local.get 3
      i32.load
      local.get 7
      i32.add
      local.tee 4
      i32.store
      local.get 3
      local.get 3
      i32.load offset=4
      local.get 7
      i32.sub
      local.tee 5
      i32.store offset=4
    end
    block  ;; label = @1
      local.get 5
      local.get 2
      local.get 5
      local.get 2
      i32.lt_u
      select
      local.tee 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      local.get 1
      local.get 5
      call $memcpy
      drop
      local.get 3
      local.get 3
      i32.load
      local.get 5
      i32.add
      local.tee 4
      i32.store
      local.get 3
      local.get 3
      i32.load offset=4
      local.get 5
      i32.sub
      i32.store offset=4
    end
    local.get 4
    i32.const 0
    i32.store8
    local.get 0
    local.get 0
    i32.load offset=40
    local.tee 3
    i32.store offset=24
    local.get 0
    local.get 3
    i32.store offset=20
    local.get 2)
  (func $vsprintf (type 0) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 2147483647
    local.get 1
    local.get 2
    call $vsnprintf)
  (func $__toread (type 4) (param i32) (result i32)
    (local i32 i32)
    local.get 0
    local.get 0
    i32.load offset=60
    local.tee 1
    i32.const -1
    i32.add
    local.get 1
    i32.or
    i32.store offset=60
    block  ;; label = @1
      local.get 0
      i32.load offset=20
      local.get 0
      i32.load offset=24
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      i32.const 0
      local.get 0
      i32.load offset=32
      call_indirect (type 0)
      drop
    end
    local.get 0
    i32.const 0
    i32.store offset=24
    local.get 0
    i64.const 0
    i64.store offset=16
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.const 4
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.const 32
      i32.or
      i32.store
      i32.const -1
      return
    end
    local.get 0
    local.get 0
    i32.load offset=40
    local.get 0
    i32.load offset=44
    i32.add
    local.tee 2
    i32.store offset=8
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 1
    i32.const 27
    i32.shl
    i32.const 31
    i32.shr_s)
  (func $__uflow (type 4) (param i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const -1
    local.set 2
    block  ;; label = @1
      local.get 0
      call $__toread
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.const 15
      i32.add
      i32.const 1
      local.get 0
      i32.load offset=28
      call_indirect (type 0)
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.load8_u offset=15
      local.set 2
    end
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2)
  (func $__shlim (type 15) (param i32 i64)
    (local i32 i32)
    local.get 0
    local.get 1
    i64.store offset=88
    local.get 0
    local.get 0
    i32.load offset=40
    local.get 0
    i32.load offset=4
    local.tee 2
    i32.sub
    i64.extend_i32_s
    i64.store offset=96
    local.get 0
    i32.load offset=8
    local.set 3
    block  ;; label = @1
      local.get 1
      i64.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      i32.sub
      i64.extend_i32_s
      local.get 1
      i64.le_s
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i32.wrap_i64
      i32.add
      local.set 3
    end
    local.get 0
    local.get 3
    i32.store offset=84)
  (func $__shgetc (type 4) (param i32) (result i32)
    (local i32 i32 i64 i64 i32)
    local.get 0
    i64.load offset=96
    local.get 0
    i32.load offset=4
    local.tee 1
    local.get 0
    i32.load offset=40
    local.tee 2
    i32.sub
    i64.extend_i32_s
    i64.add
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i64.load offset=88
          local.tee 4
          i64.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 4
          i64.ge_s
          br_if 1 (;@2;)
        end
        local.get 0
        call $__uflow
        local.tee 2
        i32.const -1
        i32.gt_s
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=4
        local.set 1
        local.get 0
        i32.load offset=40
        local.set 2
      end
      local.get 0
      i64.const -1
      i64.store offset=88
      local.get 0
      local.get 1
      i32.store offset=84
      local.get 0
      local.get 3
      local.get 2
      local.get 1
      i32.sub
      i64.extend_i32_s
      i64.add
      i64.store offset=96
      i32.const -1
      return
    end
    local.get 3
    i64.const 1
    i64.add
    local.set 3
    local.get 0
    i32.load offset=4
    local.set 1
    local.get 0
    i32.load offset=8
    local.set 5
    block  ;; label = @1
      local.get 0
      i64.load offset=88
      local.tee 4
      i64.const 0
      i64.eq
      br_if 0 (;@1;)
      local.get 4
      local.get 3
      i64.sub
      local.tee 4
      local.get 5
      local.get 1
      i32.sub
      i64.extend_i32_s
      i64.ge_s
      br_if 0 (;@1;)
      local.get 1
      local.get 4
      i32.wrap_i64
      i32.add
      local.set 5
    end
    local.get 0
    local.get 5
    i32.store offset=84
    local.get 0
    local.get 3
    local.get 0
    i32.load offset=40
    local.tee 5
    local.get 1
    i32.sub
    i64.extend_i32_s
    i64.add
    i64.store offset=96
    block  ;; label = @1
      local.get 1
      local.get 5
      i32.gt_u
      br_if 0 (;@1;)
      local.get 1
      i32.const -1
      i32.add
      local.get 2
      i32.store8
    end
    local.get 2)
  (func $__intscan (type 16) (param i32 i32 i32 i64) (result i64)
    (local i32 i32 i32 i64 i64 i64 i32 i64 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.const 36
              i32.gt_u
              br_if 0 (;@5;)
              local.get 1
              i32.const 1
              i32.eq
              br_if 0 (;@5;)
              block  ;; label = @6
                block  ;; label = @7
                  loop  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.load offset=4
                        local.tee 5
                        local.get 0
                        i32.load offset=84
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 5
                        i32.const 1
                        i32.add
                        i32.store offset=4
                        local.get 5
                        i32.load8_u
                        local.set 5
                        br 1 (;@9;)
                      end
                      local.get 0
                      call $__shgetc
                      local.set 5
                    end
                    local.get 5
                    i32.const -9
                    i32.add
                    i32.const 5
                    i32.lt_u
                    br_if 0 (;@8;)
                    block  ;; label = @9
                      local.get 5
                      i32.const -32
                      i32.add
                      br_table 1 (;@8;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 2 (;@7;) 0 (;@9;) 2 (;@7;) 0 (;@9;) 2 (;@7;)
                    end
                  end
                  i32.const -1
                  i32.const 0
                  local.get 5
                  i32.const 45
                  i32.eq
                  select
                  local.set 6
                  block  ;; label = @8
                    local.get 0
                    i32.load offset=4
                    local.tee 5
                    local.get 0
                    i32.load offset=84
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 5
                    i32.const 1
                    i32.add
                    i32.store offset=4
                    local.get 5
                    i32.load8_u
                    local.set 5
                    br 2 (;@6;)
                  end
                  local.get 0
                  call $__shgetc
                  local.set 5
                  br 1 (;@6;)
                end
                i32.const 0
                local.set 6
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 1
                  i32.const 0
                  i32.ne
                  local.get 1
                  i32.const 16
                  i32.ne
                  i32.and
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 48
                  i32.ne
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load offset=4
                      local.tee 5
                      local.get 0
                      i32.load offset=84
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 5
                      i32.const 1
                      i32.add
                      i32.store offset=4
                      local.get 5
                      i32.load8_u
                      local.set 5
                      br 1 (;@8;)
                    end
                    local.get 0
                    call $__shgetc
                    local.set 5
                  end
                  block  ;; label = @8
                    local.get 5
                    i32.const -33
                    i32.and
                    i32.const 88
                    i32.ne
                    br_if 0 (;@8;)
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.load offset=4
                        local.tee 5
                        local.get 0
                        i32.load offset=84
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 5
                        i32.const 1
                        i32.add
                        i32.store offset=4
                        local.get 5
                        i32.load8_u
                        local.set 5
                        br 1 (;@9;)
                      end
                      local.get 0
                      call $__shgetc
                      local.set 5
                    end
                    i32.const 16
                    local.set 1
                    local.get 5
                    i32.const 3473
                    i32.add
                    i32.load8_u
                    i32.const 16
                    i32.lt_u
                    br_if 4 (;@4;)
                    i64.const 0
                    local.set 3
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i64.load offset=88
                        i64.const 0
                        i64.lt_s
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 0
                        i32.load offset=4
                        local.tee 5
                        i32.const -1
                        i32.add
                        i32.store offset=4
                        local.get 2
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 0
                        local.get 5
                        i32.const -2
                        i32.add
                        i32.store offset=4
                        br 9 (;@1;)
                      end
                      local.get 2
                      br_if 8 (;@1;)
                    end
                    i64.const 0
                    local.set 3
                    local.get 0
                    i64.const 0
                    call $__shlim
                    br 7 (;@1;)
                  end
                  local.get 1
                  br_if 1 (;@6;)
                  i32.const 8
                  local.set 1
                  br 3 (;@4;)
                end
                local.get 1
                i32.const 10
                local.get 1
                select
                local.tee 1
                local.get 5
                i32.const 3473
                i32.add
                i32.load8_u
                i32.gt_u
                br_if 0 (;@6;)
                i64.const 0
                local.set 3
                block  ;; label = @7
                  local.get 0
                  i64.load offset=88
                  i64.const 0
                  i64.lt_s
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 0
                  i32.load offset=4
                  i32.const -1
                  i32.add
                  i32.store offset=4
                end
                local.get 0
                i64.const 0
                call $__shlim
                i32.const 0
                i32.const 28
                i32.store offset=4724
                br 5 (;@1;)
              end
              local.get 1
              i32.const 10
              i32.ne
              br_if 1 (;@4;)
              i64.const 0
              local.set 7
              block  ;; label = @6
                local.get 5
                i32.const -48
                i32.add
                local.tee 2
                i32.const 9
                i32.gt_u
                br_if 0 (;@6;)
                i32.const 0
                local.set 5
                loop  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load offset=4
                      local.tee 1
                      local.get 0
                      i32.load offset=84
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 1
                      i32.const 1
                      i32.add
                      i32.store offset=4
                      local.get 1
                      i32.load8_u
                      local.set 1
                      br 1 (;@8;)
                    end
                    local.get 0
                    call $__shgetc
                    local.set 1
                  end
                  local.get 5
                  i32.const 10
                  i32.mul
                  local.get 2
                  i32.add
                  local.set 5
                  block  ;; label = @8
                    local.get 1
                    i32.const -48
                    i32.add
                    local.tee 2
                    i32.const 9
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 429496729
                    i32.lt_u
                    br_if 1 (;@7;)
                  end
                end
                local.get 5
                i64.extend_i32_u
                local.set 7
              end
              local.get 2
              i32.const 9
              i32.gt_u
              br_if 3 (;@2;)
              local.get 7
              i64.const 10
              i64.mul
              local.set 8
              local.get 2
              i64.extend_i32_u
              local.set 9
              loop  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.load offset=4
                    local.tee 5
                    local.get 0
                    i32.load offset=84
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 5
                    i32.const 1
                    i32.add
                    i32.store offset=4
                    local.get 5
                    i32.load8_u
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 0
                  call $__shgetc
                  local.set 5
                end
                local.get 8
                local.get 9
                i64.add
                local.set 7
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 5
                    i32.const -48
                    i32.add
                    local.tee 2
                    i32.const 9
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 7
                    i64.const 1844674407370955162
                    i64.lt_u
                    br_if 1 (;@7;)
                  end
                  i32.const 10
                  local.set 1
                  local.get 2
                  i32.const 9
                  i32.le_u
                  br_if 4 (;@3;)
                  br 5 (;@2;)
                end
                local.get 7
                i64.const 10
                i64.mul
                local.tee 8
                local.get 2
                i64.extend_i32_u
                local.tee 9
                i64.const -1
                i64.xor
                i64.le_u
                br_if 0 (;@6;)
              end
              i32.const 10
              local.set 1
              br 2 (;@3;)
            end
            i32.const 0
            i32.const 28
            i32.store offset=4724
            i64.const 0
            local.set 3
            br 3 (;@1;)
          end
          block  ;; label = @4
            local.get 1
            local.get 1
            i32.const -1
            i32.add
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            i64.const 0
            local.set 7
            block  ;; label = @5
              local.get 1
              local.get 5
              i32.const 3473
              i32.add
              i32.load8_u
              local.tee 10
              i32.le_u
              br_if 0 (;@5;)
              i32.const 0
              local.set 2
              loop  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.load offset=4
                    local.tee 5
                    local.get 0
                    i32.load offset=84
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 5
                    i32.const 1
                    i32.add
                    i32.store offset=4
                    local.get 5
                    i32.load8_u
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 0
                  call $__shgetc
                  local.set 5
                end
                local.get 10
                local.get 2
                local.get 1
                i32.mul
                i32.add
                local.set 2
                block  ;; label = @7
                  local.get 1
                  local.get 5
                  i32.const 3473
                  i32.add
                  i32.load8_u
                  local.tee 10
                  i32.le_u
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 119304647
                  i32.lt_u
                  br_if 1 (;@6;)
                end
              end
              local.get 2
              i64.extend_i32_u
              local.set 7
            end
            local.get 1
            local.get 10
            i32.le_u
            br_if 1 (;@3;)
            local.get 1
            i64.extend_i32_u
            local.set 8
            loop  ;; label = @5
              local.get 7
              local.get 8
              i64.mul
              local.tee 9
              local.get 10
              i64.extend_i32_u
              i64.const 255
              i64.and
              local.tee 11
              i64.const -1
              i64.xor
              i64.gt_u
              br_if 2 (;@3;)
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load offset=4
                  local.tee 5
                  local.get 0
                  i32.load offset=84
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 5
                  i32.const 1
                  i32.add
                  i32.store offset=4
                  local.get 5
                  i32.load8_u
                  local.set 5
                  br 1 (;@6;)
                end
                local.get 0
                call $__shgetc
                local.set 5
              end
              local.get 9
              local.get 11
              i64.add
              local.set 7
              local.get 1
              local.get 5
              i32.const 3473
              i32.add
              i32.load8_u
              local.tee 10
              i32.le_u
              br_if 2 (;@3;)
              local.get 4
              local.get 8
              i64.const 0
              local.get 7
              i64.const 0
              call $__multi3
              local.get 4
              i64.load offset=8
              i64.const 0
              i64.ne
              br_if 2 (;@3;)
              br 0 (;@5;)
            end
          end
          local.get 1
          i32.const 23
          i32.mul
          i32.const 5
          i32.shr_u
          i32.const 7
          i32.and
          i32.const 3729
          i32.add
          i32.load8_s
          local.set 12
          i64.const 0
          local.set 7
          block  ;; label = @4
            local.get 1
            local.get 5
            i32.const 3473
            i32.add
            i32.load8_u
            local.tee 2
            i32.le_u
            br_if 0 (;@4;)
            i32.const 0
            local.set 10
            loop  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load offset=4
                  local.tee 5
                  local.get 0
                  i32.load offset=84
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 5
                  i32.const 1
                  i32.add
                  i32.store offset=4
                  local.get 5
                  i32.load8_u
                  local.set 5
                  br 1 (;@6;)
                end
                local.get 0
                call $__shgetc
                local.set 5
              end
              local.get 2
              local.get 10
              local.get 12
              i32.shl
              i32.or
              local.set 10
              block  ;; label = @6
                local.get 1
                local.get 5
                i32.const 3473
                i32.add
                i32.load8_u
                local.tee 2
                i32.le_u
                br_if 0 (;@6;)
                local.get 10
                i32.const 134217728
                i32.lt_u
                br_if 1 (;@5;)
              end
            end
            local.get 10
            i64.extend_i32_u
            local.set 7
          end
          local.get 1
          local.get 2
          i32.le_u
          br_if 0 (;@3;)
          i64.const -1
          local.get 12
          i64.extend_i32_u
          local.tee 9
          i64.shr_u
          local.tee 11
          local.get 7
          i64.lt_u
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 2
            i64.extend_i32_u
            i64.const 255
            i64.and
            local.set 8
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 5
                local.get 0
                i32.load offset=84
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 5
                i32.const 1
                i32.add
                i32.store offset=4
                local.get 5
                i32.load8_u
                local.set 5
                br 1 (;@5;)
              end
              local.get 0
              call $__shgetc
              local.set 5
            end
            local.get 7
            local.get 9
            i64.shl
            local.get 8
            i64.or
            local.set 7
            local.get 1
            local.get 5
            i32.const 3473
            i32.add
            i32.load8_u
            local.tee 2
            i32.le_u
            br_if 1 (;@3;)
            local.get 7
            local.get 11
            i64.le_u
            br_if 0 (;@4;)
          end
        end
        local.get 1
        local.get 5
        i32.const 3473
        i32.add
        i32.load8_u
        i32.le_u
        br_if 0 (;@2;)
        loop  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load offset=4
              local.tee 5
              local.get 0
              i32.load offset=84
              i32.eq
              br_if 0 (;@5;)
              local.get 0
              local.get 5
              i32.const 1
              i32.add
              i32.store offset=4
              local.get 5
              i32.load8_u
              local.set 5
              br 1 (;@4;)
            end
            local.get 0
            call $__shgetc
            local.set 5
          end
          local.get 1
          local.get 5
          i32.const 3473
          i32.add
          i32.load8_u
          i32.gt_u
          br_if 0 (;@3;)
        end
        i32.const 0
        i32.const 68
        i32.store offset=4724
        local.get 6
        i32.const 0
        local.get 3
        i64.const 1
        i64.and
        i64.eqz
        select
        local.set 6
        local.get 3
        local.set 7
      end
      block  ;; label = @2
        local.get 0
        i64.load offset=88
        i64.const 0
        i64.lt_s
        br_if 0 (;@2;)
        local.get 0
        local.get 0
        i32.load offset=4
        i32.const -1
        i32.add
        i32.store offset=4
      end
      block  ;; label = @2
        local.get 7
        local.get 3
        i64.lt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 3
          i32.wrap_i64
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          local.get 6
          br_if 0 (;@3;)
          i32.const 0
          i32.const 68
          i32.store offset=4724
          local.get 3
          i64.const -1
          i64.add
          local.set 3
          br 2 (;@1;)
        end
        local.get 7
        local.get 3
        i64.le_u
        br_if 0 (;@2;)
        i32.const 0
        i32.const 68
        i32.store offset=4724
        br 1 (;@1;)
      end
      local.get 7
      local.get 6
      i64.extend_i32_s
      local.tee 3
      i64.xor
      local.get 3
      i64.sub
      local.set 3
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3)
  (func $scalbn (type 12) (param f64 i32) (result f64)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 1024
        i32.lt_s
        br_if 0 (;@2;)
        local.get 0
        f64.const 0x1p+1023 (;=8.98847e+307;)
        f64.mul
        local.set 0
        block  ;; label = @3
          local.get 1
          i32.const 2047
          i32.ge_u
          br_if 0 (;@3;)
          local.get 1
          i32.const -1023
          i32.add
          local.set 1
          br 2 (;@1;)
        end
        local.get 0
        f64.const 0x1p+1023 (;=8.98847e+307;)
        f64.mul
        local.set 0
        local.get 1
        i32.const 3069
        local.get 1
        i32.const 3069
        i32.lt_s
        select
        i32.const -2046
        i32.add
        local.set 1
        br 1 (;@1;)
      end
      local.get 1
      i32.const -1023
      i32.gt_s
      br_if 0 (;@1;)
      local.get 0
      f64.const 0x1p-969 (;=2.00417e-292;)
      f64.mul
      local.set 0
      block  ;; label = @2
        local.get 1
        i32.const -1992
        i32.le_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 969
        i32.add
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      f64.const 0x1p-969 (;=2.00417e-292;)
      f64.mul
      local.set 0
      local.get 1
      i32.const -2960
      local.get 1
      i32.const -2960
      i32.gt_s
      select
      i32.const 1938
      i32.add
      local.set 1
    end
    local.get 0
    local.get 1
    i32.const 1023
    i32.add
    i64.extend_i32_u
    i64.const 52
    i64.shl
    f64.reinterpret_i64
    f64.mul)
  (func $fmod (type 17) (param f64 f64) (result f64)
    (local i64 i64 i64 i32 i64 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i64.reinterpret_f64
        local.tee 2
        i64.const 1
        i64.shl
        local.tee 3
        i64.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 1
        f64.ne
        br_if 0 (;@2;)
        local.get 0
        i64.reinterpret_f64
        local.tee 4
        i64.const 52
        i64.shr_u
        i32.wrap_i64
        i32.const 2047
        i32.and
        local.tee 5
        i32.const 2047
        i32.ne
        br_if 1 (;@1;)
      end
      local.get 0
      local.get 1
      f64.mul
      local.tee 1
      local.get 1
      f64.div
      return
    end
    block  ;; label = @1
      local.get 4
      i64.const 1
      i64.shl
      local.tee 6
      local.get 3
      i64.gt_u
      br_if 0 (;@1;)
      local.get 0
      f64.const 0x0p+0 (;=0;)
      f64.mul
      local.get 0
      local.get 6
      local.get 3
      i64.eq
      select
      return
    end
    local.get 2
    i64.const 52
    i64.shr_u
    i32.wrap_i64
    i32.const 2047
    i32.and
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        local.get 5
        br_if 0 (;@2;)
        i32.const 0
        local.set 5
        block  ;; label = @3
          local.get 4
          i64.const 12
          i64.shl
          local.tee 3
          i64.const 0
          i64.lt_s
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 5
            i32.const -1
            i32.add
            local.set 5
            local.get 3
            i64.const 1
            i64.shl
            local.tee 3
            i64.const -1
            i64.gt_s
            br_if 0 (;@4;)
          end
        end
        local.get 4
        i32.const 1
        local.get 5
        i32.sub
        i64.extend_i32_u
        i64.shl
        local.set 3
        br 1 (;@1;)
      end
      local.get 4
      i64.const 4503599627370495
      i64.and
      i64.const 4503599627370496
      i64.or
      local.set 3
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 7
        br_if 0 (;@2;)
        i32.const 0
        local.set 7
        block  ;; label = @3
          local.get 2
          i64.const 12
          i64.shl
          local.tee 6
          i64.const 0
          i64.lt_s
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 7
            i32.const -1
            i32.add
            local.set 7
            local.get 6
            i64.const 1
            i64.shl
            local.tee 6
            i64.const -1
            i64.gt_s
            br_if 0 (;@4;)
          end
        end
        local.get 2
        i32.const 1
        local.get 7
        i32.sub
        i64.extend_i32_u
        i64.shl
        local.set 2
        br 1 (;@1;)
      end
      local.get 2
      i64.const 4503599627370495
      i64.and
      i64.const 4503599627370496
      i64.or
      local.set 2
    end
    block  ;; label = @1
      local.get 5
      local.get 7
      i32.le_s
      br_if 0 (;@1;)
      loop  ;; label = @2
        block  ;; label = @3
          local.get 3
          local.get 2
          i64.sub
          local.tee 6
          i64.const 0
          i64.lt_s
          br_if 0 (;@3;)
          local.get 6
          local.set 3
          local.get 6
          i64.const 0
          i64.ne
          br_if 0 (;@3;)
          local.get 0
          f64.const 0x0p+0 (;=0;)
          f64.mul
          return
        end
        local.get 3
        i64.const 1
        i64.shl
        local.set 3
        local.get 5
        i32.const -1
        i32.add
        local.tee 5
        local.get 7
        i32.gt_s
        br_if 0 (;@2;)
      end
      local.get 7
      local.set 5
    end
    block  ;; label = @1
      local.get 3
      local.get 2
      i64.sub
      local.tee 6
      i64.const 0
      i64.lt_s
      br_if 0 (;@1;)
      local.get 6
      local.set 3
      local.get 6
      i64.const 0
      i64.ne
      br_if 0 (;@1;)
      local.get 0
      f64.const 0x0p+0 (;=0;)
      f64.mul
      return
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i64.const 4503599627370495
        i64.le_u
        br_if 0 (;@2;)
        local.get 3
        local.set 6
        br 1 (;@1;)
      end
      loop  ;; label = @2
        local.get 5
        i32.const -1
        i32.add
        local.set 5
        local.get 3
        i64.const 2251799813685248
        i64.lt_u
        local.set 7
        local.get 3
        i64.const 1
        i64.shl
        local.tee 6
        local.set 3
        local.get 7
        br_if 0 (;@2;)
      end
    end
    local.get 4
    i64.const -9223372036854775808
    i64.and
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 5
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        local.get 6
        i64.const -4503599627370496
        i64.add
        local.get 5
        i64.extend_i32_u
        i64.const 52
        i64.shl
        i64.or
        local.set 6
        br 1 (;@1;)
      end
      local.get 6
      i32.const 1
      local.get 5
      i32.sub
      i64.extend_i32_u
      i64.shr_u
      local.set 6
    end
    local.get 6
    local.get 3
    i64.or
    f64.reinterpret_i64)
  (func $__floatscan (type 18) (param i32 i32 i32) (result f64)
    (local i32 i32 i32 i32 f64 i32 i32 i32 i64 i32 i32 i32 i32 i32 i32 i64 i32 i64 i32 i32 f64 f64 f64)
    global.get $__stack_pointer
    i32.const 512
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const -149
    local.set 4
    i32.const 24
    local.set 5
    i32.const 0
    local.set 6
    f64.const 0x0p+0 (;=0;)
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          br_table 1 (;@2;) 0 (;@3;) 0 (;@3;) 2 (;@1;)
        end
        i32.const -1074
        local.set 4
        i32.const 53
        local.set 5
        i32.const 1
        local.set 6
      end
      block  ;; label = @2
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 1
                local.get 0
                i32.load offset=84
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 1
                i32.const 1
                i32.add
                i32.store offset=4
                local.get 1
                i32.load8_u
                local.set 1
                br 1 (;@5;)
              end
              local.get 0
              call $__shgetc
              local.set 1
            end
            local.get 1
            i32.const -9
            i32.add
            i32.const 5
            i32.lt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 1
              i32.const -32
              i32.add
              br_table 1 (;@4;) 2 (;@3;) 2 (;@3;) 2 (;@3;) 2 (;@3;) 2 (;@3;) 2 (;@3;) 2 (;@3;) 2 (;@3;) 2 (;@3;) 2 (;@3;) 0 (;@5;) 2 (;@3;) 0 (;@5;) 2 (;@3;)
            end
          end
          i32.const -1
          i32.const 1
          local.get 1
          i32.const 45
          i32.eq
          select
          local.set 8
          block  ;; label = @4
            local.get 0
            i32.load offset=4
            local.tee 1
            local.get 0
            i32.load offset=84
            i32.eq
            br_if 0 (;@4;)
            local.get 0
            local.get 1
            i32.const 1
            i32.add
            i32.store offset=4
            local.get 1
            i32.load8_u
            local.set 1
            br 2 (;@2;)
          end
          local.get 0
          call $__shgetc
          local.set 1
          br 1 (;@2;)
        end
        i32.const 1
        local.set 8
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const -33
            i32.and
            local.tee 9
            i32.const 73
            i32.ne
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 1
                local.get 0
                i32.load offset=84
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 1
                i32.const 1
                i32.add
                i32.store offset=4
                local.get 1
                i32.load8_u
                local.set 1
                br 1 (;@5;)
              end
              local.get 0
              call $__shgetc
              local.set 1
            end
            local.get 1
            i32.const -33
            i32.and
            i32.const 78
            i32.ne
            br_if 1 (;@3;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 1
                local.get 0
                i32.load offset=84
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 1
                i32.const 1
                i32.add
                i32.store offset=4
                local.get 1
                i32.load8_u
                local.set 1
                br 1 (;@5;)
              end
              local.get 0
              call $__shgetc
              local.set 1
            end
            local.get 1
            i32.const -33
            i32.and
            i32.const 70
            i32.ne
            br_if 1 (;@3;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 1
                local.get 0
                i32.load offset=84
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 1
                i32.const 1
                i32.add
                i32.store offset=4
                local.get 1
                i32.load8_u
                local.set 9
                br 1 (;@5;)
              end
              local.get 0
              call $__shgetc
              local.set 9
            end
            i32.const 3
            local.set 1
            block  ;; label = @5
              block  ;; label = @6
                local.get 9
                i32.const -33
                i32.and
                local.tee 9
                i32.const 73
                i32.ne
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.load offset=4
                    local.tee 1
                    local.get 0
                    i32.load offset=84
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 1
                    i32.const 1
                    i32.add
                    i32.store offset=4
                    local.get 1
                    i32.load8_u
                    local.set 10
                    br 1 (;@7;)
                  end
                  local.get 0
                  call $__shgetc
                  local.set 10
                end
                i32.const 4
                local.set 1
                block  ;; label = @7
                  local.get 10
                  i32.const -33
                  i32.and
                  i32.const 78
                  i32.ne
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load offset=4
                      local.tee 1
                      local.get 0
                      i32.load offset=84
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 1
                      i32.const 1
                      i32.add
                      i32.store offset=4
                      local.get 1
                      i32.load8_u
                      local.set 10
                      br 1 (;@8;)
                    end
                    local.get 0
                    call $__shgetc
                    local.set 10
                  end
                  i32.const 5
                  local.set 1
                  local.get 10
                  i32.const -33
                  i32.and
                  i32.const 73
                  i32.ne
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load offset=4
                      local.tee 1
                      local.get 0
                      i32.load offset=84
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 1
                      i32.const 1
                      i32.add
                      i32.store offset=4
                      local.get 1
                      i32.load8_u
                      local.set 10
                      br 1 (;@8;)
                    end
                    local.get 0
                    call $__shgetc
                    local.set 10
                  end
                  i32.const 6
                  local.set 1
                  local.get 10
                  i32.const -33
                  i32.and
                  i32.const 84
                  i32.ne
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load offset=4
                      local.tee 1
                      local.get 0
                      i32.load offset=84
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 1
                      i32.const 1
                      i32.add
                      i32.store offset=4
                      local.get 1
                      i32.load8_u
                      local.set 10
                      br 1 (;@8;)
                    end
                    local.get 0
                    call $__shgetc
                    local.set 10
                  end
                  i32.const 7
                  local.set 1
                  local.get 10
                  i32.const -33
                  i32.and
                  i32.const 89
                  i32.eq
                  br_if 2 (;@5;)
                end
                local.get 2
                i32.eqz
                br_if 3 (;@3;)
              end
              block  ;; label = @6
                local.get 0
                i64.load offset=88
                local.tee 11
                i64.const 0
                i64.lt_s
                br_if 0 (;@6;)
                local.get 0
                local.get 0
                i32.load offset=4
                i32.const -1
                i32.add
                i32.store offset=4
              end
              local.get 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 73
              i32.ne
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 11
                i64.const 0
                i64.lt_s
                br_if 0 (;@6;)
                local.get 0
                local.get 0
                i32.load offset=4
                i32.const -1
                i32.add
                i32.store offset=4
              end
              local.get 1
              i32.const -5
              i32.add
              i32.const -5
              i32.gt_u
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 11
                i64.const 0
                i64.lt_s
                br_if 0 (;@6;)
                local.get 0
                local.get 0
                i32.load offset=4
                i32.const -1
                i32.add
                i32.store offset=4
              end
              local.get 1
              i32.const -6
              i32.add
              i32.const -5
              i32.gt_u
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 11
                i64.const 0
                i64.lt_s
                br_if 0 (;@6;)
                local.get 0
                local.get 0
                i32.load offset=4
                i32.const -1
                i32.add
                i32.store offset=4
              end
              local.get 1
              i32.const -7
              i32.add
              i32.const -5
              i32.gt_u
              br_if 0 (;@5;)
              local.get 11
              i64.const 0
              i64.lt_s
              br_if 0 (;@5;)
              local.get 0
              local.get 0
              i32.load offset=4
              i32.const -1
              i32.add
              i32.store offset=4
            end
            local.get 8
            f32.convert_i32_s
            f32.const inf (;=inf;)
            f32.mul
            f64.promote_f32
            local.set 7
            br 3 (;@1;)
          end
          local.get 9
          i32.const 78
          i32.ne
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load offset=4
              local.tee 1
              local.get 0
              i32.load offset=84
              i32.eq
              br_if 0 (;@5;)
              local.get 0
              local.get 1
              i32.const 1
              i32.add
              i32.store offset=4
              local.get 1
              i32.load8_u
              local.set 1
              br 1 (;@4;)
            end
            local.get 0
            call $__shgetc
            local.set 1
          end
          local.get 1
          i32.const -33
          i32.and
          i32.const 65
          i32.ne
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load offset=4
              local.tee 1
              local.get 0
              i32.load offset=84
              i32.eq
              br_if 0 (;@5;)
              local.get 0
              local.get 1
              i32.const 1
              i32.add
              i32.store offset=4
              local.get 1
              i32.load8_u
              local.set 1
              br 1 (;@4;)
            end
            local.get 0
            call $__shgetc
            local.set 1
          end
          local.get 1
          i32.const -33
          i32.and
          i32.const 78
          i32.ne
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load offset=4
              local.tee 1
              local.get 0
              i32.load offset=84
              i32.eq
              br_if 0 (;@5;)
              local.get 0
              local.get 1
              i32.const 1
              i32.add
              i32.store offset=4
              local.get 1
              i32.load8_u
              local.set 1
              br 1 (;@4;)
            end
            local.get 0
            call $__shgetc
            local.set 1
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.const 40
              i32.ne
              br_if 0 (;@5;)
              i32.const 1
              local.set 10
              i32.const 1
              local.set 9
              br 1 (;@4;)
            end
            f64.const nan (;=nan;)
            local.set 7
            local.get 0
            i64.load offset=88
            i64.const 0
            i64.lt_s
            br_if 3 (;@1;)
            local.get 0
            local.get 0
            i32.load offset=4
            i32.const -1
            i32.add
            i32.store offset=4
            br 3 (;@1;)
          end
          loop  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 1
                local.get 0
                i32.load offset=84
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 1
                i32.const 1
                i32.add
                i32.store offset=4
                local.get 1
                i32.load8_u
                local.set 1
                br 1 (;@5;)
              end
              local.get 0
              call $__shgetc
              local.set 1
            end
            local.get 1
            i32.const -65
            i32.add
            local.set 12
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                i32.const -48
                i32.add
                i32.const 10
                i32.lt_u
                br_if 0 (;@6;)
                local.get 12
                i32.const 26
                i32.lt_u
                br_if 0 (;@6;)
                local.get 1
                i32.const -97
                i32.add
                local.set 12
                local.get 1
                i32.const 95
                i32.eq
                br_if 0 (;@6;)
                local.get 12
                i32.const 26
                i32.ge_u
                br_if 1 (;@5;)
              end
              local.get 10
              i32.const 1
              i32.add
              local.set 10
              local.get 9
              i32.const 1
              i32.add
              local.set 9
              br 1 (;@4;)
            end
          end
          block  ;; label = @4
            local.get 1
            i32.const 41
            i32.ne
            br_if 0 (;@4;)
            f64.const nan (;=nan;)
            local.set 7
            br 3 (;@1;)
          end
          block  ;; label = @4
            local.get 0
            i64.load offset=88
            local.tee 11
            i64.const 0
            i64.lt_s
            br_if 0 (;@4;)
            local.get 0
            local.get 0
            i32.load offset=4
            i32.const -1
            i32.add
            i32.store offset=4
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.eqz
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 9
                br_if 0 (;@6;)
                f64.const nan (;=nan;)
                local.set 7
                br 5 (;@1;)
              end
              local.get 9
              i32.const -1
              i32.add
              local.set 12
              block  ;; label = @6
                local.get 9
                i32.const 3
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 10
                i32.const 3
                i32.and
                local.set 10
                i32.const 0
                local.set 1
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 11
                    i64.const 0
                    i64.lt_s
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 0
                    i32.load offset=4
                    i32.const -1
                    i32.add
                    i32.store offset=4
                  end
                  local.get 10
                  local.get 1
                  i32.const 1
                  i32.add
                  local.tee 1
                  i32.ne
                  br_if 0 (;@7;)
                end
                local.get 9
                local.get 1
                i32.sub
                local.set 9
              end
              local.get 12
              i32.const 3
              i32.ge_u
              br_if 1 (;@4;)
              f64.const nan (;=nan;)
              local.set 7
              br 4 (;@1;)
            end
            i32.const 0
            i32.const 28
            i32.store offset=4724
            local.get 0
            i64.const 0
            call $__shlim
            br 3 (;@1;)
          end
          local.get 11
          i64.const 0
          i64.lt_s
          local.set 1
          loop  ;; label = @4
            block  ;; label = @5
              local.get 1
              br_if 0 (;@5;)
              local.get 0
              local.get 0
              i32.load offset=4
              i32.const -3
              i32.add
              i32.store offset=4
            end
            block  ;; label = @5
              local.get 1
              br_if 0 (;@5;)
              local.get 0
              local.get 0
              i32.load offset=4
              i32.const -1
              i32.add
              i32.store offset=4
            end
            local.get 9
            i32.const -4
            i32.add
            local.tee 9
            br_if 0 (;@4;)
          end
          f64.const nan (;=nan;)
          local.set 7
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 0
          i64.load offset=88
          i64.const 0
          i64.lt_s
          br_if 0 (;@3;)
          local.get 0
          local.get 0
          i32.load offset=4
          i32.const -1
          i32.add
          i32.store offset=4
        end
        i32.const 0
        i32.const 28
        i32.store offset=4724
        local.get 0
        i64.const 0
        call $__shlim
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                i32.const 48
                i32.ne
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.load offset=4
                    local.tee 1
                    local.get 0
                    i32.load offset=84
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 1
                    i32.const 1
                    i32.add
                    i32.store offset=4
                    local.get 1
                    i32.load8_u
                    local.set 1
                    br 1 (;@7;)
                  end
                  local.get 0
                  call $__shgetc
                  local.set 1
                end
                block  ;; label = @7
                  local.get 1
                  i32.const -33
                  i32.and
                  i32.const 88
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 5
                  local.get 4
                  local.get 8
                  local.get 2
                  call $hexfloat
                  local.set 7
                  br 6 (;@1;)
                end
                local.get 0
                i32.load offset=4
                local.set 1
                block  ;; label = @7
                  local.get 0
                  i64.load offset=88
                  i64.const 0
                  i64.lt_s
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 1
                  i32.const -1
                  i32.add
                  local.tee 1
                  i32.store offset=4
                end
                i32.const 0
                local.get 4
                i32.sub
                local.set 13
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 1
                    local.get 0
                    i32.load offset=84
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 1
                    i32.const 1
                    i32.add
                    i32.store offset=4
                    local.get 1
                    i32.load8_u
                    local.set 1
                    br 1 (;@7;)
                  end
                  local.get 0
                  call $__shgetc
                  local.set 1
                end
                local.get 13
                local.get 5
                i32.sub
                local.set 14
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 1
                    i32.const 48
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 46
                    i32.ne
                    br_if 4 (;@4;)
                    i32.const 1
                    local.set 15
                    br 3 (;@5;)
                  end
                  block  ;; label = @8
                    local.get 0
                    i32.load offset=4
                    local.tee 1
                    local.get 0
                    i32.load offset=84
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 1
                    i32.const 1
                    i32.add
                    i32.store offset=4
                    local.get 1
                    i32.load8_u
                    local.set 1
                    br 1 (;@7;)
                  end
                  local.get 0
                  call $__shgetc
                  local.set 1
                  br 0 (;@7;)
                end
              end
              i32.const 0
              local.set 15
              i32.const 0
              local.get 4
              i32.sub
              local.tee 13
              local.get 5
              i32.sub
              local.set 14
              i64.const 0
              local.set 11
              i32.const 0
              local.set 16
              local.get 1
              i32.const 46
              i32.ne
              br_if 3 (;@2;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 1
                local.get 0
                i32.load offset=84
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 1
                i32.const 1
                i32.add
                i32.store offset=4
                local.get 1
                i32.load8_u
                local.set 1
                br 1 (;@5;)
              end
              local.get 0
              call $__shgetc
              local.set 1
            end
            block  ;; label = @5
              local.get 1
              i32.const 48
              i32.eq
              br_if 0 (;@5;)
              i32.const 1
              local.set 16
              br 2 (;@3;)
            end
            i64.const 0
            local.set 11
            loop  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load offset=4
                  local.tee 1
                  local.get 0
                  i32.load offset=84
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 1
                  i32.const 1
                  i32.add
                  i32.store offset=4
                  local.get 1
                  i32.load8_u
                  local.set 1
                  br 1 (;@6;)
                end
                local.get 0
                call $__shgetc
                local.set 1
              end
              local.get 11
              i64.const -1
              i64.add
              local.set 11
              local.get 1
              i32.const 48
              i32.eq
              br_if 0 (;@5;)
            end
            i32.const 1
            local.set 15
            i32.const 1
            local.set 16
            br 2 (;@2;)
          end
          i32.const 0
          local.set 16
          i32.const 1
          local.set 15
        end
        i64.const 0
        local.set 11
      end
      i32.const 0
      local.set 17
      local.get 3
      i32.const 0
      i32.store
      local.get 1
      i32.const -48
      i32.add
      local.set 10
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 1
                  i32.const 46
                  i32.eq
                  local.tee 9
                  br_if 0 (;@7;)
                  i64.const 0
                  local.set 18
                  local.get 10
                  i32.const 9
                  i32.le_u
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 12
                  i32.const 0
                  local.set 19
                  br 1 (;@6;)
                end
                i64.const 0
                local.set 18
                i32.const 0
                local.set 19
                i32.const 0
                local.set 12
                i32.const 0
                local.set 17
                loop  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 9
                      i32.const 1
                      i32.and
                      i32.eqz
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        local.get 16
                        br_if 0 (;@10;)
                        local.get 18
                        local.set 11
                        i32.const 1
                        local.set 16
                        br 2 (;@8;)
                      end
                      local.get 15
                      i32.eqz
                      local.set 9
                      br 4 (;@5;)
                    end
                    local.get 18
                    i64.const 1
                    i64.add
                    local.set 18
                    block  ;; label = @9
                      local.get 12
                      i32.const 124
                      i32.gt_s
                      br_if 0 (;@9;)
                      local.get 3
                      local.get 12
                      i32.const 2
                      i32.shl
                      i32.add
                      local.set 9
                      block  ;; label = @10
                        local.get 19
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 9
                        i32.load
                        i32.const 10
                        i32.mul
                        i32.add
                        i32.const -48
                        i32.add
                        local.set 10
                      end
                      local.get 17
                      local.get 18
                      i32.wrap_i64
                      local.get 1
                      i32.const 48
                      i32.eq
                      select
                      local.set 17
                      local.get 9
                      local.get 10
                      i32.store
                      i32.const 1
                      local.set 15
                      i32.const 0
                      local.get 19
                      i32.const 1
                      i32.add
                      local.tee 1
                      local.get 1
                      i32.const 9
                      i32.eq
                      local.tee 1
                      select
                      local.set 19
                      local.get 12
                      local.get 1
                      i32.add
                      local.set 12
                      br 1 (;@8;)
                    end
                    local.get 1
                    i32.const 48
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 3
                    local.get 3
                    i32.load offset=496
                    i32.const 1
                    i32.or
                    i32.store offset=496
                    i32.const 1116
                    local.set 17
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load offset=4
                      local.tee 1
                      local.get 0
                      i32.load offset=84
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 1
                      i32.const 1
                      i32.add
                      i32.store offset=4
                      local.get 1
                      i32.load8_u
                      local.set 1
                      br 1 (;@8;)
                    end
                    local.get 0
                    call $__shgetc
                    local.set 1
                  end
                  local.get 1
                  i32.const -48
                  i32.add
                  local.set 10
                  local.get 1
                  i32.const 46
                  i32.eq
                  local.tee 9
                  br_if 0 (;@7;)
                  local.get 10
                  i32.const 10
                  i32.lt_u
                  br_if 0 (;@7;)
                end
              end
              local.get 11
              local.get 18
              local.get 16
              select
              local.set 11
              block  ;; label = @6
                local.get 15
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                i32.const -33
                i32.and
                i32.const 69
                i32.ne
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 0
                  local.get 2
                  call $scanexp
                  local.tee 20
                  i64.const -9223372036854775808
                  i64.ne
                  br_if 0 (;@7;)
                  local.get 2
                  i32.eqz
                  br_if 4 (;@3;)
                  i64.const 0
                  local.set 20
                  local.get 0
                  i64.load offset=88
                  i64.const 0
                  i64.lt_s
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 0
                  i32.load offset=4
                  i32.const -1
                  i32.add
                  i32.store offset=4
                end
                local.get 20
                local.get 11
                i64.add
                local.set 11
                br 4 (;@2;)
              end
              local.get 15
              i32.eqz
              local.set 9
              local.get 1
              i32.const 0
              i32.lt_s
              br_if 1 (;@4;)
            end
            local.get 0
            i64.load offset=88
            i64.const 0
            i64.lt_s
            br_if 0 (;@4;)
            local.get 0
            local.get 0
            i32.load offset=4
            i32.const -1
            i32.add
            i32.store offset=4
          end
          local.get 9
          i32.eqz
          br_if 1 (;@2;)
          i32.const 0
          i32.const 28
          i32.store offset=4724
          local.get 0
          i64.const 0
          call $__shlim
          f64.const 0x0p+0 (;=0;)
          local.set 7
          br 2 (;@1;)
        end
        local.get 0
        i64.const 0
        call $__shlim
        f64.const 0x0p+0 (;=0;)
        local.set 7
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 3
        i32.load
        local.tee 0
        br_if 0 (;@2;)
        local.get 8
        f64.convert_i32_s
        f64.const 0x0p+0 (;=0;)
        f64.mul
        local.set 7
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 18
        i64.const 9
        i64.gt_s
        br_if 0 (;@2;)
        local.get 11
        local.get 18
        i64.ne
        br_if 0 (;@2;)
        local.get 6
        local.get 0
        local.get 5
        i32.shr_u
        i32.eqz
        i32.or
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 8
        f64.convert_i32_s
        local.get 0
        f64.convert_i32_u
        f64.mul
        local.set 7
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 11
        local.get 13
        i32.const 1
        i32.shr_u
        i64.extend_i32_u
        i64.le_s
        br_if 0 (;@2;)
        i32.const 0
        i32.const 68
        i32.store offset=4724
        local.get 8
        f64.convert_i32_s
        f64.const 0x1.fffffffffffffp+1023 (;=1.79769e+308;)
        f64.mul
        f64.const 0x1.fffffffffffffp+1023 (;=1.79769e+308;)
        f64.mul
        local.set 7
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 11
        local.get 4
        i32.const -106
        i32.add
        i64.extend_i32_s
        i64.ge_s
        br_if 0 (;@2;)
        i32.const 0
        i32.const 68
        i32.store offset=4724
        local.get 8
        f64.convert_i32_s
        f64.const 0x1p-1022 (;=2.22507e-308;)
        f64.mul
        f64.const 0x1p-1022 (;=2.22507e-308;)
        f64.mul
        local.set 7
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 19
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 19
          i32.const 8
          i32.gt_s
          br_if 0 (;@3;)
          local.get 3
          local.get 12
          i32.const 2
          i32.shl
          i32.add
          local.tee 10
          i32.load
          local.set 0
          block  ;; label = @4
            block  ;; label = @5
              i32.const 1
              local.get 19
              i32.sub
              i32.const 7
              i32.and
              local.tee 9
              br_if 0 (;@5;)
              local.get 19
              local.set 1
              br 1 (;@4;)
            end
            local.get 19
            local.set 1
            loop  ;; label = @5
              local.get 1
              i32.const 1
              i32.add
              local.set 1
              local.get 0
              i32.const 10
              i32.mul
              local.set 0
              local.get 9
              i32.const -1
              i32.add
              local.tee 9
              br_if 0 (;@5;)
            end
          end
          block  ;; label = @4
            local.get 19
            i32.const -2
            i32.add
            i32.const 7
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const -9
            i32.add
            local.set 1
            loop  ;; label = @5
              local.get 0
              i32.const 100000000
              i32.mul
              local.set 0
              local.get 1
              i32.const 8
              i32.add
              local.tee 1
              br_if 0 (;@5;)
            end
          end
          local.get 10
          local.get 0
          i32.store
        end
        local.get 12
        i32.const 1
        i32.add
        local.set 12
      end
      local.get 11
      i32.wrap_i64
      local.set 6
      block  ;; label = @2
        local.get 17
        i32.const 9
        i32.ge_s
        br_if 0 (;@2;)
        local.get 17
        local.get 6
        i32.gt_s
        br_if 0 (;@2;)
        local.get 6
        i32.const 17
        i32.gt_s
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 6
          i32.const 9
          i32.ne
          br_if 0 (;@3;)
          local.get 8
          f64.convert_i32_s
          local.get 3
          i32.load
          f64.convert_i32_u
          f64.mul
          local.set 7
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 6
          i32.const 8
          i32.gt_s
          br_if 0 (;@3;)
          local.get 8
          f64.convert_i32_s
          local.get 3
          i32.load
          f64.convert_i32_u
          f64.mul
          i32.const 8
          local.get 6
          i32.sub
          i32.const 2
          i32.shl
          i32.const 3744
          i32.add
          i32.load
          f64.convert_i32_s
          f64.div
          local.set 7
          br 2 (;@1;)
        end
        local.get 3
        i32.load
        local.set 0
        block  ;; label = @3
          local.get 5
          local.get 6
          i32.const -3
          i32.mul
          i32.add
          i32.const 27
          i32.add
          local.tee 1
          i32.const 30
          i32.gt_s
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.shr_u
          br_if 1 (;@2;)
        end
        local.get 8
        f64.convert_i32_s
        local.get 0
        f64.convert_i32_u
        f64.mul
        local.get 6
        i32.const 2
        i32.shl
        i32.const 3704
        i32.add
        i32.load
        f64.convert_i32_s
        f64.mul
        local.set 7
        br 1 (;@1;)
      end
      local.get 12
      i32.const 1
      i32.add
      local.set 10
      local.get 12
      i32.const 2
      i32.shl
      local.get 3
      i32.add
      i32.const 4
      i32.add
      local.set 0
      loop  ;; label = @2
        local.get 10
        i32.const -1
        i32.add
        local.set 10
        local.get 0
        i32.const -8
        i32.add
        local.set 1
        local.get 0
        i32.const -4
        i32.add
        local.tee 12
        local.set 0
        local.get 1
        i32.load
        i32.eqz
        br_if 0 (;@2;)
      end
      i32.const 0
      local.set 17
      block  ;; label = @2
        block  ;; label = @3
          local.get 6
          i32.const 9
          i32.rem_s
          local.tee 0
          br_if 0 (;@3;)
          i32.const 0
          local.set 9
          br 1 (;@2;)
        end
        i32.const 0
        local.set 9
        local.get 0
        i32.const 9
        i32.add
        local.get 0
        local.get 6
        i32.const 0
        i32.lt_s
        select
        local.set 21
        block  ;; label = @3
          block  ;; label = @4
            local.get 10
            br_if 0 (;@4;)
            i32.const 0
            local.set 10
            br 1 (;@3;)
          end
          i32.const 1000000000
          i32.const 8
          local.get 21
          i32.sub
          i32.const 2
          i32.shl
          i32.const 3744
          i32.add
          i32.load
          local.tee 19
          i32.div_s
          local.set 13
          i32.const 0
          local.set 16
          local.get 3
          local.set 0
          i32.const 0
          local.set 1
          i32.const 0
          local.set 9
          loop  ;; label = @4
            local.get 0
            local.get 0
            i32.load
            local.tee 2
            local.get 19
            i32.div_u
            local.tee 15
            local.get 16
            i32.add
            local.tee 16
            i32.store
            local.get 9
            i32.const 1
            i32.add
            i32.const 127
            i32.and
            local.get 9
            local.get 1
            local.get 9
            i32.eq
            local.get 16
            i32.eqz
            i32.and
            local.tee 16
            select
            local.set 9
            local.get 6
            i32.const -9
            i32.add
            local.get 6
            local.get 16
            select
            local.set 6
            local.get 0
            i32.const 4
            i32.add
            local.set 0
            local.get 2
            local.get 15
            local.get 19
            i32.mul
            i32.sub
            local.get 13
            i32.mul
            local.set 16
            local.get 10
            local.get 1
            i32.const 1
            i32.add
            local.tee 1
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 16
          i32.eqz
          br_if 0 (;@3;)
          local.get 12
          local.get 16
          i32.store
          local.get 10
          i32.const 1
          i32.add
          local.set 10
        end
        local.get 6
        local.get 21
        i32.sub
        i32.const 9
        i32.add
        local.set 6
      end
      loop  ;; label = @2
        local.get 3
        local.get 9
        i32.const 2
        i32.shl
        i32.add
        local.set 15
        local.get 6
        i32.const 18
        i32.lt_s
        local.set 2
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 2
              br_if 0 (;@5;)
              local.get 6
              i32.const 18
              i32.ne
              br_if 2 (;@3;)
              local.get 15
              i32.load
              i32.const 9007198
              i32.gt_u
              br_if 2 (;@3;)
            end
            local.get 10
            i32.const 127
            i32.add
            local.set 16
            i32.const 0
            local.set 12
            loop  ;; label = @5
              local.get 10
              local.set 1
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  local.get 16
                  i32.const 127
                  i32.and
                  local.tee 0
                  i32.const 2
                  i32.shl
                  i32.add
                  local.tee 10
                  i64.load32_u
                  i64.const 29
                  i64.shl
                  local.get 12
                  i64.extend_i32_u
                  i64.add
                  local.tee 11
                  i64.const 1000000001
                  i64.ge_u
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 12
                  br 1 (;@6;)
                end
                local.get 11
                local.get 11
                i64.const 1000000000
                i64.div_u
                local.tee 18
                i64.const 1000000000
                i64.mul
                i64.sub
                local.set 11
                local.get 18
                i32.wrap_i64
                local.set 12
              end
              local.get 10
              local.get 11
              i32.wrap_i64
              local.tee 16
              i32.store
              local.get 1
              local.get 1
              local.get 1
              local.get 0
              local.get 16
              select
              local.get 0
              local.get 9
              i32.eq
              select
              local.get 0
              local.get 1
              i32.const -1
              i32.add
              i32.const 127
              i32.and
              local.tee 19
              i32.ne
              select
              local.set 10
              local.get 0
              i32.const -1
              i32.add
              local.set 16
              local.get 0
              local.get 9
              i32.ne
              br_if 0 (;@5;)
            end
            local.get 17
            i32.const -29
            i32.add
            local.set 17
            local.get 1
            local.set 10
            local.get 12
            i32.eqz
            br_if 0 (;@4;)
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 9
              i32.const -1
              i32.add
              i32.const 127
              i32.and
              local.tee 9
              local.get 1
              i32.eq
              br_if 0 (;@5;)
              local.get 1
              local.set 10
              br 1 (;@4;)
            end
            local.get 3
            local.get 1
            i32.const 126
            i32.add
            i32.const 127
            i32.and
            i32.const 2
            i32.shl
            i32.add
            local.tee 0
            local.get 0
            i32.load
            local.get 3
            local.get 19
            i32.const 2
            i32.shl
            i32.add
            i32.load
            i32.or
            i32.store
            local.get 19
            local.set 10
          end
          local.get 6
          i32.const 9
          i32.add
          local.set 6
          local.get 3
          local.get 9
          i32.const 2
          i32.shl
          i32.add
          local.get 12
          i32.store
          br 1 (;@2;)
        end
      end
      block  ;; label = @2
        loop  ;; label = @3
          local.get 3
          local.get 10
          i32.const 127
          i32.and
          i32.const 2
          i32.shl
          i32.add
          local.set 21
          local.get 3
          local.get 10
          i32.const -1
          i32.add
          i32.const 127
          i32.and
          i32.const 2
          i32.shl
          i32.add
          local.set 15
          local.get 3
          local.get 10
          i32.const 1
          i32.add
          i32.const 127
          i32.and
          local.tee 13
          i32.const 2
          i32.shl
          i32.add
          local.set 22
          block  ;; label = @4
            loop  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 9
                  i32.const 127
                  i32.and
                  local.tee 0
                  local.get 10
                  i32.eq
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 3
                    local.get 0
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    local.tee 0
                    i32.const 9007199
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 9007199
                    i32.ne
                    br_if 2 (;@6;)
                    local.get 9
                    i32.const 1
                    i32.add
                    i32.const 127
                    i32.and
                    local.tee 1
                    local.get 10
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 3
                    local.get 1
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    i32.const 254740991
                    i32.gt_u
                    br_if 2 (;@6;)
                    local.get 6
                    i32.const 18
                    i32.ne
                    br_if 2 (;@6;)
                    i32.const 9007199
                    local.set 0
                    local.get 10
                    local.set 1
                    br 6 (;@2;)
                  end
                  local.get 6
                  i32.const 18
                  i32.ne
                  br_if 1 (;@6;)
                  local.get 10
                  local.set 1
                  br 5 (;@2;)
                end
                local.get 6
                i32.const 18
                i32.eq
                br_if 2 (;@4;)
              end
              i32.const 9
              i32.const 1
              local.get 6
              i32.const 27
              i32.gt_s
              select
              local.set 16
              block  ;; label = @6
                block  ;; label = @7
                  local.get 9
                  local.get 10
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 16
                  local.get 17
                  i32.add
                  local.set 17
                  i32.const 1000000000
                  local.get 16
                  i32.shr_u
                  local.set 19
                  i32.const -1
                  local.get 16
                  i32.shl
                  i32.const -1
                  i32.xor
                  local.set 2
                  i32.const 0
                  local.set 1
                  local.get 9
                  local.set 0
                  loop  ;; label = @8
                    local.get 3
                    local.get 0
                    i32.const 2
                    i32.shl
                    i32.add
                    local.tee 12
                    local.get 12
                    i32.load
                    local.tee 12
                    local.get 16
                    i32.shr_u
                    local.get 1
                    i32.add
                    local.tee 1
                    i32.store
                    local.get 9
                    i32.const 1
                    i32.add
                    i32.const 127
                    i32.and
                    local.get 9
                    local.get 0
                    local.get 9
                    i32.eq
                    local.get 1
                    i32.eqz
                    i32.and
                    local.tee 1
                    select
                    local.set 9
                    local.get 6
                    i32.const -9
                    i32.add
                    local.get 6
                    local.get 1
                    select
                    local.set 6
                    local.get 12
                    local.get 2
                    i32.and
                    local.get 19
                    i32.mul
                    local.set 1
                    local.get 0
                    i32.const 1
                    i32.add
                    i32.const 127
                    i32.and
                    local.tee 0
                    local.get 10
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 1
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 13
                  local.get 9
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 3
                  local.get 10
                  i32.const 2
                  i32.shl
                  i32.add
                  local.get 1
                  i32.store
                  local.get 13
                  local.set 10
                  br 4 (;@3;)
                end
                local.get 16
                local.get 17
                i32.add
                local.set 17
                local.get 10
                i32.const 128
                i32.lt_u
                local.set 1
                local.get 6
                i32.const 18
                i32.eq
                local.set 9
                local.get 13
                local.get 10
                i32.eq
                local.set 12
                loop  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 1
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 9
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 10
                      local.set 9
                      br 5 (;@4;)
                    end
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 21
                        i32.load
                        local.tee 0
                        i32.const 9007199
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 0
                        i32.const 9007199
                        i32.ne
                        br_if 2 (;@8;)
                        local.get 12
                        br_if 0 (;@10;)
                        local.get 22
                        i32.load
                        i32.const 254740991
                        i32.gt_u
                        br_if 2 (;@8;)
                        local.get 9
                        i32.eqz
                        br_if 2 (;@8;)
                        i32.const 9007199
                        local.set 0
                        br 1 (;@9;)
                      end
                      local.get 9
                      i32.eqz
                      br_if 1 (;@8;)
                    end
                    local.get 10
                    local.set 9
                    local.get 10
                    local.set 1
                    br 6 (;@2;)
                  end
                  local.get 17
                  local.get 16
                  i32.add
                  local.set 17
                  br 0 (;@7;)
                end
              end
              local.get 15
              local.get 15
              i32.load
              i32.const 1
              i32.or
              i32.store
              br 0 (;@5;)
            end
          end
        end
        local.get 10
        i32.const 1
        i32.add
        i32.const 127
        i32.and
        local.tee 1
        i32.const 2
        i32.shl
        local.get 3
        i32.add
        i32.const -4
        i32.add
        i32.const 0
        i32.store
        local.get 3
        local.get 10
        i32.const 2
        i32.shl
        i32.add
        i32.load
        local.set 0
      end
      local.get 0
      f64.convert_i32_u
      local.set 7
      block  ;; label = @2
        local.get 9
        i32.const 1
        i32.add
        i32.const 127
        i32.and
        local.tee 0
        local.get 1
        i32.ne
        br_if 0 (;@2;)
        local.get 9
        i32.const 2
        i32.add
        i32.const 127
        i32.and
        local.tee 1
        i32.const 2
        i32.shl
        local.get 3
        i32.add
        i32.const -4
        i32.add
        i32.const 0
        i32.store
      end
      local.get 7
      f64.const 0x1.dcd65p+29 (;=1e+09;)
      f64.mul
      local.get 3
      local.get 0
      i32.const 2
      i32.shl
      i32.add
      i32.load
      f64.convert_i32_u
      f64.add
      local.get 8
      f64.convert_i32_s
      local.tee 23
      f64.mul
      local.set 24
      f64.const 0x0p+0 (;=0;)
      local.set 7
      block  ;; label = @2
        block  ;; label = @3
          local.get 17
          i32.const 53
          i32.add
          local.tee 12
          local.get 4
          i32.sub
          local.tee 0
          i32.const 0
          local.get 0
          i32.const 0
          i32.gt_s
          select
          local.get 5
          local.get 0
          local.get 5
          i32.lt_s
          local.tee 6
          select
          local.tee 10
          i32.const 52
          i32.le_u
          br_if 0 (;@3;)
          f64.const 0x0p+0 (;=0;)
          local.set 25
          br 1 (;@2;)
        end
        f64.const 0x1p+0 (;=1;)
        i32.const 105
        local.get 10
        i32.sub
        call $scalbn
        local.get 24
        f64.copysign
        local.tee 25
        local.get 24
        local.get 24
        f64.const 0x1p+0 (;=1;)
        i32.const 53
        local.get 10
        i32.sub
        call $scalbn
        call $fmod
        local.tee 7
        f64.sub
        f64.add
        local.set 24
      end
      block  ;; label = @2
        local.get 9
        i32.const 2
        i32.add
        i32.const 127
        i32.and
        local.tee 16
        local.get 1
        i32.eq
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            local.get 16
            i32.const 2
            i32.shl
            i32.add
            i32.load
            local.tee 16
            i32.const 499999999
            i32.gt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 16
              br_if 0 (;@5;)
              local.get 9
              i32.const 3
              i32.add
              i32.const 127
              i32.and
              local.get 1
              i32.eq
              br_if 2 (;@3;)
            end
            local.get 23
            f64.const 0x1p-2 (;=0.25;)
            f64.mul
            local.get 7
            f64.add
            local.set 7
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 16
            i32.const 500000000
            i32.eq
            br_if 0 (;@4;)
            local.get 23
            f64.const 0x1.8p-1 (;=0.75;)
            f64.mul
            local.get 7
            f64.add
            local.set 7
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 9
            i32.const 3
            i32.add
            i32.const 127
            i32.and
            local.get 1
            i32.ne
            br_if 0 (;@4;)
            local.get 23
            f64.const 0x1p-1 (;=0.5;)
            f64.mul
            local.get 7
            f64.add
            local.set 7
            br 1 (;@3;)
          end
          local.get 23
          f64.const 0x1.8p-1 (;=0.75;)
          f64.mul
          local.get 7
          f64.add
          local.set 7
        end
        local.get 7
        local.get 7
        local.get 7
        f64.const 0x1p+0 (;=1;)
        f64.add
        local.get 7
        f64.const 0x1p+0 (;=1;)
        call $fmod
        f64.const 0x0p+0 (;=0;)
        f64.ne
        select
        local.get 10
        i32.const 51
        i32.gt_u
        select
        local.set 7
      end
      local.get 24
      local.get 7
      f64.add
      local.get 25
      f64.sub
      local.set 24
      block  ;; label = @2
        local.get 12
        i32.const 2147483647
        i32.and
        local.get 14
        i32.const -2
        i32.add
        i32.le_s
        br_if 0 (;@2;)
        local.get 24
        f64.const 0x1p-1 (;=0.5;)
        f64.mul
        local.get 24
        local.get 24
        f64.abs
        f64.const 0x1p+53 (;=9.0072e+15;)
        f64.ge
        local.tee 1
        select
        local.set 24
        block  ;; label = @3
          local.get 17
          local.get 1
          i32.add
          local.tee 17
          i32.const 50
          i32.add
          local.get 14
          i32.gt_s
          br_if 0 (;@3;)
          local.get 6
          local.get 10
          local.get 0
          i32.ne
          local.get 1
          i32.const -1
          i32.xor
          i32.or
          i32.and
          local.get 7
          f64.const 0x0p+0 (;=0;)
          f64.ne
          i32.and
          i32.eqz
          br_if 1 (;@2;)
        end
        i32.const 0
        i32.const 68
        i32.store offset=4724
      end
      local.get 24
      local.get 17
      call $scalbn
      local.set 7
    end
    local.get 3
    i32.const 512
    i32.add
    global.set $__stack_pointer
    local.get 7)
  (func $hexfloat (type 19) (param i32 i32 i32 i32 i32) (result f64)
    (local i32 i64 i32 i32 i32 i64 f64 f64 i32 i32 i32 i64 i64 f64)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 5
        local.get 0
        i32.load offset=84
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        local.get 5
        i32.const 1
        i32.add
        i32.store offset=4
        local.get 5
        i32.load8_u
        local.set 5
        br 1 (;@1;)
      end
      local.get 0
      call $__shgetc
      local.set 5
    end
    i64.const 0
    local.set 6
    i32.const 0
    local.set 7
    i32.const 0
    local.set 8
    i32.const 0
    local.set 9
    i64.const 0
    local.set 10
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 5
          i32.const -46
          i32.add
          br_table 1 (;@2;) 2 (;@1;) 0 (;@3;) 2 (;@1;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load offset=4
            local.tee 5
            local.get 0
            i32.load offset=84
            i32.eq
            br_if 0 (;@4;)
            local.get 0
            local.get 5
            i32.const 1
            i32.add
            i32.store offset=4
            local.get 5
            i32.load8_u
            local.set 5
            br 1 (;@3;)
          end
          local.get 0
          call $__shgetc
          local.set 5
        end
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 5
              i32.const 48
              i32.eq
              br_if 0 (;@5;)
              local.get 5
              i32.const 46
              i32.ne
              br_if 2 (;@3;)
              i32.const 1
              local.set 7
              br 3 (;@2;)
            end
            block  ;; label = @5
              local.get 0
              i32.load offset=4
              local.tee 5
              local.get 0
              i32.load offset=84
              i32.eq
              br_if 0 (;@5;)
              local.get 0
              local.get 5
              i32.const 1
              i32.add
              i32.store offset=4
              local.get 5
              i32.load8_u
              local.set 5
              br 1 (;@4;)
            end
            local.get 0
            call $__shgetc
            local.set 5
            br 0 (;@4;)
          end
        end
        i32.const 1
        local.set 9
        i32.const 0
        local.set 8
        i64.const 0
        local.set 10
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 5
          local.get 0
          i32.load offset=84
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          local.get 5
          i32.const 1
          i32.add
          i32.store offset=4
          local.get 5
          i32.load8_u
          local.set 5
          br 1 (;@2;)
        end
        local.get 0
        call $__shgetc
        local.set 5
      end
      i32.const 1
      local.set 8
      local.get 7
      local.set 9
      i64.const 0
      local.set 10
      local.get 5
      i32.const 48
      i32.ne
      br_if 0 (;@1;)
      i64.const 0
      local.set 10
      loop  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load offset=4
            local.tee 5
            local.get 0
            i32.load offset=84
            i32.eq
            br_if 0 (;@4;)
            local.get 0
            local.get 5
            i32.const 1
            i32.add
            i32.store offset=4
            local.get 5
            i32.load8_u
            local.set 5
            br 1 (;@3;)
          end
          local.get 0
          call $__shgetc
          local.set 5
        end
        local.get 10
        i64.const -1
        i64.add
        local.set 10
        local.get 5
        i32.const 48
        i32.eq
        br_if 0 (;@2;)
      end
      i32.const 1
      local.set 8
      i32.const 1
      local.set 9
    end
    f64.const 0x1p+0 (;=1;)
    local.set 11
    f64.const 0x0p+0 (;=0;)
    local.set 12
    i32.const 0
    local.set 13
    i32.const 0
    local.set 14
    block  ;; label = @1
      loop  ;; label = @2
        local.get 5
        local.set 7
        block  ;; label = @3
          block  ;; label = @4
            local.get 5
            i32.const -48
            i32.add
            local.tee 15
            i32.const 10
            i32.lt_u
            br_if 0 (;@4;)
            local.get 5
            i32.const 32
            i32.or
            local.set 7
            block  ;; label = @5
              local.get 5
              i32.const 46
              i32.eq
              br_if 0 (;@5;)
              local.get 7
              i32.const -97
              i32.add
              i32.const 5
              i32.gt_u
              br_if 4 (;@1;)
            end
            local.get 5
            i32.const 46
            i32.ne
            br_if 0 (;@4;)
            local.get 8
            br_if 3 (;@1;)
            i32.const 1
            local.set 8
            local.get 6
            local.set 10
            br 1 (;@3;)
          end
          local.get 7
          i32.const -87
          i32.add
          local.get 15
          local.get 5
          i32.const 57
          i32.gt_s
          select
          local.set 5
          block  ;; label = @4
            block  ;; label = @5
              local.get 6
              i64.const 7
              i64.gt_s
              br_if 0 (;@5;)
              local.get 5
              local.get 13
              i32.const 4
              i32.shl
              i32.add
              local.set 13
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 6
              i64.const 13
              i64.gt_u
              br_if 0 (;@5;)
              local.get 5
              f64.convert_i32_s
              local.get 11
              f64.const 0x1p-4 (;=0.0625;)
              f64.mul
              local.tee 11
              f64.mul
              local.get 12
              f64.add
              local.set 12
              br 1 (;@4;)
            end
            local.get 12
            local.get 11
            f64.const 0x1p-1 (;=0.5;)
            f64.mul
            local.get 12
            f64.add
            local.get 5
            i32.eqz
            local.get 14
            i32.const 0
            i32.ne
            i32.or
            local.tee 5
            select
            local.set 12
            local.get 14
            i32.const 1
            local.get 5
            select
            local.set 14
          end
          local.get 6
          i64.const 1
          i64.add
          local.set 6
          i32.const 1
          local.set 9
        end
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 5
          local.get 0
          i32.load offset=84
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          local.get 5
          i32.const 1
          i32.add
          i32.store offset=4
          local.get 5
          i32.load8_u
          local.set 5
          br 1 (;@2;)
        end
        local.get 0
        call $__shgetc
        local.set 5
        br 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 9
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i64.load offset=88
            i64.const 0
            i64.lt_s
            br_if 0 (;@4;)
            local.get 0
            local.get 0
            i32.load offset=4
            local.tee 5
            i32.const -1
            i32.add
            i32.store offset=4
            local.get 4
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            local.get 5
            i32.const -2
            i32.add
            i32.store offset=4
            local.get 8
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            local.get 5
            i32.const -3
            i32.add
            i32.store offset=4
            br 2 (;@2;)
          end
          local.get 4
          br_if 1 (;@2;)
        end
        local.get 0
        i64.const 0
        call $__shlim
      end
      local.get 3
      f64.convert_i32_s
      f64.const 0x0p+0 (;=0;)
      f64.mul
      return
    end
    block  ;; label = @1
      local.get 6
      i64.const 7
      i64.gt_s
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          i64.const 0
          local.get 6
          i64.sub
          i64.const 7
          i64.and
          local.tee 16
          i64.eqz
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          local.set 17
          br 1 (;@2;)
        end
        local.get 6
        local.set 17
        loop  ;; label = @3
          local.get 17
          i64.const 1
          i64.add
          local.set 17
          local.get 13
          i32.const 4
          i32.shl
          local.set 13
          local.get 16
          i64.const -1
          i64.add
          local.tee 16
          i64.const 0
          i64.ne
          br_if 0 (;@3;)
        end
      end
      local.get 6
      i64.const -1
      i64.add
      i64.const 7
      i64.lt_u
      br_if 0 (;@1;)
      local.get 17
      i64.const -8
      i64.add
      local.set 17
      loop  ;; label = @2
        local.get 17
        i64.const 8
        i64.add
        local.tee 17
        i64.const 0
        i64.ne
        br_if 0 (;@2;)
      end
      i32.const 0
      local.set 13
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 5
            i32.const -33
            i32.and
            i32.const 80
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            local.get 4
            call $scanexp
            local.tee 17
            i64.const -9223372036854775808
            i64.ne
            br_if 3 (;@1;)
            block  ;; label = @5
              local.get 4
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              i64.load offset=88
              i64.const -1
              i64.gt_s
              br_if 2 (;@3;)
              br 3 (;@2;)
            end
            local.get 0
            i64.const 0
            call $__shlim
            f64.const 0x0p+0 (;=0;)
            return
          end
          i64.const 0
          local.set 17
          local.get 0
          i64.load offset=88
          i64.const 0
          i64.lt_s
          br_if 2 (;@1;)
        end
        local.get 0
        local.get 0
        i32.load offset=4
        i32.const -1
        i32.add
        i32.store offset=4
      end
      i64.const 0
      local.set 17
    end
    block  ;; label = @1
      local.get 13
      br_if 0 (;@1;)
      local.get 3
      f64.convert_i32_s
      f64.const 0x0p+0 (;=0;)
      f64.mul
      return
    end
    block  ;; label = @1
      local.get 10
      local.get 6
      local.get 8
      select
      i64.const 2
      i64.shl
      local.get 17
      i64.add
      i64.const -32
      i64.add
      local.tee 6
      i32.const 0
      local.get 2
      i32.sub
      i64.extend_i32_u
      i64.le_s
      br_if 0 (;@1;)
      i32.const 0
      i32.const 68
      i32.store offset=4724
      local.get 3
      f64.convert_i32_s
      f64.const 0x1.fffffffffffffp+1023 (;=1.79769e+308;)
      f64.mul
      f64.const 0x1.fffffffffffffp+1023 (;=1.79769e+308;)
      f64.mul
      return
    end
    block  ;; label = @1
      local.get 6
      local.get 2
      i32.const -106
      i32.add
      i64.extend_i32_s
      i64.lt_s
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 13
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 12
          local.get 12
          f64.const -0x1p+0 (;=-1;)
          f64.add
          local.get 12
          local.get 12
          f64.const 0x1p-1 (;=0.5;)
          f64.ge
          local.tee 5
          select
          f64.add
          local.set 12
          local.get 6
          i64.const -1
          i64.add
          local.set 6
          local.get 5
          local.get 13
          i32.const 1
          i32.shl
          i32.or
          local.tee 13
          i32.const -1
          i32.gt_s
          br_if 0 (;@3;)
        end
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 6
          local.get 2
          i64.extend_i32_s
          i64.sub
          i64.const 32
          i64.add
          local.tee 10
          i32.wrap_i64
          local.tee 5
          i32.const 0
          local.get 5
          i32.const 0
          i32.gt_s
          select
          local.get 1
          local.get 10
          local.get 1
          i64.extend_i32_u
          i64.lt_s
          select
          local.tee 5
          i32.const 53
          i32.lt_s
          br_if 0 (;@3;)
          local.get 3
          f64.convert_i32_s
          local.set 11
          f64.const 0x0p+0 (;=0;)
          local.set 18
          br 1 (;@2;)
        end
        f64.const 0x1p+0 (;=1;)
        i32.const 84
        local.get 5
        i32.sub
        call $scalbn
        local.get 3
        f64.convert_i32_s
        local.tee 11
        f64.copysign
        local.set 18
      end
      block  ;; label = @2
        local.get 11
        f64.const 0x0p+0 (;=0;)
        local.get 12
        local.get 13
        i32.const 1
        i32.and
        i32.eqz
        local.get 5
        i32.const 32
        i32.lt_s
        local.get 12
        f64.const 0x0p+0 (;=0;)
        f64.ne
        i32.and
        i32.and
        local.tee 5
        select
        f64.mul
        local.get 11
        local.get 13
        local.get 5
        i32.or
        f64.convert_i32_u
        f64.mul
        local.get 18
        f64.add
        f64.add
        local.get 18
        f64.sub
        local.tee 12
        f64.const 0x0p+0 (;=0;)
        f64.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.const 68
        i32.store offset=4724
      end
      local.get 12
      local.get 6
      i32.wrap_i64
      call $scalbn
      return
    end
    i32.const 0
    i32.const 68
    i32.store offset=4724
    local.get 3
    f64.convert_i32_s
    f64.const 0x1p-1022 (;=2.22507e-308;)
    f64.mul
    f64.const 0x1p-1022 (;=2.22507e-308;)
    f64.mul)
  (func $scanexp (type 20) (param i32 i32) (result i64)
    (local i32 i32 i32 i32 i64)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 2
        local.get 0
        i32.load offset=84
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        local.get 2
        i32.const 1
        i32.add
        i32.store offset=4
        local.get 2
        i32.load8_u
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      call $__shgetc
      local.set 3
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.const -43
              i32.add
              br_table 0 (;@5;) 1 (;@4;) 0 (;@5;) 1 (;@4;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 2
                local.get 0
                i32.load offset=84
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 2
                i32.const 1
                i32.add
                i32.store offset=4
                local.get 2
                i32.load8_u
                local.set 2
                br 1 (;@5;)
              end
              local.get 0
              call $__shgetc
              local.set 2
            end
            local.get 3
            i32.const 45
            i32.eq
            local.set 4
            local.get 2
            i32.const -58
            i32.add
            local.set 5
            local.get 1
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.const -11
            i32.gt_u
            br_if 1 (;@3;)
            local.get 0
            i64.load offset=88
            i64.const 0
            i64.lt_s
            br_if 2 (;@2;)
            local.get 0
            local.get 0
            i32.load offset=4
            i32.const -1
            i32.add
            i32.store offset=4
            br 2 (;@2;)
          end
          local.get 3
          i32.const -58
          i32.add
          local.set 5
          i32.const 0
          local.set 4
          local.get 3
          local.set 2
        end
        local.get 5
        i32.const -10
        i32.lt_u
        br_if 0 (;@2;)
        i64.const 0
        local.set 6
        block  ;; label = @3
          local.get 2
          i32.const -48
          i32.add
          i32.const 9
          i32.gt_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 3
          loop  ;; label = @4
            local.get 2
            local.get 3
            i32.const 10
            i32.mul
            i32.add
            local.set 3
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 2
                local.get 0
                i32.load offset=84
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 2
                i32.const 1
                i32.add
                i32.store offset=4
                local.get 2
                i32.load8_u
                local.set 2
                br 1 (;@5;)
              end
              local.get 0
              call $__shgetc
              local.set 2
            end
            local.get 3
            i32.const -48
            i32.add
            local.set 3
            block  ;; label = @5
              local.get 2
              i32.const -48
              i32.add
              local.tee 5
              i32.const 9
              i32.gt_u
              br_if 0 (;@5;)
              local.get 3
              i32.const 214748364
              i32.lt_s
              br_if 1 (;@4;)
            end
          end
          local.get 3
          i64.extend_i32_s
          local.set 6
          local.get 5
          i32.const 9
          i32.gt_u
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 2
            i64.extend_i32_u
            local.get 6
            i64.const 10
            i64.mul
            i64.add
            local.set 6
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 2
                local.get 0
                i32.load offset=84
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 2
                i32.const 1
                i32.add
                i32.store offset=4
                local.get 2
                i32.load8_u
                local.set 2
                br 1 (;@5;)
              end
              local.get 0
              call $__shgetc
              local.set 2
            end
            local.get 6
            i64.const -48
            i64.add
            local.set 6
            block  ;; label = @5
              local.get 2
              i32.const -48
              i32.add
              local.tee 3
              i32.const 9
              i32.gt_u
              br_if 0 (;@5;)
              local.get 6
              i64.const 92233720368547758
              i64.lt_s
              br_if 1 (;@4;)
            end
          end
          local.get 3
          i32.const 9
          i32.gt_u
          br_if 0 (;@3;)
          loop  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=4
                local.tee 2
                local.get 0
                i32.load offset=84
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 2
                i32.const 1
                i32.add
                i32.store offset=4
                local.get 2
                i32.load8_u
                local.set 2
                br 1 (;@5;)
              end
              local.get 0
              call $__shgetc
              local.set 2
            end
            local.get 2
            i32.const -48
            i32.add
            i32.const 10
            i32.lt_u
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 0
          i64.load offset=88
          i64.const 0
          i64.lt_s
          br_if 0 (;@3;)
          local.get 0
          local.get 0
          i32.load offset=4
          i32.const -1
          i32.add
          i32.store offset=4
        end
        i64.const 0
        local.get 6
        i64.sub
        local.get 6
        local.get 4
        select
        local.set 6
        br 1 (;@1;)
      end
      i64.const -9223372036854775808
      local.set 6
      local.get 0
      i64.load offset=88
      i64.const 0
      i64.lt_s
      br_if 0 (;@1;)
      local.get 0
      local.get 0
      i32.load offset=4
      i32.const -1
      i32.add
      i32.store offset=4
      i64.const -9223372036854775808
      return
    end
    local.get 6)
  (func $mbrtowc (type 6) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    local.get 3
    i32.const 5816
    local.get 3
    select
    local.tee 4
    i32.load
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            br_if 0 (;@4;)
            local.get 3
            br_if 1 (;@3;)
            i32.const 0
            return
          end
          i32.const -2
          local.set 5
          local.get 2
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              local.set 6
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 1
              i32.load8_u
              local.tee 5
              i32.extend8_s
              local.tee 3
              i32.const 0
              i32.lt_s
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 0
                i32.eqz
                br_if 0 (;@6;)
                local.get 0
                local.get 5
                i32.store
              end
              local.get 3
              i32.const 0
              i32.ne
              return
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=4752
              local.tee 7
              br_if 0 (;@5;)
              i32.const 4728
              local.set 7
              i32.const 0
              i32.const 4728
              i32.store offset=4752
            end
            block  ;; label = @5
              local.get 7
              i32.load
              br_if 0 (;@5;)
              i32.const 1
              local.set 5
              local.get 0
              i32.eqz
              br_if 3 (;@2;)
              local.get 0
              local.get 3
              i32.const 57343
              i32.and
              i32.store
              i32.const 1
              return
            end
            local.get 5
            i32.const -194
            i32.add
            local.tee 3
            i32.const 50
            i32.gt_u
            br_if 1 (;@3;)
            local.get 3
            i32.const 2
            i32.shl
            i32.const 3776
            i32.add
            i32.load
            local.set 3
            local.get 2
            i32.const -1
            i32.add
            local.tee 6
            i32.eqz
            br_if 3 (;@1;)
            local.get 1
            i32.const 1
            i32.add
            local.set 1
          end
          local.get 1
          i32.load8_u
          local.tee 5
          i32.const 3
          i32.shr_u
          local.tee 7
          i32.const -16
          i32.add
          local.get 3
          i32.const 26
          i32.shr_s
          local.get 7
          i32.add
          i32.or
          i32.const 7
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 1
          i32.add
          local.set 7
          local.get 6
          i32.const -1
          i32.add
          local.set 1
          loop  ;; label = @4
            block  ;; label = @5
              local.get 5
              i32.const 255
              i32.and
              i32.const -128
              i32.add
              local.get 3
              i32.const 6
              i32.shl
              i32.or
              local.tee 3
              i32.const 0
              i32.lt_s
              br_if 0 (;@5;)
              local.get 4
              i32.const 0
              i32.store
              block  ;; label = @6
                local.get 0
                i32.eqz
                br_if 0 (;@6;)
                local.get 0
                local.get 3
                i32.store
              end
              local.get 2
              local.get 1
              i32.sub
              return
            end
            local.get 1
            i32.eqz
            br_if 3 (;@1;)
            local.get 1
            i32.const -1
            i32.add
            local.set 1
            local.get 7
            i32.load8_u
            local.set 5
            local.get 7
            i32.const 1
            i32.add
            local.set 7
            local.get 5
            i32.const 192
            i32.and
            i32.const 128
            i32.eq
            br_if 0 (;@4;)
          end
        end
        local.get 4
        i32.const 0
        i32.store
        i32.const 0
        i32.const 25
        i32.store offset=4724
        i32.const -1
        local.set 5
      end
      local.get 5
      return
    end
    local.get 4
    local.get 3
    i32.store
    i32.const -2)
  (func $mbsinit (type 4) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 1
      return
    end
    local.get 0
    i32.load
    i32.eqz)
  (func $vfscanf (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i32 i32 i32 i32 i64 f64 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 304
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          br_if 0 (;@3;)
          local.get 0
          call $__toread
          drop
          local.get 0
          i32.load offset=4
          i32.eqz
          br_if 1 (;@2;)
        end
        block  ;; label = @3
          local.get 1
          i32.load8_u
          local.tee 4
          br_if 0 (;@3;)
          i32.const 0
          local.set 5
          br 2 (;@1;)
        end
        local.get 3
        i32.const 16
        i32.add
        i32.const 1
        i32.or
        local.set 6
        local.get 3
        i32.const 16
        i32.add
        i32.const 10
        i32.or
        local.set 7
        i64.const 0
        local.set 8
        i32.const 0
        local.set 5
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  loop  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 4
                          i32.const 255
                          i32.and
                          local.tee 4
                          i32.const 32
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 4
                          i32.const -14
                          i32.add
                          i32.const -5
                          i32.lt_u
                          br_if 1 (;@10;)
                        end
                        local.get 1
                        i32.const 1
                        i32.add
                        local.set 4
                        loop  ;; label = @11
                          local.get 4
                          i32.load8_u
                          local.tee 1
                          i32.const -14
                          i32.add
                          local.set 9
                          local.get 4
                          i32.const 1
                          i32.add
                          local.tee 10
                          local.set 4
                          local.get 1
                          i32.const 32
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 10
                          local.set 4
                          local.get 9
                          i32.const -6
                          i32.gt_u
                          br_if 0 (;@11;)
                        end
                        local.get 0
                        i64.const 0
                        call $__shlim
                        local.get 10
                        i32.const -2
                        i32.add
                        local.set 9
                        loop  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 0
                              i32.load offset=4
                              local.tee 4
                              local.get 0
                              i32.load offset=84
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 0
                              local.get 4
                              i32.const 1
                              i32.add
                              i32.store offset=4
                              local.get 4
                              i32.load8_u
                              local.set 4
                              br 1 (;@12;)
                            end
                            local.get 0
                            call $__shgetc
                            local.set 4
                          end
                          local.get 4
                          i32.const -9
                          i32.add
                          i32.const 5
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 4
                          i32.const 32
                          i32.eq
                          br_if 0 (;@11;)
                        end
                        local.get 0
                        i32.load offset=4
                        local.set 4
                        block  ;; label = @11
                          local.get 0
                          i64.load offset=88
                          i64.const 0
                          i64.lt_s
                          br_if 0 (;@11;)
                          local.get 0
                          local.get 4
                          i32.const -1
                          i32.add
                          local.tee 4
                          i32.store offset=4
                        end
                        local.get 0
                        i64.load offset=96
                        local.get 8
                        i64.add
                        local.get 4
                        local.get 0
                        i32.load offset=40
                        i32.sub
                        i64.extend_i32_s
                        i64.add
                        local.set 8
                        br 1 (;@9;)
                      end
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 4
                              i32.const 37
                              i32.ne
                              br_if 0 (;@13;)
                              local.get 1
                              i32.load8_u offset=1
                              local.tee 4
                              i32.const -37
                              i32.add
                              br_table 0 (;@13;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 1 (;@12;) 2 (;@11;)
                            end
                            local.get 0
                            i64.const 0
                            call $__shlim
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 1
                                i32.load8_u
                                i32.const 37
                                i32.ne
                                br_if 0 (;@14;)
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      local.get 0
                                      i32.load offset=4
                                      local.tee 4
                                      local.get 0
                                      i32.load offset=84
                                      i32.eq
                                      br_if 0 (;@17;)
                                      local.get 0
                                      local.get 4
                                      i32.const 1
                                      i32.add
                                      i32.store offset=4
                                      local.get 4
                                      i32.load8_u
                                      local.set 4
                                      br 1 (;@16;)
                                    end
                                    local.get 0
                                    call $__shgetc
                                    local.set 4
                                  end
                                  local.get 4
                                  i32.const -9
                                  i32.add
                                  i32.const 5
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                  local.get 4
                                  i32.const 32
                                  i32.eq
                                  br_if 0 (;@15;)
                                end
                                local.get 1
                                i32.const 1
                                i32.add
                                local.set 1
                                br 1 (;@13;)
                              end
                              block  ;; label = @14
                                local.get 0
                                i32.load offset=4
                                local.tee 4
                                local.get 0
                                i32.load offset=84
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 0
                                local.get 4
                                i32.const 1
                                i32.add
                                i32.store offset=4
                                local.get 4
                                i32.load8_u
                                local.set 4
                                br 1 (;@13;)
                              end
                              local.get 0
                              call $__shgetc
                              local.set 4
                            end
                            block  ;; label = @13
                              local.get 4
                              local.get 1
                              i32.load8_u
                              i32.eq
                              br_if 0 (;@13;)
                              block  ;; label = @14
                                local.get 0
                                i64.load offset=88
                                i64.const 0
                                i64.lt_s
                                br_if 0 (;@14;)
                                local.get 0
                                local.get 0
                                i32.load offset=4
                                i32.const -1
                                i32.add
                                i32.store offset=4
                              end
                              local.get 4
                              i32.const -1
                              i32.gt_s
                              br_if 12 (;@1;)
                              local.get 5
                              br_if 12 (;@1;)
                              br 11 (;@2;)
                            end
                            local.get 0
                            i64.load offset=96
                            local.get 8
                            i64.add
                            local.get 0
                            i32.load offset=4
                            local.get 0
                            i32.load offset=40
                            i32.sub
                            i64.extend_i32_s
                            i64.add
                            local.set 8
                            local.get 1
                            local.set 9
                            br 3 (;@9;)
                          end
                          local.get 1
                          i32.const 2
                          i32.add
                          local.set 1
                          i32.const 0
                          local.set 11
                          br 1 (;@10;)
                        end
                        block  ;; label = @11
                          local.get 4
                          i32.const -48
                          i32.add
                          local.tee 4
                          i32.const 9
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 1
                          i32.load8_u offset=2
                          i32.const 36
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 3
                          local.get 2
                          i32.store offset=300
                          local.get 3
                          local.get 2
                          local.get 4
                          i32.const 2
                          i32.shl
                          i32.add
                          i32.const -4
                          i32.add
                          local.get 2
                          local.get 4
                          i32.const 1
                          i32.gt_u
                          select
                          local.tee 4
                          i32.const 4
                          i32.add
                          i32.store offset=296
                          local.get 4
                          i32.load
                          local.set 11
                          local.get 1
                          i32.const 3
                          i32.add
                          local.set 1
                          br 1 (;@10;)
                        end
                        local.get 1
                        i32.const 1
                        i32.add
                        local.set 1
                        local.get 2
                        i32.load
                        local.set 11
                        local.get 2
                        i32.const 4
                        i32.add
                        local.set 2
                      end
                      i32.const 0
                      local.set 12
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 1
                          i32.load8_u
                          local.tee 4
                          i32.const -48
                          i32.add
                          i32.const 9
                          i32.le_u
                          br_if 0 (;@11;)
                          local.get 1
                          local.set 9
                          i32.const 0
                          local.set 10
                          br 1 (;@10;)
                        end
                        i32.const 0
                        local.set 10
                        loop  ;; label = @11
                          local.get 10
                          i32.const 10
                          i32.mul
                          local.get 4
                          i32.add
                          i32.const -48
                          i32.add
                          local.set 10
                          local.get 1
                          i32.load8_u offset=1
                          local.set 4
                          local.get 1
                          i32.const 1
                          i32.add
                          local.tee 9
                          local.set 1
                          local.get 4
                          i32.const -48
                          i32.add
                          i32.const 10
                          i32.lt_u
                          br_if 0 (;@11;)
                        end
                      end
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 4
                          i32.const 109
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 9
                          local.set 13
                          br 1 (;@10;)
                        end
                        local.get 9
                        i32.const 1
                        i32.add
                        local.set 13
                        i32.const 0
                        local.set 14
                        local.get 11
                        i32.const 0
                        i32.ne
                        local.set 12
                        local.get 9
                        i32.load8_u offset=1
                        local.set 4
                        i32.const 0
                        local.set 15
                      end
                      local.get 13
                      i32.const 1
                      i32.add
                      local.set 9
                      i32.const 3
                      local.set 1
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 4
                                  i32.const 255
                                  i32.and
                                  i32.const -65
                                  i32.add
                                  br_table 4 (;@11;) 11 (;@4;) 4 (;@11;) 11 (;@4;) 4 (;@11;) 4 (;@11;) 4 (;@11;) 11 (;@4;) 11 (;@4;) 11 (;@4;) 11 (;@4;) 3 (;@12;) 11 (;@4;) 11 (;@4;) 11 (;@4;) 11 (;@4;) 11 (;@4;) 11 (;@4;) 4 (;@11;) 11 (;@4;) 11 (;@4;) 11 (;@4;) 11 (;@4;) 4 (;@11;) 11 (;@4;) 11 (;@4;) 4 (;@11;) 11 (;@4;) 11 (;@4;) 11 (;@4;) 11 (;@4;) 11 (;@4;) 4 (;@11;) 11 (;@4;) 4 (;@11;) 4 (;@11;) 4 (;@11;) 4 (;@11;) 4 (;@11;) 0 (;@15;) 4 (;@11;) 5 (;@10;) 11 (;@4;) 1 (;@14;) 11 (;@4;) 4 (;@11;) 4 (;@11;) 4 (;@11;) 11 (;@4;) 11 (;@4;) 4 (;@11;) 2 (;@13;) 4 (;@11;) 11 (;@4;) 11 (;@4;) 4 (;@11;) 11 (;@4;) 2 (;@13;) 11 (;@4;)
                                end
                                local.get 13
                                i32.const 2
                                i32.add
                                local.get 9
                                local.get 13
                                i32.load8_u offset=1
                                i32.const 104
                                i32.eq
                                local.tee 4
                                select
                                local.set 9
                                i32.const -2
                                i32.const -1
                                local.get 4
                                select
                                local.set 1
                                br 4 (;@10;)
                              end
                              local.get 13
                              i32.const 2
                              i32.add
                              local.get 9
                              local.get 13
                              i32.load8_u offset=1
                              i32.const 108
                              i32.eq
                              local.tee 4
                              select
                              local.set 9
                              i32.const 3
                              i32.const 1
                              local.get 4
                              select
                              local.set 1
                              br 3 (;@10;)
                            end
                            i32.const 1
                            local.set 1
                            br 2 (;@10;)
                          end
                          i32.const 2
                          local.set 1
                          br 1 (;@10;)
                        end
                        i32.const 0
                        local.set 1
                        local.get 13
                        local.set 9
                      end
                      i32.const 1
                      local.get 1
                      local.get 9
                      i32.load8_u
                      local.tee 4
                      i32.const 47
                      i32.and
                      i32.const 3
                      i32.eq
                      local.tee 13
                      select
                      local.set 16
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 4
                              i32.const 32
                              i32.or
                              local.get 4
                              local.get 13
                              select
                              local.tee 13
                              i32.const -91
                              i32.add
                              br_table 3 (;@10;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 0 (;@13;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 2 (;@11;) 1 (;@12;) 2 (;@11;)
                            end
                            local.get 10
                            i32.const 1
                            local.get 10
                            i32.const 1
                            i32.gt_s
                            select
                            local.set 10
                            br 2 (;@10;)
                          end
                          local.get 11
                          i32.eqz
                          br_if 2 (;@9;)
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 16
                                  i32.const 2
                                  i32.add
                                  br_table 0 (;@15;) 1 (;@14;) 2 (;@13;) 2 (;@13;) 6 (;@9;) 3 (;@12;) 6 (;@9;)
                                end
                                local.get 11
                                local.get 8
                                i64.store8
                                br 5 (;@9;)
                              end
                              local.get 11
                              local.get 8
                              i64.store16
                              br 4 (;@9;)
                            end
                            local.get 11
                            local.get 8
                            i64.store32
                            br 3 (;@9;)
                          end
                          local.get 11
                          local.get 8
                          i64.store
                          br 2 (;@9;)
                        end
                        local.get 0
                        i64.const 0
                        call $__shlim
                        loop  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 0
                              i32.load offset=4
                              local.tee 4
                              local.get 0
                              i32.load offset=84
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 0
                              local.get 4
                              i32.const 1
                              i32.add
                              i32.store offset=4
                              local.get 4
                              i32.load8_u
                              local.set 4
                              br 1 (;@12;)
                            end
                            local.get 0
                            call $__shgetc
                            local.set 4
                          end
                          local.get 4
                          i32.const -9
                          i32.add
                          i32.const 5
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 4
                          i32.const 32
                          i32.eq
                          br_if 0 (;@11;)
                        end
                        local.get 0
                        i32.load offset=4
                        local.set 4
                        block  ;; label = @11
                          local.get 0
                          i64.load offset=88
                          i64.const 0
                          i64.lt_s
                          br_if 0 (;@11;)
                          local.get 0
                          local.get 4
                          i32.const -1
                          i32.add
                          local.tee 4
                          i32.store offset=4
                        end
                        local.get 0
                        i64.load offset=96
                        local.get 8
                        i64.add
                        local.get 4
                        local.get 0
                        i32.load offset=40
                        i32.sub
                        i64.extend_i32_s
                        i64.add
                        local.set 8
                      end
                      local.get 0
                      local.get 10
                      i64.extend_i32_s
                      local.tee 17
                      call $__shlim
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 0
                          i32.load offset=4
                          local.tee 4
                          local.get 0
                          i32.load offset=84
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 0
                          local.get 4
                          i32.const 1
                          i32.add
                          i32.store offset=4
                          br 1 (;@10;)
                        end
                        local.get 0
                        call $__shgetc
                        i32.const 0
                        i32.lt_s
                        br_if 6 (;@4;)
                      end
                      block  ;; label = @10
                        local.get 0
                        i64.load offset=88
                        i64.const 0
                        i64.lt_s
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 0
                        i32.load offset=4
                        i32.const -1
                        i32.add
                        i32.store offset=4
                      end
                      i32.const 16
                      local.set 4
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          local.get 13
                                          i32.const -65
                                          i32.add
                                          br_table 5 (;@14;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 5 (;@14;) 5 (;@14;) 5 (;@14;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 4 (;@15;) 9 (;@10;) 9 (;@10;) 0 (;@19;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 5 (;@14;) 9 (;@10;) 0 (;@19;) 2 (;@17;) 5 (;@14;) 5 (;@14;) 5 (;@14;) 9 (;@10;) 3 (;@16;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 9 (;@10;) 1 (;@18;) 4 (;@15;) 9 (;@10;) 9 (;@10;) 0 (;@19;) 9 (;@10;) 2 (;@17;) 9 (;@10;) 9 (;@10;) 4 (;@15;) 9 (;@10;)
                                        end
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            local.get 13
                                            i32.const -99
                                            i32.add
                                            br_table 0 (;@20;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 1 (;@19;) 0 (;@20;) 1 (;@19;)
                                          end
                                          local.get 3
                                          i32.const 16
                                          i32.add
                                          i32.const 255
                                          i32.const 257
                                          call $memset
                                          drop
                                          local.get 3
                                          i32.const 0
                                          i32.store8 offset=16
                                          local.get 13
                                          i32.const 115
                                          i32.ne
                                          br_if 8 (;@11;)
                                          local.get 7
                                          i32.const 0
                                          i32.store align=2
                                          local.get 7
                                          i32.const 4
                                          i32.add
                                          i32.const 0
                                          i32.store8
                                          local.get 3
                                          i32.const 0
                                          i32.store8 offset=49
                                          br 8 (;@11;)
                                        end
                                        local.get 3
                                        i32.const 16
                                        i32.add
                                        local.get 9
                                        i32.load8_u offset=1
                                        i32.const 94
                                        i32.eq
                                        local.tee 1
                                        i32.const 257
                                        call $memset
                                        drop
                                        local.get 3
                                        i32.const 0
                                        i32.store8 offset=16
                                        local.get 9
                                        i32.const 2
                                        i32.add
                                        local.get 9
                                        i32.const 1
                                        i32.add
                                        local.get 1
                                        select
                                        local.set 4
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              local.get 9
                                              i32.const 2
                                              i32.const 1
                                              local.get 1
                                              select
                                              i32.add
                                              i32.load8_u
                                              local.tee 9
                                              i32.const 45
                                              i32.eq
                                              br_if 0 (;@21;)
                                              local.get 9
                                              i32.const 93
                                              i32.eq
                                              br_if 1 (;@20;)
                                              local.get 1
                                              i32.const 1
                                              i32.xor
                                              local.set 9
                                              br 8 (;@13;)
                                            end
                                            local.get 3
                                            local.get 1
                                            i32.const 1
                                            i32.xor
                                            local.tee 9
                                            i32.store8 offset=62
                                            br 1 (;@19;)
                                          end
                                          local.get 3
                                          local.get 1
                                          i32.const 1
                                          i32.xor
                                          local.tee 9
                                          i32.store8 offset=110
                                        end
                                        i32.const 0
                                        local.set 1
                                        br 6 (;@12;)
                                      end
                                      i32.const 8
                                      local.set 4
                                      br 2 (;@15;)
                                    end
                                    i32.const 10
                                    local.set 4
                                    br 1 (;@15;)
                                  end
                                  i32.const 0
                                  local.set 4
                                end
                                local.get 0
                                local.get 4
                                i32.const 0
                                i64.const -1
                                call $__intscan
                                local.set 17
                                local.get 0
                                i64.load offset=96
                                i64.const 0
                                local.get 0
                                i32.load offset=4
                                local.get 0
                                i32.load offset=40
                                i32.sub
                                i64.extend_i32_s
                                i64.sub
                                i64.eq
                                br_if 11 (;@3;)
                                block  ;; label = @15
                                  local.get 13
                                  i32.const 112
                                  i32.ne
                                  br_if 0 (;@15;)
                                  local.get 11
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 11
                                  local.get 17
                                  i64.store32
                                  br 5 (;@10;)
                                end
                                local.get 11
                                i32.eqz
                                br_if 4 (;@10;)
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        local.get 16
                                        i32.const 2
                                        i32.add
                                        br_table 0 (;@18;) 1 (;@17;) 2 (;@16;) 2 (;@16;) 8 (;@10;) 3 (;@15;) 8 (;@10;)
                                      end
                                      local.get 11
                                      local.get 17
                                      i64.store8
                                      br 7 (;@10;)
                                    end
                                    local.get 11
                                    local.get 17
                                    i64.store16
                                    br 6 (;@10;)
                                  end
                                  local.get 11
                                  local.get 17
                                  i64.store32
                                  br 5 (;@10;)
                                end
                                local.get 11
                                local.get 17
                                i64.store
                                br 4 (;@10;)
                              end
                              local.get 0
                              local.get 16
                              i32.const 0
                              call $__floatscan
                              local.set 18
                              local.get 0
                              i64.load offset=96
                              i64.const 0
                              local.get 0
                              i32.load offset=4
                              local.get 0
                              i32.load offset=40
                              i32.sub
                              i64.extend_i32_s
                              i64.sub
                              i64.eq
                              br_if 10 (;@3;)
                              local.get 11
                              i32.eqz
                              br_if 3 (;@10;)
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 16
                                    br_table 0 (;@16;) 1 (;@15;) 2 (;@14;) 6 (;@10;)
                                  end
                                  local.get 11
                                  local.get 18
                                  f32.demote_f64
                                  f32.store
                                  br 5 (;@10;)
                                end
                                local.get 11
                                local.get 18
                                f64.store
                                br 4 (;@10;)
                              end
                              call $long_double_not_supported.1
                              unreachable
                            end
                            i32.const 1
                            local.set 1
                          end
                          loop  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 1
                                br_table 0 (;@14;) 1 (;@13;) 1 (;@13;)
                              end
                              local.get 4
                              i32.const 1
                              i32.add
                              local.set 4
                              i32.const 1
                              local.set 1
                              br 1 (;@12;)
                            end
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 4
                                i32.load8_u
                                local.tee 1
                                i32.const 45
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 1
                                i32.eqz
                                br_if 10 (;@4;)
                                local.get 1
                                i32.const 93
                                i32.ne
                                br_if 1 (;@13;)
                                local.get 4
                                local.set 9
                                br 3 (;@11;)
                              end
                              i32.const 45
                              local.set 1
                              local.get 4
                              i32.load8_u offset=1
                              local.tee 19
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 19
                              i32.const 93
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 4
                              i32.const 1
                              i32.add
                              local.set 20
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 4
                                  i32.const -1
                                  i32.add
                                  i32.load8_u
                                  local.tee 4
                                  local.get 19
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                  local.get 19
                                  local.set 1
                                  br 1 (;@14;)
                                end
                                loop  ;; label = @15
                                  local.get 6
                                  local.get 4
                                  i32.add
                                  local.get 9
                                  i32.store8
                                  local.get 4
                                  i32.const 1
                                  i32.add
                                  local.tee 4
                                  local.get 20
                                  i32.load8_u
                                  local.tee 1
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 20
                              local.set 4
                            end
                            local.get 1
                            local.get 3
                            i32.const 16
                            i32.add
                            i32.add
                            i32.const 1
                            i32.add
                            local.get 9
                            i32.store8
                            i32.const 0
                            local.set 1
                            br 0 (;@12;)
                          end
                        end
                        i32.const 31
                        local.get 10
                        i32.const 1
                        i32.add
                        local.get 13
                        i32.const 99
                        i32.ne
                        local.tee 19
                        select
                        local.set 20
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 16
                            i32.const 1
                            i32.ne
                            br_if 0 (;@12;)
                            local.get 11
                            local.set 1
                            block  ;; label = @13
                              local.get 12
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 20
                              i32.const 2
                              i32.shl
                              call $malloc
                              local.tee 1
                              i32.eqz
                              br_if 8 (;@5;)
                            end
                            local.get 3
                            i64.const 0
                            i64.store offset=288 align=4
                            i32.const 0
                            local.set 4
                            block  ;; label = @13
                              loop  ;; label = @14
                                local.get 1
                                local.set 10
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      local.get 0
                                      i32.load offset=4
                                      local.tee 1
                                      local.get 0
                                      i32.load offset=84
                                      i32.eq
                                      br_if 0 (;@17;)
                                      local.get 0
                                      local.get 1
                                      i32.const 1
                                      i32.add
                                      i32.store offset=4
                                      local.get 1
                                      i32.load8_u
                                      local.set 1
                                      br 1 (;@16;)
                                    end
                                    local.get 0
                                    call $__shgetc
                                    local.set 1
                                  end
                                  local.get 1
                                  local.get 3
                                  i32.const 16
                                  i32.add
                                  i32.add
                                  i32.const 1
                                  i32.add
                                  i32.load8_u
                                  i32.eqz
                                  br_if 2 (;@13;)
                                  local.get 3
                                  local.get 1
                                  i32.store8 offset=11
                                  local.get 3
                                  i32.const 12
                                  i32.add
                                  local.get 3
                                  i32.const 11
                                  i32.add
                                  i32.const 1
                                  local.get 3
                                  i32.const 288
                                  i32.add
                                  call $mbrtowc
                                  local.tee 1
                                  i32.const -2
                                  i32.eq
                                  br_if 0 (;@15;)
                                  local.get 1
                                  i32.const -1
                                  i32.eq
                                  br_if 8 (;@7;)
                                  block  ;; label = @16
                                    local.get 10
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    local.get 10
                                    local.get 4
                                    i32.const 2
                                    i32.shl
                                    i32.add
                                    local.get 3
                                    i32.load offset=12
                                    i32.store
                                    local.get 4
                                    i32.const 1
                                    i32.add
                                    local.set 4
                                  end
                                  local.get 12
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 4
                                  local.get 20
                                  i32.ne
                                  br_if 0 (;@15;)
                                end
                                local.get 10
                                local.get 20
                                i32.const 1
                                i32.shl
                                i32.const 1
                                i32.or
                                local.tee 20
                                i32.const 2
                                i32.shl
                                call $realloc
                                local.tee 1
                                br_if 0 (;@14;)
                              end
                              i32.const 0
                              local.set 14
                              local.get 10
                              local.set 15
                              i32.const 1
                              local.set 12
                              br 9 (;@4;)
                            end
                            i32.const 0
                            local.set 14
                            local.get 10
                            local.set 15
                            local.get 3
                            i32.const 288
                            i32.add
                            call $mbsinit
                            br_if 1 (;@11;)
                            br 6 (;@6;)
                          end
                          block  ;; label = @12
                            local.get 12
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 20
                            call $malloc
                            local.tee 1
                            i32.eqz
                            br_if 7 (;@5;)
                            i32.const 0
                            local.set 4
                            loop  ;; label = @13
                              local.get 1
                              local.set 10
                              loop  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 0
                                    i32.load offset=4
                                    local.tee 1
                                    local.get 0
                                    i32.load offset=84
                                    i32.eq
                                    br_if 0 (;@16;)
                                    local.get 0
                                    local.get 1
                                    i32.const 1
                                    i32.add
                                    i32.store offset=4
                                    local.get 1
                                    i32.load8_u
                                    local.set 1
                                    br 1 (;@15;)
                                  end
                                  local.get 0
                                  call $__shgetc
                                  local.set 1
                                end
                                block  ;; label = @15
                                  local.get 1
                                  local.get 3
                                  i32.const 16
                                  i32.add
                                  i32.add
                                  i32.const 1
                                  i32.add
                                  i32.load8_u
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.set 15
                                  local.get 10
                                  local.set 14
                                  br 4 (;@11;)
                                end
                                local.get 10
                                local.get 4
                                i32.add
                                local.get 1
                                i32.store8
                                local.get 20
                                local.get 4
                                i32.const 1
                                i32.add
                                local.tee 4
                                i32.ne
                                br_if 0 (;@14;)
                              end
                              local.get 10
                              local.get 20
                              i32.const 1
                              i32.shl
                              i32.const 1
                              i32.or
                              local.tee 20
                              call $realloc
                              local.tee 1
                              br_if 0 (;@13;)
                            end
                            i32.const 0
                            local.set 15
                            local.get 10
                            local.set 14
                            i32.const 1
                            local.set 12
                            br 8 (;@4;)
                          end
                          block  ;; label = @12
                            local.get 11
                            i32.eqz
                            br_if 0 (;@12;)
                            i32.const 0
                            local.set 4
                            loop  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 0
                                  i32.load offset=4
                                  local.tee 1
                                  local.get 0
                                  i32.load offset=84
                                  i32.eq
                                  br_if 0 (;@15;)
                                  local.get 0
                                  local.get 1
                                  i32.const 1
                                  i32.add
                                  i32.store offset=4
                                  local.get 1
                                  i32.load8_u
                                  local.set 1
                                  br 1 (;@14;)
                                end
                                local.get 0
                                call $__shgetc
                                local.set 1
                              end
                              block  ;; label = @14
                                local.get 1
                                local.get 3
                                i32.const 16
                                i32.add
                                i32.add
                                i32.const 1
                                i32.add
                                i32.load8_u
                                br_if 0 (;@14;)
                                i32.const 0
                                local.set 15
                                local.get 11
                                local.set 10
                                local.get 11
                                local.set 14
                                br 3 (;@11;)
                              end
                              local.get 11
                              local.get 4
                              i32.add
                              local.get 1
                              i32.store8
                              local.get 4
                              i32.const 1
                              i32.add
                              local.set 4
                              br 0 (;@13;)
                            end
                          end
                          loop  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 0
                                i32.load offset=4
                                local.tee 4
                                local.get 0
                                i32.load offset=84
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 0
                                local.get 4
                                i32.const 1
                                i32.add
                                i32.store offset=4
                                local.get 4
                                i32.load8_u
                                local.set 4
                                br 1 (;@13;)
                              end
                              local.get 0
                              call $__shgetc
                              local.set 4
                            end
                            local.get 4
                            local.get 3
                            i32.const 16
                            i32.add
                            i32.add
                            i32.const 1
                            i32.add
                            i32.load8_u
                            br_if 0 (;@12;)
                          end
                          i32.const 0
                          local.set 10
                          i32.const 0
                          local.set 14
                          i32.const 0
                          local.set 15
                          i32.const 0
                          local.set 4
                        end
                        local.get 0
                        i32.load offset=4
                        local.set 1
                        block  ;; label = @11
                          local.get 0
                          i64.load offset=88
                          i64.const 0
                          i64.lt_s
                          br_if 0 (;@11;)
                          local.get 0
                          local.get 1
                          i32.const -1
                          i32.add
                          local.tee 1
                          i32.store offset=4
                        end
                        local.get 0
                        i64.load offset=96
                        local.get 1
                        local.get 0
                        i32.load offset=40
                        i32.sub
                        i64.extend_i32_s
                        i64.add
                        local.tee 21
                        i64.eqz
                        br_if 7 (;@3;)
                        local.get 19
                        local.get 21
                        local.get 17
                        i64.eq
                        i32.or
                        i32.eqz
                        br_if 7 (;@3;)
                        block  ;; label = @11
                          local.get 12
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 11
                          local.get 10
                          i32.store
                        end
                        local.get 13
                        i32.const 99
                        i32.eq
                        br_if 0 (;@10;)
                        block  ;; label = @11
                          local.get 15
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 15
                          local.get 4
                          i32.const 2
                          i32.shl
                          i32.add
                          i32.const 0
                          i32.store
                        end
                        block  ;; label = @11
                          local.get 14
                          br_if 0 (;@11;)
                          i32.const 0
                          local.set 14
                          br 1 (;@10;)
                        end
                        local.get 14
                        local.get 4
                        i32.add
                        i32.const 0
                        i32.store8
                      end
                      local.get 0
                      i64.load offset=96
                      local.get 8
                      i64.add
                      local.get 0
                      i32.load offset=4
                      local.get 0
                      i32.load offset=40
                      i32.sub
                      i64.extend_i32_s
                      i64.add
                      local.set 8
                      local.get 5
                      local.get 11
                      i32.const 0
                      i32.ne
                      i32.add
                      local.set 5
                    end
                    local.get 9
                    i32.const 1
                    i32.add
                    local.set 1
                    local.get 9
                    i32.load8_u offset=1
                    local.tee 4
                    br_if 0 (;@8;)
                    br 7 (;@1;)
                  end
                end
                i32.const 0
                local.set 14
              end
              local.get 10
              local.set 15
              br 1 (;@4;)
            end
            i32.const 1
            local.set 12
            i32.const 0
            local.set 14
            i32.const 0
            local.set 15
          end
          local.get 5
          i32.const -1
          local.get 5
          select
          local.set 5
        end
        local.get 12
        i32.eqz
        br_if 1 (;@1;)
        local.get 14
        call $free
        local.get 15
        call $free
        br 1 (;@1;)
      end
      i32.const -1
      local.set 5
    end
    local.get 3
    i32.const 304
    i32.add
    global.set $__stack_pointer
    local.get 5)
  (func $long_double_not_supported.1 (type 8)
    i32.const 1104
    i32.const 3984
    call $fputs
    drop
    call $abort
    unreachable)
  (func $vsscanf (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 0
    i32.const 112
    call $memset
    local.tee 3
    local.get 0
    i32.store offset=68
    local.get 3
    local.get 0
    i32.store offset=40
    local.get 3
    i32.const 6
    i32.store offset=28
    local.get 3
    local.get 1
    local.get 2
    call $vfscanf
    local.set 0
    local.get 3
    i32.const 112
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $string_read (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    i32.load offset=68
    local.set 3
    local.get 1
    local.get 3
    local.get 3
    i32.const 0
    local.get 2
    i32.const 256
    i32.add
    local.tee 4
    call $memchr
    local.tee 5
    local.get 3
    i32.sub
    local.get 4
    local.get 5
    select
    local.tee 4
    local.get 2
    local.get 4
    local.get 2
    i32.lt_u
    select
    local.tee 2
    call $memcpy
    drop
    local.get 0
    local.get 3
    local.get 4
    i32.add
    local.tee 4
    i32.store offset=68
    local.get 0
    local.get 4
    i32.store offset=8
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=4
    local.get 2)
  (func $atoi (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32)
    loop  ;; label = @1
      local.get 0
      i32.load8_s
      local.tee 1
      i32.const -14
      i32.add
      local.set 2
      local.get 0
      i32.const 1
      i32.add
      local.tee 3
      local.set 0
      local.get 1
      i32.const 255
      i32.and
      local.tee 4
      i32.const 32
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      local.set 0
      local.get 2
      i32.const -6
      i32.gt_u
      br_if 0 (;@1;)
    end
    i32.const 1
    local.set 5
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 4
            i32.const -43
            i32.add
            br_table 2 (;@2;) 0 (;@4;) 1 (;@3;) 0 (;@4;)
          end
          local.get 3
          i32.const -1
          i32.add
          local.set 3
          i32.const 1
          local.set 5
          br 2 (;@1;)
        end
        i32.const 0
        local.set 5
      end
      local.get 3
      i32.load8_s
      local.set 1
    end
    i32.const 0
    local.set 2
    block  ;; label = @1
      local.get 1
      i32.const -48
      i32.add
      local.tee 1
      i32.const 9
      i32.gt_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 1
      i32.add
      local.set 0
      i32.const 0
      local.set 2
      loop  ;; label = @2
        local.get 2
        i32.const 10
        i32.mul
        local.get 1
        i32.sub
        local.set 2
        local.get 0
        i32.load8_s
        local.set 1
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 1
        i32.const -48
        i32.add
        local.tee 1
        i32.const 10
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    i32.const 0
    local.get 2
    i32.sub
    local.get 2
    local.get 5
    select)
  (func $memchr (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    local.get 2
    i32.const 0
    i32.ne
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.const 3
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 0
              i32.load8_u
              local.get 1
              i32.const 255
              i32.and
              i32.ne
              br_if 0 (;@5;)
              local.get 0
              local.set 4
              local.get 2
              local.set 5
              br 3 (;@2;)
            end
            local.get 2
            i32.const -1
            i32.add
            local.tee 5
            i32.const 0
            i32.ne
            local.set 3
            local.get 0
            i32.const 1
            i32.add
            local.tee 4
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            i32.load8_u
            local.get 1
            i32.const 255
            i32.and
            i32.eq
            br_if 2 (;@2;)
            local.get 2
            i32.const -2
            i32.add
            local.tee 5
            i32.const 0
            i32.ne
            local.set 3
            local.get 0
            i32.const 2
            i32.add
            local.tee 4
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            i32.load8_u
            local.get 1
            i32.const 255
            i32.and
            i32.eq
            br_if 2 (;@2;)
            local.get 2
            i32.const -3
            i32.add
            local.tee 5
            i32.const 0
            i32.ne
            local.set 3
            local.get 0
            i32.const 3
            i32.add
            local.tee 4
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            i32.load8_u
            local.get 1
            i32.const 255
            i32.and
            i32.eq
            br_if 2 (;@2;)
            local.get 0
            i32.const 4
            i32.add
            local.set 4
            local.get 2
            i32.const -4
            i32.add
            local.tee 5
            i32.const 0
            i32.ne
            local.set 3
            br 1 (;@3;)
          end
          local.get 2
          local.set 5
          local.get 0
          local.set 4
        end
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
        block  ;; label = @3
          local.get 4
          i32.load8_u
          local.get 1
          i32.const 255
          i32.and
          i32.eq
          br_if 0 (;@3;)
          local.get 5
          i32.const 4
          i32.lt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 255
          i32.and
          i32.const 16843009
          i32.mul
          local.set 0
          loop  ;; label = @4
            local.get 4
            i32.load
            local.get 0
            i32.xor
            local.tee 2
            i32.const -1
            i32.xor
            local.get 2
            i32.const -16843009
            i32.add
            i32.and
            i32.const -2139062144
            i32.and
            br_if 2 (;@2;)
            local.get 4
            i32.const 4
            i32.add
            local.set 4
            local.get 5
            i32.const -4
            i32.add
            local.tee 5
            i32.const 3
            i32.gt_u
            br_if 0 (;@4;)
          end
        end
        local.get 5
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 1
      i32.const 255
      i32.and
      local.set 2
      loop  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.load8_u
          local.get 2
          i32.ne
          br_if 0 (;@3;)
          local.get 4
          return
        end
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 5
        i32.const -1
        i32.add
        local.tee 5
        br_if 0 (;@2;)
      end
    end
    i32.const 0)
  (func $memcpy (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 32
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          local.get 1
          i32.load8_u
          i32.store8
          local.get 2
          i32.const -1
          i32.add
          local.set 3
          local.get 0
          i32.const 1
          i32.add
          local.set 4
          local.get 1
          i32.const 1
          i32.add
          local.tee 5
          i32.const 3
          i32.and
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u offset=1
          i32.store8 offset=1
          local.get 2
          i32.const -2
          i32.add
          local.set 3
          local.get 0
          i32.const 2
          i32.add
          local.set 4
          local.get 1
          i32.const 2
          i32.add
          local.tee 5
          i32.const 3
          i32.and
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u offset=2
          i32.store8 offset=2
          local.get 2
          i32.const -3
          i32.add
          local.set 3
          local.get 0
          i32.const 3
          i32.add
          local.set 4
          local.get 1
          i32.const 3
          i32.add
          local.tee 5
          i32.const 3
          i32.and
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u offset=3
          i32.store8 offset=3
          local.get 2
          i32.const -4
          i32.add
          local.set 3
          local.get 0
          i32.const 4
          i32.add
          local.set 4
          local.get 1
          i32.const 4
          i32.add
          local.set 5
          br 2 (;@1;)
        end
        local.get 0
        local.get 1
        local.get 2
        memory.copy
        local.get 0
        return
      end
      local.get 2
      local.set 3
      local.get 0
      local.set 4
      local.get 1
      local.set 5
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.const 3
        i32.and
        local.tee 2
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 16
            i32.ge_u
            br_if 0 (;@4;)
            local.get 3
            local.set 2
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 3
            i32.const -16
            i32.add
            local.tee 2
            i32.const 16
            i32.and
            br_if 0 (;@4;)
            local.get 4
            local.get 5
            i64.load align=4
            i64.store align=4
            local.get 4
            local.get 5
            i64.load offset=8 align=4
            i64.store offset=8 align=4
            local.get 4
            i32.const 16
            i32.add
            local.set 4
            local.get 5
            i32.const 16
            i32.add
            local.set 5
            local.get 2
            local.set 3
          end
          local.get 2
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 3
          local.set 2
          loop  ;; label = @4
            local.get 4
            local.get 5
            i64.load align=4
            i64.store align=4
            local.get 4
            local.get 5
            i64.load offset=8 align=4
            i64.store offset=8 align=4
            local.get 4
            local.get 5
            i64.load offset=16 align=4
            i64.store offset=16 align=4
            local.get 4
            local.get 5
            i64.load offset=24 align=4
            i64.store offset=24 align=4
            local.get 4
            i32.const 32
            i32.add
            local.set 4
            local.get 5
            i32.const 32
            i32.add
            local.set 5
            local.get 2
            i32.const -32
            i32.add
            local.tee 2
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 2
          i32.const 8
          i32.lt_u
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i64.load align=4
          i64.store align=4
          local.get 5
          i32.const 8
          i32.add
          local.set 5
          local.get 4
          i32.const 8
          i32.add
          local.set 4
        end
        block  ;; label = @3
          local.get 2
          i32.const 4
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.load
          i32.store
          local.get 5
          i32.const 4
          i32.add
          local.set 5
          local.get 4
          i32.const 4
          i32.add
          local.set 4
        end
        block  ;; label = @3
          local.get 2
          i32.const 2
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.load16_u align=1
          i32.store16 align=1
          local.get 4
          i32.const 2
          i32.add
          local.set 4
          local.get 5
          i32.const 2
          i32.add
          local.set 5
        end
        local.get 2
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        local.get 5
        i32.load8_u
        i32.store8
        local.get 0
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.const 32
                i32.lt_u
                br_if 0 (;@6;)
                local.get 4
                local.get 5
                i32.load
                local.tee 3
                i32.store8
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.const -1
                    i32.add
                    br_table 3 (;@5;) 0 (;@8;) 1 (;@7;) 3 (;@5;)
                  end
                  local.get 4
                  local.get 3
                  i32.const 8
                  i32.shr_u
                  i32.store8 offset=1
                  local.get 4
                  local.get 5
                  i32.const 6
                  i32.add
                  i64.load align=2
                  i64.store offset=6 align=4
                  local.get 4
                  local.get 5
                  i32.load offset=4
                  i32.const 16
                  i32.shl
                  local.get 3
                  i32.const 16
                  i32.shr_u
                  i32.or
                  i32.store offset=2
                  local.get 4
                  i32.const 18
                  i32.add
                  local.set 2
                  local.get 5
                  i32.const 18
                  i32.add
                  local.set 1
                  i32.const 14
                  local.set 6
                  local.get 5
                  i32.const 14
                  i32.add
                  i32.load align=2
                  local.set 5
                  i32.const 14
                  local.set 3
                  br 3 (;@4;)
                end
                local.get 4
                local.get 5
                i32.const 5
                i32.add
                i64.load align=1
                i64.store offset=5 align=4
                local.get 4
                local.get 5
                i32.load offset=4
                i32.const 24
                i32.shl
                local.get 3
                i32.const 8
                i32.shr_u
                i32.or
                i32.store offset=1
                local.get 4
                i32.const 17
                i32.add
                local.set 2
                local.get 5
                i32.const 17
                i32.add
                local.set 1
                i32.const 13
                local.set 6
                local.get 5
                i32.const 13
                i32.add
                i32.load align=1
                local.set 5
                i32.const 15
                local.set 3
                br 2 (;@4;)
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  i32.const 16
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 4
                  local.set 2
                  local.get 5
                  local.set 1
                  br 1 (;@6;)
                end
                local.get 4
                local.get 5
                i32.load8_u
                i32.store8
                local.get 4
                local.get 5
                i32.load offset=1 align=1
                i32.store offset=1 align=1
                local.get 4
                local.get 5
                i64.load offset=5 align=1
                i64.store offset=5 align=1
                local.get 4
                local.get 5
                i32.load16_u offset=13 align=1
                i32.store16 offset=13 align=1
                local.get 4
                local.get 5
                i32.load8_u offset=15
                i32.store8 offset=15
                local.get 4
                i32.const 16
                i32.add
                local.set 2
                local.get 5
                i32.const 16
                i32.add
                local.set 1
              end
              local.get 3
              i32.const 8
              i32.and
              br_if 2 (;@3;)
              br 3 (;@2;)
            end
            local.get 4
            local.get 3
            i32.const 16
            i32.shr_u
            i32.store8 offset=2
            local.get 4
            local.get 3
            i32.const 8
            i32.shr_u
            i32.store8 offset=1
            local.get 4
            local.get 5
            i32.const 7
            i32.add
            i64.load align=1
            i64.store offset=7 align=4
            local.get 4
            local.get 5
            i32.load offset=4
            i32.const 8
            i32.shl
            local.get 3
            i32.const 24
            i32.shr_u
            i32.or
            i32.store offset=3
            local.get 4
            i32.const 19
            i32.add
            local.set 2
            local.get 5
            i32.const 19
            i32.add
            local.set 1
            i32.const 15
            local.set 6
            local.get 5
            i32.const 15
            i32.add
            i32.load align=1
            local.set 5
            i32.const 13
            local.set 3
          end
          local.get 4
          local.get 6
          i32.add
          local.get 5
          i32.store
        end
        local.get 2
        local.get 1
        i64.load align=1
        i64.store align=1
        local.get 2
        i32.const 8
        i32.add
        local.set 2
        local.get 1
        i32.const 8
        i32.add
        local.set 1
      end
      block  ;; label = @2
        local.get 3
        i32.const 4
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.load align=1
        i32.store align=1
        local.get 2
        i32.const 4
        i32.add
        local.set 2
        local.get 1
        i32.const 4
        i32.add
        local.set 1
      end
      block  ;; label = @2
        local.get 3
        i32.const 2
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.load16_u align=1
        i32.store16 align=1
        local.get 2
        i32.const 2
        i32.add
        local.set 2
        local.get 1
        i32.const 2
        i32.add
        local.set 1
      end
      local.get 3
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i32.load8_u
      i32.store8
    end
    local.get 0)
  (func $memset (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i64)
    block  ;; label = @1
      local.get 2
      i32.const 33
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      memory.fill
      local.get 0
      return
    end
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8
      local.get 0
      local.get 2
      i32.add
      local.tee 3
      i32.const -1
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8 offset=2
      local.get 0
      local.get 1
      i32.store8 offset=1
      local.get 3
      i32.const -3
      i32.add
      local.get 1
      i32.store8
      local.get 3
      i32.const -2
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8 offset=3
      local.get 3
      i32.const -4
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.tee 5
      local.get 1
      i32.const 255
      i32.and
      i32.const 16843009
      i32.mul
      local.tee 3
      i32.store
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      i32.const -4
      i32.and
      local.tee 1
      i32.add
      local.tee 2
      i32.const -4
      i32.add
      local.get 3
      i32.store
      local.get 1
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 5
      local.get 3
      i32.store offset=8
      local.get 5
      local.get 3
      i32.store offset=4
      local.get 2
      i32.const -8
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -12
      i32.add
      local.get 3
      i32.store
      local.get 1
      i32.const 25
      i32.lt_u
      br_if 0 (;@1;)
      local.get 5
      local.get 3
      i32.store offset=24
      local.get 5
      local.get 3
      i32.store offset=20
      local.get 5
      local.get 3
      i32.store offset=16
      local.get 5
      local.get 3
      i32.store offset=12
      local.get 2
      i32.const -16
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -20
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -24
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -28
      i32.add
      local.get 3
      i32.store
      local.get 1
      local.get 5
      i32.const 4
      i32.and
      i32.const 24
      i32.or
      local.tee 2
      i32.sub
      local.tee 1
      i32.const 32
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i64.extend_i32_u
      i64.const 4294967297
      i64.mul
      local.set 6
      local.get 5
      local.get 2
      i32.add
      local.set 2
      loop  ;; label = @2
        local.get 2
        local.get 6
        i64.store offset=24
        local.get 2
        local.get 6
        i64.store offset=16
        local.get 2
        local.get 6
        i64.store offset=8
        local.get 2
        local.get 6
        i64.store
        local.get 2
        i32.const 32
        i32.add
        local.set 2
        local.get 1
        i32.const -32
        i32.add
        local.tee 1
        i32.const 31
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $strlen (type 4) (param i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 0
          i32.load8_u
          br_if 0 (;@3;)
          local.get 0
          local.get 0
          i32.sub
          return
        end
        local.get 0
        i32.const 1
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 2
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 3
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 4
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        br_if 1 (;@1;)
      end
      local.get 1
      i32.const -4
      i32.add
      local.set 2
      local.get 1
      i32.const -5
      i32.add
      local.set 1
      loop  ;; label = @2
        local.get 1
        i32.const 4
        i32.add
        local.set 1
        local.get 2
        i32.const 4
        i32.add
        local.tee 2
        i32.load
        local.tee 3
        i32.const -1
        i32.xor
        local.get 3
        i32.const -16843009
        i32.add
        i32.and
        i32.const -2139062144
        i32.and
        i32.eqz
        br_if 0 (;@2;)
      end
      loop  ;; label = @2
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.load8_u
        local.set 3
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        local.get 3
        br_if 0 (;@2;)
      end
    end
    local.get 1
    local.get 0
    i32.sub)
  (func $strnlen (type 3) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.const 0
    local.get 1
    call $memchr
    local.tee 2
    local.get 0
    i32.sub
    local.get 1
    local.get 2
    select)
  (func $__multi3 (type 21) (param i32 i64 i64 i64 i64)
    (local i64)
    local.get 0
    local.get 4
    local.get 1
    i64.mul
    local.get 2
    local.get 3
    i64.mul
    i64.add
    local.get 3
    i64.const 32
    i64.shr_u
    local.tee 2
    local.get 1
    i64.const 32
    i64.shr_u
    local.tee 4
    i64.mul
    i64.add
    local.get 3
    i64.const 4294967295
    i64.and
    local.tee 3
    local.get 1
    i64.const 4294967295
    i64.and
    local.tee 1
    i64.mul
    local.tee 5
    i64.const 32
    i64.shr_u
    local.get 3
    local.get 4
    i64.mul
    i64.add
    local.tee 3
    i64.const 32
    i64.shr_u
    i64.add
    local.get 3
    i64.const 4294967295
    i64.and
    local.get 2
    local.get 1
    i64.mul
    i64.add
    local.tee 1
    i64.const 32
    i64.shr_u
    i64.add
    i64.store offset=8
    local.get 0
    local.get 1
    i64.const 32
    i64.shl
    local.get 5
    i64.const 4294967295
    i64.and
    i64.or
    i64.store)
  (table (;0;) 7 7 funcref)
  (memory (;0;) 2)
  (global $__stack_pointer (mut i32) (i32.const 71360))
  (global $GOT.data.internal.__memory_base i32 (i32.const 0))
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (elem (;0;) (i32.const 1) func $__stdio_close $__stdio_write $__stdio_seek $__stdout_write $sn_write $string_read)
  (data $.rodata (i32.const 1024) "-+   0X0x\00-0X+0X 0X-0x+0x 0x\00malloc error\00nan\00inf\00merger_%d_%d\00NAN\00INF\00.\00(null)\00Support for formatting long double values is currently disabled.\0aTo enable it, add -lc-printscan-long-double to the link command.\0a\00merger_%d start!\0a\00merger_%d all finished!\0a\00Success\00Illegal byte sequence\00Domain error\00Result not representable\00Not a tty\00Permission denied\00Operation not permitted\00No such file or directory\00No such process\00File exists\00Value too large for data type\00No space left on device\00Out of memory\00Resource busy\00Interrupted system call\00Resource temporarily unavailable\00Invalid seek\00Cross-device link\00Read-only file system\00Directory not empty\00Connection reset by peer\00Operation timed out\00Connection refused\00Host is unreachable\00Address in use\00Broken pipe\00I/O error\00No such device or address\00No such device\00Not a directory\00Is a directory\00Text file busy\00Exec format error\00Invalid argument\00Argument list too long\00Symbolic link loop\00Filename too long\00Too many open files in system\00No file descriptors available\00Bad file descriptor\00No child process\00Bad address\00File too large\00Too many links\00No locks available\00Resource deadlock would occur\00State not recoverable\00Previous owner died\00Operation canceled\00Function not implemented\00No message of desired type\00Identifier removed\00Link has been severed\00Protocol error\00Bad message\00Not a socket\00Destination address required\00Message too large\00Protocol wrong type for socket\00Protocol not available\00Protocol not supported\00Not supported\00Address family not supported by protocol\00Address not available\00Network is down\00Network unreachable\00Connection reset by network\00Connection aborted\00No buffer space available\00Socket is connected\00Socket not connected\00Operation already in progress\00Operation in progress\00Stale file handle\00Quota exceeded\00Multihop attempted\00Capabilities insufficient\00\00\00\00\00u\02N\00\d6\01\e2\04\b9\04\18\01\8e\05\ed\02\16\04\f2\00\97\03\01\038\05\af\01\82\01O\03/\04\1e\00\d4\05\a2\00\12\03\1e\03\c2\01\de\03\08\00\ac\05\00\01d\02\f1\01e\054\02\8c\02\cf\02-\03L\04\e3\05\9f\02\f8\04\1c\05\08\05\b1\02K\05\15\02x\00R\02<\03\f1\03\e4\00\c3\03}\04\cc\00\aa\03y\05$\02n\01m\03\22\04\ab\04D\00\fb\01\ae\00\83\03`\00\e5\01\07\04\94\04^\04+\00X\019\01\92\00\c2\05\9b\01C\02F\01\f6\05\00\00\00\00\00\00\19\00\0a\00\19\19\19\00\00\00\00\05\00\00\00\00\00\00\09\00\00\00\00\0b\00\00\00\00\00\00\00\00\19\00\11\0a\19\19\19\03\0a\07\00\01\1b\09\0b\18\00\00\09\06\0b\00\00\0b\00\06\19\00\00\00\19\19\19\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\0e\00\00\00\00\00\00\00\00\19\00\0a\0d\19\19\19\00\0d\00\00\02\00\09\0e\00\00\00\09\00\0e\00\00\0e\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\0c\00\00\00\00\00\00\00\00\00\00\00\13\00\00\00\00\13\00\00\00\00\09\0c\00\00\00\00\00\0c\00\00\0c\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\00\00\00\00\0f\00\00\00\04\0f\00\00\00\00\09\10\00\00\00\00\00\10\00\00\10\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\12\00\00\00\00\00\00\00\00\00\00\00\11\00\00\00\00\11\00\00\00\00\09\12\00\00\00\00\00\12\00\00\12\00\00\1a\00\00\00\1a\1a\1a\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\1a\00\00\00\1a\1a\1a\00\00\00\00\00\00\09\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\14\00\00\00\00\00\00\00\00\00\00\00\17\00\00\00\00\17\00\00\00\00\09\14\00\00\00\00\00\14\00\00\14\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\16\00\00\00\00\00\00\00\00\00\00\00\15\00\00\00\00\15\00\00\00\00\09\16\00\00\00\00\00\16\00\00\16\00\000123456789ABCDEF\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\00\01\02\03\04\05\06\07\08\09\ff\ff\ff\ff\ff\ff\ff\0a\0b\0c\0d\0e\0f\10\11\12\13\14\15\16\17\18\19\1a\1b\1c\1d\1e\1f !\22#\ff\ff\ff\ff\ff\ff\0a\0b\0c\0d\0e\0f\10\11\12\13\14\15\16\17\18\19\1a\1b\1c\1d\1e\1f !\22#\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\00\01\02\04\07\03\06\05\00\00\00\00\00\00\00\0a\00\00\00d\00\00\00\e8\03\00\00\10'\00\00\a0\86\01\00@B\0f\00\80\96\98\00\00\e1\f5\05\02\00\00\c0\03\00\00\c0\04\00\00\c0\05\00\00\c0\06\00\00\c0\07\00\00\c0\08\00\00\c0\09\00\00\c0\0a\00\00\c0\0b\00\00\c0\0c\00\00\c0\0d\00\00\c0\0e\00\00\c0\0f\00\00\c0\10\00\00\c0\11\00\00\c0\12\00\00\c0\13\00\00\c0\14\00\00\c0\15\00\00\c0\16\00\00\c0\17\00\00\c0\18\00\00\c0\19\00\00\c0\1a\00\00\c0\1b\00\00\c0\1c\00\00\c0\1d\00\00\c0\1e\00\00\c0\1f\00\00\c0\00\00\00\b3\01\00\00\c3\02\00\00\c3\03\00\00\c3\04\00\00\c3\05\00\00\c3\06\00\00\c3\07\00\00\c3\08\00\00\c3\09\00\00\c3\0a\00\00\c3\0b\00\00\c3\0c\00\00\c3\0d\00\00\d3\0e\00\00\c3\0f\00\00\c3\00\00\0c\bb\01\00\0c\c3\02\00\0c\c3\03\00\0c\c3\04\00\0c\db")
  (data $.data (i32.const 3984) "\05\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\00\00\00\03\00\00\00\a4\12\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\ff\ff\ff\ff\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\90\0f\00\00\00\00\00\00\05\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\04\00\00\00\03\00\00\00\b8\12\00\00\00\04\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\0a\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\08\10\00\00"))
