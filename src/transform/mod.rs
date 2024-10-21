/*! The transformation engine.

A [Transform] performs processing, control flow, calculations, navigation, and construction to produce a [Sequence]. It starts with an initial context, the most important component of which is the current [Item]; this is often a [Node] that is the source document.

All functions in the [Transform] operate via the [Node] trait. This makes the transformation engine independent of the syntax of the source, stylesheet, and result documents. Any [Node]s created by the transformation use the context's result document object.

The following transformation implements the expression "1 + 1". The result is (hopefully) "2".

```rust
# use std::rc::Rc;
# use xrust::xdmerror::{Error, ErrorKind};
# use xrust::trees::smite::{RNode, Node as SmiteNode};
use xrust::value::Value;
use xrust::item::{Item, Node, Sequence, SequenceTrait};
use xrust::transform::{Transform, ArithmeticOperand, ArithmeticOperator};
use xrust::transform::context::{Context, StaticContext, StaticContextBuilder};

let xform = Transform::Arithmetic(vec![
        ArithmeticOperand::new(
            ArithmeticOperator::Noop,
            Transform::Literal(Item::<RNode>::Value(Rc::new(Value::from(1))))
        ),
        ArithmeticOperand::new(
            ArithmeticOperator::Add,
            Transform::Literal(Item::<RNode>::Value(Rc::new(Value::from(1))))
        )
    ]);
let mut static_context = StaticContextBuilder::new()
    .message(|_| Ok(()))
    .fetcher(|_| Ok(String::new()))
    .parser(|_| Err(Error::new(ErrorKind::NotImplemented, "not implemented")))
    .build();
let sequence = Context::new()
    .dispatch(&mut static_context, &xform)
    .expect("evaluation failed");
assert_eq!(sequence.to_string(), "2")
```
*/

pub(crate) mod booleans;
pub mod callable;
pub(crate) mod construct;
pub mod context;
pub(crate) mod controlflow;
pub(crate) mod datetime;
pub(crate) mod functions;
pub(crate) mod grouping;
mod keys;
pub(crate) mod logic;
pub(crate) mod misc;
pub(crate) mod navigate;
pub mod numbers;
pub(crate) mod strings;
pub mod template;
pub(crate) mod variables;

use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use url::Url;
#[allow(unused_imports)]
use crate::item::Sequence;
use crate::item::{Item, Node, NodeType, SequenceTrait};
use crate::qname::QualifiedName;
use crate::namespace::NamespaceMap;
use crate::transform::callable::ActualParameters;
use crate::transform::context::{Context, ContextBuilder, StaticContext};
use crate::transform::numbers::Numbering;
use crate::value::Operator;
#[allow(unused_imports)]
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};

/// Specifies how a [Sequence] is constructed.
#[derive(Clone)]
pub enum Transform<N: Node> {
    /// Produces the root node of the tree containing the context item.
    Root,
    /// Produces a copy of the context item.
    ContextItem,
    /// Produces a copy of the current item (see XSLT 20.4.1).
    CurrentItem,

    /// A path in a tree. Each element of the outer vector is a step in the path.
    /// The result of each step becomes the new context for the next step.
    Compose(Vec<Transform<N>>),
    /// A step in a path.
    Step(NodeMatch),
    ///
    /// Filters the selected items.
    /// Each item in the context is evaluated against the predicate.
    /// If the resulting sequence has an effective boolean value of 'true' then it is kept,
    /// otherwise it is discarded.
    Filter(Box<Transform<N>>),

    /// An empty sequence
    Empty,
    /// A literal, atomic value.
    Literal(Item<N>),
    /// A literal element. Consists of the element name and content.
    LiteralElement(Rc<QualifiedName>, Box<Transform<N>>),
    /// A constructed element. Consists of the name and content.
    Element(Box<Transform<N>>, Box<Transform<N>>),
    /// A literal text node. Consists of the value of the node. Second argument gives whether to disable output escaping.
    LiteralText(Box<Transform<N>>, bool),
    /// A literal attribute. Consists of the attribute name and value.
    /// NB. The value may be produced by an Attribute Value Template, so must be dynamic.
    LiteralAttribute(Rc<QualifiedName>, Box<Transform<N>>),
    /// A literal comment. Consists of the value.
    LiteralComment(Box<Transform<N>>),
    /// A literal processing instruction. Consists of the name and value.
    LiteralProcessingInstruction(Box<Transform<N>>, Box<Transform<N>>),
    /// Produce a [Sequence]. Each element in the vector becomes one, or more, item in the sequence.
    SequenceItems(Vec<Transform<N>>),

