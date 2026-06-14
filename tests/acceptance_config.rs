use spec_executor_testing_05_rust_config::parse_config;

#[test]
fn parses_comments_blank_lines_and_trimmed_pairs() {
    let parsed = parse_config(
        r#"
        # service configuration
        host = localhost

        port= 8080
        mode = test
        "#,
    )
    .expect("valid config should parse");

    assert_eq!(parsed.get("host").map(String::as_str), Some("localhost"));
    assert_eq!(parsed.get("port").map(String::as_str), Some("8080"));
    assert_eq!(parsed.get("mode").map(String::as_str), Some("test"));
}

#[test]
fn reports_line_number_for_missing_equals() {
    let err = parse_config("ok=yes\ninvalid-line\n").expect_err("missing equals should fail");
    assert_eq!(err.line, 2);
    assert!(
        err.message.contains("="),
        "error message should explain missing '=': {}",
        err.message
    );
}

#[test]
fn reports_line_number_for_empty_key() {
    let err = parse_config("=value\n").expect_err("empty key should fail");
    assert_eq!(err.line, 1);
    assert!(
        err.message.to_lowercase().contains("key"),
        "error message should mention key: {}",
        err.message
    );
}
