# MSlibOS

## 构建
```bash
cargo build --workspace
```

## 增加新工作流
```bash
cargo run -p gen-file
vim config.json
mv config.json isol_config/[your-isol-name].json
```

## 测试
运行所有测试
```bash
cargo test
```