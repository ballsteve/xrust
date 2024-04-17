/*! The transformation engine.

A [Transform] performs processing, control flow, calculations, navigation, and construction to produce a [Sequence]. It starts with an initial context, the most important component of which is the current [Item]; this is often a [Node] that is the source document.

All functions in the [Transform] operate via the [Node] trait. This makes the transformation engine independent of the syntax of the source, stylesheet, and result documents. Any [Node]s created by the transformation use the context's result document object.

The following transformation implements the expression "1 + 1". The result is (hopefully) "2".

```rust
# use std::rc::Rc;
# use xrust::xdmerror::Error;
# use xrust::trees::intmuttree::RNode;
use xrust::value::Value;
use xrust::item::{Item, Node, Sequence, SequenceTrait};
use xrust::transform::{Transform, ArithmeticOperand, ArithmeticOperator};
use xrust::transform::context::{Context, StaticContext};
# type F = Box<dyn FnMut(&str) -> Result<(), Error>>;

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
let sequence = Context::new()
    .dispatch(&mut StaticContext::<F>::new(), &xform)
    .expect("evaluation failed");
assert_eq!(sequence.to_string(), "2")
```
*/

pub(crate) mod booleans;
pub(crate) mod construct;
pub mod context;
pub(crate) mod controlflow;
pub(crate) mod datetime;
pub(crate) mod functions;
pub(crate) mod grouping;
pub(crate) mod logic;
pub(crate) mod misc;
pub(crate) mod navigate;
pub(crate) mod numbers;
pub(crate) mod strings;
pub mod template;
pub(crate) mod variables;

#[allow(unused_imports)]
use crate::item::Sequence;
use crate::item::{Item, Node, NodeType};
use crate::qname::QualifiedName;
use crate::value::Operator;
#[allow(unused_imports)]
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};

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
    LiteralElement(QualifiedName, Box<Transform<N>>),
    /// A constructed element. Consists of the name and content.
    Element(Box<Transform<N>>, Box<Transform<N>>),
    /// A literal text node. Consists of the value of the node. Second argument gives whether to disable output escaping.
    LiteralText(Box<Transform<N>>, bool),
    /// A literal attribute. Consists of the attribute name and value.
    /// NB. The value may be produced by an Attribute Value Template, so must be dynamic.
    LiteralAttribute(QualifiedName, Box<Transform<N>>),
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

    /// Evaluate a transformation for each selected item, with possible grouping.
    ForEach(Option<Grouping<N>>, Box<Transform<N>>, Box<Transform<N>>),
    /// Find a template that matches an item and evaluate its body with the item as the context.
    /// Consists of the selector for items to be matched.
    ApplyTemplates(Box<Transform<N>>),
    /// Find templates at the next import level and evaluate its body.
    ApplyImports,
    NextMatch,

    /// Set union
    Union(Vec<Transform<N>>),

    /// Evaluate a named template or function, with arguments.
    /// Consists of the body of the template/function and the actual arguments (variable declarations).
    Call(Box<Transform<N>>, Vec<Transform<N>>),

    /// Declare a variable in the current context.
    /// Consists of the variable name, its value, and a transformation to perform with the variable in scope.
    VariableDeclaration(String, Box<Transform<N>>, Box<Transform<N>>),
    /// Reference a variable.
    /// The result is the value stored for that variable in the current context and current scope.
    VariableReference(String),

    /// Set the value of an attribute. The context item must be an element-type node.
    /// Consists of the name of the attribute and its value. The [Sequence] produced will be cast to a [Value].
    SetAttribute(QualifiedName, Box<Transform<N>>),

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
    CurrentGroup,
    CurrentGroupingKey,
    /// A user-defined callable. Consists of a name, an argument list, and a body.
    /// TODO: merge with Call?
    UserDefined(
        QualifiedName,
        Vec<(String, Transform<N>)>,
        Box<Transform<N>>,
    ),

    /// Emit a message. Consists of a select expression, a terminate attribute, an error-code, and a body.
    Message(
        Box<Transform<N>>,
        Option<Box<Transform<N>>>,
        Box<Transform<N>>,
        Box<Transform<N>>,
    ),

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
            Transform::ForEach(_g, _, _) => write!(f, "for-each"),
            Transform::Union(v) => write!(f, "union of {} operands", v.len()),
            Transform::ApplyTemplates(_) => write!(f, "Apply templates"),
            Transform::Call(_, a) => write!(f, "Call transform with {} arguments", a.len()),
            Transform::ApplyImports => write!(f, "Apply imports"),
            Transform::NextMatch => write!(f, "next-match"),
            Transform::VariableDeclaration(n, _, _) => write!(f, "declare variable \"{}\"", n),
            Transform::VariableReference(n) => write!(f, "reference variable \"{}\"", n),
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
            Transform::CurrentGroup => write!(f, "current-group"),
            Transform::CurrentGroupingKey => write!(f, "current-grouping-key"),
            Transform::UserDefined(qn, _a, _b) => write!(f, "user-defined \"{}\"", qn),
            Transform::Message(_, _, _, _) => write!(f, "message"),
            Transform::NotImplemented(s) => write!(f, "Not implemented: \"{}\"", s),
            Transform::Error(k, s) => write!(f, "Error: {} \"{}\"", k, s),
        }
    }
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
                                WildcardOrName::Name(s) => *s == n.name().get_localname(),
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
                    KindTest::Any => match n.node_type() {
                        NodeType::Document => false,
                        _ => true,
                    },
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
                        name: Some(WildcardOrName::Name(tok[0].to_string())),
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
                            name: Some(WildcardOrName::Name(tok[1].to_string())),
                            ns: Some(WildcardOrName::Wildcard),
                            prefix: None,
                        }))
                    }
                } else if tok[1] == "*" {
                    Ok(NodeTest::Name(NameTest {
                        name: Some(WildcardOrName::Wildcard),
                        ns: None,
                        prefix: Some(tok[0].to_string()),
                    }))
                } else {
                    Ok(NodeTest::Name(NameTest {
                        name: Some(WildcardOrName::Name(tok[1].to_string())),
                        ns: None,
                        prefix: Some(tok[0].to_string()),
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
    pub prefix: Option<String>,
    pub name: Option<WildcardOrName>,
}

impl NameTest {
    pub fn new(
        ns: Option<WildcardOrName>,
        prefix: Option<String>,
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
                        match (
                            self.ns.as_ref(),
                            self.name.as_ref(),
                            n.name().get_nsuri_ref(),
                            n.name().get_localname().as_str(),
                        ) {
                            (None, None, _, _) => false,
                            (None, Some(WildcardOrName::Wildcard), None, _) => true,
                            (None, Some(WildcardOrName::Wildcard), Some(_), _) => false,
                            (None, Some(WildcardOrName::Name(_)), None, "") => false,
                            (None, Some(WildcardOrName::Name(wn)), None, qn) => wn == qn,
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
                            ) => wn == qn,
                            (Some(WildcardOrName::Name(_)), Some(_), None, _) => false,
                            (
                                Some(WildcardOrName::Name(wnsuri)),
                                Some(WildcardOrName::Name(wn)),
                                Some(qnsuri),
                                qn,
                            ) => wnsuri == qnsuri && wn == qn,
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
    Name(String),
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
