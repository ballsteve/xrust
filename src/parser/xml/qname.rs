use crate::parser::{ParseInput, ParseResult};
use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_one, take_while, take_while_m_n};
use crate::parser::combinators::tuple::{tuple2, tuple3};
use crate::parser::combinators::validate::validate;
use crate::parser::common::{is_namestartchar, is_namechar, is_ncnamestartchar, is_ncnamechar};
use crate::qname::QualifiedName;

// QualifiedName
pub(crate) fn qualname() -> impl Fn(ParseInput) -> ParseResult<QualifiedName> {
    alt2(prefixed_name(), unprefixed_name())
}
fn unprefixed_name() -> impl Fn(ParseInput) -> ParseResult<QualifiedName> {
    map(ncname(), |localpart| {
        QualifiedName::new(None, None, localpart)
    })
}
fn prefixed_name() -> impl Fn(ParseInput) -> ParseResult<QualifiedName> {
    map(
        tuple3(ncname(), tag(":"), ncname()),
        |(prefix, _, localpart)| QualifiedName::new(None, Some(prefix), localpart),
    )
}

// NCName ::= Name - (Char* ':' Char*)
// Name ::= NameStartChar NameChar*
// NameStartChar ::= ':' | [A-Z] | '_' | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
// NameChar ::= NameStartChar | '-' | '.' | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]
pub(crate) fn ncname<'a>() -> impl Fn(ParseInput) -> ParseResult<String> + 'a {
    map(
        tuple2(
            validate(take_one(), |c| is_ncnamestartchar(&c)),
            opt(take_while(|c| is_ncnamechar(&c))),
        ),
        |(a, b)| [a.to_string(), b.unwrap_or_default()].concat(),
    )
}

pub(crate) fn name() -> impl Fn(ParseInput) -> ParseResult<String> {
    map(
        tuple2(
            validate(take_one(), |c| is_namestartchar(&c)),
            opt(take_while(|c| is_namechar(&c))),
        ),
        |(nsc, nc)| match nc {
            None => nsc.to_string(),
            Some(nc) => [nsc.to_string(), nc].concat(),
        },
    )
}