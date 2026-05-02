//! 文件操作。
//!
//! - 打开文件对话框
//! - 保存文件对话框
//! - 文件拖放到窗口
//!
//! TODO: 使用 `rfd` (Rust File Dialogs) 实现。
//!
//! ## 依赖
//!
//! ```toml
//! [dependencies]
//! rfd = "0.15"
//! ```

/// 打开文件对话框，返回选中的文件内容。
///
/// TODO: 使用 rfd::FileDialog 实现异步文件选择。
pub async fn open_file_dialog() -> Option<(String, String)> {
    // (filename, content)
    log::info!("File open dialog (stub)");
    None
}

/// 保存文件对话框。
///
/// TODO: 使用 rfd::FileDialog 实现。
pub async fn save_file_dialog(default_name: &str, _content: &str) -> bool {
    log::info!("File save dialog (stub): {default_name}");
    false
}
