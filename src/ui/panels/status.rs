//! 底部状态栏 - 字符数、行数、编码等信息。
//!
//! 同时展示短暂的状态反馈消息（如"已复制到剪贴板"），显示一帧后自动消失。

use egui::Ui;
use crate::app::state::AppState;
use crate::i18n;

pub fn render(app: &mut AppState, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let char_count = app.input_text.chars().count();
        let line_count = app.input_text.lines().count();
        let byte_count = app.input_text.len();

        ui.label(format!(
            "{}: {char_count}  |  {}: {line_count}  |  {}: {byte_count}",
            i18n::tr("status.chars"),
            i18n::tr("status.lines"),
            i18n::tr("status.bytes"),
        ));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 状态反馈消息 (持续约 1.5 秒后自动消失)
            if let Some(msg) = &app.status_message {
                ui.colored_label(
                    egui::Color32::GREEN,
                    egui::RichText::new(msg.clone()).strong(),
                );
            }

            let encoding_label = if app.input_text.is_empty() || app.input_text.is_ascii() {
                "UTF-8"
            } else {
                "UTF-8 (non-ASCII)"
            };
            ui.label(format!("{}: {encoding_label}", i18n::tr("status.encoding")));

            let theme_name = if app.config.ui.dark_mode {
                i18n::tr("status.theme_dark")
            } else {
                i18n::tr("status.theme_light")
            };

            let indent_text = i18n::tr("status.indent_info")
                .replace("{}", &app.config.format.indent.to_string())
                .replace("{}", &theme_name);

            ui.label(indent_text);
        });
    });
}
