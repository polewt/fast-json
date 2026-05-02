//! 系统托盘。
//!
//! 程序最小化到托盘后：
//! - 左键单击托盘图标 -> 显示主窗口
//! - 右键菜单 -> Show / Quit
//!
//! TODO: 使用 `tray-icon` crate 实现。
//!
//! ## 依赖
//!
//! ```toml
//! [dependencies]
//! tray-icon = "0.19"
//! ```

use crate::app::action::Action;

/// 初始化系统托盘。
pub fn init() {
    // TODO: 创建托盘图标 + 菜单
    log::info!("System tray initialized (stub)");
}

/// 托盘事件 -> Action 转换。
pub fn handle_event() -> Option<Action> {
    // TODO: 处理托盘菜单点击
    None
}
