//! 翻译加载器。

use std::collections::HashMap;
use std::sync::OnceLock;

/// 当前语言。
static CURRENT_LANG: OnceLock<String> = OnceLock::new();

/// 翻译映射表。
static TRANSLATIONS: OnceLock<HashMap<String, String>> = OnceLock::new();

/// 获取当前语言。
pub fn current_language() -> &'static str {
    CURRENT_LANG.get().map(|s| s.as_str()).unwrap_or("en")
}

/// 设置当前语言。
pub fn set_language(lang: &str) {
    let _ = CURRENT_LANG.set(lang.to_string());
    let translations = load_translations(lang);
    let _ = TRANSLATIONS.set(translations);
}

/// 获取翻译文本。
pub fn tr(key: &str) -> &str {
    #[cfg(not(feature = "i18n"))]
    {
        return key;
    }

    #[cfg(feature = "i18n")]
    {
        TRANSLATIONS
            .get_or_init(|| load_translations("en"))
            .get(key)
            .map(|s| s.as_str())
            .unwrap_or(key)
    }
}

/// 从文件或嵌入资源加载翻译。
fn load_translations(lang: &str) -> HashMap<String, String> {
    let content = match lang {
        #[cfg(feature = "i18n")]
        "zh-CN" => include_str!("../../assets/locales/zh-CN.toml").to_string(),
        #[cfg(feature = "i18n")]
        "en" => include_str!("../../assets/locales/en.toml").to_string(),
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
