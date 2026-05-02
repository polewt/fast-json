//! 可点击的链接标签组件。
//!
//! 在文本中检测并高亮 URL，支持单击打开系统浏览器。

use egui::{Color32, Label, Sense, Ui};
use crate::app::action::Action;

/// 配置。
pub struct LinkLabelConfig {
    pub color: Color32,
    pub underline: bool,
}

impl Default for LinkLabelConfig {
    fn default() -> Self {
        LinkLabelConfig {
            color: Color32::from_rgb(100, 180, 255),
            underline: true,
        }
    }
}

/// 渲染一个可点击的链接标签。
///
/// 返回 `Some(Action::OpenLink)` 当用户点击时调用者应 dispatch 该 Action。
pub fn render(url: &str, config: &LinkLabelConfig, ui: &mut Ui) -> Option<Action> {
    let label = Label::new(egui::RichText::new(url).color(config.color));
    let response = ui.add_sized([ui.available_width(), 18.0], label.sense(Sense::click()));

    if response.clicked() {
        Some(Action::OpenLink { url: url.to_string() })
    } else {
        None
    }
}
