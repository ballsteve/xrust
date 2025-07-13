use crate::item::Node;
use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::many::many1;
use crate::parser::combinators::map::{map, map_with_state};
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
use crate::parser::{ParseError, ParseInput};
use crate::qname::{Interner, QualifiedName};

// QualifiedName
pub(crate) fn qualname<'a, 'i, I: Interner + 'i, N: Node>() -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<
    (ParseInput<'a, 'i, I, N>, QualifiedName<'i, I>),
    ParseError,
> {
    alt2(prefixed_name(), unprefixed_name())
}

// Expanded Qualified Name
// EQName ::= QName | URIQualifiedName
pub(crate) fn eqname<'a, 'i, I: Interner + 'i, N: Node>() -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<
    (ParseInput<'a, 'i, I, N>, QualifiedName<'i, I>),
    ParseError,
> {
    alt2(uriqualname(), qualname())
}
// URIQualifiedName ::= "Q" "{" [^{}]* "}" NCName
pub(crate) fn uriqualname<'a, 'i, I: Interner + 'i, N: Node>() -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<
    (ParseInput<'a, 'i, I, N>, QualifiedName<'i, I>),
    ParseError,
> {
    map_with_state(
        pair(
            tuple3(
                tag("Q{"),
                map(many1(none_of("{}")), |v| v.iter().collect()),
                tag("}"),
            ),
            ncname(),
        ),
        |((_, uri, _), localpart), state| {
            QualifiedName::new(localpart, Some(uri), None, state.interner)
        },
    )
}

fn unprefixed_name<'a, 'i, I: Interner + 'i, N: Node>() -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<
    (ParseInput<'a, 'i, I, N>, QualifiedName<'i, I>),
    ParseError,
> {
    map_with_state(alt2(petextreference(), ncname()), |localpart, state| {
        QualifiedName::new(localpart, None, None, state.interner)
    })
}
fn prefixed_name<'a, 'i, I: Interner + 'i, N: Node>() -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<
    (ParseInput<'a, 'i, I, N>, QualifiedName<'i, I>),
    ParseError,
> {
    map_with_state(
        tuple3(
            alt2(petextreference(), ncname()),
            tag(":"),
            alt2(petextreference(), ncname()),
        ),
        |(prefix, _, localpart), state| {
            QualifiedName::new(localpart, None, Some(prefix), state.interner)
        },
    )
}

// NCName ::= Name - (Char* ':' Char*)
// Name ::= NameStartChar NameChar*
// NameStartChar ::= ':' | [A-Z] | '_' | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
// NameChar ::= NameStartChar | '-' | '.' | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]
pub(crate) fn ncname<'a, 'i, I: Interner + 'i, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, String), ParseError> {
    map(
        tuple2(
            wellformed(take_one(), is_ncnamestartchar),
            opt(take_while(|c| is_ncnamechar(&c))),
        ),
        |(a, b)| [a.to_string(), b.unwrap_or_default()].concat(),
    )
}

pub(crate) fn name<'a, 'i, I: Interner + 'i, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, String), ParseError> {
    map(
        tuple2(
            wellformed(take_one(), is_namestartchar),
            opt(take_while(|c| is_namechar(&c))),
        ),
        |(nsc, nc)| match nc {
            None => nsc.to_string(),
            Some(nc) => [nsc.to_string(), nc].concat(),
        },
    )
}
