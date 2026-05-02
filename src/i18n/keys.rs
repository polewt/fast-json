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
    pub const SPACES: &str = "toolbar.spaces";
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
    pub const ENCODING: &str = "status.encoding";
    pub const INDENT_INFO: &str = "status.indent_info";
    pub const THEME_DARK: &str = "status.theme_dark";
    pub const THEME_LIGHT: &str = "status.theme_light";
}

// -- 设置 --
pub mod settings {
    pub const TITLE: &str = "settings.title";
    pub const GENERAL: &str = "settings.general";
    pub const EDITOR: &str = "settings.editor";
    pub const FORMAT: &str = "settings.format";
    pub const SHORTCUTS: &str = "settings.shortcuts";
    pub const LANGUAGE: &str = "settings.language";
    pub const SAVE_CLOSE: &str = "settings.save_close";
    pub const RESET: &str = "settings.reset";
    // -- 通用设置标签 --
    pub const AUTO_PASTE: &str = "settings.auto_paste";
    pub const BACKGROUND_HOTKEY: &str = "settings.background_hotkey";
    pub const CLOSE_TO_TRAY: &str = "settings.close_to_tray";
    // -- 编辑器标签 --
    pub const FONT_SIZE: &str = "settings.font_size";
    pub const WORD_WRAP: &str = "settings.word_wrap";
    pub const LINE_NUMBERS: &str = "settings.line_numbers";
    pub const SYNTAX_HIGHLIGHT: &str = "settings.syntax_highlight";
    // -- 格式化标签 --
    pub const INDENT: &str = "settings.indent";
    pub const SORT_KEYS: &str = "settings.sort_keys";
    pub const TRAILING_COMMA: &str = "settings.trailing_comma";
    // -- 外观 --
    pub const APPEARANCE: &str = "settings.appearance";
    pub const DARK_MODE: &str = "settings.dark_mode";
}

// -- 通用 --
pub mod common {
    pub const ABOUT: &str = "common.about";
    pub const QUIT: &str = "common.quit";
    pub const OK: &str = "common.ok";
    pub const CANCEL: &str = "common.cancel";
}
