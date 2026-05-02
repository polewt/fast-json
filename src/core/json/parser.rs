//! JSON 解析器。
//!
//! 使用 sonic-rs 进行高性能 SIMD 解析，
//! 返回本项目的抽象类型 `JsonValue`。

use crate::core::json::types::ParseError;

/// 解析 JSON 字符串。
///
/// TODO: 改用 sonic-rs 原生解析 + 精确定位错误行列号。
/// 当前阶段使用 serde_json 作为过渡。
pub fn parse(input: &str) -> Result<serde_json::Value, ParseError> {
    serde_json::from_str(input).map_err(|e| ParseError {
        message: e.to_string(),
        line: e.line(),
        column: e.column(),
    })
}
