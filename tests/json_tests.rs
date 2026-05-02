//! JSON 解析/格式化集成测试。

use fast_json::core::json;
use fast_json::core::json::types::FormatOptions;

#[test]
fn parse_valid_object() {
    let result = json::parse(r#"{"name":"Alice","age":30}"#);
    assert!(result.is_ok());
}

#[test]
fn parse_invalid_json() {
    let result = json::parse(r#"{"name":"Alice","age":}"#);
    assert!(result.is_err());
}

#[test]
fn format_pretty_with_indent() {
    let value = json::parse(r#"{"b":2,"a":1}"#).unwrap();
    let opts = FormatOptions { indent: 2, ..Default::default() };
    let formatted = json::format_pretty(&value, &opts);
    assert!(formatted.contains('\n'));
    assert!(formatted.contains("  "));
}

#[test]
fn format_compact() {
    let value = json::parse(r#"{"b":2,"a":1}"#).unwrap();
    let compact = json::format_compact(&value);
    assert!(!compact.contains('\n'));
}

#[test]
fn validate_correct_json() {
    assert!(json::validate(r#"{"valid":true}"#).is_ok());
}

#[test]
fn validate_broken_json() {
    assert!(json::validate(r#"{"broken":}"#).is_err());
}
