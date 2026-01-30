//! # Templates

use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use url::Url;

use crate::transform::context::{Context, ContextBuilder, StaticContext};
use crate::transform::{Order, Transform, do_sort};
use crate::xdmerror::Error;
use crate::{Node, Pattern, Sequence};
use qualname::QName;

#[derive(Clone)]
pub struct Template<N: Node> {
    pub(crate) pattern: Pattern<N>,
    pub(crate) body: Transform<N>,
    pub(crate) priority: Option<f64>,
    pub(crate) import: Vec<usize>,
    pub(crate) document_order: Option<usize>,
    pub(crate) mode: Option<QName>,
    pub(crate) mtch: String, // used for debugging
}

impl<N: Node> Template<N> {
    pub fn new(
        pattern: Pattern<N>,
        body: Transform<N>,
        priority: Option<f64>,
        import: Vec<usize>,
        document_order: Option<usize>,
        mode: Option<QName>,
        mtch: String,
    ) -> Self {
        Template {
            pattern,
            body,
            priority,
            import,
            document_order,
            mode,
            mtch,
        }
    }
}

/// Two templates are equal if they have the same priority, import precedence, and mode.
impl<N: Node> PartialEq for Template<N> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.import == other.import && self.mode == other.mode
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
            || {
                other
                    .priority
                    .map_or_else(|| Ordering::Equal, |_| Ordering::Greater)
            },
            |s| {
                other.priority.map_or_else(
                    || Ordering::Less,
                    |t| {
                        if s < t {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    },
                )
            },
        )
    }
}

impl<N: Node> Debug for Template<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "template match \"{:?}\" ({:?}) priority {:?} mode {:?}",
            self.mtch, self.pattern, self.priority, self.mode
        )
    }
}

/// Apply templates to the select expression.
pub(crate) fn apply_templates<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    s: &Transform<N>,
    m: &Option<QName>,
    o: &Vec<(Order, Transform<N>)>, // sort keys
) -> Result<Sequence<N>, Error> {
    // s is the select expression. Evaluate it, and then iterate over its items.
    // Each iteration becomes an item in the result sequence.
    let mut seq = ctxt.dispatch(stctxt, s)?;
    do_sort(&mut seq, o, ctxt, stctxt)?;
    seq.iter().try_fold(vec![], |mut result, i| {
        let templates = ctxt.find_templates(stctxt, i, m)?;
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
            .context(vec![i.clone()])
            .context_item(Some(i.clone()))
            .current_templates(templates)
            .build()
            .dispatch(stctxt, &matching.body)?;
        result.append(&mut u);
        Ok(result)
    })
}

/// Apply template with a higher import precedence.
pub(crate) fn apply_imports<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
) -> Result<Sequence<N>, Error> {
    // Find the template with the next highest level within the same import tree
    // current_templates[0] is the currently matching template
    if ctxt.current_templates.is_empty() {
        // TODO: select built-in templates instead
        return Ok(vec![]);
    }
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
            .dispatch(stctxt, &next[0].body)
    } else {
        Ok(vec![])
    }
}

/// Apply the next template that matches.
pub(crate) fn next_match<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
) -> Result<Sequence<N>, Error> {
    if ctxt.current_templates.len() > 2 {
        ContextBuilder::from(ctxt)
            .current_templates(ctxt.current_templates.iter().skip(1).cloned().collect())
            .build()
            .dispatch(stctxt, &ctxt.current_templates[1].body)
    } else {
        Ok(vec![])
    }
}
