//! 异步流处理工具。
//!
//! 预留用于：
//! - 大文件流式读取 JSONL
//! - 剪贴板变更事件流
//! - 全局快捷键事件流

use std::future::Future;

/// 异步事件流 trait (预留)。
pub trait EventStream<T> {
    fn poll(&mut self) -> impl Future<Output = Option<T>>;
}
