//! # Templates

use std::rc::Rc;
use std::cmp::Ordering;

use crate::xdmerror::{Error, ErrorKind};
use crate::{Node, Pattern, Sequence, Item};
use crate::transform::context::{Context, ContextBuilder};
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

/// Apply templates to the select expression.
pub(crate) fn apply_templates<N: Node>(
    ctxt: &Context<N>,
    s: &Transform<N>,
) -> Result<Sequence<N>, Error> {
        // s is the select expression. Evaluate it, and then iterate over it's items.
        // Each iteration becomes an item in the result sequence.
        ctxt.dispatch(s)?.iter().try_fold(vec![], |mut result, i| {
            let templates = match_templates(ctxt, i)?;
            // If there are two or more templates with the same priority and import level, then take the one that has the higher document order
            let matching = if templates.len() > 1 {
                if templates[0].priority == templates[1].priority
                    && templates[0].import.len() == templates[1].import.len()
                {
                    let mut candidates: Vec<Rc<Template<N>>> = templates
                        .iter()
                        .take_while(|t| {
                            t.priority == templates[0].priority
                                && t.import.len() == templates[0].import.len()
                        })
                        .cloned()
                        .collect();
                    candidates.sort_unstable_by(|a, b| {
                        a.document_order.map_or(Ordering::Greater, |v| {
                            b.document_order.map_or(Ordering::Less, |u| v.cmp(&u))
                        })
                    });
                    candidates.last().unwrap().clone()
                } else {
                    templates[0].clone()
                }
            } else {
                templates[0].clone()
            };
            // Create a new context using the current templates, then evaluate the highest priority and highest import precedence
            let mut u = ContextBuilder::from(ctxt)
                    .current(vec![i.clone()])
                    .current_templates(templates)
                    .build()
                    .dispatch(&matching.body)?;
            result.append(&mut u);
            Ok(result)
        })
}

/// Apply template with a higher import precedence.
pub(crate) fn apply_imports<N: Node>(
    ctxt: &Context<N>,
) -> Result<Sequence<N>, Error> {
        // Find the template with the next highest level within the same import tree
        // current_templates[0] is the currently matching template
        let cur = &(ctxt.current_templates[0]);
        let next: Vec<Rc<Template<N>>> = ctxt
            .current_templates
            .iter()
            .skip(1)
            .skip_while(|t| t.import.len() == cur.import.len()) // import level is the same (iow, different priority templates in the same import level)
            .cloned()
            .collect();

        if !next.is_empty() {
            ContextBuilder::from(ctxt)
                    .current_templates(next.clone())
                    .build()
                    .dispatch(&next[0].body)
        } else {
            Ok(vec![])
        }
}

/// Apply the next template that matches.
pub(crate) fn next_match<N: Node>(
    ctxt: &Context<N>,
) -> Result<Sequence<N>, Error> {
        if ctxt.current_templates.len() > 2 {
            ContextBuilder::from(ctxt)
                    .current_templates(ctxt.current_templates.iter().skip(1).cloned().collect())
                    .build()
                    .dispatch(&ctxt.current_templates[1].body)
        } else {
            Ok(vec![])
        }
}

// Find all potential templates. Evaluate the match pattern against this item.
// Sort the result by priority and import precedence.
fn match_templates<N: Node>(
    ctxt: &Context<N>,
    i: &Rc<Item<N>>,
) -> Result<Vec<Rc<Template<N>>>, Error> {
    let mut candidates = ctxt.templates.iter().try_fold(vec![], |mut cand, t| {
        let e = t.pattern.matches(ctxt, i);
        if e {
            cand.push(t.clone())
        }
        Ok(cand)
    })?;
    if candidates.len() != 0 {
        // Find the template(s) with the lowest priority.

        candidates.sort_unstable_by(|a, b| (*a).cmp(&*b));
        Ok(candidates)
    } else {
        Err(Error::new(
            ErrorKind::Unknown,
            String::from("no matching template"),
        ))
    }
}
