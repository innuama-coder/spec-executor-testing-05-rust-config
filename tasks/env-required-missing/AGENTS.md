# Codex Agent 指令

## 角色

验证 ENV 必需但缺失时的 pre-JOB 诊断边界。

## 任务边界

- 遵守 `spec.yaml` 中的 Agent、ENV、review、verification 与 reporting 合同。
- `ENV` 仅用于 tmux session 环境变量注入，不得作为普通上下文文件读取。
- 默认输出、报告和交付说明不得包含 `ENV` 变量值。
- 对 review items 的修复必须形成可验证交付结果。
