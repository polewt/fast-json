//! 设置窗口。

use egui::{Context, Window};
use crate::app::state::AppState;

pub fn render(app: &mut AppState, ctx: &Context) {
    let mut open = app.settings_open;
    let mut show = true;

    Window::new("Settings")
        .open(&mut open)
        .resizable(true)
        .default_width(480.0)
        .default_height(400.0)
        .show(ctx, |ui| {
            // -- General --
            ui.heading("General");
            ui.checkbox(&mut app.config.general.auto_paste_on_start, "Auto-paste on startup");
            ui.checkbox(&mut app.config.general.background_hotkey, "Enable global hotkey");
            ui.checkbox(&mut app.config.general.close_to_tray, "Close to system tray");

            ui.separator();

            // -- Editor --
            ui.heading("Editor");
            ui.add(egui::Slider::new(&mut app.config.editor.font_size, 10.0..=28.0).text("Font Size"));
            ui.checkbox(&mut app.config.editor.word_wrap, "Word Wrap");
            ui.checkbox(&mut app.config.editor.line_numbers, "Line Numbers");
            ui.checkbox(&mut app.config.editor.syntax_highlight, "Syntax Highlight");

            ui.separator();

            // -- Format --
            ui.heading("Format");
            ui.add(egui::Slider::new(&mut app.config.format.indent, 0..=8).text("Indent"));
            ui.checkbox(&mut app.config.format.sort_keys, "Sort Keys");
            ui.checkbox(&mut app.config.format.trailing_comma, "Trailing Commas");

            ui.separator();

            // -- Appearance --
            ui.heading("Appearance");
            let mut dark = app.config.ui.dark_mode;
            if ui.checkbox(&mut dark, "Dark Mode").changed() {
                app.config.ui.dark_mode = dark;
            }

            ui.separator();

            // -- Buttons --
            ui.horizontal(|ui| {
                if ui.button("Save & Close").clicked() {
                    let _ = app.config.save();
                    show = false;
                }
                if ui.button("Reset Defaults").clicked() {
                    app.config = crate::core::config::AppConfig::default();
                    let _ = app.config.save();
                }
            });
        });

    app.settings_open = open && show;
}
