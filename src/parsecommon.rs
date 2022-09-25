//! # xrust::parsecommon
//!
//! Common definitions for XML and XPath parsers

extern crate nom;
use nom::{
    bytes::complete::{take_while, take_while1, take_while_m_n},
    combinator::recognize,
    sequence::pair,
    IResult,
};
// NCName ::= Name - (Char* ':' Char*)
// Name ::= NameStartChar NameChar*
// NameStartChar ::= ':' | [A-Z] | '_' | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
// NameChar ::= NameStartChar | '-' | '.' | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]
pub fn ncname(input: &str) -> IResult<&str, &str> {
    recognize(pair(ncnamestartchar, take_while(is_ncnamechar)))(input)
}
pub fn name(input: &str) -> IResult<&str, &str> {
    recognize(pair(namestartchar, take_while1(is_namechar)))(input)
}
fn is_namechar(ch: char) -> bool {
    if is_namestartchar(ch) {
        true
    } else {
        match ch {
            '.' => true,
            '-' => true,
            '0'..='9' => true,
            '\u{B7}' => true,
            '\u{0300}'..='\u{036F}' => true,
            '\u{203F}'..='\u{2040}' => true,
            _ => false,
        }
    }
}
#[cfg(test)]
fn ncnamechar(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 1, is_ncnamechar)(input)
}
fn is_ncnamechar(ch: char) -> bool {
    if is_ncnamestartchar(ch) {
        true
    } else {
        match ch {
            '.'
            | '-'
            | '0'..='9'
            | '\u{B7}'
            | '\u{0300}'..='\u{036F}'
            | '\u{203F}'..='\u{2040}' => true,
            _ => false,
        }
    }
}
//fn namestartchar(input: &str) -> IResult<&str, char> {
//  alt((
//    one_of(":_"),
//    one_of(('A'..='Z').map(char::from).collect::<Vec<_>>()),
//    one_of(('a'..='z').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{C0}'..='\u{D6}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{D8}'..='\u{F6}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{F8}'..='\u{2FF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{370}'..='\u{37D}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{37F}'..='\u{1FFF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{200C}'..='\u{200D}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{2070}'..='\u{218F}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{2C00}'..='\u{2FEF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{3001}'..='\u{D7FF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{F900}'..='\u{FDCF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{FDF0}'..='\u{FFFD}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{10000}'..='\u{EFFFF}').map(char::from).collect::<Vec<_>>()),
//  ))
//  (input)
//}
fn namestartchar(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 1, is_namestartchar)(input)
}
fn is_namestartchar(ch: char) -> bool {
    match ch {
        ':' => true,
        _ => is_ncnamestartchar(ch),
    }
}
// Same as above, but without the colon
//fn ncnamestartchar(input: &str) -> IResult<&str, char> {
//  alt((
//    one_of("_"),
//    one_of(('A'..='Z').map(char::from).collect::<Vec<_>>()),
//    one_of(('a'..='z').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{C0}'..='\u{D6}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{D8}'..='\u{F6}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{F8}'..='\u{2FF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{370}'..='\u{37D}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{37F}'..='\u{1FFF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{200C}'..='\u{200D}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{2070}'..='\u{218F}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{2C00}'..='\u{2FEF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{3001}'..='\u{D7FF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{F900}'..='\u{FDCF}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{FDF0}'..='\u{FFFD}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{10000}'..='\u{EFFFF}').map(char::from).collect::<Vec<_>>()),
//  ))
//  (input)
//}
fn ncnamestartchar(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 1, is_ncnamestartchar)(input)
}
fn is_ncnamestartchar(ch: char) -> bool {
    match ch {
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
    => {
      true
    },
    // etc
    _ => false
  }
}

pub fn is_char(ch: &char) -> bool {
    match ch {
    '\u{0009}' // #x9
    | '\u{000A}' // #xA
    | '\u{000D}' // #xD
    | '\u{0020}'..='\u{D7FF}' //  [#x0020-#xD7FF]
    | '\u{E000}'..='\u{FFFD}' //  [#xE000-#xFFFD]
    | '\u{10000}'..='\u{10FFFF}' //  [#x10000-#10FFFF]
    => {
      true
    },
    // etc
    _ => false
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(name("Foo"), Ok(("", "Foo")))
    }
    #[test]
    fn test_ncnamechar() {
        assert_eq!(ncnamechar("F"), Ok(("", "F")))
    }
}
