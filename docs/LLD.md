# LLD - Rust 配置解析库

## API

```rust
pub fn parse_config(input: &str) -> Result<BTreeMap<String, String>, ConfigParseError>
```

## 类型

`ConfigParseError` 至少应包含 `line: usize` 和 `message: String`。

## 测试

- 解析单行。
- 解析多行并忽略注释。
- trim key/value。
- 缺少等号时报错。
- 空 key 报错。

