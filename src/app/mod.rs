//! 应用层 - 状态管理 & 行为编排。
//!
//! 本层依赖 egui，是 core 和 ui 之间的桥梁：
//! - `action` - 所有用户操作的统一抽象
//! - `state`  - 全局可变状态
//! - `shortcut` - 快捷键 -> Action 映射
//! - `theme`  - 颜色、字体等样式定义

pub mod action;
pub mod shortcut;
pub mod state;
pub mod theme;

pub use action::Action;
pub use state::AppState;

/// 启动 GUI 应用。
#[cfg(feature = "gui")]
pub fn run_gui() {
    use egui;
    use state::AppState;

    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 640.0])
            .with_min_inner_size([480.0, 320.0])
            .with_title("Fast JSON"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Fast JSON",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_zoom_factor(1.0);
            Ok(Box::new(AppState::new(cc)))
        }),
    );
}

/// 启动 CLI 模式。
#[cfg(not(feature = "gui"))]
pub fn run_cli() {
    env_logger::init();
    log::info!("Fast JSON CLI mode (not yet implemented)");
    println!("Fast JSON - CLI mode coming soon.");
}
