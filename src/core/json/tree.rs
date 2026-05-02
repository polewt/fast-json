//! JSON 树构建器。
//!
//! 将 serde_json::Value 转换为扁平化的 `FlatTreeNode` 数组，
//! 每个节点记录深度 (depth) 和子孙总数 (descendant_count)，
//! 供树形视图做虚拟滚动渲染和折叠/展开。

use serde_json::Value;
use crate::core::json::types::{FlatTreeNode, JsonNodeKind};

/// 从 JSON Value 构建扁平化节点树。
///
/// `expand_depth`: 0 = 全部展开，N = 仅展开前 N 层。
pub fn build_flat_tree(value: &Value, expand_depth: usize) -> Vec<FlatTreeNode> {
    let max_depth = if expand_depth == 0 { usize::MAX } else { expand_depth };
    let mut nodes = Vec::new();
    build_recursive(value, None, 0, max_depth, &mut nodes);
    nodes
}

/// 递归构建，返回当前节点的子孙总数 (不含自身)。
fn build_recursive(
    value: &Value,
    key: Option<String>,
    depth: usize,
    max_expand_depth: usize,
    nodes: &mut Vec<FlatTreeNode>,
) -> usize {
    match value {
        Value::Object(map) => {
            let idx = nodes.len();
            nodes.push(FlatTreeNode {
                key,
                value: String::new(),
                kind: JsonNodeKind::Object,
                depth,
                expanded: depth < max_expand_depth,
                descendant_count: 0,
            });

            let mut total = 0usize;
            for (k, v) in map {
                total += 1 + build_recursive(v, Some(k.clone()), depth + 1, max_expand_depth, nodes);
            }
            nodes[idx].descendant_count = total;
            total
        }
        Value::Array(arr) => {
            let idx = nodes.len();
            let summary = format!("[{}]", arr.len());
            nodes.push(FlatTreeNode {
                key,
                value: summary,
                kind: JsonNodeKind::Array,
                depth,
                expanded: depth < max_expand_depth,
                descendant_count: 0,
            });

            let mut total = 0usize;
            for (i, v) in arr.iter().enumerate() {
                total += 1 + build_recursive(v, Some(i.to_string()), depth + 1, max_expand_depth, nodes);
            }
            nodes[idx].descendant_count = total;
            total
        }
        _ => {
            nodes.push(FlatTreeNode {
                key,
                value: value_to_string(value),
                kind: value_to_kind(value),
                depth,
                expanded: false,
                descendant_count: 0,
            });
            0
        }
    }
}

/// 计算可见节点索引列表 (跳过已折叠节点的子孙)。
pub fn visible_indices(nodes: &[FlatTreeNode]) -> Vec<usize> {
    let mut indices = Vec::with_capacity(nodes.len());
    let mut i = 0;
    while i < nodes.len() {
        indices.push(i);
        if nodes[i].expanded {
            i += 1;
        } else {
            i += 1 + nodes[i].descendant_count;
        }
    }
    indices
}

/// 切换指定节点的展开状态。
pub fn toggle_node(nodes: &mut [FlatTreeNode], index: usize) {
    if let Some(node) = nodes.get_mut(index) {
        if node.kind.is_container() {
            node.expanded = !node.expanded;
        }
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{s}\""),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".into(),
        _ => String::new(),
    }
}

fn value_to_kind(value: &Value) -> JsonNodeKind {
    match value {
        Value::String(_) => JsonNodeKind::String,
        Value::Number(_) => JsonNodeKind::Number,
        Value::Bool(_) => JsonNodeKind::Boolean,
        Value::Null => JsonNodeKind::Null,
        Value::Array(_) => JsonNodeKind::Array,
        Value::Object(_) => JsonNodeKind::Object,
    }
}
