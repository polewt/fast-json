//! 视图层 - 纯 UI 渲染。
//!
//! 本层只负责"画什么"，不包含业务逻辑。
//! 所有状态变更通过返回 `Action` 或调用 `ctx.dispatch(...)` 向应用层传递。
//!
//! 模块组织：
//! - `layout`   - 整体面板编排 (Top/Bottom/Side/Center)
//! - `panels/*` - 各功能面板的渲染
//! - `widgets/*`- 可复用的 UI 组件
//! - `settings/*`- 设置界面

pub mod highlight;
pub mod layout;
pub mod panels;
pub mod widgets;

#[cfg(feature = "gui")]
pub mod settings;