    /// A shallow copy of an item. Consists of the selector of the item to be copied,
    /// and the content of the target.
    Copy(Box<Transform<N>>, Box<Transform<N>>),
    /// A deep copy of an item. That is, it copies an item including its descendants.
    DeepCopy(Box<Transform<N>>),

    /// Logical OR. Each element of the outer vector is an operand.
    Or(Vec<Transform<N>>),
    /// Logical AND. Each element of the outer vector is an operand.
    And(Vec<Transform<N>>),

    /// XPath general comparison.
    /// Each item in the first sequence is compared against all items in the second sequence.
    GeneralComparison(Operator, Box<Transform<N>>, Box<Transform<N>>),
    /// XPath value comparison.
    /// The first singleton sequence is compared against the second singleton sequence.
    ValueComparison(Operator, Box<Transform<N>>, Box<Transform<N>>),

    /// Concatenate string values
    Concat(Vec<Transform<N>>),
    /// Produce a range of integer values.
    /// Consists of the start value and end value.
    Range(Box<Transform<N>>, Box<Transform<N>>),
    /// Perform arithmetic operations
    Arithmetic(Vec<ArithmeticOperand<N>>),

    /// A repeating transformation. Consists of variable declarations and the loop body.
    Loop(Vec<(String, Transform<N>)>, Box<Transform<N>>),
    /// A branching transformation. Consists of (test, body) clauses and an otherwise clause.
    Switch(Vec<(Transform<N>, Transform<N>)>, Box<Transform<N>>),

    /// Evaluate a transformation for each selected item, with possible grouping and sorting.
    ForEach(
        Option<Grouping<N>>,
        Box<Transform<N>>,
        Box<Transform<N>>,
        Vec<(Order, Transform<N>)>,
    ),
    /// Find a template that matches an item and evaluate its body with the item as the context.
    /// Consists of the selector for items to be matched, the mode, and sort keys.
    ApplyTemplates(
        Box<Transform<N>>,
        Option<Rc<QualifiedName>>,
        Vec<(Order, Transform<N>)>,
    ),
    /// Find templates at the next import level and evaluate its body.
    ApplyImports,
    NextMatch,

    /// Set union
    Union(Vec<Transform<N>>),

    /// Evaluate a named template or function, with arguments.
    /// Consists of the body of the template/function, the actual arguments (variable declarations), and in-scope namespace declarations.
    Call(Box<Transform<N>>, Vec<Transform<N>>, Rc<NamespaceMap>),

    /// Declare a variable in the current context.
    /// Consists of the variable name, its value, a transformation to perform with the variable in scope, and in-scope namespace declarations.
    VariableDeclaration(String, Box<Transform<N>>, Box<Transform<N>>, Rc<NamespaceMap>),
    /// Reference a variable.
    /// The result is the value stored for that variable in the current context and current scope.
    VariableReference(String, Rc<NamespaceMap>),

    /// Set the value of an attribute. The context item must be an element-type node.
    /// Consists of the name of the attribute and its value. The [Sequence] produced will be cast to a [Value].
    SetAttribute(Rc<QualifiedName>, Box<Transform<N>>),

