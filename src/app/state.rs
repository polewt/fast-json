//! 全局应用状态。
//!
//! `AppState` 是 egui App trait 的实现者，
//! 持有所有可变的运行时状态，并负责事件分发。

use egui::Context;
use crate::core::config::AppConfig;
use crate::core::json;
use crate::core::json::types::FormatOptions;
use crate::app::action::Action;
use crate::core::clipboard::ClipboardProvider;

/// 全局应用状态。
pub struct AppState {
    // -- 输入/输出 --
    pub input_text: String,
    pub output_text: String,
    pub error_message: Option<String>,

    // -- 模式 --
    pub view_mode: ViewMode,
    pub settings_open: bool,

    // -- 配置 --
    pub config: AppConfig,

    // -- UI 状态 --
    pub show_line_numbers: bool,
    pub word_wrap: bool,
    pub link_spans: Vec<crate::core::link::UrlSpan>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Formatted,
    Compact,
    Tree,
}

impl AppState {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = AppConfig::load();

        // 注册中文字体到 egui (解决乱码)
        if let Some(cjk_data) = crate::core::font::load_cjk_font() {
            let mut fonts = egui::FontDefinitions::default();
            fonts
                .font_data
                .insert("cjk".to_owned(), egui::FontData::from_owned(cjk_data).into());
            // 将 CJK 字体插入到所有字族的最前面
            for family in fonts.families.values_mut() {
                family.push("cjk".to_owned());
            }
            cc.egui_ctx.set_fonts(fonts);
        }

        // 启动时根据配置设置语言
        #[cfg(feature = "i18n")]
        crate::i18n::set_language(&config.general.language);

        AppState {
            input_text: String::new(),
            output_text: String::new(),
            error_message: None,
            view_mode: ViewMode::Formatted,
            settings_open: false,
            config,
            show_line_numbers: true,
            word_wrap: true,
            link_spans: Vec::new(),
        }
    }

    // -- Action 分发 --

    /// 统一处理所有用户操作。
    pub fn dispatch(&mut self, action: Action) {
        match action {
            Action::Format => {
                self.view_mode = ViewMode::Formatted;
                self.process_json();
            }
            Action::Compact => {
                self.view_mode = ViewMode::Compact;
                self.process_json();
            }
            Action::Validate => {
                self.error_message = match json::validate(&self.input_text) {
                    Ok(()) => Some("[OK] Valid JSON".into()),
                    Err(e) => Some(e.to_string()),
                };
            }
            Action::SortKeys => {
                // TODO: toggle sort_keys in config and re-process
            }
            Action::CopyOutput => {
                self.copy_to_clipboard(&self.output_text.clone());
            }
            Action::CopyInput => {
                self.copy_to_clipboard(&self.input_text.clone());
            }
            Action::PasteFromClipboard => {
                self.paste_from_clipboard();
            }
            Action::ClearInput => { self.input_text.clear(); }
            Action::ClearOutput => { self.output_text.clear(); }
            Action::ClearAll => {
                self.input_text.clear();
                self.output_text.clear();
                self.error_message = None;
            }
            Action::Undo => { /* TODO */ }
            Action::Redo => { /* TODO */ }
            Action::OpenFile => { /* TODO: rfd file dialog */ }
            Action::SaveOutput => { /* TODO */ }
            Action::SaveAs => { /* TODO */ }
            Action::ToggleSettings => {
                self.settings_open = !self.settings_open;
            }
            Action::ToggleDarkMode => {
                self.config.ui.dark_mode = !self.config.ui.dark_mode;
                let _ = self.config.save();
            }
            Action::ToggleTreeView => {
                self.view_mode = match self.view_mode {
                    ViewMode::Tree => ViewMode::Formatted,
                    _ => ViewMode::Tree,
                };
            }
            Action::ToggleWordWrap => {
                self.word_wrap = !self.word_wrap;
            }
            Action::ToggleLineNumbers => {
                self.show_line_numbers = !self.show_line_numbers;
            }
            Action::ZoomIn => { /* TODO */ }
            Action::ZoomOut => { /* TODO */ }
            Action::ResetZoom => { /* TODO */ }
            Action::Quit => { /* handled by eframe */ }
            Action::MinimizeToTray => { /* TODO: platform tray */ }
            Action::ShowFromTray => { /* TODO: platform tray */ }
            Action::SwitchLanguage { lang } => {
                self.config.general.language = lang.clone();
                #[cfg(feature = "i18n")]
                crate::i18n::set_language(&lang);
                let _ = self.config.save();
            }
            Action::OpenLink { url } => {
                crate::core::link::open_url(&url);
            }
        }
    }

    // -- 内部方法 --

    fn process_json(&mut self) {
        match json::parse(&self.input_text) {
            Ok(value) => {
                self.error_message = None;
                let opts = FormatOptions {
                    indent: if matches!(self.view_mode, ViewMode::Compact) { 0 } else { self.config.format.indent },
                    sort_keys: self.config.format.sort_keys,
                    ..Default::default()
                };
                self.output_text = if matches!(self.view_mode, ViewMode::Compact) {
                    json::format_compact(&value)
                } else {
                    json::format_pretty(&value, &opts)
                };
                // 提取输出中的链接
                self.link_spans = crate::core::link::extract_urls(&self.output_text);
            }
            Err(e) => {
                self.error_message = Some(e.to_string());
                self.output_text.clear();
                self.link_spans.clear();
            }
        }
    }

    fn copy_to_clipboard(&self, text: &str) {
        #[cfg(feature = "platform-clipboard")]
        crate::core::clipboard::default_provider().write(text);
    }

    fn paste_from_clipboard(&mut self) {
        #[cfg(feature = "platform-clipboard")]
        if let Some(text) = crate::core::clipboard::default_provider().read() {
            self.input_text = text;
            self.process_json();
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // -- 处理键盘快捷键 --
        let action = crate::app::shortcut::detect(ctx, &self.config);
        if let Some(action) = action {
            self.dispatch(action);
        }

        // -- 渲染 UI --
        crate::ui::layout::render(self, ctx);

        // -- 设置窗口 --
        if self.settings_open {
            crate::ui::settings::window::render(self, ctx);
        }
    }
}
