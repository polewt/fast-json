//! 底部状态栏 - 字符数、行数、编码等信息。

use egui::Ui;
use crate::app::state::AppState;

pub fn render(app: &AppState, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let char_count = app.input_text.chars().count();
        let line_count = app.input_text.lines().count();
        let byte_count = app.input_text.len();

        ui.label(format!(
            "Chars: {char_count}  |  Lines: {line_count}  |  Bytes: {byte_count}"
        ));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let encoding = if app.input_text.is_empty() || app.input_text.is_ascii() {
                "UTF-8"
            } else {
                // 简易检测
                "UTF-8 (non-ASCII)"
            };
            ui.label(format!("Encoding: {encoding}"));

            ui.label(format!(
                "Indent: {} spaces  |  Theme: {}",
                app.config.format.indent,
                if app.config.ui.dark_mode { "Dark" } else { "Light" }
            ));
        });
    });
}
