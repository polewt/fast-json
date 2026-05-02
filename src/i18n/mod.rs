//! 国际化 (Internationalization)。
//!
//! 当前支持：简体中文 (zh-CN) 和 English (en)。
//!
//! ## 使用方式
//!
//! ```ignore
//! use crate::i18n;
//! let text = i18n::tr("app.title");
//! ```

pub mod keys;
pub mod loader;

pub use loader::{current_language, set_language, tr};
