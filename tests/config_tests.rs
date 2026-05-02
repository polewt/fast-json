//! 配置管理测试。

use fast_json::core::config::AppConfig;

#[test]
fn default_config_has_expected_values() {
    let config = AppConfig::default();
    assert_eq!(config.general.language, "en");
    assert_eq!(config.format.indent, 2);
    assert!(config.ui.dark_mode);
    assert_eq!(config.shortcuts.format, "Ctrl+F");
}

#[test]
fn roundtrip_toml() {
    let config = AppConfig::default();
    let toml_str = toml::to_string_pretty(&config).unwrap();
    let parsed: AppConfig = toml::from_str(&toml_str).unwrap();
    assert_eq!(parsed.format.indent, 2);
    assert_eq!(parsed.shortcuts.format, "Ctrl+F");
}
