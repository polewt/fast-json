//! JSON 类型定义。
//!
//! 定义本项目使用的 JSON 抽象类型，隔离底层库 (sonic-rs/serde_json) 的实现细节。

use serde::{Deserialize, Serialize};

/// 格式化选项。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatOptions {
    /// 缩进空格数 (0 = compact)
    pub indent: usize,
    /// 是否按 key 排序
    pub sort_keys: bool,
    /// 是否将 Unicode 转义为 \uXXXX
    pub escape_unicode: bool,
}

impl Default for FormatOptions {
    fn default() -> Self {
        FormatOptions {
            indent: 2,
            sort_keys: false,
            escape_unicode: false,
        }
    }
}

/// JSON 解析错误。
#[derive(Debug, Clone)]
pub struct ParseError {
    /// 错误信息
    pub message: String,
    /// 错误所在行号 (1-based)
    pub line: usize,
    /// 错误所在列号 (1-based)
    pub column: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (line {}, col {})", self.message, self.line, self.column)
    }
}

/// JSON 树节点--用于树形视图展示。
#[derive(Debug, Clone)]
pub struct JsonNode {
    /// 键名 (数组元素为 None)
    pub key: Option<String>,
    /// 值 (字符串表示)
    pub value: String,
    /// 节点类型
    pub kind: JsonNodeKind,
    /// 子节点
    pub children: Vec<JsonNode>,
    /// 节点深度
    pub depth: usize,
    /// 已展开
    pub expanded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonNodeKind {
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
}
