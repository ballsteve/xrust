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
use crate::transform::Transform;
use crate::transform::booleans::*;
use crate::transform::callable::{Callable, invoke};
use crate::transform::construct::*;
use crate::transform::controlflow::*;
use crate::transform::datetime::*;
use crate::transform::functions::*;
use crate::transform::grouping::*;
use crate::transform::keys::{key, populate_key_values};
use crate::transform::logic::*;
use crate::transform::misc::*;
use crate::transform::navigate::*;
use crate::transform::numbers::*;
use crate::transform::strings::*;
use crate::transform::template::{Template, apply_imports, apply_templates, next_match};
use crate::transform::variables::{declare_variable, reference_variable};
use crate::xdmerror::Error;
use crate::{ErrorKind, Item, SequenceTrait, Value};
use qualname::{NamespaceMap, QName};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;
use url::Url;

/// The transformation context. This is the dynamic context.
/// The static parts of the context are in a separate structure.
/// Contexts are immutable, but frequently are cloned to provide a new context.
/// Although it is optional, it would be very unusual not to set a result document in a context since nodes cannot be created in the result without one.
#[derive(Clone, Debug)]
pub struct Context<N: Node> {
    pub(crate) context: Sequence<N>,          // The (outer) context
    pub(crate) i: usize,                      // The index to the item that is the context item
    pub(crate) context_item: Option<Item<N>>, // The context item, which is usually context[i]. Sometimes the inner focus is different to the context, e.g. when evaluating predicates
    pub(crate) current: Sequence<N>,
    pub(crate) current_item: Option<Item<N>>, // The "current" XPath item, which is really the context item for the invoking context. See XSLT 20.4.1.
    pub(crate) depth: usize,                  // Depth of evaluation
    pub(crate) rd: Option<N>,                 // Result document
    // There is no distinction between built-in and user-defined templates
    // Built-in templates have no priority and no document order
    pub(crate) templates: Vec<Rc<Template<N>>>,
    pub(crate) current_templates: Vec<Rc<Template<N>>>,
    // Named templates and functions
    pub(crate) callables: HashMap<QName, Callable<N>>,
    // Variables, with scoping
    pub(crate) vars: HashMap<String, Vec<Sequence<N>>>,
    // Stylesheet variables, to be evaluated before template processing.
    // (name, value) pairs
    pub(crate) pre_vars: Vec<(String, Transform<N>)>,
    // Grouping
    pub(crate) current_grouping_key: Option<Rc<Value>>,
    pub(crate) current_group: Sequence<N>,
    // Keys
    // The declaration of a key. Keys are named, and each key can have multiple definitions.
    // Each definition is the pattern that matches nodes and the expression that computes the key value.
    pub(crate) keys: HashMap<String, Vec<(Pattern<N>, Transform<N>)>>,
    // The calculated values of keys.
    pub(crate) key_values: HashMap<String, HashMap<String, Vec<N>>>,
    // Output control
    pub(crate) od: OutputDefinition,
    pub(crate) base_url: Option<Url>,
    // Namespace resolution. If any transforms contain a QName that needs to be resolved to an EQName,
    // then these URI -> prefix mappings may be used. These are usually derived from the stylesheet document.
    // The search order is: Namespace declarations in the source document; namespace declarations in the result document; this NamespaceMap.
    pub(crate) namespaces: Option<Rc<NamespaceMap>>,
}
impl<N: Node> Default for Context<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: Node> Context<N> {
    pub fn new() -> Self {
        Context {
            context: Sequence::new(),
            current: Sequence::new(),
            i: 0,
            context_item: None,
            current_item: None,
            depth: 0,
            rd: None,
            templates: vec![],
            current_templates: vec![],
            callables: HashMap::new(),
            vars: HashMap::new(),
            pre_vars: Vec::new(),
            current_grouping_key: None,
            current_group: Sequence::new(),
            keys: HashMap::new(),
            key_values: HashMap::new(),
            od: OutputDefinition::new(),
            base_url: None,
            namespaces: None,
        }
    }
    /// Sets the outer context and the context item.
    pub fn context(&mut self, s: Sequence<N>, i: usize) {
        self.context_item = Some(s[i].clone());
        self.context = s;
        self.i = i;
    }
    /// Sets the context item (which must be a member of the context) without changing the outer context.
    pub fn context_item(&mut self, i: Option<Item<N>>) {
        self.context_item = i;
    }
    /// Sets the "current" item.
    pub fn current_item(&mut self, i: Item<N>) {
        self.current_item = Some(i);
    }
    /// Sets the current context
    pub fn current(&mut self, s: Sequence<N>) {
        self.current = s;
    }
    /// Sets the result document. Any nodes created by the transformation are owned by this document.
    pub fn result_document(&mut self, rd: N) {
        self.rd = Some(rd);
    }
    /// Declare a key
    pub fn declare_key(&mut self, name: String, m: Pattern<N>, u: Transform<N>) {
        if let Some(v) = self.keys.get_mut(&name) {
            v.push((m, u))
        } else {
            self.keys.insert(name.clone(), vec![(m, u)]);
        }
        // Initialise the key values store with an empty hashmap
        if self.key_values.get_mut(&name).is_some() {
            // Already initialised
        } else {
            self.key_values.insert(name, HashMap::new());
        }
    }
    /// Calculate the key values for a source document
    pub fn populate_key_values<
        F: FnMut(&str) -> Result<(), Error>,
        G: FnMut(&str) -> Result<N, Error>,
        H: FnMut(&Url) -> Result<String, Error>,
    >(
        &mut self,
        stctxt: &mut StaticContext<N, F, G, H>,
        sd: N,
    ) -> Result<(), Error> {
        populate_key_values(self, stctxt, sd)
    }
    pub fn dump_key_values(&self) {
        self.key_values.iter().for_each(|(k, v)| {
            println!("key \"{}\":", k);
            v.iter()
                .for_each(|(kk, vv)| println!("\tvalue \"{}\" {} nodes", kk, vv.len()))
        })
    }
    /// Add a named attribute set. This replaces any previously declared attribute set with the same name
    pub fn attribute_set(&mut self, _name: QName, _body: Vec<Transform<N>>) {}
    /// Set the value of a variable. If the variable already exists, then this creates a new inner scope.
    pub fn var_push(&mut self, name: String, value: Sequence<N>) {
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
    #[allow(dead_code)]
    pub(crate) fn dump_vars(&self) -> String {
        self.vars.iter().fold(String::new(), |mut acc, (k, v)| {
            acc.push_str(format!("{}==\"{}\", ", k, v[0].to_string()).as_str());
            acc
        })
    }
    /// Add a stylesheet variable
    pub fn pre_var_push(&mut self, name: String, x: Transform<N>) {
        self.pre_vars.push((name, x));
    }

    /// Callable components: named templates and user-defined functions
    pub fn callable_push(&mut self, qn: QName, c: Callable<N>) {
        self.callables.insert(qn, c);
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

    // Does the work of evaluating the context,
    // but without the initial setup.
    pub(crate) fn evaluate_internal<
        F: FnMut(&str) -> Result<(), Error>,
        G: FnMut(&str) -> Result<N, Error>,
        H: FnMut(&Url) -> Result<String, Error>,
    >(
        &self,
        stctxt: &mut StaticContext<N, F, G, H>,
    ) -> Result<Sequence<N>, Error> {
        if self.context.is_empty() {
            Ok(Sequence::new())
        } else {
            self.context_item.as_ref().map_or_else(
                || {
                    Err(Error::new(
                        ErrorKind::DynamicAbsent,
                        String::from("no context item"),
                    ))
                },
                |i| {
                    // There may be 0, 1, or more matching templates.
                    // If there are more than one with the same priority and import level,
                    // then take the one with the higher document order.
                    let templates = self.find_templates(stctxt, i, &None)?;
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
    /// Evaluate finds a template matching the current item and evaluates the body of the template,
    /// returning the resulting [Sequence].
    /// ```rust
    /// use std::rc::Rc;
    /// use url::Url;
    /// use xrust::ErrorKind;
    /// use xrust::xdmerror::Error;
    /// use xrust::item::{Item, Sequence, SequenceTrait, Node, NodeType};
    /// use xrust::transform::Transform;
    /// use xrust::transform::context::{Context, StaticContext, StaticContextBuilder};
    /// use xrust::trees::smite::RNode;
    /// use xrust::parser::ParseError;
    /// use xrust::parser::xml::parse;
    /// use xrust::xslt::from_document;
    ///
    /// // A little helper function to parse a string to a Document Node
    /// fn make_from_str(s: &str) -> RNode {
    ///   let mut d = RNode::new_document();
    ///   parse(d.clone(), s,
    ///     Some(|_: &_| Err(ParseError::MissingNameSpace)))
    ///     .expect("failed to parse XML");
    ///   d
    /// }
    ///
    /// let sd = Item::Node(make_from_str("<Example/>"));
    /// let style = make_from_str("<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'>
    /// <xsl:template match='/'><xsl:apply-templates/></xsl:template>
    /// <xsl:template match='child::Example'>This template will match</xsl:template>
    /// </xsl:stylesheet>");
    /// let mut stctxt = StaticContextBuilder::new()
    ///     .message(|_| Ok(()))
    ///     .fetcher(|_| Ok(String::new()))
    ///     .parser(|s| Ok(make_from_str(s)))
    ///     .build();
    /// let mut context = from_document(style, None, |s| Ok(make_from_str(s)), |_| Ok(String::new())).expect("unable to compile stylesheet");
    /// context.context(vec![sd], 0);
    /// context.result_document(make_from_str("<Result/>"));
    /// let sequence = context.evaluate(&mut stctxt).expect("evaluation failed");
    /// assert_eq!(sequence.to_string(), "This template will match")
    /// ```
    pub fn evaluate<
        F: FnMut(&str) -> Result<(), Error>,
        G: FnMut(&str) -> Result<N, Error>,
        H: FnMut(&Url) -> Result<String, Error>,
    >(
        &self,
        stctxt: &mut StaticContext<N, F, G, H>,
    ) -> Result<Sequence<N>, Error> {
        // Define initial (stylesheet) variables by evaluating their transformation with the root node as the context
        if self.context.is_empty() {
            // There is no context item
            Ok(Sequence::new())
        } else {
            let mut ctxt = self.clone();
            // If the context item is a node then set the new context to the root node
            // otherwise there is no context
            ctxt.context(
                self.context.get(self.i).map_or_else(
                    Vec::new,
                    |i| {
                        if let Item::Node(n) = i {
                            vec![Item::Node(n.owner_document())]
                        } else {
                            vec![]
                        }
                    },
                ),
                0,
            );
            // Populate the context with stylesheet-level variables.
            // Each variable creates a new context for the evaluation of subsequent variables.
            for (name, x) in &self.pre_vars {
                ctxt.var_push(name.clone(), ctxt.dispatch(stctxt, x)?);
            }
            let result = ctxt.evaluate_internal(stctxt)?;
            // If any of the result items are nodes then add them as children of the result document
            if let Some(mut rd) = ctxt.rd {
                Ok(result
                    .into_iter()
                    .map(|i| {
                        if let Item::Node(ref n) = i {
                            if n.owner_document().is_same(&rd) {
                                if n.is_unattached() {
                                    // Attach it and leave it in the result
                                    rd.push(n.clone())
                                        .expect("unable to attach to result document");
                                    //Some(i)
                                    i
                                } else {
                                    // leave it where it is in the result document
                                    //Some(i)
                                    i
                                }
                            } else {
                                // This is a node from the source document.
                                // Copy it to the result document
                                // and replace the node in the result with the copy
                                let cp = n.deep_copy().expect("unable to copy node");
                                rd.push(cp.clone())
                                    .expect("unable to attach to result document");
                                //Some(Item::Node(cp))
                                Item::Node(cp)
                            }
                        } else {
                            //Some(i)
                            i
                        }
                    })
                    .collect())
            } else {
                Ok(result)
            }
        }
    }

    /// Find a template with a matching [Pattern] in the given mode.
    pub fn find_templates<
        F: FnMut(&str) -> Result<(), Error>,
        G: FnMut(&str) -> Result<N, Error>,
        H: FnMut(&Url) -> Result<String, Error>,
    >(
        &self,
        stctxt: &mut StaticContext<N, F, G, H>,
        i: &Item<N>,
        m: &Option<QName>,
    ) -> Result<Vec<Rc<Template<N>>>, Error> {
        let mut candidates =
            self.templates
                .iter()
                .filter(|t| t.mode == *m)
                .try_fold(vec![], |mut cand, t| {
                    let e = t.pattern.matches(self, stctxt, i);
                    if e {
                        cand.push(t.clone())
                    }
                    Ok(cand)
                })?;
        if !candidates.is_empty() {
            // Find the template(s) with the lowest priority.

            candidates.sort_unstable_by(|a, b| (*a).cmp(b));
            Ok(candidates)
        } else {
            Err(Error::new(
                ErrorKind::Unknown,
                format!("no matching template for item {:?} in mode \"{:?}\"", i, m),
            ))
        }
    }

    /// Interpret the given [Transform] object
    /// ```rust
    /// use std::rc::Rc;
    /// use url::Url;
    /// use xrust::xdmerror::{Error, ErrorKind};
    /// use xrust::item::{Item, Sequence, SequenceTrait, Node, NodeType};
    /// use xrust::transform::{Transform, NodeMatch, NodeTest, KindTest,  Axis};
    /// use xrust::transform::context::{Context, ContextBuilder, StaticContext, StaticContextBuilder};
    /// use xrust::trees::smite::RNode;
    /// use xrust::parser::ParseError;
    /// use xrust::parser::xml::parse;
    ///
    /// // A little helper function to parse a string to a Document Node
    /// fn make_from_str(s: &str) -> RNode {
    ///   let mut d = RNode::new_document();
    ///   parse(d.clone(), s, Some(|_: &_| Err(ParseError::MissingNameSpace)))
    ///     .expect("failed to parse XML");
    ///   d
    /// }
    ///
    /// // Equivalent to "child::*"
    /// let t = Transform::Step(NodeMatch {axis: Axis::Child, nodetest: NodeTest::Kind(KindTest::Any)});
    /// let sd = Item::Node(make_from_str("<Example/>"));
    /// let mut stctxt = StaticContextBuilder::new()
    ///    .message(|_| Ok(()))
    ///    .fetcher(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
    ///    .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
    ///     .build();
    /// let context = ContextBuilder::new()
    ///   .context(vec![sd])
    ///   .build();
    /// let sequence = context.dispatch(&mut stctxt, &t).expect("evaluation failed");
    /// assert_eq!(sequence.to_xml(), "<Example/>")
    /// ```
    pub fn dispatch<
        F: FnMut(&str) -> Result<(), Error>,
        G: FnMut(&str) -> Result<N, Error>,
        H: FnMut(&Url) -> Result<String, Error>,
        //L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
    >(
        &self,
        stctxt: &mut StaticContext<N, F, G, H>,
        t: &Transform<N>,
    ) -> Result<Sequence<N>, Error> {
        match t {
            Transform::Root => root(self),
            Transform::ContextItem => context(self),
            Transform::CurrentItem => current(self),
            Transform::Compose(v) => compose(self, stctxt, v),
            Transform::Step(nm) => step(self, nm),
            Transform::Filter(t) => filter(self, stctxt, t),
            Transform::Empty => empty(self),
            Transform::Literal(v) => literal(self, v),
            Transform::LiteralElement(qn, t) => literal_element(self, stctxt, qn, t),
            Transform::Element(qn, t) => element(self, stctxt, qn, t),
            Transform::LiteralText(t, b) => literal_text(self, stctxt, t, b),
            Transform::LiteralAttribute(qn, t) => literal_attribute(self, stctxt, qn, t),
            Transform::LiteralComment(t) => literal_comment(self, stctxt, t),
            Transform::LiteralProcessingInstruction(n, t) => {
                literal_processing_instruction(self, stctxt, n, t)
            }
            Transform::NamespaceDeclaration(p, u, s) => {
                namespace_declaration(self, stctxt, p, u, s)
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
            Transform::ForEach(g, s, b, o) => for_each(self, stctxt, g, s, b, o),
            Transform::ApplyTemplates(s, m, o) => apply_templates(self, stctxt, s, m, o),
            Transform::ApplyImports => apply_imports(self, stctxt),
            Transform::NextMatch => next_match(self, stctxt),
            Transform::VariableDeclaration(n, v, f, _) => {
                declare_variable(self, stctxt, n.clone(), v, f)
            }
            Transform::VariableReference(n, _) => reference_variable(self, n),
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
            Transform::GenerateId(s) => generate_id(self, stctxt, s),
            Transform::Boolean(b) => boolean(self, stctxt, b),
            Transform::Not(b) => not(self, stctxt, b),
            Transform::True => tr_true(self),
            Transform::False => tr_false(self),
            Transform::Number(n) => number(self, stctxt, n),
            Transform::Sum(s) => sum(self, stctxt, s),
            Transform::Avg(s) => avg(self, stctxt, s),
            Transform::Min(s) => min(self, stctxt, s),
            Transform::Max(s) => max(self, stctxt, s),
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
            Transform::FormatNumber(v, p, d) => format_number(self, stctxt, v, p, d),
            Transform::FormatInteger(i, s) => format_integer(self, stctxt, i, s),
            Transform::GenerateIntegers(start_at, select, n) => {
                generate_integers(self, stctxt, start_at, select, n)
            }
            Transform::Key(n, v, _, _) => key(self, stctxt, n, v),
            Transform::SystemProperty(p, ns) => system_property(self, stctxt, p, ns),
            Transform::AvailableSystemProperties => available_system_properties(),
            Transform::Document(uris, base) => document(self, stctxt, uris, base),
            Transform::Invoke(qn, a, ns) => invoke(self, stctxt, qn, a, ns),
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
        let ci = if value.is_empty() {
            None
        } else {
            Some(value[0].clone())
        };
        Context {
            context_item: ci,
            context: value,
            i: 0,
            current_item: None,
            current: Sequence::new(),
            depth: 0,
            rd: None,
            templates: vec![],
            current_templates: vec![],
            callables: HashMap::new(),
            vars: HashMap::new(),
            pre_vars: Vec::new(),
            keys: HashMap::new(),
            key_values: HashMap::new(),
            current_grouping_key: None,
            current_group: Sequence::new(),
            od: OutputDefinition::new(),
            base_url: None,
            namespaces: None,
        }
    }
}

/// Builder for a [Context]
pub struct ContextBuilder<N: Node>(Context<N>);

impl<N: Node> Default for ContextBuilder<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: Node> ContextBuilder<N> {
    pub fn new() -> Self {
        ContextBuilder(Context::new())
    }
    /// Set the context sequence. The first item is the context item.
    pub fn context(mut self, s: Sequence<N>) -> Self {
        if !s.is_empty() {
            self.0.context_item = Some(s[0].clone());
        }
        self.0.context = s;
        self.0.i = 0;
        self
    }
    /// Set which item is the context item.
    /// Does not check for validity.
    /// Does not set the context item (see context_item()).
    pub fn index(mut self, i: usize) -> Self {
        self.0.i = i;
        self
    }
    /// Sets the context item. This is usually context[index], but not always so this is not checked.
    pub fn context_item(mut self, c: Option<Item<N>>) -> Self {
        self.0.context_item = c;
        self
    }
    pub fn current_item(mut self, i: Option<Item<N>>) -> Self {
        self.0.current_item = i;
        self
    }
    pub fn current(mut self, s: Sequence<N>) -> Self {
        self.0.current = s;
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
    pub fn current_grouping_key(mut self, k: Rc<Value>) -> Self {
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
    pub fn callable(mut self, qn: QName, c: Callable<N>) -> Self {
        self.0.callables.insert(qn, c);
        self
    }
    pub fn namespaces(mut self, nm: NamespaceMap) -> Self {
        self.0.namespaces = Some(Rc::new(nm));
        self
    }
    pub fn build(self) -> Context<N> {
        self.0
    }
}

/// Derive a new [Context] from an old [Context]. The context sequence and item in the old context becomes the "current" sequence and item in the new context.
impl<N: Node> From<&Context<N>> for ContextBuilder<N> {
    fn from(c: &Context<N>) -> Self {
        c.context_item.as_ref().map_or_else(
            || ContextBuilder(c.clone()).current(vec![]).current_item(None),
            |i| {
                ContextBuilder(c.clone())
                    .current(c.context.clone())
                    .current_item(Some(i.clone()))
            },
        )
    }
}

/// The static context. This is not cloneable, since it includes the storage of a closure.
/// The main feature of the static context is the ability to set up a callback for messages.
/// See [StaticContextBuilder] for details.
pub struct StaticContext<N: Node, F, G, H>
where
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>, // Parses a string into a tree
    H: FnMut(&Url) -> Result<String, Error>, // Fetches the data from a URL
{
    pub(crate) message: Option<F>,
    pub(crate) parser: Option<G>,
    pub(crate) fetcher: Option<H>,
}
impl<N: Node, F, G, H> Default for StaticContext<N, F, G, H>
where
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
    {
        fn default() -> Self {
            Self::new()
    }
}

impl<N: Node, F, G, H> StaticContext<N, F, G, H>
where
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
{
    pub fn new() -> Self {
        StaticContext {
            message: None,
            parser: None,
            fetcher: None,
        }
    }
}

/// Builder for a [StaticContext].
/// The main feature of the static context is the ability to set up a callback for messages.
/// ```rust
/// use std::rc::Rc;
/// use qualname::{QName, NcName};
/// use xrust::{Error, ErrorKind};
/// use xrust::value::Value;
/// use xrust::item::{Item, Sequence, SequenceTrait, Node, NodeType};
/// use xrust::trees::smite::RNode;
/// use xrust::transform::Transform;
/// use xrust::transform::context::{Context, ContextBuilder, StaticContext, StaticContextBuilder};
///
/// let mut message = String::from("no message received");
/// let xform = Transform::LiteralElement(
///   QName::from_local_name(NcName::try_from("Example").unwrap()),
///   Box::new(Transform::SequenceItems(vec![
///    Transform::Message(
///        Box::new(Transform::Literal(Item::Value(Rc::new(Value::from("a message from the transformation"))))),
///        None,
///        Box::new(Transform::Empty),
///        Box::new(Transform::Empty),
///    ),
///    Transform::Literal(Item::Value(Rc::new(Value::from("element content")))),
///   ]))
/// );
/// let mut context = ContextBuilder::new()
///    .result_document(RNode::new_document())
///    .build();
/// let mut static_context = StaticContextBuilder::new()
///    .message(|m| {message = String::from(m); Ok(())})
///    .fetcher(|_| Ok(String::new()))
///    .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
///    .build();
/// let sequence = context.dispatch(&mut static_context, &xform).expect("evaluation failed");
///
/// assert_eq!(sequence.to_xml(), "<Example>element content</Example>");
/// assert_eq!(message, "a message from the transformation")
/// ```
pub struct StaticContextBuilder<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(StaticContext<N, F, G, H>);

impl<N: Node, F, G, H> Default for StaticContextBuilder<N, F, G, H>
where
     F: FnMut(&str) -> Result<(), Error>,
     G: FnMut(&str) -> Result<N, Error>,
     H: FnMut(&Url) -> Result<String, Error>,
    {
        fn default() -> Self {
            Self::new()
    }
}

impl<N: Node, F, G, H> StaticContextBuilder<N, F, G, H>
where
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
{
    pub fn new() -> Self {
        StaticContextBuilder(StaticContext::new())
    }
    pub fn message(mut self, f: F) -> Self {
        self.0.message = Some(f);
        self
    }
    pub fn parser(mut self, p: G) -> Self {
        self.0.parser = Some(p);
        self
    }
    pub fn fetcher(mut self, f: H) -> Self {
        self.0.fetcher = Some(f);
        self
    }
    pub fn build(self) -> StaticContext<N, F, G, H> {
        self.0
    }
}
