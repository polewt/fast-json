//! # Fast JSON - 高性能跨平台 JSON 格式化工具
//!
//! ## 架构分层
//!
//! - **`core`** - 核心层，零 GUI 依赖，可独立用于 CLI / WASM / 库调用
//! - **`app`**  - 应用层，状态管理、Action 分发、快捷键映射
//! - **`ui`**   - 视图层，纯 egui 渲染，不包含业务逻辑
//! - **`i18n`** - 国际化 (zh-CN / en)
//! - **`platform`** - 平台抽象 (系统托盘、全局快捷键、文件对话框)
//! - **`util`** - 通用工具
//!
//! ## 库调用示例
//!
//! ```rust
//! use fast_json::{parse, format_pretty, FormatOptions};
//!
//! let value = parse(r#"{"name":"Alice","age":30}"#).unwrap();
//! let pretty = format_pretty(&value, &FormatOptions::default());
//! println!("{pretty}");
//! ```

// -- 核心模块 (no feature gate, always available) --
pub mod core;

// -- 应用层 (requires `gui` feature) --
#[cfg(feature = "gui")]
pub mod app;

// -- 视图层 (requires `gui` feature) --
#[cfg(feature = "gui")]
pub mod ui;

// -- 国际化 (requires `i18n` feature) --
#[cfg(feature = "i18n")]
pub mod i18n;

// -- 平台抽象 (requires `gui` feature) --
#[cfg(any(
    feature = "platform-clipboard",
    feature = "platform-file",
    feature = "platform-tray",
    feature = "platform-hotkey",
    feature = "platform-link",
))]
pub mod platform;

// -- 工具模块 --
pub mod util;

// -- 常用类型重导出 --
pub use core::json::types::{FormatOptions, JsonNode, ParseError};
pub use core::json::{parse, format_compact, format_pretty, validate};
