//! Fast JSON - 二进制入口点。
//!
//! 此二进制文件刻意保持精简：所有逻辑都位于 `fast_json` 库中。
//! 该库可被 CLI、测试、基准测试和其他二进制文件重用。

fn main() {
    #[cfg(feature = "gui")]
    fast_json::app::run_gui();

    #[cfg(not(feature = "gui"))]
    fast_json::app::run_cli();
}
