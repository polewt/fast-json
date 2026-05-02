# Fast JSON

一个基于 Rust + egui + sonic-rs 的高性能、跨平台 JSON 格式化 GUI 工具。

## 特性

- **极速解析** — 基于 sonic-rs SIMD 加速，比 serde_json 快 2-5 倍
- **轻量体积** — 二进制约 5-8 MB，秒启动，远超 Electron 方案 (~100MB)
- **跨平台** — Windows / Linux / macOS (PC 端)
- **离线运行** — 无需网络，数据完全本地处理
- **语法高亮** — JSON key/string/number/boolean/null 分色显示
- **多模式** — 美化缩进 / 单行压缩 / 树形视图
- **快捷键** — 可自定义键盘快捷键
- **系统托盘** — 关闭后最小化到托盘，全局热键一键呼出
- **剪贴板联动** — 启动自动读取剪贴板，一键粘贴
- **国际化** — 简体中文 / English 双语言

## 平台支持

| 平台 | 状态 |
|------|------|
| Windows (x86_64) | [OK] |
| Linux (x86_64) | [OK] |
| macOS (Apple Silicon / x86_64) | [OK] |
| 移动端 | 不支持 |

## 安装

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/polewt/fast-json.git
cd fast-json

# 构建 (release)
cargo build --release

# 二进制位于
#   target/release/fast-json (Linux/macOS)
#   target/release/fast-json.exe (Windows)
```

### 运行

```bash
cargo run --release
```

## 使用方式

### GUI 桌面应用 (默认)

直接启动即可：

```bash
fast-json
```

1. 在左侧输入框粘贴 JSON 文本
2. 点击工具栏 **Format** 得到美化输出
3. 点击 **Compact** 得到单行压缩
4. 点击 **Copy** 复制结果到剪贴板

### 命令行模式

```bash
cargo run --no-default-features -F cli
```

### 作为库调用

```rust
use fast_json::{parse, format_pretty, FormatOptions};

let value = parse(r#"{"name":"Alice","age":30}"#).unwrap();
let pretty = format_pretty(&value, &FormatOptions::default());
println!("{pretty}");
```

## 快捷键

| 快捷键 | 操作 |
|--------|------|
| `Ctrl+F` | 格式化 |
| `Ctrl+Shift+F` | 压缩 |
| `Ctrl+C` | 复制输出 |
| `Ctrl+L` | 清空全部 |
| `Ctrl+O` | 打开文件 |
| `Ctrl+,` | 打开设置 |

## 项目架构

```
src/
  lib.rs          -- 库入口，导出公共 API
  main.rs         -- 二进制入口
  core/           -- 核心层 (零 GUI 依赖)
    json/         -- JSON 解析/格式化/校验
    config.rs     -- 配置管理
    clipboard.rs  -- 剪贴板抽象
    link.rs       -- URL 检测与打开
  app/            -- 应用层 (状态管理与行为编排)
    action.rs     -- Action 枚举 (统一操作抽象)
    state.rs      -- 全局状态 + dispatch
    shortcut.rs   -- 快捷键系统
    theme.rs      -- 主题与样式
  ui/             -- 视图层 (纯渲染)
    layout.rs     -- 面板布局
    panels/       -- 功能面板
    widgets/      -- 可复用组件
    settings/     -- 设置界面
  i18n/           -- 国际化 (zh-CN / en)
  platform/       -- 平台抽象层
  util/           -- 工具函数
```

## 技术栈

| 层 | 技术 |
|----|------|
| GUI | egui + eframe |
| JSON | sonic-rs (SIMD 加速) |
| 配置 | serde + TOML |
| 剪贴板 | arboard |
| 文件对话框 | rfd |
| 系统托盘 | tray-icon |
| 全局快捷键 | global-hotkey |
| 国际化 | TOML 扁平化 + OnceLock |

## 开发

```bash
# 编译检查
cargo check

# 运行测试
cargo test

# 运行示例
cargo run --example simple_formatter
```

## 许可

MIT License - 详见 [LICENSE](LICENSE)

Copyright (c) 2026 polewt
