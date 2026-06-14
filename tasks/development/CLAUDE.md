# CLAUDE.md

## Assignment

完成 Rust 开发任务：根据 PRD/HLD/LLD 实现 `parse_config`，补齐测试和交付说明。

## Context To Load

- `docs/PRD.md`
- `docs/HLD.md`
- `docs/LLD.md`
- `src/lib.rs`

## Constraints

- 不引入外部 crate。
- 不修改 task package。

## Acceptance Criteria

- `cargo test` 通过。
- 错误信息包含行号。
- `docs/DELIVERY.md` 记录实现和验证，并包含独立一行 `执行结果：通过`。

## Verification Method

运行 `cargo test`。

## Handoff

说明实现、测试和错误边界。
