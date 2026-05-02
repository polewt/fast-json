//! 左侧输入面板 - 原始 JSON 文本编辑器。
//!
//! 尺寸由布局层的 SidePanel 锁定。使用双层 ScrollArea
//! (外层水平 + 内层垂直)，长行不换行时自动出现横向滚动条。

use egui::{Color32, FontId, ScrollArea, TextEdit, TextStyle, Ui};
use crate::app::state::AppState;
use crate::app::theme;
use crate::i18n;

/// 输入超过此字节数则跳过自定义 layouter，直接使用 TextEdit 默认渲染。
const MAX_LAYOUTER_BYTES: usize = 500_000;

pub fn render(app: &mut AppState, ui: &mut Ui) {
    let available = ui.available_size();

    ui.label(egui::RichText::new(i18n::tr("panel.input")).strong());
    ui.add_space(2.0);

    let edit_h = (available.y - 22.0).max(40.0);
    // 记录面板宽度，在 ScrollArea 内层 available_width 会被撑到无限大
    let panel_w = available.x;

    // 超大输入跳过自定义 layouter，避免布局整段文本导致卡顿
    let use_custom_layouter = app.input_text.len() <= MAX_LAYOUTER_BYTES;

    // 外层水平滚动 + 内层垂直滚动，长行不挤压布局
    ScrollArea::horizontal()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(edit_h)
                .show(ui, |ui| {
                    ui.set_min_width(panel_w);

                    if use_custom_layouter {
                        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                            let font_id = FontId::monospace(app.config.editor.font_size);
                            let highlight_colors = if app.config.ui.dark_mode {
                                theme::SyntaxColors::dark()
                            } else {
                                theme::SyntaxColors::light()
                            };

                            let mut job = egui::text::LayoutJob::default();
                            // 根据自动换行设置决定是否限制行宽
                            if app.config.editor.word_wrap {
                                job.wrap.max_width = wrap_width;
                            } else {
                                job.wrap.max_width = f32::INFINITY;
                            }

                            if app.config.editor.syntax_highlight {
                                crate::ui::highlight::highlight_json(
                                    &mut job, string, &highlight_colors, font_id, &[],
                                );
                            } else {
                                let color = if app.config.ui.dark_mode {
                                    Color32::LIGHT_GRAY
                                } else {
                                    Color32::DARK_GRAY
                                };
                                job.append(string, 0.0, egui::TextFormat::simple(font_id, color));
                            }

                            ui.fonts(|f| f.layout_job(job))
                        };

                        ui.add(
                            TextEdit::multiline(&mut app.input_text)
                                .font(TextStyle::Monospace)
                                .desired_width(panel_w)
                                .desired_rows(5)
                                .hint_text(i18n::tr("panel.input_hint"))
                                .layouter(&mut layouter),
                        );
                    } else {
                        ui.add(
                            TextEdit::multiline(&mut app.input_text)
                                .font(TextStyle::Monospace)
                                .desired_width(panel_w)
                                .desired_rows(5)
                                .hint_text(i18n::tr("panel.input_hint")),
                        );
                    }
                });
        });
}
