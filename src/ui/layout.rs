//! 整体布局编排。
//!
//! 面板排布 (从上到下):
//! - Toolbar  (顶部)
//! - Input    (左侧)
//! - Output   (中央)
//! - Status   (底部)

use egui::{CentralPanel, Context, SidePanel, TopBottomPanel};
use crate::app::state::AppState;
use crate::app::theme;

/// 渲染完整的应用布局。
pub fn render(app: &mut AppState, ctx: &Context) {
    // 应用主题
    ctx.set_visuals(theme::visuals(app.config.ui.dark_mode));

    // -- 顶部工具栏 --
    TopBottomPanel::top("toolbar")
        .min_height(theme::TOOLBAR_HEIGHT)
        .show(ctx, |ui| {
            crate::ui::panels::toolbar::render(app, ui);
        });

    // -- 底部状态栏 --
    TopBottomPanel::bottom("status_bar")
        .min_height(theme::STATUSBAR_HEIGHT)
        .show(ctx, |ui| {
            crate::ui::panels::status::render(app, ui);
        });

    // -- 左侧：输入面板 --
    SidePanel::left("input_panel")
        .resizable(true)
        .default_width(400.0)
        .min_width(theme::PANEL_MIN_WIDTH)
        .show(ctx, |ui| {
            crate::ui::panels::input::render(app, ui);
        });

    // -- 中央：输出面板 / 树形视图 --
    CentralPanel::default().show(ctx, |ui| {
        match app.view_mode {
            crate::app::state::ViewMode::Tree => {
                // TODO: 交互式 JSON 树形视图
                crate::ui::panels::output::render(app, ui);
            }
            _ => {
                crate::ui::panels::output::render(app, ui);
            }
        }
    });
}
