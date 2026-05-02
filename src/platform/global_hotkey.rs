//! 全局快捷键。
//!
//! 程序在后台运行时，按下全局快捷键 (如 Ctrl+Shift+J) 即可呼出主窗口。
//!
//! TODO: 使用 `global-hotkey` crate 实现。
//!
//! ## 平台注意事项
//!
//! - **Windows**: 需要消息循环 (event loop)
//! - **macOS**: 需要辅助功能权限
//! - **Linux (X11)**: 需要 X11 扩展；Wayland 仅部分支持
//!
//! ## 依赖
//!
//! ```toml
//! [dependencies]
//! global-hotkey = "0.6"
//! ```

use crate::app::action::Action;

/// 默认全局快捷键。
pub const DEFAULT_HOTKEY: &str = "Ctrl+Shift+J";

/// 注册全局快捷键。
pub fn register(hotkey_str: &str) {
    // TODO: 解析 hotkey_str -> global-hotkey 的 HotKey -> register
    log::info!("Global hotkey registered (stub): {hotkey_str}");
}

/// 注销全局快捷键。
pub fn unregister() {
    // TODO
    log::info!("Global hotkey unregistered (stub)");
}

/// 轮询全局快捷键事件 (返回触发时对应的 Action)。
pub fn poll() -> Option<Action> {
    // TODO: 非阻塞轮询
    None
}
