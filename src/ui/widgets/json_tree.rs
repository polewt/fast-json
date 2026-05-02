//! JSON 树形视图组件 (可复用)。

use egui::Ui;

/// 树形视图配置。
pub struct JsonTreeConfig {
    pub default_expand_depth: usize,
    pub show_type_icons: bool,
    pub indent_per_level: f32,
}

impl Default for JsonTreeConfig {
    fn default() -> Self {
        JsonTreeConfig {
            default_expand_depth: 3,
            show_type_icons: true,
            indent_per_level: 16.0,
        }
    }
}

/// 渲染 JSON 树形视图。
///
/// TODO: 实际实现 (egui_json_tree 或自行实现折叠树)。
pub fn render(_value: &serde_json::Value, _config: &JsonTreeConfig, _ui: &mut Ui) {
    // stub
}
