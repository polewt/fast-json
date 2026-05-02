//! 平台抽象层。
//!
//! 封装操作系统相关功能：
//! - 系统托盘图标 & 菜单
//! - 全局快捷键 (后台呼出)
//! - 文件打开/保存对话框
//! - 文件拖放
//!
//! 所有模块通过 feature flag 控制编译，
//! 平台不支持时提供空实现或日志警告。

#[cfg(feature = "platform-tray")]
pub mod tray;

#[cfg(feature = "platform-hotkey")]
pub mod global_hotkey;

#[cfg(feature = "platform-file")]
pub mod file;
