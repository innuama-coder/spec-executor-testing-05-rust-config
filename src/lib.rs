//! 无依赖的简单配置解析库。
//!
//! 解析 `key=value` 文本配置，忽略空行与 `#` 注释，trim 两侧空白，
//! 并在遇到非法行（缺少 `=` 或空 key）时返回包含行号的错误。

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

/// 配置解析错误，包含出错的行号（从 1 开始）与人类可读的原因。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigParseError {
    /// 出错行号，从 1 开始计数。
    pub line: usize,
    /// 错误原因描述。
    pub message: String,
}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl Error for ConfigParseError {}

/// 注释行前缀。
const COMMENT_PREFIX: char = '#';

/// key 与 value 的分隔符。
const KV_SEPARATOR: char = '=';

/// 将配置文本解析为有序的 `BTreeMap`。
///
/// 规则：
/// - 支持 `key=value` 行。
/// - 忽略空行和以 `#` 开头的注释行（trim 之后判断）。
/// - 去除 key 与 value 两侧的空白。
/// - 缺少 `=` 或 key 为空时返回包含行号的 [`ConfigParseError`]。
///
/// 后出现的同名 key 会覆盖先前的值。
pub fn parse_config(input: &str) -> Result<BTreeMap<String, String>, ConfigParseError> {
    let mut config = BTreeMap::new();

    for (index, raw_line) in input.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = raw_line.trim();

        // 忽略空行与注释行。
        if trimmed.is_empty() || trimmed.starts_with(COMMENT_PREFIX) {
            continue;
        }

        let Some((raw_key, raw_value)) = trimmed.split_once(KV_SEPARATOR) else {
            return Err(ConfigParseError {
                line: line_number,
                message: format!("missing '{KV_SEPARATOR}' separator"),
            });
        };

        let key = raw_key.trim();
        if key.is_empty() {
            return Err(ConfigParseError {
                line: line_number,
                message: "empty key".to_string(),
            });
        }

        let value = raw_value.trim();
        config.insert(key.to_string(), value.to_string());
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_line() {
        // Arrange
        let input = "key=value";

        // Act
        let config = parse_config(input).expect("single line should parse");

        // Assert
        assert_eq!(config.get("key"), Some(&"value".to_string()));
        assert_eq!(config.len(), 1);
    }

    #[test]
    fn parses_multiple_lines_ignoring_comments_and_blanks() {
        // Arrange
        let input = "\
# this is a comment
host=localhost

port=8080
   # indented comment
name=demo
";

        // Act
        let config = parse_config(input).expect("multi line should parse");

        // Assert
        assert_eq!(config.get("host"), Some(&"localhost".to_string()));
        assert_eq!(config.get("port"), Some(&"8080".to_string()));
        assert_eq!(config.get("name"), Some(&"demo".to_string()));
        assert_eq!(config.len(), 3);
    }

    #[test]
    fn trims_key_and_value_whitespace() {
        // Arrange
        let input = "   key   =    value   ";

        // Act
        let config = parse_config(input).expect("whitespace should be trimmed");

        // Assert
        assert_eq!(config.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn allows_empty_value() {
        // Arrange
        let input = "key=";

        // Act
        let config = parse_config(input).expect("empty value should be allowed");

        // Assert
        assert_eq!(config.get("key"), Some(&"".to_string()));
    }

    #[test]
    fn errors_when_separator_missing() {
        // Arrange
        let input = "host=localhost\nnoseparator\nport=8080";

        // Act
        let error = parse_config(input).expect_err("missing separator should error");

        // Assert
        assert_eq!(error.line, 2);
        assert!(error.message.contains("separator"));
    }

    #[test]
    fn errors_when_key_is_empty() {
        // Arrange
        let input = "  =value";

        // Act
        let error = parse_config(input).expect_err("empty key should error");

        // Assert
        assert_eq!(error.line, 1);
        assert!(error.message.contains("empty key"));
    }

    #[test]
    fn later_keys_override_earlier_ones() {
        // Arrange
        let input = "key=first\nkey=second";

        // Act
        let config = parse_config(input).expect("duplicate keys should be allowed");

        // Assert
        assert_eq!(config.get("key"), Some(&"second".to_string()));
    }

    #[test]
    fn value_may_contain_separator() {
        // Arrange
        let input = "url=http://example.com/?a=1&b=2";

        // Act
        let config = parse_config(input).expect("value may contain '='");

        // Assert
        assert_eq!(
            config.get("url"),
            Some(&"http://example.com/?a=1&b=2".to_string())
        );
    }

    #[test]
    fn error_display_includes_line_number() {
        // Arrange
        let error = ConfigParseError {
            line: 7,
            message: "empty key".to_string(),
        };

        // Act
        let rendered = error.to_string();

        // Assert
        assert!(rendered.contains("line 7"));
        assert!(rendered.contains("empty key"));
    }

    #[test]
    fn empty_input_yields_empty_map() {
        // Arrange
        let input = "";

        // Act
        let config = parse_config(input).expect("empty input should yield empty map");

        // Assert
        assert!(config.is_empty());
    }
}
