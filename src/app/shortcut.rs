//! 快捷键系统。

use egui::{Context, KeyboardShortcut, Modifiers, Key};
use crate::app::action::Action;
use crate::core::config::AppConfig;

/// 解析快捷键字符串 (如 "Ctrl+F") 为 egui KeyboardShortcut。
pub fn parse_shortcut(s: &str) -> Option<KeyboardShortcut> {
    let parts: Vec<&str> = s.split('+').map(|p| p.trim()).collect();
    let mut modifiers = Modifiers::NONE;
    let mut key = None;

    for part in parts {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => modifiers.ctrl = true,
            "shift" => modifiers.shift = true,
            "alt" => modifiers.alt = true,
            "meta" | "cmd" | "super" => modifiers.mac_cmd = true,
            k => {
                key = Some(match k {
                    "f" => Key::F,
                    "c" => Key::C,
                    "v" => Key::V,
                    "o" => Key::O,
                    "s" => Key::S,
                    "l" => Key::L,
                    "z" => Key::Z,
                    "y" => Key::Y,
                    "," | "comma" => Key::Comma,
                    "enter" => Key::Enter,
                    "escape" | "esc" => Key::Escape,
                    "tab" => Key::Tab,
                    other => {
                        log::warn!("unknown shortcut key: {other}");
                        return None;
                    }
                });
            }
        }
    }

    key.map(|k| KeyboardShortcut::new(modifiers, k))
}

/// 检测当前帧是否有快捷键输入，返回匹配的 Action。
pub fn detect(ctx: &Context, config: &AppConfig) -> Option<Action> {
    let consumed = ctx.input_mut(|i| {
        if let Some(shortcut) = parse_shortcut(&config.shortcuts.format) {
            if i.consume_shortcut(&shortcut) { return Some(Action::Format); }
        }
        if let Some(shortcut) = parse_shortcut(&config.shortcuts.compact) {
            if i.consume_shortcut(&shortcut) { return Some(Action::Compact); }
        }
        if let Some(shortcut) = parse_shortcut(&config.shortcuts.copy) {
            if i.consume_shortcut(&shortcut) { return Some(Action::CopyOutput); }
        }
        if let Some(shortcut) = parse_shortcut(&config.shortcuts.clear) {
            if i.consume_shortcut(&shortcut) { return Some(Action::ClearAll); }
        }
        if let Some(shortcut) = parse_shortcut(&config.shortcuts.open_file) {
            if i.consume_shortcut(&shortcut) { return Some(Action::OpenFile); }
        }
        if let Some(shortcut) = parse_shortcut(&config.shortcuts.toggle_settings) {
            if i.consume_shortcut(&shortcut) { return Some(Action::ToggleSettings); }
        }
        None
    });
    consumed
}
