# AlloyStack

<!-- [![CI](https://github.com/anti-entropy123/mslibos/actions/workflows/main.yml/badge.svg)](https://github.com/anti-entropy123/mslibos/actions/workflows/main.yml) -->

## 构建

### 全部构建

不使用 MPK：

```bash
./scripts/build_all_common.sh
./scripts/build_user.sh
```

使用 MPK

```bash
./scripts/build_all_common_mpk.sh
./scripts/build_user.sh mpk
```

### 单个构建

不使用 MPK：

```bash
./scripts/build_all_common.sh
cargo build --manifest-path user/<appname>/Cargo.toml
```

使用 MPK:

```bash
./scripts/build_all_common_mpk.sh
Cargo build --features mpk --manifest-path user/<appname>/Cargo.toml
```

## 增加新工作流

```bash
cargo run -p gen-file
vim config.json
mv config.json isol_config/[your-isol-name].json
```

## 测试

### 全部测试
不使用 MPK：

```bash
./scripts/run_tests.sh
```

使用 MPK：

```bash
./scripts/run_tests.sh mpk
```

### 单个测试

不使用 MPK：

```bash
cargo run -- --files isol_config/<workflowname>.json
```

使用 MPK：

```bash
cargo run --features mpk -- --files isol_config/<workflowname>.json
```

