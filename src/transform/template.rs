//! # Templates

use std::cmp::Ordering;
use std::default;
use crate::{Node, Pattern};
use crate::transform::Transform;

#[derive(Clone, Debug)]
pub struct Template<N: Node> {
    pub(crate) pattern: Pattern<N>,
    pub(crate) body: Transform<N>,
    pub(crate) priority: Option<f64>,
    pub(crate) import: Vec<usize>,
    pub(crate) document_order: Option<usize>,
    mode: Option<String>,
}

impl<N: Node> Template<N> {
    pub fn new(pattern: Pattern<N>, body: Transform<N>, priority: Option<f64>, import: Vec<usize>, document_order: Option<usize>, mode: Option<String>) -> Self {
        Template{pattern, body, priority, import, document_order, mode}
    }
}

/// Two templates are equal if they have the same priority and import precedence.
impl<N: Node> PartialEq for Template<N> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority &&
            self.import == other.import
    }
}
impl<N: Node> Eq for Template<N> {}

impl<N: Node> PartialOrd for Template<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<N: Node> Ord for Template<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.map_or_else(
            || other.priority.map_or_else(
                || Ordering::Equal,
                |t| Ordering::Greater
            ),
            |s| other.priority.map_or_else(
                || Ordering::Less,
                |t| if s < t {Ordering::Less} else {Ordering::Greater},
            )
        )
    }
}