    /// XPath functions
    Position,
    Last,
    Count(Box<Transform<N>>),
    LocalName(Option<Box<Transform<N>>>),
    Name(Option<Box<Transform<N>>>),
    String(Box<Transform<N>>),
    StartsWith(Box<Transform<N>>, Box<Transform<N>>),
    EndsWith(Box<Transform<N>>, Box<Transform<N>>),
    Contains(Box<Transform<N>>, Box<Transform<N>>),
    Substring(
        Box<Transform<N>>,
        Box<Transform<N>>,
        Option<Box<Transform<N>>>,
    ),
    SubstringBefore(Box<Transform<N>>, Box<Transform<N>>),
    SubstringAfter(Box<Transform<N>>, Box<Transform<N>>),
    NormalizeSpace(Option<Box<Transform<N>>>),
    Translate(Box<Transform<N>>, Box<Transform<N>>, Box<Transform<N>>),
    GenerateId(Option<Box<Transform<N>>>),
    Boolean(Box<Transform<N>>),
    Not(Box<Transform<N>>),
    True,
    False,
    Number(Box<Transform<N>>),
    Sum(Box<Transform<N>>),
    Floor(Box<Transform<N>>),
    Ceiling(Box<Transform<N>>),
    Round(Box<Transform<N>>, Option<Box<Transform<N>>>),
    CurrentDateTime,
    CurrentDate,
    CurrentTime,
    FormatDateTime(
        Box<Transform<N>>,
        Box<Transform<N>>,
        Option<Box<Transform<N>>>,
        Option<Box<Transform<N>>>,
        Option<Box<Transform<N>>>,
    ),
    FormatDate(
        Box<Transform<N>>,
        Box<Transform<N>>,
        Option<Box<Transform<N>>>,
        Option<Box<Transform<N>>>,
        Option<Box<Transform<N>>>,
    ),
    FormatTime(
        Box<Transform<N>>,
        Box<Transform<N>>,
        Option<Box<Transform<N>>>,
        Option<Box<Transform<N>>>,
        Option<Box<Transform<N>>>,
    ),
    FormatNumber(
        Box<Transform<N>>,
        Box<Transform<N>>,
        Option<Box<Transform<N>>>,
    ),
    /// Convert a number to a string.
    /// This is one half of the functionality of xsl:number, as well as format-integer().
    /// See XSLT 12.4.
    /// First argument is the integer to be formatted.
    /// Second argument is the format specification.
    FormatInteger(Box<Transform<N>>, Box<Transform<N>>),
    /// Generate a sequence of integers. This is one half of the functionality of xsl:number.
    /// First argument is the start-at specification.
    /// Second argument is the select expression.
    /// Third argument is the level.
    /// Fourth argument is the count pattern.
    /// Fifth argument is the from pattern.
    GenerateIntegers(Box<Transform<N>>, Box<Transform<N>>, Box<Numbering<N>>),
    CurrentGroup,
    CurrentGroupingKey,
    /// Look up a key. The first argument is the key name, the second argument is the key value,
    /// the third argument is the top of the tree for the resulting nodes,
    /// the fourth argument is the in-scope namespaces.
    Key(
        Box<Transform<N>>,
        Box<Transform<N>>,
        Option<Box<Transform<N>>>,
        Rc<NamespaceMap>,
    ),
    /// Get information about the processor
    SystemProperty(Box<Transform<N>>, Rc<NamespaceMap>),
    AvailableSystemProperties,
    /// Read an external document
    Document(Box<Transform<N>>, Option<Box<Transform<N>>>),

    /// Invoke a callable component. Consists of a name, an actual argument list, and in-scope namespace declarations.
    Invoke(Rc<QualifiedName>, ActualParameters<N>, Rc<NamespaceMap>),

    /// Emit a message. Consists of a select expression, a terminate attribute, an error-code, and a body.
    Message(
        Box<Transform<N>>,
        Option<Box<Transform<N>>>,
        Box<Transform<N>>,
        Box<Transform<N>>,
    ),

    // Why not just use Invoke?
    /// Extension elements/functions.
    /// The QName identifies the extension.
    Extension(Rc<QualifiedName>, Extension),

    /// For things that are not yet implemented, such as:
    /// Union, IntersectExcept, InstanceOf, Treat, Castable, Cast, Arrow, Unary, SimpleMap, Is, Before, After.
    NotImplemented(String),

    /// Error condition.
    Error(ErrorKind, String),
}

