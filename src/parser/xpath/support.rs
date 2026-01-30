//! Supporting functions.

use crate::item::Node;
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::transform::{NameTest, NodeTest, Transform};
use qualname::{NamespacePrefix, NamespaceUri};

pub(crate) fn get_nt_localname(nt: &NodeTest) -> String {
    match nt {
        NodeTest::Name(NameTest::Name(qn)) => qn.local_name().to_string(),
        _ => String::from("invalid qname"),
    }
}

pub(crate) fn noop<'a, N: Node, L>() -> impl Fn(
    ParseInput<'a, N>,
    &mut StaticState<L>,
) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |_, _| Err(ParseError::Combinator(String::from("noop - xpath")))
}
