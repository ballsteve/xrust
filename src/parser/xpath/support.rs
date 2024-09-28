//! Supporting functions.

use crate::item::Node;
use crate::parser::{ParseError, ParseInput};
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

pub(crate) fn noop<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> {
    move |_| Err(ParseError::Combinator)
}
