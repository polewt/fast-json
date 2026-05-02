//! 整体布局编排。
//!
//! 使用 egui 内建面板系统 (SidePanel + CentralPanel::show_inside)
//! 进行空间划分。SidePanel 在 show_inside 内部手动控制父级光标位置，
//! 不受子内容 min_rect 影响，从根本上解决了 TextEdit 大文件溢出
//! 推挤输出面板的问题。
//!
//! 排布:
//! - Toolbar    (顶部)
//! - [ Input (33%) | Output (剩余) ]  (中央)
//! - Status     (底部)

use egui::{CentralPanel, Context, SidePanel, TopBottomPanel};
use crate::app::state::AppState;
use crate::app::theme;

/// 渲染完整的应用布局。
pub fn render(app: &mut AppState, ctx: &Context) {
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

    // -- 中央分栏 --
    CentralPanel::default().show(ctx, |ui| {
        let input_w = (ui.available_width() * 0.33).max(theme::PANEL_MIN_WIDTH);

        // 左侧输入面板，固定宽度，不受内容溢出影响
        SidePanel::left("input_panel")
            .exact_width(input_w)
            .resizable(false)
            .show_inside(ui, |ui| {
                crate::ui::panels::input::render(app, ui);
            });

        // 右侧输出面板，填充剩余空间
        CentralPanel::default().show_inside(ui, |ui| {
            crate::ui::panels::output::render(app, ui);
        });
    });

}
