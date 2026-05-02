//! 主题 & 样式常量。
//!
//! 所有颜色、字体、间距等视觉参数集中定义于此。

use egui::{Color32, Visuals};

/// 获取当前主题的 Visuals (根据 dark_mode 配置)。
pub fn visuals(dark_mode: bool) -> Visuals {
    if dark_mode {
        dark_visuals()
    } else {
        Visuals::light()
    }
}

fn dark_visuals() -> Visuals {
    let mut v = Visuals::dark();
    v.widgets.noninteractive.bg_fill = Color32::from_rgb(30, 30, 30);
    v.widgets.inactive.bg_fill = Color32::from_rgb(45, 45, 45);
    v.widgets.active.bg_fill = Color32::from_rgb(60, 60, 60);
    v.widgets.hovered.bg_fill = Color32::from_rgb(55, 55, 55);
    v.selection.bg_fill = Color32::from_rgba_premultiplied(60, 100, 200, 100);
    v
}

// -- 语义颜色 --

/// 语法高亮颜色 (JSON).
pub struct SyntaxColors {
    pub key: Color32,
    pub string: Color32,
    pub number: Color32,
    pub boolean: Color32,
    pub null: Color32,
    pub bracket: Color32,
    pub error: Color32,
}

impl SyntaxColors {
    pub fn dark() -> Self {
        SyntaxColors {
            key: Color32::from_rgb(137, 221, 255),
            string: Color32::from_rgb(195, 232, 141),
            number: Color32::from_rgb(247, 140, 108),
            boolean: Color32::from_rgb(255, 203, 107),
            null: Color32::from_rgb(137, 221, 255),
            bracket: Color32::from_rgb(158, 158, 158),
            error: Color32::from_rgb(255, 83, 112),
        }
    }
}

// -- 布局常量 --

pub const FONT_SIZE_MONO: f32 = 14.0;
pub const FONT_SIZE_UI: f32 = 13.0;
pub const PANEL_MIN_WIDTH: f32 = 200.0;
pub const TOOLBAR_HEIGHT: f32 = 36.0;
pub const STATUSBAR_HEIGHT: f32 = 24.0;
