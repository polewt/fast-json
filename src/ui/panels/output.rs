//! 中央输出面板 - 格式化后的 JSON 展示。
//!
//! 采用行级虚拟渲染 (ScrollArea::show_rows)，
//! 仅绘制可见行，大文件下性能不受文本总量影响。
//! 每行使用 Label::selectable(true) 实现可选中 + 可复制。

use egui::{Color32, FontId, Key, Label, ScrollArea, Ui};
use crate::app::action::Action;
use crate::app::state::{AppState, ViewMode};
use crate::app::theme;
use crate::i18n;

pub fn render(app: &mut AppState, ui: &mut Ui) {
    // Ctrl+A 全选复制
    let panel_rect = ui.max_rect();
    let pointer_in_panel = ui.input(|i| i.pointer.hover_pos())
        .map_or(false, |pos| panel_rect.contains(pos));
    if pointer_in_panel && ui.input(|i| i.key_pressed(Key::A) && i.modifiers.ctrl) {
        app.dispatch(Action::CopyOutput);
    }
    // 双击输出区域自动复制全部
    if pointer_in_panel && ui.input(|i| i.pointer.button_double_clicked(egui::PointerButton::Primary)) {
        app.dispatch(Action::CopyOutput);
    }

    // 树形视图模式
    if app.view_mode == ViewMode::Tree {
        crate::ui::panels::tree::render(app, ui);
        return;
    }

    // 标题行 + 内联复制按钮
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(i18n::tr("panel.output")).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(i18n::tr("toolbar.copy")).clicked() {
                app.dispatch(Action::CopyOutput);
            }
        });
    });

    // 错误信息
    if let Some(ref err) = app.error_message {
        ui.colored_label(Color32::RED, err);
    }

    // 链接栏
    if !app.link_spans.is_empty() {
        let mut link_to_open: Option<String> = None;
        egui::Frame::NONE
            .fill(if app.config.ui.dark_mode {
                Color32::from_rgb(35, 35, 40)
            } else {
                Color32::from_rgb(240, 240, 245)
            })
            .corner_radius(4)
            .inner_margin(egui::Margin::symmetric(8, 2))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new("Links:").size(11.0).weak());
                    for span in &app.link_spans {
                        if ui.link(&span.url).clicked() {
                            link_to_open = Some(span.url.clone());
                        }
                    }
                });
            });
        if let Some(url) = link_to_open {
            app.dispatch(Action::OpenLink { url });
        }
    }

    ui.add_space(2.0);

    // -- 虚拟化文本视图 --
    if app.output_text.is_empty() {
        ui.label(egui::RichText::new(i18n::tr("panel.output_hint")).weak());
        return;
    }

    let font_id = FontId::monospace(app.config.editor.font_size);
    let colors = if app.config.ui.dark_mode {
        theme::SyntaxColors::dark()
    } else {
        theme::SyntaxColors::light()
    };

    // 拆分为行
    let lines: Vec<&str> = app.output_text.lines().collect();
    let num_rows = lines.len();
    if num_rows == 0 {
        return;
    }

    // 行高
    let row_height = ui.fonts(|f| f.row_height(&font_id));

    // 计算最长行的像素宽度 (用近似值避免逐行排版)
    let max_line_chars = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let char_w = ui.fonts(|f| {
        let galley = f.layout(
            "W".into(), font_id.clone(), Color32::WHITE, f32::INFINITY,
        );
        galley.size().x
    });
    let content_w = (max_line_chars as f32 * char_w + 40.0).max(ui.available_width());

    let link_spans = app.link_spans.clone();

    // 外层水平滚动 + 内层虚拟垂直滚动
    ScrollArea::horizontal()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_min_width(content_w);

            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show_rows(ui, row_height, num_rows, |ui, row_range| {
                    for i in row_range {
                        let line = lines[i];
                        let line_offset = line_start_offset(&app.output_text, i);

                        let mut job = egui::text::LayoutJob::default();
                        // 禁用自动换行
                        job.wrap.max_width = f32::INFINITY;

                        // 语法高亮 (每行独立，开销 O(line_len))
                        crate::ui::highlight::highlight_json(
                            &mut job, line, &colors, font_id.clone(),
                            &relevant_spans(&link_spans, line_offset, line_offset + line.len()),
                        );

                        ui.add(Label::new(job).selectable(true));
                    }
                });
        });
}

/// 计算第 `line_idx` 行在完整文本中的字节偏移。
fn line_start_offset(text: &str, line_idx: usize) -> usize {
    text.lines()
        .take(line_idx)
        .map(|l| l.len() + 1) // +1 为换行符
        .sum()
}

/// 筛选出与指定区间相交的 URL span。
fn relevant_spans(spans: &[crate::core::link::UrlSpan], start: usize, end: usize) -> Vec<crate::core::link::UrlSpan> {
    spans
        .iter()
        .filter(|s| s.start < end && s.end > start)
        .cloned()
        .collect()
}
