//! JSON 解析/格式化/校验子系统。
//!
//! 这是整个应用的核心--所有 JSON 处理逻辑集中于此，
//! 不依赖任何 GUI 库，可独立用于 CLI、WASM、嵌入式等场景。

pub mod formatter;
pub mod parser;
pub mod tree;
pub mod types;
pub mod validator;

pub use formatter::{format_compact, format_pretty};
pub use parser::parse;
pub use tree::build_flat_tree;
pub use validator::validate;
