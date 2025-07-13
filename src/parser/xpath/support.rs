//! Supporting functions.

use crate::item::Node;
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;
use crate::transform::{NameTest, NodeTest, Transform, WildcardOrName};

pub(crate) fn get_nt_localname(nt: &NodeTest) -> String {
    match nt {
        NodeTest::Name(NameTest {
            name: Some(WildcardOrName::Name(localpart)),
            ns: None,
            prefix: None,
        }) => localpart.to_string(),
        _ => String::from("invalid qname"),
    }
}

pub(crate) fn noop<'a, 'i, I: Interner, N: Node>() -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<
    (ParseInput<'a, 'i, I, N>, Transform<'i, I, N>),
    ParseError,
> {
    move |_| Err(ParseError::Combinator)
}
