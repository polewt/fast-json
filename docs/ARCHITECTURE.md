# Fast JSON

> 高性能、跨平台的 JSON 格式化 GUI 工具

## 架构

```
src/
  lib.rs                # 库入口 - 导出所有公共 API
  main.rs               # 二进制入口 - 极薄
  core/                 # 核心层 - 零 GUI 依赖，可独立复用
    json/               # JSON 解析/格式化/校验子系统
    link.rs             # 链接检测 & 打开
    clipboard.rs        # 剪贴板抽象
    config.rs           # 配置管理 (serde + TOML)
  app/                  # 应用层 - 状态管理 & 行为编排
    action.rs           # Action 枚举 (所有用户操作的统一抽象)
    state.rs            # 全局应用状态
    shortcut.rs         # 快捷键定义 & 映射
    theme.rs            # 主题 & 样式常量
  ui/                   # 视图层 - 纯 UI 渲染
    layout.rs           # 面板布局编排
    panels/             # 各功能面板
    widgets/            # 可复用 UI 组件
    settings/           # 设置界面
  i18n/                 # 国际化 (zh-CN / en)
  platform/             # 平台抽象层 (托盘/全局快捷键/文件对话框)
  util/                 # 工具函数 (防抖/流处理)
```

## Feature Flags

| Feature | 说明 |
|---------|------|
| `gui-full` (default) | 完整桌面应用 |
| `gui` | 仅 GUI 基础 |
| `cli` | 命令行界面 |
| `i18n` | 国际化支持 |
| `platform-*` | 各平台特性可选开启 |
