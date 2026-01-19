use crate::item::Node;
use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::many::many1;
use crate::parser::combinators::map::{map, map_with_state_and_result};
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::support::none_of;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_one, take_while};
use crate::parser::combinators::tuple::{tuple2, tuple3};
//use crate::parser::combinators::debug::inspect;
use crate::parser::combinators::wellformed::wellformed;
use crate::parser::common::{is_namechar, is_namestartchar, is_ncnamechar, is_ncnamestartchar};
use crate::parser::xml::dtd::pereference::petextreference;
use crate::parser::{ParseError, ParseInput, StaticState};
use qualname::{NamespacePrefix, NamespaceUri, NcName, QName};

// QualifiedName, returning a QName
pub(crate) fn qualname_to_qname<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, QName), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    alt2(prefixed_name_to_qname(), unprefixed_name_to_qname())
}

// QualifiedName, returning the pieces: (prefix, local-part)
// NB. Cannot use NamespacePrefix or NcName since values can legitimately be invalid (empty string). E.g. xmlns=""
pub(crate) fn qualname_to_parts<'a, N: Node, L>() -> impl Fn(
    ParseInput<'a, N>,
    &mut StaticState<L>,
) -> Result<
    (ParseInput<'a, N>, (Option<String>, String)),
    ParseError,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    alt2(prefixed_name_to_parts(), unprefixed_name_to_parts())
}

// Expanded Qualified Name
// EQName ::= QName | URIQualifiedName
#[allow(dead_code)]
pub(crate) fn eqname_to_qname<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, QName), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    alt2(uriqualname(), qualname_to_qname())
}
// URIQualifiedName ::= "Q" "{" [^{}]* "}" NCName
#[allow(dead_code)]
pub(crate) fn uriqualname<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, QName), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map(
        pair(
            tuple3(
                tag("Q{"),
                map(many1(none_of("{}")), |v| v.iter().collect::<String>()),
                tag("}"),
            ),
            ncname(),
        ),
        |((_, uri, _), localpart)| {
            QName::new_from_parts(
                NcName::try_from(localpart.as_str()).expect("not a valid NcName"),
                Some(NamespaceUri::try_from(uri.as_str()).expect("not a valid XML Namespace URI")),
            )
        },
    )
}

fn unprefixed_name_to_qname<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, QName), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map(alt2(petextreference(), ncname()), |localpart| {
        QName::from_local_name(NcName::try_from(localpart.as_str()).expect("not a valid QName")) // TODO: should be able to use unchecked creation, since we've already parsed the NCName
    })
}
fn unprefixed_name_to_parts<'a, N: Node, L>() -> impl Fn(
    ParseInput<'a, N>,
    &mut StaticState<L>,
) -> Result<
    (ParseInput<'a, N>, (Option<String>, String)),
    ParseError,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map(alt2(petextreference(), ncname()), |localpart| {
        (None, localpart)
    })
}
fn prefixed_name_to_parts<'a, N: Node, L>() -> impl Fn(
    ParseInput<'a, N>,
    &mut StaticState<L>,
) -> Result<
    (ParseInput<'a, N>, (Option<String>, String)),
    ParseError,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map(
        tuple3(
            alt2(petextreference(), ncname()),
            tag(":"),
            alt2(petextreference(), ncname()),
        ),
        |(prefix, _, localpart)| (Some(prefix), localpart),
    )
}
fn prefixed_name_to_qname<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, QName), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map_with_state_and_result(
        tuple3(
            alt2(petextreference::<N, L>(), ncname()),
            tag(":"),
            alt2(petextreference::<N, L>(), ncname()),
        ),
        |(prefix, _, localpart), _state, ss| {
            if let Some(f) = &mut ss.namespace {
                if let Ok(uri) = f(&NamespacePrefix::try_from(prefix.as_str()).unwrap()) {
                    if let Ok(lp) = NcName::try_from(localpart.as_str()) {
                        Ok(QName::new_from_parts(
                            lp,
                            //Some(NamespaceUri::try_from(uri).expect("not a valid Namespace URI")),
                            Some(uri),
                        ))
                    } else {
                        Err(ParseError::NSResolveError(format!(
                            "name \"{}\" is not valid",
                            localpart
                        )))
                    }
                } else {
                    Err(ParseError::NSResolveError(format!(
                        "namespace resolver failed on prefix \"{}\"",
                        prefix
                    )))
                }
            } else {
                // No closure to resolve prefix
                Err(ParseError::NSResolveError(String::from(
                    "no closure to resolve prefix",
                )))
            }
        },
    )
}

// NCName ::= Name - (Char* ':' Char*)
// Name ::= NameStartChar NameChar*
// NameStartChar ::= ':' | [A-Z] | '_' | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
// NameChar ::= NameStartChar | '-' | '.' | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]
pub(crate) fn ncname<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map(
        tuple2(
            wellformed(
                take_one(),
                is_ncnamestartchar,
                "invalid character in NcName",
            ),
            opt(take_while(|c| is_ncnamechar(&c))),
        ),
        |(a, b)| [a.to_string(), b.unwrap_or_default()].concat(),
    )
}

/// XML 1.0 (Fifth Edition) [5] Name ::= NameStartChar ( NameChar )*
pub(crate) fn name<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, String), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map(
        tuple2(
            wellformed(take_one(), is_namestartchar, "invalid name start character"),
            opt(take_while(|c| is_namechar(&c))),
        ),
        |(nsc, nc)| match nc {
            None => nsc.to_string(),
            Some(nc) => [nsc.to_string(), nc].concat(),
        },
    )
}