impl<N: Node> Debug for Transform<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Transform::Root => write!(f, "root node"),
            Transform::ContextItem => write!(f, "context item"),
            Transform::CurrentItem => write!(f, "current item"),
            Transform::SequenceItems(v) => write!(f, "Sequence of {} items", v.len()),
            Transform::Compose(v) => {
                write!(f, "Compose {} steps [", v.len()).expect("unable to format step");
                v.iter().for_each(|s| {
                    s.fmt(f).expect("unable to format step");
                    write!(f, "; ").expect("unable to format step")
                });
                write!(f, "]")
            }
            Transform::Step(nm) => write!(f, "Step matching {}", nm),
            Transform::Filter(_) => write!(f, "Filter"),
            Transform::Empty => write!(f, "Empty"),
            Transform::Literal(_) => write!(f, "literal value"),
            Transform::LiteralElement(qn, _) => write!(f, "literal element named \"{}\"", qn),
            Transform::Element(_, _) => write!(f, "constructed element"),
            Transform::LiteralText(_, b) => write!(f, "literal text (disable escaping {})", b),
            Transform::LiteralAttribute(qn, _) => write!(f, "literal attribute named \"{}\"", qn),
            Transform::LiteralComment(_) => write!(f, "literal comment"),
            Transform::LiteralProcessingInstruction(_, _) => {
                write!(f, "literal processing-instruction")
            }
            Transform::Copy(_, _) => write!(f, "shallow copy"),
            Transform::DeepCopy(_) => write!(f, "deep copy"),
            Transform::GeneralComparison(o, v, u) => {
                write!(f, "general comparison {} of {:?} and {:?}", o, v, u)
            }
            Transform::ValueComparison(o, v, u) => {
                write!(f, "value comparison {} of {:?} and {:?}", o, v, u)
            }
            Transform::Concat(o) => write!(f, "Concatenate {} operands", o.len()),
            Transform::Range(_, _) => write!(f, "range"),
            Transform::Arithmetic(o) => write!(f, "Arithmetic {} operands", o.len()),
            Transform::And(o) => write!(f, "AND {} operands", o.len()),
            Transform::Or(o) => write!(f, "OR {} operands", o.len()),
            Transform::Loop(_, _) => write!(f, "loop"),
            Transform::Switch(c, _) => write!(f, "switch {} clauses", c.len()),
            Transform::ForEach(_g, _, _, o) => write!(f, "for-each ({} sort keys)", o.len()),
            Transform::Union(v) => write!(f, "union of {} operands", v.len()),
            Transform::ApplyTemplates(_, m, o) => {
                write!(f, "Apply templates (mode {:?}, {} sort keys)", m, o.len())
            }
            Transform::Call(_, a, _) => write!(f, "Call transform with {} arguments", a.len()),
            Transform::ApplyImports => write!(f, "Apply imports"),
            Transform::NextMatch => write!(f, "next-match"),
            Transform::VariableDeclaration(n, _, _, _) => write!(f, "declare variable \"{}\"", n),
            Transform::VariableReference(n, _) => write!(f, "reference variable \"{}\"", n),
            Transform::SetAttribute(n, _) => write!(f, "set attribute named \"{}\"", n),
            Transform::Position => write!(f, "position"),
            Transform::Last => write!(f, "last"),
            Transform::Count(_s) => write!(f, "count()"),
            Transform::Name(_n) => write!(f, "name()"),
            Transform::LocalName(_n) => write!(f, "local-name()"),
            Transform::String(s) => write!(f, "string({:?})", s),
            Transform::StartsWith(s, t) => write!(f, "starts-with({:?}, {:?})", s, t),
            Transform::EndsWith(s, t) => write!(f, "ends-with({:?}, {:?})", s, t),
            Transform::Contains(s, t) => write!(f, "contains({:?}, {:?})", s, t),
            Transform::Substring(s, t, _l) => write!(f, "substring({:?}, {:?}, ...)", s, t),
            Transform::SubstringBefore(s, t) => write!(f, "substring-before({:?}, {:?})", s, t),
            Transform::SubstringAfter(s, t) => write!(f, "substring-after({:?}, {:?})", s, t),
            Transform::NormalizeSpace(_s) => write!(f, "normalize-space()"),
            Transform::Translate(s, t, u) => write!(f, "translate({:?}, {:?}, {:?})", s, t, u),
            Transform::GenerateId(_) => write!(f, "generate-id()"),
            Transform::Boolean(b) => write!(f, "boolean({:?})", b),
            Transform::Not(b) => write!(f, "not({:?})", b),
            Transform::True => write!(f, "true"),
            Transform::False => write!(f, "false"),
            Transform::Number(n) => write!(f, "number({:?})", n),
            Transform::Sum(n) => write!(f, "sum({:?})", n),
            Transform::Floor(n) => write!(f, "floor({:?})", n),
            Transform::Ceiling(n) => write!(f, "ceiling({:?})", n),
            Transform::Round(n, _p) => write!(f, "round({:?},...)", n),
            Transform::CurrentDateTime => write!(f, "current-date-time"),
            Transform::CurrentDate => write!(f, "current-date"),
            Transform::CurrentTime => write!(f, "current-time"),
            Transform::FormatDateTime(p, q, _, _, _) => {
                write!(f, "format-date-time({:?}, {:?}, ...)", p, q)
            }
            Transform::FormatDate(p, q, _, _, _) => write!(f, "format-date({:?}, {:?}, ...)", p, q),
            Transform::FormatTime(p, q, _, _, _) => write!(f, "format-time({:?}, {:?}, ...)", p, q),
            Transform::FormatNumber(v, p, _) => write!(f, "format-number({:?}, {:?})", v, p),
            Transform::FormatInteger(i, s) => write!(f, "format-integer({:?}, {:?})", i, s),
            Transform::GenerateIntegers(_start_at, _select, _n) => write!(f, "generate-integers"),
            Transform::CurrentGroup => write!(f, "current-group"),
            Transform::CurrentGroupingKey => write!(f, "current-grouping-key"),
            Transform::Key(s, _, _, _) => write!(f, "key({:?}, ...)", s),
            Transform::SystemProperty(p, _) => write!(f, "system-properties({:?})", p),
            Transform::AvailableSystemProperties => write!(f, "available-system-properties"),
            Transform::Document(uris, _) => write!(f, "document({:?})", uris),
            Transform::Invoke(qn, _a, _) => write!(f, "invoke \"{}\"", qn),
            Transform::Message(_, _, _, _) => write!(f, "message"),
            Transform::NotImplemented(s) => write!(f, "Not implemented: \"{}\"", s),
            Transform::Error(k, s) => write!(f, "Error: {} \"{}\"", k, s),
        }
    }
}

