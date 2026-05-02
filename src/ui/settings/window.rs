//! 设置窗口。
//!
//! 以 egui Window 形式弹出，支持运行时语言切换。

use egui::{Context, Window};
use crate::app::action::Action;
use crate::app::state::AppState;
use crate::i18n;

pub fn render(app: &mut AppState, ctx: &Context) {
    let mut open = app.settings_open;
    let mut show = true;

    let screen_center = ctx.screen_rect().center();
    Window::new(i18n::tr("settings.title"))
        .open(&mut open)
        .resizable(true)
        .default_width(480.0)
        .default_height(450.0)
        .default_pos([screen_center.x - 240.0, screen_center.y - 225.0])
        .show(ctx, |ui| {
            // -- 通用 --
            ui.heading(i18n::tr("settings.general"));

            // 语言选择
            ui.horizontal(|ui| {
                ui.label(format!("{}:", i18n::tr("settings.language")));
                let current_lang = i18n::current_language();
                let mut selected = 0usize;
                if current_lang == "zh-CN" { selected = 1; }
                if current_lang == "ru" { selected = 2; }

                let selected_text = match selected {
                    1 => "简体中文",
                    2 => "Русский",
                    _ => "English",
                };

                egui::ComboBox::from_id_salt("language_selector")
                    .selected_text(selected_text)
                    .show_ui(ui, |ui| {
                        if ui.selectable_value(&mut selected, 0, "English").clicked() {
                            app.dispatch(Action::SwitchLanguage { lang: "en".into() });
                        }
                        if ui.selectable_value(&mut selected, 1, "简体中文").clicked() {
                            app.dispatch(Action::SwitchLanguage { lang: "zh-CN".into() });
                        }
                        if ui.selectable_value(&mut selected, 2, "Русский").clicked() {
                            app.dispatch(Action::SwitchLanguage { lang: "ru".into() });
                        }
                    });
            });

            ui.checkbox(&mut app.config.general.auto_paste_on_start, i18n::tr("settings.auto_paste"));
            ui.checkbox(&mut app.config.general.background_hotkey, i18n::tr("settings.background_hotkey"));
            ui.checkbox(&mut app.config.general.close_to_tray, i18n::tr("settings.close_to_tray"));

            ui.separator();

            // -- 编辑器 --
            ui.heading(i18n::tr("settings.editor"));
            ui.add(egui::Slider::new(&mut app.config.editor.font_size, 10.0..=28.0).text(i18n::tr("settings.font_size")));
            ui.checkbox(&mut app.config.editor.word_wrap, i18n::tr("settings.word_wrap"));
            ui.checkbox(&mut app.config.editor.line_numbers, i18n::tr("settings.line_numbers"));
            ui.checkbox(&mut app.config.editor.syntax_highlight, i18n::tr("settings.syntax_highlight"));

            ui.separator();

            // -- 格式化 --
            ui.heading(i18n::tr("settings.format"));
            ui.add(egui::Slider::new(&mut app.config.format.indent, 0..=8).text(i18n::tr("settings.indent")));
            ui.add(egui::Slider::new(&mut app.config.format.tree_expand_depth, 0..=10).text(i18n::tr("settings.tree_expand_depth")));
            ui.checkbox(&mut app.config.format.sort_keys, i18n::tr("settings.sort_keys"));
            ui.checkbox(&mut app.config.format.trailing_comma, i18n::tr("settings.trailing_comma"));

            ui.separator();

            // -- 外观 --
            ui.heading(i18n::tr("settings.appearance"));
            let mut dark = app.config.ui.dark_mode;
            if ui.checkbox(&mut dark, i18n::tr("settings.dark_mode")).changed() {
                app.config.ui.dark_mode = dark;
            }

            ui.separator();

            // -- 操作按钮 --
            ui.horizontal(|ui| {
                if ui.button(i18n::tr("settings.save_close")).clicked() {
                    let _ = app.config.save();
                    show = false;
                }
                if ui.button(i18n::tr("settings.reset")).clicked() {
                    app.config = crate::core::config::AppConfig::default();
                    let _ = app.config.save();
                }
            });
        });

    app.settings_open = open && show;
}
