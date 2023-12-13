/*! Context for a transformation

A dynamic and static context for a transformation. These are both necessary to give the transformation all the data it needs to performs its functions.

The dynamic [Context] stores data that changes. It is frequently cloned to create a new context. A [ContextBuilder] can be used to create the dynamic context incrementally.

The [StaticContext] stores immutable data and is not cloneable. A [StaticContextBuilder] can be used to create the static context incrementally.

A [Context] is used to evaluate a [Transform]. The evaluate method matches the current item to a template and then evaluates the body of that template. The dispatch method directly evaluates a given [Transform].

 */

use crate::item::{Node, Sequence};
use crate::output::OutputDefinition;
#[allow(unused_imports)]
use crate::pattern::Pattern;
use crate::transform::booleans::*;
use crate::transform::construct::*;
use crate::transform::controlflow::*;
use crate::transform::datetime::*;
use crate::transform::functions::*;
use crate::transform::grouping::*;
use crate::transform::logic::*;
use crate::transform::misc::message;
use crate::transform::navigate::*;
use crate::transform::numbers::*;
use crate::transform::strings::*;
use crate::transform::template::{apply_imports, apply_templates, next_match, Template};
use crate::transform::variables::{declare_variable, reference_variable};
use crate::transform::Transform;
use crate::xdmerror::Error;
use crate::{ErrorKind, Item, Value};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;
use url::Url;

//pub type Message = FnMut(&str) -> Result<(), Error>;

/// The transformation context. This is the dynamic context.
/// The static parts of the context are in a separate structure.
/// Contexts are immutable, but frequently are cloned to provide a new context.
/// Although it is optional, it would be very unusual not to set a result document in a context since nodes cannot be created in the result without one.
#[derive(Clone, Debug)]
pub struct Context<N: Node> {
    pub(crate) cur: Sequence<N>, // The current context
    pub(crate) i: usize,         // The index to the item that is the current context item
    pub(crate) depth: usize,     // Depth of evaluation
    pub(crate) rd: Option<N>,    // Result document
    // There is no distinction between built-in and user-defined templates
    // Built-in templates have no priority and no document order
    pub(crate) templates: Vec<Rc<Template<N>>>,
    pub(crate) current_templates: Vec<Rc<Template<N>>>,
    // Variables, with scoping
    pub(crate) vars: HashMap<String, Vec<Sequence<N>>>,
    // Grouping
    pub(crate) current_grouping_key: Option<Value>,
    pub(crate) current_group: Sequence<N>,
    // Output control
    pub(crate) od: OutputDefinition,
    pub(crate) base_url: Option<Url>,
}

impl<N: Node> Context<N> {
    pub fn new() -> Self {
        Context {
            cur: Sequence::new(),
            i: 0,
            depth: 0,
            rd: None,
            templates: vec![],
            current_templates: vec![],
            vars: HashMap::new(),
            current_grouping_key: None,
            current_group: Sequence::new(),
            od: OutputDefinition::new(),
            base_url: None,
        }
    }
    /// Sets the context item.
    pub fn context(&mut self, s: Sequence<N>, i: usize) {
        self.cur = s;
        self.i = i;
    }
    /// Sets the result document. Any nodes created by the transformation are owned by this document.
    pub fn result_document(&mut self, rd: N) {
        self.rd = Some(rd);
    }
    /// Set the value of a variable. If the variable already exists, then this creates a new inner scope.
    pub(crate) fn var_push(&mut self, name: String, value: Sequence<N>) {
        match self.vars.get_mut(name.as_str()) {
            Some(u) => {
                // If the variable already has a value, then this is a new, inner scope
                u.push(value);
            }
            None => {
                // Otherwise this is the first scope for the variable
                self.vars.insert(name, vec![value]);
            }
        }
    }
    /// Remove a variable
    #[allow(dead_code)]
    fn var_pop(&mut self, name: String) {
        self.vars.get_mut(name.as_str()).map(|u| u.pop());
    }