/// A convenience function to create a namespace mapping from a [Node].
pub fn in_scope_namespaces<N: Node>(n: Option<N>) -> Rc<NamespaceMap> {
    if let Some(nn) = n {
        Rc::new(nn.namespace_iter().fold(NamespaceMap::new(), |mut hm, ns| {
            hm.insert(Some(ns.name().localname()), ns.value());
            hm
        }))
    } else {
        Rc::new(NamespaceMap::new())
    }
}

/// The sort order
#[derive(Clone, PartialEq, Debug)]
pub enum Order {
    Ascending,
    Descending,
}

/// Performing sorting of a [Sequence] using the given sort keys.
pub(crate) fn do_sort<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    seq: &mut Sequence<N>,
    o: &Vec<(Order, Transform<N>)>,
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
) -> Result<(), Error> {
    // Optionally sort the select sequence
    // TODO: multiple sort keys
    if !o.is_empty() {
        seq.sort_by_cached_key(|k| {
            // TODO: Don't panic
            let key_seq = ContextBuilder::from(ctxt)
                .context(vec![k.clone()])
                .build()
                .dispatch(stctxt, &o[0].1)
                .expect("unable to determine key value");
            // Assume string data type for now
            // TODO: support number data type
            // TODO: support all data types
            key_seq.to_string()
        });
        if o[0].0 == Order::Descending {
            seq.reverse();
        }
    }
    Ok(())
}

/// Determine how a collection is to be divided into groups.
/// This value would normally be inside an Option.
/// A None value for the option means that the collection is not to be grouped.
#[derive(Clone, Debug)]
pub enum Grouping<N: Node> {
    By(Vec<Transform<N>>),
    StartingWith(Vec<Transform<N>>),
    EndingWith(Vec<Transform<N>>),
    Adjacent(Vec<Transform<N>>),
}

impl<N: Node> Grouping<N> {
    fn to_string(&self) -> String {
        match self {
            Grouping::By(_) => "group-by".to_string(),
            Grouping::Adjacent(_) => "group-adjacent".to_string(),
            Grouping::StartingWith(_) => "group-starting-with".to_string(),
            Grouping::EndingWith(_) => "group-ending-with".to_string(),
        }
    }
}

impl<N: Node> fmt::Display for Grouping<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct NodeMatch {
    pub axis: Axis,
    pub nodetest: NodeTest,
}

impl NodeMatch {
    pub fn new(axis: Axis, nodetest: NodeTest) -> Self {
        NodeMatch { axis, nodetest }
    }
    pub fn matches_item<N: Node>(&self, i: &Item<N>) -> bool {
        match i {
            Item::Node(n) => self.matches(n),
            _ => false,
        }
    }
    pub fn matches<N: Node>(&self, n: &N) -> bool {
        match &self.nodetest {
            NodeTest::Name(t) => {
                match n.node_type() {
                    NodeType::Element | NodeType::Attribute => {
                        // TODO: namespaces
                        match &t.name {
                            Some(a) => match a {
                                WildcardOrName::Wildcard => true,
                                WildcardOrName::Name(s) => *s == n.name().localname(),
                            },
                            None => false,
                        }
                    }
                    _ => false,
                }
            }
            NodeTest::Kind(k) => {
                match k {
                    KindTest::Document => matches!(n.node_type(), NodeType::Document),
                    KindTest::Element => matches!(n.node_type(), NodeType::Element),
                    KindTest::PI => matches!(n.node_type(), NodeType::ProcessingInstruction),
                    KindTest::Comment => matches!(n.node_type(), NodeType::Comment),
                    KindTest::Text => matches!(n.node_type(), NodeType::Text),
                    //Note: This one is matching not NodeType::Document
                    KindTest::Any => !matches!(n.node_type(), NodeType::Document),
                    KindTest::Attribute
                    | KindTest::SchemaElement
                    | KindTest::SchemaAttribute
                    | KindTest::Namespace => false, // TODO: not yet implemented
                }
            }
        }
    }
}

