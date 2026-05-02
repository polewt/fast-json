//! 示例：使用 fast-json 库进行命令行 JSON 格式化。
//!
//! ```bash
//! cargo run --example simple_formatter -- '{"hello":"world"}'
//! ```

use fast_json::core::json;
use fast_json::core::json::types::FormatOptions;

fn main() {
    let json_str = r#"{"name":"Alice","age":30,"skills":["Rust","JSON"]}"#;

    println!("=== Input ===");
    println!("{json_str}\n");

    let value = json::parse(json_str).expect("Invalid JSON");

    println!("=== Pretty ===");
    let opts = FormatOptions::default();
    println!("{}", json::format_pretty(&value, &opts));

    println!("\n=== Compact ===");
    println!("{}", json::format_compact(&value));
}
