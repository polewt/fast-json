//! 中央输出面板 - 格式化后的 JSON 展示。

use egui::{Color32, FontId, ScrollArea, TextEdit, TextFormat, TextStyle, Ui};
use crate::app::state::AppState;
use crate::app::theme;

pub fn render(app: &AppState, ui: &mut Ui) {
    ui.heading("Output");
    ui.separator();

    // 错误信息
    if let Some(ref err) = app.error_message {
        ui.colored_label(Color32::RED, err);
        ui.separator();
    }

    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
                let font_id = FontId::monospace(theme::FONT_SIZE_MONO);
                let mut job = egui::text::LayoutJob::default();

                // TODO: 复用 input 的高亮逻辑 + 为链接添加下划线/可点击样式
                job.append(string, 0.0, TextFormat::simple(font_id, Color32::LIGHT_GRAY));

                ui.fonts(|f| f.layout_job(job))
            };

            let mut output_clone = app.output_text.clone();

            ui.add(
                TextEdit::multiline(&mut output_clone)
                    .font(TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .desired_rows(20)
                    .hint_text("Formatted JSON will appear here...")
                    .interactive(false)
                    .layouter(&mut layouter),
            );
        });
}