impl fmt::Display for NodeMatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "node match {} {}", self.axis, self.nodetest)
    }
}

#[derive(Clone, Debug)]
pub enum NodeTest {
    Kind(KindTest),
    Name(NameTest),
}

impl NodeTest {
    pub fn matches<N: Node>(&self, i: &Item<N>) -> bool {
        match i {
            Item::Node(_) => match self {
                NodeTest::Kind(k) => k.matches(i),
                NodeTest::Name(nm) => nm.matches(i),
            },
            _ => false,
        }
    }
}

impl fmt::Display for NodeTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NodeTest::Kind(k) => write!(f, "kind test {}", k),
            NodeTest::Name(nm) => write!(f, "name test {}", nm),
        }
    }
}

impl TryFrom<&str> for NodeTest {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // Import this from xpath.rs?
        let tok: Vec<&str> = s.split(':').collect();
        match tok.len() {
            1 => {
                // unprefixed
                if tok[0] == "*" {
                    Ok(NodeTest::Name(NameTest {
                        name: Some(WildcardOrName::Wildcard),
                        ns: None,
                        prefix: None,
                    }))
                } else {
                    Ok(NodeTest::Name(NameTest {
                        name: Some(WildcardOrName::Name(Rc::new(Value::from(tok[0])))),
                        ns: None,
                        prefix: None,
                    }))
                }
            }
            2 => {
                // prefixed
                if tok[0] == "*" {
                    if tok[1] == "*" {
                        Ok(NodeTest::Name(NameTest {
                            name: Some(WildcardOrName::Wildcard),
                            ns: Some(WildcardOrName::Wildcard),
                            prefix: None,
                        }))
                    } else {
                        Ok(NodeTest::Name(NameTest {
                            name: Some(WildcardOrName::Name(Rc::new(Value::from(tok[1])))),
                            ns: Some(WildcardOrName::Wildcard),
                            prefix: None,
                        }))
                    }
                } else if tok[1] == "*" {
                    Ok(NodeTest::Name(NameTest {
                        name: Some(WildcardOrName::Wildcard),
                        ns: None,
                        prefix: Some(Rc::new(Value::from(tok[0]))),
                    }))
                } else {
                    Ok(NodeTest::Name(NameTest {
                        name: Some(WildcardOrName::Name(Rc::new(Value::from(tok[1])))),
                        ns: None,
                        prefix: Some(Rc::new(Value::from(tok[0]))),
                    }))
                }
            }
            _ => Result::Err(Error::new(
                ErrorKind::TypeError,
                "invalid NodeTest".to_string(),
            )),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum KindTest {
    Document,
    Element,
    Attribute,
    SchemaElement,
    SchemaAttribute,
    PI,
    Comment,
    Text,
    Namespace,
    Any,
}

impl fmt::Display for KindTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            KindTest::Document => write!(f, "document"),
            KindTest::Element => write!(f, "element"),
            KindTest::Attribute => write!(f, "attribute"),
            KindTest::SchemaElement => write!(f, "schema element"),
            KindTest::SchemaAttribute => write!(f, "schema attribute"),
            KindTest::PI => write!(f, "processing instruction"),
            KindTest::Comment => write!(f, "comment"),
            KindTest::Text => write!(f, "text"),
            KindTest::Namespace => write!(f, "namespace"),
            KindTest::Any => write!(f, "any node"),
        }
    }
}

