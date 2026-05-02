//! 配置管理。
//!
//! 使用 serde + TOML 读写配置文件，支持默认值合并。

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 通用设置
    #[serde(default)]
    pub general: GeneralConfig,
    /// 编辑器设置
    #[serde(default)]
    pub editor: EditorConfig,
    /// 格式化设置
    #[serde(default)]
    pub format: FormatConfig,
    /// 快捷键设置
    #[serde(default)]
    pub shortcuts: ShortcutConfig,
    /// 界面设置
    #[serde(default)]
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// 界面语言
    #[serde(default = "default_language")]
    pub language: String,
    /// 启动时自动读取剪贴板
    #[serde(default = "default_true")]
    pub auto_paste_on_start: bool,
    /// 后台运行时启用全局快捷键
    #[serde(default = "default_true")]
    pub background_hotkey: bool,
    /// 关闭到系统托盘 (而非退出)
    #[serde(default = "default_true")]
    pub close_to_tray: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    /// 等宽字体族名
    #[serde(default = "default_font_family")]
    pub font_family: String,
    /// Tab 替换为空格数
    #[serde(default)]
    pub tab_spaces: usize,
    /// 自动换行
    #[serde(default = "default_true")]
    pub word_wrap: bool,
    /// 显示行号
    #[serde(default = "default_true")]
    pub line_numbers: bool,
    /// 语法高亮
    #[serde(default = "default_true")]
    pub syntax_highlight: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatConfig {
    /// 默认缩进
    #[serde(default = "default_indent")]
    pub indent: usize,
    /// 默认按 key 排序
    #[serde(default)]
    pub sort_keys: bool,
    /// 尾随逗号
    #[serde(default)]
    pub trailing_comma: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutConfig {
    pub format: String,
    pub compact: String,
    pub copy: String,
    pub clear: String,
    pub open_file: String,
    pub toggle_settings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// 暗色模式
    #[serde(default = "default_true")]
    pub dark_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            general: GeneralConfig::default(),
            editor: EditorConfig::default(),
            format: FormatConfig::default(),
            shortcuts: ShortcutConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        GeneralConfig {
            language: default_language(),
            auto_paste_on_start: true,
            background_hotkey: true,
            close_to_tray: true,
        }
    }
}

impl Default for EditorConfig {
    fn default() -> Self {
        EditorConfig {
            font_size: default_font_size(),
            font_family: default_font_family(),
            tab_spaces: 4,
            word_wrap: true,
            line_numbers: true,
            syntax_highlight: true,
        }
    }
}

impl Default for FormatConfig {
    fn default() -> Self {
        FormatConfig {
            indent: default_indent(),
            sort_keys: false,
            trailing_comma: false,
        }
    }
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        ShortcutConfig {
            format: "Ctrl+F".into(),
            compact: "Ctrl+Shift+F".into(),
            copy: "Ctrl+C".into(),
            clear: "Ctrl+L".into(),
            open_file: "Ctrl+O".into(),
            toggle_settings: "Ctrl+,".into(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        UiConfig { dark_mode: true }
    }
}

// -- 默认值函数 --

fn default_language() -> String { "en".into() }
fn default_true() -> bool { true }
fn default_font_size() -> f32 { 14.0 }
fn default_font_family() -> String { "Consolas, monospace".into() }
fn default_indent() -> usize { 2 }

// -- 配置读写 --

impl AppConfig {
    /// 从默认路径加载配置，文件不存在则创建默认配置。
    pub fn load() -> Self {
        let path = config_path();
        if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_default()
        } else {
            let config = AppConfig::default();
            let _ = config.save();
            config
        }
    }

    /// 保存配置到默认路径。
    pub fn save(&self) -> Result<(), String> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let content = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(&path, content).map_err(|e| e.to_string())
    }

    /// 在程序运行中动态更新并保存。
    pub fn update_and_save<F: FnOnce(&mut Self)>(&mut self, f: F) -> Result<(), String> {
        f(self);
        self.save()
    }
}

/// 配置文件路径: {config_dir}/fast-json/config.toml
fn config_path() -> PathBuf {
    directories::ProjectDirs::from("com", "fast-json", "fast-json")
        .map(|d| d.config_dir().join("config.toml"))
        .unwrap_or_else(|| PathBuf::from("./config.toml"))
}
