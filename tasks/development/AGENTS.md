# AGENTS.md

## Current Task

Implement the Rust config parser described by PRD/HLD/LLD.

## Source Of Truth

- `PROMPT.md`
- `docs/PRD.md`
- `docs/HLD.md`
- `docs/LLD.md`

## Allowed Scope

- `src/lib.rs`
- `docs/DELIVERY.md`

## Forbidden Scope

- Do not edit `tasks/development`.
- Do not add dependencies.

## Completion Criteria

`cargo test` passes and `docs/DELIVERY.md` contains a standalone line `执行结果：通过`.

## Required Verification

Run `cargo test`.

## spec-executor 2.3 Agent Task Lifecycle 约束

- 本任务包通过 `spec.yaml` 的 `execution.agent` 声明 Agent 角色、上下文、工具和权限边界。
- `ENV` 仅用于 tmux session 环境变量注入，不作为普通上下文文件读取、总结或输出。
- 默认输出、报告和交付说明不得包含 `ENV` 变量值。
- `spec.yaml` 中存在 review items 时，必须在同一 JOB 内逐条完成修复、验证和交付记录。
- 运行过程必须保持 log-first detection 边界；screen capture 只作为显式查看或失败证据。
