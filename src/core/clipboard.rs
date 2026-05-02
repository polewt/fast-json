//! 剪贴板抽象。
//!
//! 提供平台无关的剪贴板读写接口，
//! 并为未来的剪贴板监听功能预留 trait 接口。

/// 剪贴板操作 trait。
///
/// 不同平台 (Windows/macOS/Linux) 各有实现，
/// 调用方只需依赖此 trait。
pub trait ClipboardProvider: Send + Sync {
    fn read(&self) -> Option<String>;
    fn write(&self, text: &str);
}

// -- 平台实现 --

#[cfg(feature = "platform-clipboard")]
pub struct SystemClipboard;

#[cfg(feature = "platform-clipboard")]
impl ClipboardProvider for SystemClipboard {
    fn read(&self) -> Option<String> {
        arboard::Clipboard::new()
            .and_then(|mut c| c.get_text())
            .ok()
    }

    fn write(&self, text: &str) {
        if let Ok(mut c) = arboard::Clipboard::new() {
            let _ = c.set_text(text);
        }
    }
}

/// 获取默认剪贴板实现。
#[cfg(feature = "platform-clipboard")]
pub fn default_provider() -> impl ClipboardProvider {
    SystemClipboard
}

// -- 剪贴板监听 (预留) --

/// 剪贴板变更事件。
#[derive(Debug, Clone)]
pub struct ClipboardChangeEvent {
    pub text: String,
}

/// 剪贴板监听器 trait (预留)。
///
/// TODO: 平台原生实现或轮询方案。
pub trait ClipboardWatcher {
    fn watch<F: Fn(ClipboardChangeEvent) + Send + 'static>(&self, callback: F);
}
