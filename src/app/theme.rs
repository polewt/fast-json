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
    pub colon: Color32,
    pub comma: Color32,
    pub plain: Color32,
    pub url: Color32,
    pub error: Color32,
}

impl SyntaxColors {
    pub fn dark() -> Self {
        SyntaxColors {
            key: Color32::from_rgb(137, 221, 255),   // 浅蓝
            string: Color32::from_rgb(195, 232, 141), // 浅绿
            number: Color32::from_rgb(247, 140, 108), // 橙色
            boolean: Color32::from_rgb(255, 203, 107), // 琥珀
            null: Color32::from_rgb(137, 221, 255),   // 浅蓝 (同 key)
            bracket: Color32::from_rgb(212, 212, 212), // 浅灰
            colon: Color32::from_rgb(128, 128, 128),   // 中灰
            comma: Color32::from_rgb(128, 128, 128),   // 中灰
            plain: Color32::from_rgb(200, 200, 200),   // 普通文本
            url: Color32::from_rgb(86, 156, 214),      // 链接蓝
            error: Color32::from_rgb(255, 83, 112),    // 红色
        }
    }

    pub fn light() -> Self {
        SyntaxColors {
            key: Color32::from_rgb(4, 81, 140),        // 深蓝
            string: Color32::from_rgb(10, 101, 35),     // 深绿
            number: Color32::from_rgb(175, 56, 0),      // 深橙
            boolean: Color32::from_rgb(190, 130, 0),    // 深琥珀
            null: Color32::from_rgb(4, 81, 140),        // 深蓝 (同 key)
            bracket: Color32::from_rgb(60, 60, 60),     // 深灰
            colon: Color32::from_rgb(128, 128, 128),    // 中灰
            comma: Color32::from_rgb(128, 128, 128),    // 中灰
            plain: Color32::from_rgb(45, 45, 45),       // 普通文本
            url: Color32::from_rgb(0, 80, 160),         // 链接蓝
            error: Color32::from_rgb(200, 30, 50),      // 红色
        }
    }
}

// -- 布局常量 --

pub const PANEL_MIN_WIDTH: f32 = 200.0;
pub const TOOLBAR_HEIGHT: f32 = 36.0;
pub const STATUSBAR_HEIGHT: f32 = 24.0;
