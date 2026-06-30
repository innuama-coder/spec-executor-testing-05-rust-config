# DELIVERY - Rust 配置解析库

## 验收用途

本文档用于记录执行者完成 `rust-config-parser` 后的交付证据。执行者必须在完成实现后更新“执行记录”，否则验收不通过。

## 交付物

| 交付物 | 验收要点 |
| --- | --- |
| `src/lib.rs` | 实现 `parse_config` 和 `ConfigParseError`，处理正常配置、注释、空行、trim 和错误行。 |
| `docs/DELIVERY.md` | 记录实现摘要、验证命令和执行结果。 |

## 验收命令

```bash
cargo test
```

## 执行记录

实现摘要：在 `src/lib.rs` 中实现了无外部依赖的 `parse_config(input: &str) -> Result<BTreeMap<String, String>, ConfigParseError>`。逐行解析配置文本，先对每行做 `trim`：空行与以 `#` 开头的注释行被忽略；其余行使用 `split_once('=')` 拆分 key/value 并分别 `trim`。缺少 `=` 分隔符或 key 为空时，返回携带行号（从 1 计数）的 `ConfigParseError`，其 `Display` 输出形如 `line {n}: {message}`，错误信息包含行号。结果使用标准库 `BTreeMap` 保证稳定有序输出；value 允许为空或包含 `=`，同名 key 后值覆盖先值。`ConfigParseError` 包含 `line: usize` 与 `message: String` 字段并实现 `std::error::Error`。

测试：`src/lib.rs` 含 10 个单元测试（单行、多行+注释+空行、trim、空 value、缺少等号报错并校验行号、空 key 报错、重复 key 覆盖、value 含等号、Display 含行号、空输入）。`tests/acceptance_config.rs` 含 4 个集成验收测试，覆盖注释/空行/trim、缺少等号的行号、空 key 的行号以及 Display 行号。共 14 个测试全部通过。

验证命令：`cargo test`

验证输出：`test result: ok. 10 passed; 0 failed`（单元）、`test result: ok. 4 passed; 0 failed`（集成），进程退出码 0。

执行结果：通过

## 通过标准

- `cargo test` 返回 0。
- 集成验收测试覆盖注释、空行、trim、缺少等号、空 key 和行号。
- 执行记录中的 `执行结果` 字段必须更新为通过状态。
