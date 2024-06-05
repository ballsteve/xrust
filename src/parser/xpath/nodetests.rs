//! Functions that produce tests for nodes.

use crate::item::Node;
use crate::parser::combinators::alt::{alt2, alt5};
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::{ParseError, ParseInput};
use crate::transform::{KindTest, NameTest, NodeTest, WildcardOrName};
//use crate::parser::combinators::debug::inspect;
use crate::parser::xml::qname::{ncname, qualname};

pub(crate) fn qualname_test<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    Box::new(alt2(prefixed_name(), unprefixed_name()))
}
fn unprefixed_name<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    Box::new(map(ncname(), |localpart| {
        NodeTest::Name(NameTest {
            ns: None,
            prefix: None,
            name: Some(WildcardOrName::Name(String::from(localpart))),
        })
    }))
}
fn prefixed_name<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    Box::new(map(
        tuple3(ncname(), tag(":"), ncname()),
        |(prefix, _, localpart)| {
            NodeTest::Name(NameTest {
                ns: None,
                prefix: Some(String::from(prefix)),
                name: Some(WildcardOrName::Name(String::from(localpart))),
            })
        },
    ))
}

// NodeTest ::= KindTest | NameTest
// NameTest ::= EQName | Wildcard
pub(crate) fn nodetest<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    Box::new(alt2(kindtest(), nametest()))
}

// KindTest ::= DocumentTest | ElementTest | AttributeTest | SchemaElementTest | SchemaAttributeTest | PITest | CommentTest | TextTest | NamespaceNodeTest | AnyKindTest
fn kindtest<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    // Need alt10
    Box::new(alt2(
        alt5(
            document_test(),
            element_test(),
            attribute_test(),
            schema_element_test(),
            schema_attribute_test(),
        ),
        alt5(
            pi_test(),
            comment_test(),
            text_test(),
            namespace_node_test(),
            any_kind_test(),
        ),
    ))
}
// DocumentTest ::= "document-node" "(" ElementTest | SchemaElementTest ")"
fn document_test<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    // TODO: ElementTest|SchemaElementTest
    Box::new(map(tag("document-node()"), |_| {
        NodeTest::Kind(KindTest::Document)
    }))
}
// ElementTest ::= "element" "(" (ElementNameOrWildcard ("," TypeName)?)? ")"
fn element_test<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    // TODO: ElementTest|SchemaElementTest
    Box::new(map(
        tuple3(
            tag("element("),
            opt(map(
                alt2(map(qualname(), |_| ()), map(tag("*"), |_| ())),
                |_| (),
            )),
            tag(")"),
        ),
        |_| NodeTest::Kind(KindTest::Element),
    ))
}
// SchemaElementTest ::= "schema-element" "(" ElementNameDeclaration ")"
fn schema_element_test<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    // TODO: ElementTest|SchemaElementTest
    Box::new(map(
        tuple3(tag("schema-element("), qualname(), tag(")")),
        |_| NodeTest::Kind(KindTest::SchemaElement),
    ))
}
// AttributeTest ::= "attribute" "(" (AttribNameOrWildcard ("," TypeName))? ")"
fn attribute_test<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    Box::new(map(
        tuple3(
            tag("attribute("),
            opt(map(
                alt2(map(qualname(), |_| ()), map(tag("*"), |_| ())),
                |_| (),
            )),
            tag(")"),
        ),
        |_| NodeTest::Kind(KindTest::Attribute),
    ))
}
// SchemaAttributeTest ::= "attribute" "(" AttributeDeclaration ")"
fn schema_attribute_test<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    // TODO: AttributeDeclaration
    Box::new(map(
        tuple3(tag("schema-attribute("), qualname(), tag(")")),
        |_| NodeTest::Kind(KindTest::SchemaAttribute),
    ))
}
// PITest ::= "processing-instruction" "(" (NCName | StringLiteral)? ")"
fn pi_test<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    // TODO: NCName | StringLiteral
    Box::new(map(tag("processing-instruction()"), |_| {
        NodeTest::Kind(KindTest::PI)
    }))
}
// CommentTest ::= "comment" "(" ")"
fn comment_test<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    Box::new(map(tag("comment()"), |_| NodeTest::Kind(KindTest::Comment)))
}
// TextTest ::= "text" "(" ")"
fn text_test<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    Box::new(map(tag("text()"), |_| NodeTest::Kind(KindTest::Text)))
}
// NamespaceNodeTest ::= "namespace-node" "(" ")"
fn namespace_node_test<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    Box::new(map(tag("namespace-node()"), |_| {
        NodeTest::Kind(KindTest::Namespace)
    }))
}
// NamespaceNodeTest ::= "namespace-node" "(" ")"
fn any_kind_test<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    Box::new(map(tag("node()"), |_| NodeTest::Kind(KindTest::Any)))
}

// NameTest ::= EQName | Wildcard
// TODO: allow EQName rather than QName
fn nametest<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    Box::new(alt2(qualname_test(), wildcard()))
}

// Wildcard ::= '*' | (NCName ':*') | ('*:' NCName) | (BracedURILiteral '*')
// TODO: more specific wildcards
fn wildcard<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, NodeTest), ParseError> + 'a> {
    Box::new(map(tag("*"), |_| {
        NodeTest::Name(NameTest {
            ns: Some(WildcardOrName::Wildcard),
            prefix: None,
            name: Some(WildcardOrName::Wildcard),
        })
    }))
}
