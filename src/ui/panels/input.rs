//! 左侧输入面板 - 原始 JSON 文本编辑器。

use egui::{Color32, FontId, ScrollArea, TextEdit, TextFormat, TextStyle, Ui};
use crate::app::state::AppState;
use crate::app::theme;

pub fn render(app: &mut AppState, ui: &mut Ui) {
    ui.heading("Input");
    ui.separator();

    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
                let font_id = FontId::monospace(theme::FONT_SIZE_MONO);
                let highlight = theme::SyntaxColors::dark();

                let mut job = egui::text::LayoutJob::default();

                if app.config.editor.syntax_highlight {
                    // TODO: 完整的 JSON 语法高亮
                    // 当前先做简单的 key/string/value 着色
                    highlight_json_syntax(&mut job, string, &highlight, font_id.clone());
                } else {
                    job.append(string, 0.0, TextFormat::simple(font_id, Color32::LIGHT_GRAY));
                }

                ui.fonts(|f| f.layout_job(job))
            };

            ui.add(
                TextEdit::multiline(&mut app.input_text)
                    .font(TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .desired_rows(20)
                    .hint_text("Paste JSON here...")
                    .layouter(&mut layouter),
            );
        });
}

/// 简易 JSON 语法高亮 (后续可用 syntect 替代)。
fn highlight_json_syntax(
    job: &mut egui::text::LayoutJob,
    text: &str,
    colors: &theme::SyntaxColors,
    font_id: FontId,
) {
    // 简单状态机：在字符串/数字/布尔/null 之间切换颜色
    let mut i = 0;
    let bytes = text.as_bytes();

    while i < bytes.len() {
        let ch = bytes[i] as char;

        if ch == '"' {
            // 字符串
            let start = i;
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'"' && bytes[i - 1] != b'\\' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            // 判断是 key 还是 value：如果后面紧跟 ':'
            let end = i;
            let is_key = bytes[end..]
                .iter()
                .take_while(|b| b.is_ascii_whitespace())
                .copied()
                .chain(std::iter::once(b' '))
                .find(|&b| !b.is_ascii_whitespace())
                == Some(b':');

            let color = if is_key { colors.key } else { colors.string };
            job.append(&text[start..end], 0.0, TextFormat::simple(font_id.clone(), color));
        } else if ch == '{' || ch == '}' || ch == '[' || ch == ']' {
            job.append(&text[i..i + 1], 0.0, TextFormat::simple(font_id.clone(), colors.bracket));
            i += 1;
        } else if ch == 't' && text[i..].starts_with("true") {
            job.append("true", 0.0, TextFormat::simple(font_id.clone(), colors.boolean));
            i += 4;
        } else if ch == 'f' && text[i..].starts_with("false") {
            job.append("false", 0.0, TextFormat::simple(font_id.clone(), colors.boolean));
            i += 5;
        } else if ch == 'n' && text[i..].starts_with("null") {
            job.append("null", 0.0, TextFormat::simple(font_id.clone(), colors.null));
            i += 4;
        } else if ch.is_ascii_digit() || ch == '-' {
            let start = i;
            while i < bytes.len()
                && (bytes[i] as char).is_ascii_digit() || bytes[i] == b'.' || bytes[i] == b'-'
                    || bytes[i] == b'e' || bytes[i] == b'E' || bytes[i] == b'+'
            {
                i += 1;
            }
            job.append(&text[start..i], 0.0, TextFormat::simple(font_id.clone(), colors.number));
        } else {
            job.append(&text[i..i + 1], 0.0, TextFormat::simple(font_id.clone(), Color32::LIGHT_GRAY));
            i += 1;
        }
    }
}
