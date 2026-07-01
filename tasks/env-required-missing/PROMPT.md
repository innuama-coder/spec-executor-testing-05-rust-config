# rust-config-env-required-missing 执行说明

## 目标

验证 ENV 必需但缺失时的 pre-JOB 诊断边界。

## 2.3 约束

- ENV required missing pre-JOB typed diagnostic；该任务包预期在 run 前失败且不创建 JOB。
- 本任务包声明 ENV；变量只能通过进程环境读取，不能输出变量值。
- 本任务包不包含外部评审意见。
- 不得修改任务包指令文件本身。
- 必须满足 `spec.yaml` 中的验证命令和交付物要求。
