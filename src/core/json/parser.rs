//! JSON 解析器。
//!
//! 使用 sonic-rs 进行高性能 SIMD 解析。

use crate::core::json::types::ParseError;

/// 解析 JSON 字符串。
///
/// 使用 sonic-rs 的 SIMD 加速解析，比 serde_json 快 2-3 倍。
/// 解析结果仍为 serde_json::Value，保持与格式化器的兼容性。
pub fn parse(input: &str) -> Result<serde_json::Value, ParseError> {
    sonic_rs::from_str::<serde_json::Value>(input).map_err(|e| ParseError {
        message: e.to_string(),
        line: 0,
        column: 0,
    })
}
