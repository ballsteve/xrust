use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::take::{take_while, take_while_m_n};
use crate::parser::combinators::tuple::tuple2;
use crate::parser::{ParseInput, ParseResult};

// NCName ::= Name - (Char* ':' Char*)
// Name ::= NameStartChar NameChar*
// NameStartChar ::= ':' | [A-Z] | '_' | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
// NameChar ::= NameStartChar | '-' | '.' | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]
pub(crate) fn ncname<'a>() -> impl Fn(ParseInput) -> ParseResult<String> + 'a {
    //move |input, index|
    map(
        tuple2(
            take_while_m_n(1, 1, |c| is_ncnamestartchar(&c)),
            opt(take_while(|c| is_ncnamechar(&c))),
        ),
        |(a, b)| [a, b.unwrap_or_default()].concat(),
    )
    //(input, index)
}

pub(crate) fn name() -> impl Fn(ParseInput) -> ParseResult<String> {
    //move |input, index|
    map(
        tuple2(
            take_while_m_n(1, 1, |c| is_namestartchar(&c)),
            opt(take_while(|c| is_namechar(&c))),
        ),
        |(nsc, nc)| match nc {
            None => nsc,
            Some(nc) => [nsc, nc].concat(),
        },
    )
    //(input, index)
}

pub(crate) fn is_namechar(ch: &char) -> bool {
    if is_namestartchar(ch) {
        true
    } else {
        matches!(ch,
            '.'
            | '-'
            | '0'..='9'
            | '\u{B7}'
            | '\u{0300}'..='\u{036F}'
            | '\u{203F}'..='\u{2040}'
        )
    }
}

fn is_ncnamechar(ch: &char) -> bool {
    if is_ncnamestartchar(ch) {
        true
    } else {
        matches!(ch,
            '.'
            | '-'
            | '0'..='9'
            | '\u{B7}'
            | '\u{0300}'..='\u{036F}'
            | '\u{203F}'..='\u{2040}'
        )
    }
}

fn is_namestartchar(ch: &char) -> bool {
    match ch {
        ':' => true,
        _ => is_ncnamestartchar(ch),
    }
}
fn is_ncnamestartchar(ch: &char) -> bool {
    matches!(ch,
        '\u{0041}'..='\u{005A}' // A-Z
        | '\u{005F}' // _
        | '\u{0061}'..='\u{007A}' // a-z
        | '\u{00C0}'..='\u{00D6}' //  [#xC0-#xD6]
        | '\u{00D8}'..='\u{00F6}' //  [#xD8-#xF6]
        | '\u{00F8}'..='\u{02FF}' //  [#xF8-#x2FF]
        | '\u{0370}'..='\u{037D}' //  [#x370-#x37D]
        | '\u{037F}'..='\u{1FFF}' //  [#x37F-#x1FFF]
        | '\u{200C}'..='\u{200D}' //  [#x200C-#x200D]
        | '\u{2070}'..='\u{218F}' //  [#x2070-#x218F]
        | '\u{2C00}'..='\u{2FEF}' //  [#x2C00-#x2FEF]
        | '\u{3001}'..='\u{D7FF}' //  [#x3001-#xD7FF]
        | '\u{F900}'..='\u{FDCF}' //  [#xF900-#xFDCF]
        | '\u{FDF0}'..='\u{FFFD}' //  [#xFDF0-#xFFFD]
        | '\u{10000}'..='\u{EFFFF}' //  [#x10000-#xEFFFF]
    )
}

pub fn is_char(ch: &char) -> bool {
    matches!(ch,
        '\u{0009}' // #x9
        | '\u{000A}' // #xA
        | '\u{000D}' // #xD
        | '\u{0020}'..='\u{D7FF}' //  [#x0020-#xD7FF]
        | '\u{E000}'..='\u{FFFD}' //  [#xE000-#xFFFD]
        | '\u{10000}'..='\u{10FFFF}' //  [#x10000-#10FFFF]
    )
}

pub(crate) fn is_pubid_charwithapos(ch: &char) -> bool {
    match ch {
        '\'' => true,
        _ => is_pubid_char(ch),
    }
}

pub(crate) fn is_pubid_char(ch: &char) -> bool {
    matches!(ch,
        '\u{0020}' // #x0020
        | '\u{000A}' // #xA
        | '\u{000D}' // #xD
        | 'a'..='z'
        | 'A'..='Z'
        | '0'..='9'
        | '-'
        | '('
        | ')'
        | '+'
        | ','
        | '.'
        | '/'
        | ':'
        | '='
        | '?'
        | ';'
        | '!'
        | '*'
        | '#'
        | '@'
        | '$'
        | '_'
        | '%'
    )
}
