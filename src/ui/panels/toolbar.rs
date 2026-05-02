//! 顶部工具栏。
//!
//! 提供格式化、压缩、复制、清空等操作按钮和缩进选择器。

use egui::Ui;
use crate::app::action::Action;
use crate::app::state::AppState;
use crate::i18n;

pub fn render(app: &mut AppState, ui: &mut Ui) {
    ui.horizontal(|ui| {
        // -- 格式化 --
        if ui.button(i18n::tr("toolbar.format")).clicked() {
            app.dispatch(Action::Format);
        }

        // -- 压缩 --
        if ui.button(i18n::tr("toolbar.compact")).clicked() {
            app.dispatch(Action::Compact);
        }

        // -- 校验 --
        if ui.button(i18n::tr("toolbar.validate")).clicked() {
            app.dispatch(Action::Validate);
        }

        ui.separator();

        // -- 缩进选择 --
        ui.label(format!("{}:", i18n::tr("toolbar.indent")));
        egui::ComboBox::from_id_salt("indent")
            .selected_text(format!("{} {}", app.config.format.indent, i18n::tr("toolbar.spaces")))
            .show_ui(ui, |ui| {
                for &n in &[2, 4, 8] {
                    if ui.selectable_value(&mut app.config.format.indent, n, format!("{n}")).clicked() {
                        let _ = app.config.save();
                        app.dispatch(Action::Format);
                    }
                }
            });

        ui.separator();

        // -- 从剪贴板粘贴 --
        if ui.button(i18n::tr("toolbar.paste")).clicked() {
            app.dispatch(Action::PasteFromClipboard);
        }

        // -- 复制输出 --
        if ui.button(i18n::tr("toolbar.copy")).clicked() {
            app.dispatch(Action::CopyOutput);
        }

        // -- 清空 --
        if ui.button(i18n::tr("toolbar.clear")).clicked() {
            app.dispatch(Action::ClearAll);
        }

        // -- 右侧 --
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 状态指示
            if app.error_message.is_some() {
                ui.colored_label(egui::Color32::RED, i18n::tr("status.invalid"));
            } else if !app.output_text.is_empty() {
                ui.colored_label(egui::Color32::GREEN, i18n::tr("status.valid"));
            }

            // 设置
            if ui.button(i18n::tr("toolbar.settings")).clicked() {
                app.dispatch(Action::ToggleSettings);
            }

            // 打开文件
            if ui.button(i18n::tr("toolbar.open")).clicked() {
                app.dispatch(Action::OpenFile);
            }
        });
    });
}
