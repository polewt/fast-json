//! JSON 格式化器。
//!
//! 将 JSON Value 格式化为缩进美观或紧凑的字符串。

use serde_json::Value;
use crate::core::json::types::FormatOptions;

/// 美化格式化 (带缩进)。
pub fn format_pretty(value: &Value, opts: &FormatOptions) -> String {
    if opts.sort_keys {
        let sorted = sort_value_keys(value);
        format_with_indent(&sorted, opts.indent)
    } else {
        format_with_indent(value, opts.indent)
    }
}

/// 紧凑格式化 (单行，无空白)。
pub fn format_compact(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

fn format_with_indent(value: &Value, indent: usize) -> String {
    if indent == 0 {
        return format_compact(value);
    }
    serde_json::to_string_pretty(value).unwrap_or_default()
}

/// 递归对 object 的 key 排序 (深度优先)。
fn sort_value_keys(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<_> = map
                .iter()
                .map(|(k, v)| (k.clone(), sort_value_keys(v)))
                .collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            Value::Object(
                entries
                    .into_iter()
                    .collect(),
            )
        }
        Value::Array(arr) => Value::Array(arr.iter().map(sort_value_keys).collect()),
        other => other.clone(),
    }
}