impl KindTest {
    /// Does an item match the Kind Test?
    pub fn matches<N: Node>(&self, i: &Item<N>) -> bool {
        match i {
            Item::Node(n) => {
                match (self, n.node_type()) {
                    (KindTest::Document, NodeType::Document) => true,
                    (KindTest::Document, _) => false,
                    (KindTest::Element, NodeType::Element) => true,
                    (KindTest::Element, _) => false,
                    (KindTest::Attribute, NodeType::Attribute) => true,
                    (KindTest::Attribute, _) => false,
                    (KindTest::SchemaElement, _) => false, // not supported
                    (KindTest::SchemaAttribute, _) => false, // not supported
                    (KindTest::PI, NodeType::ProcessingInstruction) => true,
                    (KindTest::PI, _) => false,
                    (KindTest::Comment, NodeType::Comment) => true,
                    (KindTest::Comment, _) => false,
                    (KindTest::Text, NodeType::Text) => true,
                    (KindTest::Text, _) => false,
                    (KindTest::Namespace, _) => false, // not yet implemented
                    (KindTest::Any, _) => true,
                }
            }
            _ => false,
        }
    }
    pub fn to_string(&self) -> &'static str {
        match self {
            KindTest::Document => "DocumentTest",
            KindTest::Element => "ElementTest",
            KindTest::Attribute => "AttributeTest",
            KindTest::SchemaElement => "SchemaElementTest",
            KindTest::SchemaAttribute => "SchemaAttributeTest",
            KindTest::PI => "PITest",
            KindTest::Comment => "CommentTest",
            KindTest::Text => "TextTest",
            KindTest::Namespace => "NamespaceNodeTest",
            KindTest::Any => "AnyKindTest",
        }
    }
}

#[derive(Clone, Debug)]
pub struct NameTest {
    pub ns: Option<WildcardOrName>,
    pub prefix: Option<Rc<Value>>,
    pub name: Option<WildcardOrName>,
}

impl NameTest {
    pub fn new(
        ns: Option<WildcardOrName>,
        prefix: Option<Rc<Value>>,
        name: Option<WildcardOrName>,
    ) -> Self {
        NameTest { ns, prefix, name }
    }
    /// Does an Item match this name test? To match, the item must be a node, have a name,
    /// have the namespace URI and local name be equal or a wildcard
    pub fn matches<N: Node>(&self, i: &Item<N>) -> bool {
        match i {
            Item::Node(n) => {
                match n.node_type() {
                    NodeType::Element | NodeType::ProcessingInstruction | NodeType::Attribute => {
                        // TODO: avoid converting the values into strings just for comparison
                        // Value interning should fix this
                        match (
                            self.ns.as_ref(),
                            self.name.as_ref(),
                            n.name().namespace_uri(),
                            n.name().localname_to_string().as_str(),
                        ) {
                            (None, None, _, _) => false,
                            (None, Some(WildcardOrName::Wildcard), None, _) => true,
                            (None, Some(WildcardOrName::Wildcard), Some(_), _) => false,
                            (None, Some(WildcardOrName::Name(_)), None, "") => false,
                            (None, Some(WildcardOrName::Name(wn)), None, qn) => wn.to_string() == qn,
                            (None, Some(WildcardOrName::Name(_)), Some(_), _) => false,
                            (Some(_), None, _, _) => false, // A namespace URI without a local name doesn't make sense
                            (
                                Some(WildcardOrName::Wildcard),
                                Some(WildcardOrName::Wildcard),
                                _,
                                _,
                            ) => true,
                            (
                                Some(WildcardOrName::Wildcard),
                                Some(WildcardOrName::Name(_)),
                                _,
                                "",
                            ) => false,
                            (
                                Some(WildcardOrName::Wildcard),
                                Some(WildcardOrName::Name(wn)),
                                _,
                                qn,
                            ) => wn.to_string() == qn,
                            (Some(WildcardOrName::Name(_)), Some(_), None, _) => false,
                            (
                                Some(WildcardOrName::Name(wnsuri)),
                                Some(WildcardOrName::Name(wn)),
                                Some(qnsuri),
                                qn,
                            ) => wnsuri.clone() == qnsuri && wn.to_string() == qn,
                            _ => false, // maybe should panic?
                        }
                    }
                    _ => false, // all other node types don't have names
                }
            }
            _ => false, // other item types don't have names
        }
    }
}

impl fmt::Display for NameTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let result = if self.name.is_some() {
            match self.name.as_ref().unwrap() {
                WildcardOrName::Wildcard => "*".to_string(),
                WildcardOrName::Name(n) => n.to_string(),
            }
        } else {
            "--no name--".to_string()
        };
        f.write_str(result.as_str())
    }
}

