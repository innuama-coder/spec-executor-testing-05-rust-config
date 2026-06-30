//! 集成验收测试：覆盖注释、空行、trim、缺少等号、空 key 与行号。

use spec_executor_testing_05_rust_config::{parse_config, ConfigParseError};

#[test]
fn parses_full_config_with_comments_blanks_and_trim() {
    let input = "\
# server configuration
host =   localhost

   port=8080
name = demo service
";

    let config = parse_config(input).expect("valid config should parse");

    assert_eq!(config.get("host"), Some(&"localhost".to_string()));
    assert_eq!(config.get("port"), Some(&"8080".to_string()));
    assert_eq!(config.get("name"), Some(&"demo service".to_string()));
    assert_eq!(config.len(), 3);
}

#[test]
fn reports_line_number_for_missing_separator() {
    let input = "host=localhost\n# comment\nbroken-line\nport=8080";

    let error = parse_config(input).expect_err("missing separator should error");

    assert_eq!(error.line, 3);
    assert!(error.message.contains("separator"));
}

#[test]
fn reports_line_number_for_empty_key() {
    let input = "valid=1\n=oops";

    let error: ConfigParseError = parse_config(input).expect_err("empty key should error");

    assert_eq!(error.line, 2);
    assert!(error.message.contains("empty key"));
}

#[test]
fn error_display_contains_line_number() {
    let input = "noseparator";

    let error = parse_config(input).expect_err("should error");

    assert!(error.to_string().contains("line 1"));
}
