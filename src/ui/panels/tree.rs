//! 树形视图面板——JSON 折叠/展开。
//!
//! 使用扁平化节点数组 + 虚拟滚动 (show_rows)，
//! 仅渲染可见行。通过 [+] / [-] 按钮切换容器节点的折叠状态。
//!
//! 渲染缓存：节点的高亮 LayoutJob 构建一次后存入内存，
//! 后续帧直接复用，避免重复解析着色。

use egui::{Color32, FontId, ScrollArea, TextFormat, Ui};
use crate::app::action::Action;
use crate::app::state::AppState;
use crate::app::theme::SyntaxColors;
use crate::core::json::tree;
use crate::core::json::types::{FlatTreeNode, JsonNodeKind};
use crate::i18n;

/// 每级缩进像素宽度
const INDENT_WIDTH: f32 = 20.0;

pub fn render(app: &mut AppState, ui: &mut Ui) {
    if app.tree_nodes.is_empty() {
        ui.label(egui::RichText::new(i18n::tr("panel.output_hint")).weak());
        return;
    }

    let font_id = FontId::monospace(app.config.editor.font_size);
    let colors = if app.config.ui.dark_mode {
        SyntaxColors::dark()
    } else {
        SyntaxColors::light()
    };

    // 计算可见节点 (仅在折叠状态变化时重建)
    if app.visible_indices_dirty {
        app.cached_visible_indices = tree::visible_indices(&app.tree_nodes);
        app.visible_indices_dirty = false;
    }
    let visible = &app.cached_visible_indices;
    let num_rows = visible.len();
    if num_rows == 0 {
        return;
    }

    // 确保缓存长度与节点数一致
    if app.tree_node_cache.len() != app.tree_nodes.len() {
        app.tree_node_cache.resize(app.tree_nodes.len(), None);
    }

    // 按需为可见节点预构建高亮缓存
    for &row_idx in visible.iter() {
        if app.tree_node_cache[row_idx].is_none() {
            let node = &app.tree_nodes[row_idx];
            app.tree_node_cache[row_idx] = Some(build_node_job(node, &font_id, &colors));
        }
    }

    let row_height = ui.fonts(|f| f.row_height(&font_id));

    // 使用预计算的字符宽度 (树构建时计算一次)
    let max_chars = app.max_tree_width_chars;
    let char_w = ui.fonts(|f| {
        let galley = f.layout("W".into(), font_id.clone(), Color32::WHITE, f32::INFINITY);
        galley.size().x
    });
    let content_w = (max_chars as f32 * char_w + 60.0).max(ui.available_width());

    // 提取字段引用以避免闭包捕获整个 app
    let tree_nodes = &app.tree_nodes;
    let tree_node_cache = &app.tree_node_cache;
    let visible = visible.as_slice();

    // 水平滚动 + 垂直虚拟滚动
    let toggles = ScrollArea::horizontal()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_min_width(content_w);

            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show_rows(ui, row_height, num_rows, |ui, row_range| {
                    let mut toggled = Vec::new();
                    for &row_idx in &visible[row_range] {
                        let node = &tree_nodes[row_idx];
                        ui.horizontal(|ui| {
                            // 缩进
                            ui.add_space(node.depth as f32 * INDENT_WIDTH);

                            // 折叠/展开按钮
                            if node.kind.is_container() {
                                let toggle_text = if node.expanded { "[-]" } else { "[+]" };
                                if ui
                                    .add_sized(
                                        [24.0, row_height],
                                        egui::Button::new(
                                            egui::RichText::new(toggle_text)
                                                .font(FontId::monospace(12.0)),
                                        )
                                        .fill(Color32::TRANSPARENT),
                                    )
                                    .clicked()
                                {
                                    toggled.push(row_idx);
                                }
                            } else {
                                ui.add_space(24.0);
                            }

                            // 从缓存取预构建的高亮文本
                            if let Some(job) = &tree_node_cache[row_idx] {
                                ui.add(egui::Label::new(job.clone()).selectable(true));
                            }
                        });
                    }
                    toggled
                })
                .inner
        })
        .inner;

    // 批量分发折叠变更
    for idx in toggles {
        app.dispatch(Action::ToggleTreeNode { index: idx });
    }
}

/// 为单个树节点构建高亮 LayoutJob (仅执行一次，结果缓存复用)。
fn build_node_job(node: &FlatTreeNode, font_id: &FontId, colors: &SyntaxColors) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    job.wrap.max_width = f32::INFINITY;

    // 键名着色
    if let Some(ref key) = node.key {
        job.append(
            &format!("\"{key}\": "),
            0.0,
            TextFormat::simple(font_id.clone(), colors.key),
        );
    }

    // 值按类型着色
    let value_color = match node.kind {
        JsonNodeKind::String => colors.string,
        JsonNodeKind::Number => colors.number,
        JsonNodeKind::Boolean => colors.boolean,
        JsonNodeKind::Null => colors.null,
        JsonNodeKind::Object => colors.bracket,
        JsonNodeKind::Array => colors.bracket,
    };
    job.append(&node.value, 0.0, TextFormat::simple(font_id.clone(), value_color));

    // 展开容器附加括号提示
    match node.kind {
        JsonNodeKind::Object if node.expanded => {
            job.append(" {", 0.0, TextFormat::simple(font_id.clone(), colors.bracket));
        }
        JsonNodeKind::Array if node.expanded => {
            job.append(" [", 0.0, TextFormat::simple(font_id.clone(), colors.bracket));
        }
        _ => {}
    }

    job
}
