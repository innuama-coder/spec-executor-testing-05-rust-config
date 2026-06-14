# spec-executor-testing-05-rust-config

Rust 开发型独立测试仓库。该仓库用于验证 spec-executor 是否能够驱动执行者完成一个小型 Rust 库任务：解析简单 `key=value` 配置文本，处理注释、空行和错误输入。

## 目录

- `docs/PRD.md`
- `docs/HLD.md`
- `docs/LLD.md`
- `docs/DELIVERY.md`
- `Cargo.toml`
- `src/lib.rs`
- `tests/acceptance_config.rs`
- `tasks/development/`

## 运行

```bash
spec-executor run --spec tasks/development/spec.yaml --workspace ./workspace
```
