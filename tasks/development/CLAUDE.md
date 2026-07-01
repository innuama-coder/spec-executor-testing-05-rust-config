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

## spec-executor 2.3 Agent Task Lifecycle 约束

- 本任务包通过 `spec.yaml` 的 `execution.agent` 声明 Agent 角色、上下文、工具和权限边界。
- `ENV` 仅用于 tmux session 环境变量注入，不作为普通上下文文件读取、总结或输出。
- 默认输出、报告和交付说明不得包含 `ENV` 变量值。
- `spec.yaml` 中存在 review items 时，必须在同一 JOB 内逐条完成修复、验证和交付记录。
- 运行过程必须保持 log-first detection 边界；screen capture 只作为显式查看或失败证据。
