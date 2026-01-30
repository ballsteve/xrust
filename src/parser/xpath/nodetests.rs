//! Functions that produce tests for nodes.

use crate::item::Node;
use crate::parser::combinators::alt::{alt2, alt5};
//use crate::parser::combinators::debug::inspect;
use crate::parser::combinators::map::{map, map_with_state_and_result};
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple3;
use crate::parser::xml::qname::{ncname, qualname_to_qname};
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::transform::{KindTest, NameTest, NodeTest, WildcardOrName, WildcardOrNamespaceUri};
use qualname::{NamespacePrefix, NamespaceUri, NcName, QName};

pub(crate) fn qualname_test<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt2(prefixed_name(), unprefixed_name()))
}
fn unprefixed_name<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(ncname(), |localpart| {
        NodeTest::Name(NameTest::Name(QName::from_local_name(
            NcName::try_from(localpart.as_str())
                .map_err(|_| ParseError::MissingNameSpace)
                .expect("not a NcName"),
        )))
    }))
}
fn prefixed_name<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map_with_state_and_result(
        tuple3(ncname::<N, L>(), tag(":"), ncname()),
        |(prefix, _, localpart), _state, ss| {
            let lp = NcName::try_from(localpart.as_str()).map_err(|_| {
                ParseError::NSResolveError(format!("invalid local part \"{}\"", localpart))
            })?;
            if let Some(nsr) = ss.namespace.as_mut() {
                let prefix = NamespacePrefix::try_from(prefix.as_str()).map_err(|_| {
                    ParseError::NSResolveError(format!("invalid prefix \"{}\"", prefix))
                })?;
                Ok(NodeTest::Name(NameTest::Name(QName::new_from_parts(
                    lp,
                    Some(nsr(&prefix)?),
                ))))
            } else {
                Err(ParseError::NSResolveError(String::from(
                    "no namespace resolver",
                )))
            }
        },
    ))
}

// NodeTest ::= KindTest | NameTest
// NameTest ::= EQName | Wildcard
pub(crate) fn nodetest<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt2(kindtest(), nametest()))
}

// KindTest ::= DocumentTest | ElementTest | AttributeTest | SchemaElementTest | SchemaAttributeTest | PITest | CommentTest | TextTest | NamespaceNodeTest | AnyKindTest
pub(crate) fn kindtest<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
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
fn document_test<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    // TODO: ElementTest|SchemaElementTest
    Box::new(map(tag("document-node()"), |_| {
        NodeTest::Kind(KindTest::Document)
    }))
}
// ElementTest ::= "element" "(" (ElementNameOrWildcard ("," TypeName)?)? ")"
fn element_test<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    // TODO: ElementTest|SchemaElementTest
    Box::new(map(
        tuple3(
            tag("element("),
            opt(map(
                alt2(map(qualname_to_qname(), |_| ()), map(tag("*"), |_| ())),
                |_| (),
            )),
            tag(")"),
        ),
        |_| NodeTest::Kind(KindTest::Element),
    ))
}
// SchemaElementTest ::= "schema-element" "(" ElementNameDeclaration ")"
fn schema_element_test<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    // TODO: ElementTest|SchemaElementTest
    Box::new(map(
        tuple3(tag("schema-element("), qualname_to_qname(), tag(")")),
        |_| NodeTest::Kind(KindTest::SchemaElement),
    ))
}
// AttributeTest ::= "attribute" "(" (AttribNameOrWildcard ("," TypeName))? ")"
fn attribute_test<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(
        tuple3(
            tag("attribute("),
            opt(map(
                alt2(map(qualname_to_qname(), |_| ()), map(tag("*"), |_| ())),
                |_| (),
            )),
            tag(")"),
        ),
        |_| NodeTest::Kind(KindTest::Attribute),
    ))
}
// SchemaAttributeTest ::= "attribute" "(" AttributeDeclaration ")"
fn schema_attribute_test<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    // TODO: AttributeDeclaration
    Box::new(map(
        tuple3(tag("schema-attribute("), qualname_to_qname(), tag(")")),
        |_| NodeTest::Kind(KindTest::SchemaAttribute),
    ))
}
// PITest ::= "processing-instruction" "(" (NCName | StringLiteral)? ")"
fn pi_test<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    // TODO: NCName | StringLiteral
    Box::new(map(tag("processing-instruction()"), |_| {
        NodeTest::Kind(KindTest::PI)
    }))
}
// CommentTest ::= "comment" "(" ")"
fn comment_test<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(tag("comment()"), |_| NodeTest::Kind(KindTest::Comment)))
}
// TextTest ::= "text" "(" ")"
fn text_test<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(tag("text()"), |_| NodeTest::Kind(KindTest::Text)))
}
// NamespaceNodeTest ::= "namespace-node" "(" ")"
fn namespace_node_test<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(tag("namespace-node()"), |_| {
        NodeTest::Kind(KindTest::Namespace)
    }))
}
// NamespaceNodeTest ::= "namespace-node" "(" ")"
fn any_kind_test<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(tag("node()"), |_| NodeTest::Kind(KindTest::Any)))
}

// NameTest ::= EQName | Wildcard
// TODO: allow EQName rather than QName
fn nametest<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt2(qualname_test(), wildcard()))
}

// Wildcard ::= '*' | (NCName ':*') | ('*:' NCName) | (BracedURILiteral '*')
// TODO: more specific wildcards
fn wildcard<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, NodeTest), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(tag("*"), |_| {
        NodeTest::Name(NameTest::Wildcard(
            WildcardOrNamespaceUri::Wildcard,
            WildcardOrName::Wildcard,
        ))
    }))
}
