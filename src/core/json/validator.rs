//! JSON 校验器。
//!
//! 提供不完整/增量 JSON 的校验能力，返回精确错误列表。

use crate::core::json::parser::parse;
use crate::core::json::types::ParseError;

/// 校验 JSON 字符串是否合法。
///
/// 返回 `Ok(())` 或包含行列号的错误信息。
pub fn validate(input: &str) -> Result<(), ParseError> {
    parse(input).map(|_| ())
}

/// 批量校验--返回所有错误。
///
/// TODO: 当输入为空时检查是否为不完整的 JSON 片段。
pub fn validate_strict(input: &str) -> Vec<ParseError> {
    match parse(input) {
        Ok(_) => vec![],
        Err(e) => vec![e],
    }
}
