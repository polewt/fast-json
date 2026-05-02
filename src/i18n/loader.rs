//! 翻译加载器。
//!
//! 使用 RwLock 存储翻译数据，支持运行时语言切换。

use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::RwLock;

/// 当前语言。
static CURRENT_LANG: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new(String::new()));

/// 翻译映射表。
static TRANSLATIONS: LazyLock<RwLock<HashMap<String, String>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// 获取当前语言。
pub fn current_language() -> String {
    CURRENT_LANG
        .read()
        .ok()
        .filter(|s| !s.is_empty())
        .map(|s| s.clone())
        .unwrap_or_else(|| "en".into())
}

/// 设置当前语言并重新加载翻译。
pub fn set_language(lang: &str) {
    if let Ok(mut cur) = CURRENT_LANG.write() {
        *cur = lang.to_string();
    }
    let translations = load_translations(lang);
    if let Ok(mut t) = TRANSLATIONS.write() {
        *t = translations;
    }
}

/// 获取翻译文本，返回 String 以避开 RwLock 生命周期问题。
pub fn tr(key: &str) -> String {
    #[cfg(not(feature = "i18n"))]
    {
        return key.to_string();
    }

    #[cfg(feature = "i18n")]
    {
        // 如果尚未初始化，先加载默认语言
        let needs_init = TRANSLATIONS
            .read()
            .ok()
            .map(|t| t.is_empty())
            .unwrap_or(true);
        if needs_init {
            if let Ok(mut t) = TRANSLATIONS.write() {
                *t = load_translations("en");
            }
        }

        TRANSLATIONS
            .read()
            .ok()
            .and_then(|t| t.get(key).cloned())
            .unwrap_or_else(|| key.to_string())
    }
}

/// 从文件或嵌入资源加载翻译。
fn load_translations(lang: &str) -> HashMap<String, String> {
    let content = match lang {
        #[cfg(feature = "i18n")]
        "zh-CN" => include_str!("../../assets/locales/zh-CN.toml").to_string(),
        #[cfg(feature = "i18n")]
        "en" => include_str!("../../assets/locales/en.toml").to_string(),
        #[cfg(feature = "i18n")]
        "ru" => include_str!("../../assets/locales/ru.toml").to_string(),
        _ => {
            let path = format!("assets/locales/{lang}.toml");
            std::fs::read_to_string(&path).unwrap_or_default()
        }
    };
    parse_flattened_toml(&content)
}

/// 将嵌套的 TOML 扁平化为 `section.key` -> `value` 映射。
fn parse_flattened_toml(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let value: Result<toml::Value, _> = toml::from_str(content);
    if let Ok(toml::Value::Table(table)) = value {
        flatten_table("", &table, &mut map);
    }
    map
}

fn flatten_table(prefix: &str, table: &toml::Table, output: &mut HashMap<String, String>) {
    for (key, value) in table {
        let full_key = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{prefix}.{key}")
        };

        match value {
            toml::Value::Table(inner) => flatten_table(&full_key, inner, output),
            toml::Value::String(s) => {
                output.insert(full_key, s.clone());
            }
            _ => {}
        }
    }
}
