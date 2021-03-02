//! # xdm::parsecommon
//!
//! Common definitions for XML and XPath parsers

extern crate nom;
use nom:: {
  IResult,
  sequence::{pair,},
  combinator::{map, recognize},
  bytes::complete::{take_while, take_while1, take_while_m_n},
};
// NCName ::= Name - (Char* ':' Char*)
// Name ::= NameStartChar NameChar*
// NameStartChar ::= ':' | [A-Z] | '_' | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
// NameChar ::= NameStartChar | '-' | '.' | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]
pub fn ncname(input: &str) -> IResult<&str, &str> {
  //println!("ncname: input=\"{}\"", input);
//  recognize (
  map (
    pair (
      ncnamestartchar,
      take_while(is_ncnamechar),
    ),
    |(a, _b)| {
      //println!("ncname: got \"{}\" and \"{}\"", a, b);
      a
    }
  )
  (input)
}
//fn ncname_old(input: &str) -> IResult<&str, &str> {
//  recognize (
//    pair (
//      ncnamestartchar,
//      many0(ncnamechar),
//    )
//  )
//  (input)
//}
//fn ncname_broken(input: &str) -> IResult<&str, String> {
//  map (
//    many1(none_of(":")),
//    |v| {
//      v.iter().collect::<String>()
//    }
//  )
//  (input)
//}
pub fn name(input: &str) -> IResult<&str, &str> {
  //println!("name: input=\"{}\"", input);
  recognize (
    pair (
      namestartchar,
      take_while1(is_namechar),
    )
  )
  (input)
}
//fn namechar(input: &str) -> IResult<&str, char> {
//  alt((
//    namestartchar,
//    one_of(".-0123456789\u{B7}"),
//    take_while1(is_namechar_range),
//  ))
//  (input)
//}
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
      _ => false
    }
  }
}
//fn ncnamechar(input: &str) -> IResult<&str, char> {
//  alt((
//    ncnamestartchar,
//    one_of(".-0123456789"),
//    one_of("\u{B7}"),
//    one_of(('\u{0300}'..='\u{036F}').map(char::from).collect::<Vec<_>>()),
//    one_of(('\u{203F}'..='\u{2040}').map(char::from).collect::<Vec<_>>()),
//  ))
//  (input)
//}
fn ncnamechar(input: &str) -> IResult<&str, &str> {
  take_while_m_n(1, 1, is_ncnamechar)
  (input)
}
fn is_ncnamechar(ch: char) -> bool {
  //println!("is_ncnamechar: input \"{}\"", ch);
  if is_ncnamestartchar(ch) {
    //println!("is_ncnamechar: input is a ncnamestartchar");
    true
  } else {
    match ch {
      '.' |
      '-' |
      '0'..='9' |
      '\u{B7}' |
      '\u{0300}'..='\u{036F}' |
      '\u{203F}'..='\u{2040}' => {
        println!("is_ncnamechar: true");
        true
      },
      _ => false
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
  take_while_m_n(1, 1, is_namestartchar)
  (input)
}
fn is_namestartchar(ch: char) -> bool {
  match ch {
    ':' => true,
    _ => is_ncnamestartchar(ch)
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
  //println!("ncnamestartchar: input \"{}\"", input);
  take_while_m_n(1, 1, is_ncnamestartchar)
  (input)
}
fn is_ncnamestartchar(ch: char) -> bool {
  //println!("is_ncnamestartchar: input \"{}\"", ch);
  match ch {
    '_' |
    'A'..='Z' |
    'a'..='z' |
    '\u{C0}'..='\u{D6}' => {
      //println!("is_ncnamestartchar: true");
      true
    },
    // etc
    _ => false
  }
}

