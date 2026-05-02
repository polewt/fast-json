//! JSON 编辑器组件 (可复用)。
//!
//! 封装 TextEdit + 语法高亮 layouter，可在多个面板中复用。
//!
//! TODO: 行号 gutter、括号匹配、错误下划线、自动补全、折叠。

use egui::{Color32, FontId, TextFormat, TextStyle, Ui};

/// 定义一个 JSON 编辑器的外观和行为。
pub struct JsonEditorConfig {
    pub font_size: f32,
    pub editable: bool,
    pub syntax_highlight: bool,
    pub line_numbers: bool,
    pub placeholder: String,
}

impl Default for JsonEditorConfig {
    fn default() -> Self {
        JsonEditorConfig {
            font_size: 14.0,
            editable: true,
            syntax_highlight: true,
            line_numbers: false,
            placeholder: String::new(),
        }
    }
}

/// 渲染一个 JSON 文本编辑器。
///
/// TODO:
/// - 实现完整的 JSON 语法高亮 (或集成 syntect)
/// - 行号 gutter
/// - 括号匹配高亮
/// - 错误位置下划线
pub fn render(text: &mut String, config: &JsonEditorConfig, ui: &mut Ui) {
    let font_id = FontId::monospace(config.font_size);

    let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
        let mut job = egui::text::LayoutJob::default();
        job.append(string, 0.0, TextFormat::simple(font_id.clone(), Color32::LIGHT_GRAY));
        ui.fonts(|f| f.layout_job(job))
    };

    let mut edit = egui::TextEdit::multiline(text)
        .font(TextStyle::Monospace)
        .desired_width(f32::INFINITY)
        .layouter(&mut layouter);

    if !config.placeholder.is_empty() {
        edit = edit.hint_text(&config.placeholder);
    }
    if !config.editable {
        edit = edit.interactive(false);
    }

    ui.add(edit);
}
