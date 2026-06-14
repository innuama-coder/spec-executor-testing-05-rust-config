# PRD - Rust 配置解析库

## 目标

提供一个无依赖 Rust 函数 `parse_config`，将简单文本配置解析为 `BTreeMap<String, String>`。

## 功能需求

| ID | 需求 |
| --- | --- |
| FR-001 | 支持 `key=value` 行。 |
| FR-002 | 忽略空行和以 `#` 开头的注释。 |
| FR-003 | 去除 key 与 value 两侧空白。 |
| FR-004 | 遇到缺少 `=` 或空 key 的行返回错误。 |

## 验收

`cargo test` 必须通过。