#[derive(Clone, Debug)]
pub enum WildcardOrName {
    Wildcard,
    Name(Rc<Value>),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Axis {
    Child,
    Descendant,
    DescendantOrSelf,
    DescendantOrSelfOrRoot,
    Attribute,
    SelfAttribute, // a special axis, only for matching an attribute in a pattern match
    SelfAxis,
    SelfDocument,  // a special axis, only for matching the Document in a pattern match
    SelfNamespace, // a special axis, only for matching the namespace in a pattern match
    Following,
    FollowingSibling,
    Namespace,
    Parent,
    ParentDocument, // a special axis, only for matching in a pattern match. Matches the parent as well as the Document.
    Ancestor,
    AncestorOrSelf,
    AncestorOrSelfOrRoot, // a special axis for matching in a pattern
    Preceding,
    PrecedingSibling,
    Unknown,
}

impl From<&str> for Axis {
    fn from(s: &str) -> Self {
        match s {
            "child" => Axis::Child,
            "descendant" => Axis::Descendant,
            "descendant-or-self" => Axis::DescendantOrSelf,
            "attribute" => Axis::Attribute,
            "self" => Axis::SelfAxis,
            "following" => Axis::Following,
            "following-sibling" => Axis::FollowingSibling,
            "namespace" => Axis::Namespace,
            "parent" => Axis::Parent,
            "ancestor" => Axis::Ancestor,
            "ancestor-or-self" => Axis::AncestorOrSelf,
            "preceding" => Axis::Preceding,
            "preceding-sibling" => Axis::PrecedingSibling,
            _ => Axis::Unknown,
        }
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let result = match self {
            Axis::Child => "child".to_string(),
            Axis::Descendant => "descendant".to_string(),
            Axis::DescendantOrSelf => "descendant-or-self".to_string(),
            Axis::DescendantOrSelfOrRoot => "descendant-or-self-or-root".to_string(),
            Axis::Attribute => "attribute".to_string(),
            Axis::SelfAttribute => "self-attribute".to_string(),
            Axis::SelfAxis => "self".to_string(),
            Axis::SelfDocument => "self-document".to_string(),
            Axis::Following => "following".to_string(),
            Axis::FollowingSibling => "following-sibling".to_string(),
            Axis::Namespace => "namespace".to_string(),
            Axis::Parent => "parent".to_string(),
            Axis::ParentDocument => "parent-document".to_string(),
            Axis::Ancestor => "ancestor".to_string(),
            Axis::AncestorOrSelf => "ancestor-or-self".to_string(),
            Axis::Preceding => "preceding".to_string(),
            Axis::PrecedingSibling => "preceding-sibling".to_string(),
            _ => "unknown".to_string(),
        };
        f.write_str(result.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct ArithmeticOperand<N: Node> {
    pub op: ArithmeticOperator,
    pub operand: Transform<N>,
}

impl<N: Node> ArithmeticOperand<N> {
    pub fn new(op: ArithmeticOperator, operand: Transform<N>) -> Self {
        ArithmeticOperand { op, operand }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticOperator {
    Noop,
    Add,
    Multiply,
    Divide,
    IntegerDivide,
    Subtract,
    Modulo,
}

impl fmt::Display for ArithmeticOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithmeticOperator::Add => f.write_str("+"),
            ArithmeticOperator::Multiply => f.write_str("*"),
            ArithmeticOperator::Divide => f.write_str("div"),
            ArithmeticOperator::IntegerDivide => f.write_str("idiv"),
            ArithmeticOperator::Subtract => f.write_str("-"),
            ArithmeticOperator::Modulo => f.write_str("mod"),
            ArithmeticOperator::Noop => f.write_str("noop"),
        }
    }
}

impl From<&str> for ArithmeticOperator {
    fn from(a: &str) -> Self {
        match a {
            "+" => ArithmeticOperator::Add,
            "*" => ArithmeticOperator::Multiply,
            "div" => ArithmeticOperator::Divide,
            "idiv" => ArithmeticOperator::IntegerDivide,
            "-" => ArithmeticOperator::Subtract,
            "mod" => ArithmeticOperator::Modulo,
            _ => ArithmeticOperator::Noop,
        }
    }
}

impl From<String> for ArithmeticOperator {
    fn from(a: String) -> Self {
        match a.as_str() {
            "+" => ArithmeticOperator::Add,
            "*" => ArithmeticOperator::Multiply,
            "div" => ArithmeticOperator::Divide,
            "idiv" => ArithmeticOperator::IntegerDivide,
            "-" => ArithmeticOperator::Subtract,
            "mod" => ArithmeticOperator::Modulo,
            _ => ArithmeticOperator::Noop,
        }
    }
}
