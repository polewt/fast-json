//! 翻译 key 常量。
//!
//! 所有 key 集中定义，便于 IDE 自动补全和避免拼写错误。
//! 对应 `assets/locales/{lang}.toml` 中的条目。

pub struct I18nKey;

// -- 应用 --
impl I18nKey {
    pub const APP_TITLE: &str = "app.title";
}

// -- 工具栏 --
pub mod toolbar {
    pub const FORMAT: &str = "toolbar.format";
    pub const COMPACT: &str = "toolbar.compact";
    pub const VALIDATE: &str = "toolbar.validate";
    pub const PASTE: &str = "toolbar.paste";
    pub const COPY: &str = "toolbar.copy";
    pub const CLEAR: &str = "toolbar.clear";
    pub const OPEN: &str = "toolbar.open";
    pub const SETTINGS: &str = "toolbar.settings";
    pub const INDENT: &str = "toolbar.indent";
}

// -- 面板 --
pub mod panel {
    pub const INPUT: &str = "panel.input";
    pub const OUTPUT: &str = "panel.output";
    pub const INPUT_HINT: &str = "panel.input_hint";
    pub const OUTPUT_HINT: &str = "panel.output_hint";
}

// -- 状态栏 --
pub mod status {
    pub const CHARS: &str = "status.chars";
    pub const LINES: &str = "status.lines";
    pub const BYTES: &str = "status.bytes";
    pub const VALID: &str = "status.valid";
    pub const INVALID: &str = "status.invalid";
}

// -- 设置 --
pub mod settings {
    pub const TITLE: &str = "settings.title";
    pub const GENERAL: &str = "settings.general";
    pub const EDITOR: &str = "settings.editor";
    pub const FORMAT: &str = "settings.format";
    pub const SHORTCUTS: &str = "settings.shortcuts";
    pub const SAVE_CLOSE: &str = "settings.save_close";
    pub const RESET: &str = "settings.reset";
}

// -- 通用 --
pub mod common {
    pub const ABOUT: &str = "common.about";
    pub const QUIT: &str = "common.quit";
    pub const OK: &str = "common.ok";
    pub const CANCEL: &str = "common.cancel";
}
