//! Action - 所有用户操作的统一抽象。
//!
//! ## 设计原则
//!
//! 所有用户交互 (鼠标点击、键盘快捷键、全局热键、剪贴板事件)
//! 都映射为 `Action` 枚举，通过 `AppState::dispatch(action)` 统一处理。
//!
//! 这样做的好处：
//! - 快捷键、菜单、按钮点击天然一致
//! - 易于实现 undo/redo
//! - 易于编写自动化测试
//! - 可序列化，可实现宏录制

/// 用户操作枚举。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    // -- JSON 处理 --
    Format,
    Compact,
    Validate,
    SortKeys,

    // -- 编辑 --
    CopyOutput,
    CopyInput,
    PasteFromClipboard,
    ClearInput,
    ClearOutput,
    ClearAll,
    Undo,
    Redo,

    // -- 文件 --
    OpenFile,
    SaveOutput,
    SaveAs,

    // -- 视图 --
    ToggleSettings,
    ToggleDarkMode,
    ToggleTreeView,
    ToggleWordWrap,
    ToggleLineNumbers,
    ZoomIn,
    ZoomOut,
    ResetZoom,

    // -- 窗口 --
    Quit,
    MinimizeToTray,
    ShowFromTray,

    // -- 国际化 --
    SwitchLanguage { lang: String },

    // -- 链接 --
    OpenLink { url: String },
}
