//! 防抖工具。
//!
//! 用于输入框实时格式化场景--用户输入时延迟处理，
//! 避免每次按键都重新解析大 JSON。

use std::time::{Duration, Instant};

/// 防抖器。
pub struct Debouncer {
    delay: Duration,
    last_trigger: Option<Instant>,
}

impl Debouncer {
    pub fn new(delay_ms: u64) -> Self {
        Debouncer {
            delay: Duration::from_millis(delay_ms),
            last_trigger: None,
        }
    }

    /// 如果距离上次触发超过 delay，返回 true 并更新计时器。
    pub fn should_fire(&mut self) -> bool {
        let now = Instant::now();
        if self.last_trigger.map_or(true, |t| now - t >= self.delay) {
            self.last_trigger = Some(now);
            true
        } else {
            false
        }
    }

    /// 重置计时器。
    pub fn reset(&mut self) {
        self.last_trigger = None;
    }
}