    /// Returns the Base URL.
    #[allow(dead_code)]
    fn baseurl(&self) -> Option<Url> {
        self.base_url.clone()
    }
    /// Set the Base URL. This is used to resolve relative URLs.
    #[allow(dead_code)]
    fn set_baseurl(&mut self, url: Url) {
        self.base_url = Some(url);
    }

    /// Evaluate finds a template matching the current item and evaluates the body of the template,
    /// returning the resulting [Sequence].
    /// ```rust
    /// use std::rc::Rc;
    /// use xrust::xdmerror::Error;
    /// use xrust::item::{Item, Sequence, SequenceTrait, Node, NodeType};
    /// use xrust::transform::Transform;
    /// use xrust::transform::context::{Context, StaticContext};
    /// use xrust::trees::intmuttree::{Document, RNode, NodeBuilder};
    /// use xrust::xslt::from_document;
    ///
    /// // This is necessary since there is no callback defined for the static context
    /// type F = Box<dyn FnMut(&str) -> Result<(), Error>>;
    ///
    /// // A little helper function to parse a string to a Document Node
    /// fn make_from_str(s: &str) -> RNode {
    ///   let e = Document::try_from((s, None, None))
    ///     .expect("failed to parse XML")
    ///     .content[0].clone();
    ///   let mut d = NodeBuilder::new(NodeType::Document).build();
    ///   d.push(e).expect("unable to append node");
    ///   d
    /// }
    ///
    /// let sd = Rc::new(Item::Node(make_from_str("<Example/>")));
    /// let style = make_from_str("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
    /// <xsl:template match='/'><xsl:apply-templates/></xsl:template>
    /// <xsl:template match='child::Example'>This template will match</xsl:template>
    /// </xsl:stylesheet>");
    /// let mut context = from_document(style, None, |s| Ok(make_from_str(s)), |_| Ok(String::new())).expect("unable to compile stylesheet");
    /// context.context(vec![sd], 0);
    /// context.result_document(make_from_str("<Result/>"));
    /// let sequence = context.evaluate(&mut StaticContext::<F>::new()).expect("evaluation failed");
    /// assert_eq!(sequence.to_string(), "This template will match")
    /// ```
    pub fn evaluate<F: FnMut(&str) -> Result<(), Error>>(
        &self,
        stctxt: &mut StaticContext<F>,
    ) -> Result<Sequence<N>, Error> {
        if self.cur.is_empty() {
            Ok(Sequence::new())
        } else {
            self.cur.get(self.i).map_or_else(
                || {
                    Err(Error::new(
                        ErrorKind::DynamicAbsent,
                        String::from("bad index into current sequence"),
                    ))
                },
                |i| {
                    // There may be 0, 1, or more matching templates.
                    // If there are more than one with the same priority and import level,
                    // then take the one with the higher document order.
                    let templates = self.find_templates(stctxt, i)?;
                    match templates.len() {
                        0 => Err(Error::new(
                            ErrorKind::DynamicAbsent,
                            String::from("no matching template"),
                        )),
                        1 => self.dispatch(stctxt, &templates[0].body),
                        _ => {
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
                                self.dispatch(stctxt, &candidates.last().unwrap().body)
                            } else {
                                self.dispatch(stctxt, &templates[0].body)
                            }
                        }
                    }
                },
            )
        }
    }

    /// Find a template with a matching [Pattern]
    pub fn find_templates<F: FnMut(&str) -> Result<(), Error>>(
        &self,
        stctxt: &mut StaticContext<F>,
        i: &Rc<Item<N>>,
    ) -> Result<Vec<Rc<Template<N>>>, Error> {
        let mut candidates = self.templates.iter().try_fold(vec![], |mut cand, t| {
            let e = t.pattern.matches(self, stctxt, i);
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

    /// Interpret the given [Transform] object
    /// ```rust
    /// use std::rc::Rc;
    /// use xrust::xdmerror::Error;
    /// use xrust::item::{Item, Sequence, SequenceTrait, Node, NodeType};
    /// use xrust::transform::{Transform, NodeMatch, NodeTest, KindTest,  Axis};
    /// use xrust::transform::context::{Context, ContextBuilder, StaticContext};
    /// use xrust::trees::intmuttree::{Document, RNode, NodeBuilder};
    ///
    /// // This is necessary since there is no callback defined for the static context
    /// type F = Box<dyn FnMut(&str) -> Result<(), Error>>;
    ///
    /// // A little helper function to parse a string to a Document Node
    /// fn make_from_str(s: &str) -> RNode {
    ///   let e = Document::try_from((s, None, None))
    ///     .expect("failed to parse XML")
    ///     .content[0].clone();
    ///   let mut d = NodeBuilder::new(NodeType::Document).build();
    ///   d.push(e).expect("unable to append node");
    ///   d
    /// }
    ///
    /// // Equivalent to "child::*"
    /// let t = Transform::Step(NodeMatch {axis: Axis::Child, nodetest: NodeTest::Kind(KindTest::Any)});
    /// let sd = Rc::new(Item::Node(make_from_str("<Example/>")));
    /// let context = ContextBuilder::new()
    ///   .current(vec![sd])
    ///   .build();
    /// let sequence = context.dispatch(&mut StaticContext::<F>::new(), &t).expect("evaluation failed");
    /// assert_eq!(sequence.to_xml(), "<Example></Example>")
    /// ```
    pub fn dispatch<F: FnMut(&str) -> Result<(), Error>>(
        &self,
        stctxt: &mut StaticContext<F>,
        t: &Transform<N>,
    ) -> Result<Sequence<N>, Error> {
        match t {
            Transform::Root => root(self),
            Transform::ContextItem => context(self),
            Transform::Compose(v) => compose(self, stctxt, v),
            Transform::Step(nm) => step(self, nm),
            Transform::Filter(t) => filter(self, stctxt, t),
            Transform::Empty => empty(self),
            Transform::Literal(v) => literal(self, v),
            Transform::LiteralElement(qn, t) => literal_element(self, stctxt, qn, t),
            Transform::Element(qn, t) => element(self, stctxt, qn, t),
            Transform::LiteralAttribute(qn, t) => literal_attribute(self, stctxt, qn, t),
            Transform::LiteralComment(t) => literal_comment(self, stctxt, t),
            Transform::LiteralProcessingInstruction(n, t) => {
                literal_processing_instruction(self, stctxt, n, t)
            }
            Transform::SetAttribute(qn, v) => set_attribute(self, stctxt, qn, v),
            Transform::SequenceItems(v) => make_sequence(self, stctxt, v),
            Transform::Copy(f, t) => copy(self, stctxt, f, t),
            Transform::DeepCopy(d) => deep_copy(self, stctxt, d),
            Transform::Or(v) => tr_or(self, stctxt, v),
            Transform::And(v) => tr_and(self, stctxt, v),
            Transform::Union(b) => union(self, stctxt, b),
            Transform::GeneralComparison(o, l, r) => general_comparison(self, stctxt, o, l, r),
            Transform::ValueComparison(o, l, r) => value_comparison(self, stctxt, o, l, r),
            Transform::Concat(v) => tr_concat(self, stctxt, v),
            Transform::Range(s, e) => tr_range(self, stctxt, s, e),
            Transform::Arithmetic(v) => arithmetic(self, stctxt, v),
            Transform::Loop(v, b) => tr_loop(self, stctxt, v, b),
            Transform::Switch(c, o) => switch(self, stctxt, c, o),
            Transform::ForEach(g, s, b) => for_each(self, stctxt, g, s, b),
            Transform::ApplyTemplates(s) => apply_templates(self, stctxt, s),
            Transform::ApplyImports => apply_imports(self, stctxt),
            Transform::NextMatch => next_match(self, stctxt),
            Transform::VariableDeclaration(n, v, f) => {
                declare_variable(self, stctxt, n.clone(), v, f)
            }
            Transform::VariableReference(n) => reference_variable(self, n),
            Transform::Position => position(self),
            Transform::Last => last(self),
            Transform::Count(s) => tr_count(self, stctxt, s),
            Transform::LocalName(s) => local_name(self, stctxt, s),
            Transform::Name(s) => name(self, stctxt, s),
            Transform::String(s) => string(self, stctxt, s),
            Transform::StartsWith(s, t) => starts_with(self, stctxt, s, t),
            Transform::EndsWith(s, t) => ends_with(self, stctxt, s, t),
            Transform::Contains(s, t) => contains(self, stctxt, s, t),
            Transform::Substring(s, t, l) => substring(self, stctxt, s, t, l),
            Transform::SubstringBefore(s, t) => substring_before(self, stctxt, s, t),
            Transform::SubstringAfter(s, t) => substring_after(self, stctxt, s, t),
            Transform::NormalizeSpace(s) => normalize_space(self, stctxt, s),
            Transform::Translate(s, m, t) => translate(self, stctxt, s, m, t),
            Transform::Boolean(b) => boolean(self, stctxt, b),
            Transform::Not(b) => not(self, stctxt, b),
            Transform::True => tr_true(self),
            Transform::False => tr_false(self),
            Transform::Number(n) => number(self, stctxt, n),
            Transform::Sum(s) => sum(self, stctxt, s),
            Transform::Floor(n) => floor(self, stctxt, n),
            Transform::Ceiling(n) => ceiling(self, stctxt, n),
            Transform::Round(n, p) => round(self, stctxt, n, p),
            Transform::CurrentGroup => current_group(self),
            Transform::CurrentGroupingKey => current_grouping_key(self),
            Transform::CurrentDateTime => current_date_time(self),
            Transform::CurrentDate => current_date(self),
            Transform::CurrentTime => current_time(self),
            Transform::FormatDateTime(t, p, l, c, q) => {
                format_date_time(self, stctxt, t, p, l, c, q)
            }
            Transform::FormatDate(t, p, l, c, q) => format_date(self, stctxt, t, p, l, c, q),
            Transform::FormatTime(t, p, l, c, q) => format_time(self, stctxt, t, p, l, c, q),
            Transform::UserDefined(_qn, a, b) => user_defined(self, stctxt, a, b),
            Transform::Message(b, s, e, t) => message(self, stctxt, b, s, e, t),
            Transform::Error(k, m) => tr_error(self, k, m),
            Transform::NotImplemented(s) => not_implemented(self, s),
            _ => Err(Error::new(
                ErrorKind::NotImplemented,
                "not implemented".to_string(),
            )),
        }
    }
}

impl<N: Node> From<Sequence<N>> for Context<N> {
    fn from(value: Sequence<N>) -> Self {
        Context {
            cur: value,
            i: 0,
            depth: 0,
            rd: None,
            templates: vec![],
            current_templates: vec![],
            vars: HashMap::new(),
            current_grouping_key: None,
            current_group: Sequence::new(),
            od: OutputDefinition::new(),
            base_url: None,
        }
    }
}

/// Builder for a [Context]
pub struct ContextBuilder<N: Node>(Context<N>);

impl<N: Node> ContextBuilder<N> {
    pub fn new() -> Self {
        ContextBuilder(Context::new())
    }
    pub fn current(mut self, s: Sequence<N>) -> Self {
        self.0.cur = s;
        self
    }
    pub fn index(mut self, i: usize) -> Self {
        self.0.i = i;
        self
    }
    pub fn depth(mut self, d: usize) -> Self {
        self.0.depth = d;
        self
    }
    pub fn variable(mut self, n: String, v: Sequence<N>) -> Self {
        self.0.var_push(n, v);
        self
    }
    pub fn variables(mut self, v: HashMap<String, Vec<Sequence<N>>>) -> Self {
        self.0.vars = v;
        self
    }
    pub fn result_document(mut self, rd: N) -> Self {
        self.0.rd = Some(rd);
        self
    }
    pub fn template(mut self, t: Template<N>) -> Self {
        self.0.templates.push(Rc::new(t));
        self
    }
    pub fn template_all(mut self, v: Vec<Template<N>>) -> Self {
        for t in v {
            self.0.templates.push(Rc::new(t))
        }
        self
    }
    pub fn current_templates(mut self, c: Vec<Rc<Template<N>>>) -> Self {
        self.0.current_templates = c;
        self
    }
    pub fn current_group(mut self, c: Sequence<N>) -> Self {
        self.0.current_group = c;
        self
    }
    pub fn current_grouping_key(mut self, k: Value) -> Self {
        self.0.current_grouping_key = Some(k);
        self
    }
    pub fn output_definition(mut self, od: OutputDefinition) -> Self {
        self.0.od = od;
        self
    }
    pub fn base_url(mut self, b: Url) -> Self {
        self.0.base_url = Some(b);
        self
    }
    pub fn build(self) -> Context<N> {
        self.0
    }
}

impl<N: Node> From<&Context<N>> for ContextBuilder<N> {
    fn from(c: &Context<N>) -> Self {
        ContextBuilder(c.clone())
    }
}

/// The static context. This is not clonable, since it includes the storage of a closure.
/// The main feature of the static context is the ability to set up a callback for messages.
/// See [StaticContextBuilder] for details.
pub struct StaticContext<F>
where
    F: FnMut(&str) -> Result<(), Error>,
{
    pub(crate) message: Option<F>,
}

impl<F> StaticContext<F>
where
    F: FnMut(&str) -> Result<(), Error>,
{
    pub fn new() -> Self {
        StaticContext { message: None }
    }
}

/// Builder for a [StaticContext].
/// The main feature of the static context is the ability to set up a callback for messages.
/// ```rust
/// use std::rc::Rc;
/// use xrust::qname::QualifiedName;
/// use xrust::value::Value;
/// use xrust::item::{Item, Sequence, SequenceTrait, NodeType};
/// use xrust::trees::intmuttree::NodeBuilder;
/// use xrust::transform::Transform;
/// use xrust::transform::context::{Context, ContextBuilder, StaticContext, StaticContextBuilder};
///
/// let mut message = String::from("no message received");
/// let xform = Transform::LiteralElement(
///   QualifiedName::new(None, None, String::from("Example")),
///   Box::new(Transform::SequenceItems(vec![
///	Transform::Message(
///		Box::new(Transform::Literal(Rc::new(Item::Value(Value::from("a message from the transformation"))))),
///		None,
///		Box::new(Transform::Empty),
///		Box::new(Transform::Empty),
///	),
///	Transform::Literal(Rc::new(Item::Value(Value::from("element content")))),
///   ]))
/// );
/// let mut context = ContextBuilder::new()
///	.result_document(NodeBuilder::new(NodeType::Document).build())
///	.build();
/// let mut static_context = StaticContextBuilder::new()
///	.message(|m| {message = String::from(m); Ok(())})
///	.build();
/// let sequence = context.dispatch(&mut static_context, &xform).expect("evaluation failed");
///
/// assert_eq!(sequence.to_xml(), "<Example>element content</Example>");
/// assert_eq!(message, "a message from the transformation")
/// ```
pub struct StaticContextBuilder<F: FnMut(&str) -> Result<(), Error>>(StaticContext<F>);

impl<F> StaticContextBuilder<F>
where
    F: FnMut(&str) -> Result<(), Error>,
{
    pub fn new() -> Self {
        StaticContextBuilder(StaticContext::new())
    }
    pub fn message(mut self, f: F) -> Self {
        self.0.message = Some(f);
        self
    }
    pub fn build(self) -> StaticContext<F> {
        self.0
    }
}
