mod attribute;
mod chardata;
mod dtd;
mod element;
mod misc;
pub mod qname;
mod reference;
mod strings;
mod xmldecl;

use crate::trees::intmuttree::{DocumentBuilder, ExtDTDresolver, RNode, XMLDecl};
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tuple::{tuple3, tuple4};
use crate::parser::xml::dtd::doctypedecl;
use crate::parser::xml::element::element;
use crate::parser::xml::misc::misc;
use crate::parser::xml::xmldecl::xmldecl;
use crate::parser::{ParseError, ParseInput, ParseResult, ParserState};
use crate::{xdmerror, Document};

// For backward compatibility
pub type XMLDocument = Document;

pub fn parse(
    input: &str,
    entityresolver: Option<ExtDTDresolver>,
    docloc: Option<String>,
) -> Result<XMLDocument, xdmerror::Error> {
    let state = ParserState::new(entityresolver, docloc);
    match document((input, state)) {
        Ok((_, xmldoc)) => Result::Ok(xmldoc),
        Err(err) => {
            match err {
                ParseError::Combinator => Result::Err(xdmerror::Error::new(
                    xdmerror::ErrorKind::ParseError,
                    "Unrecoverable parser error.".to_string(),
                )),
                /*
                ParseError::InvalidChar { row, col } => {
                    Result::Err(Error {
                        kind: ErrorKind::ParseError,
                        message: "Invalid character in document.".to_string(),
                    })
                }
                 */
                ParseError::MissingGenEntity { .. } => Result::Err(xdmerror::Error::new(
                    xdmerror::ErrorKind::ParseError,
                    "Missing Gen Entity.".to_string(),
                )),
                ParseError::MissingParamEntity { .. } => Result::Err(xdmerror::Error::new(
                    xdmerror::ErrorKind::ParseError,
                    "Missing Param Entity.".to_string(),
                )),
                ParseError::EntityDepth { .. } => Result::Err(xdmerror::Error::new(
                    xdmerror::ErrorKind::ParseError,
                    "Entity depth limit exceeded".to_string(),
                )),
                ParseError::Validation { .. } => Result::Err(xdmerror::Error::new(
                    xdmerror::ErrorKind::ParseError,
                    "Validation error.".to_string(),
                )),
                ParseError::MissingNameSpace => Result::Err(xdmerror::Error::new(
                    xdmerror::ErrorKind::ParseError,
                    "Missing namespace declaration.".to_string(),
                )),
                ParseError::NotWellFormed => Result::Err(xdmerror::Error::new(
                    xdmerror::ErrorKind::ParseError,
                    "XML document not well formed.".to_string(),
                )),
                ParseError::ExtDTDLoadError => Result::Err(xdmerror::Error::new(
                    xdmerror::ErrorKind::ParseError,
                    "Unable to open external DTD.".to_string(),
                )),
                ParseError::Notimplemented => Result::Err(xdmerror::Error::new(
                    xdmerror::ErrorKind::ParseError,
                    "Unimplemented feature.".to_string(),
                )),
                _ => Result::Err(xdmerror::Error::new(
                    xdmerror::ErrorKind::Unknown,
                    "Unknown error.".to_string(),
                )),
            }
        }
    }
}

fn document(input: ParseInput) -> ParseResult<XMLDocument> {
    match tuple3(opt(prolog()), element(), opt(misc()))(input) {
        Err(err) => Err(err),
        Ok(((input1, state1), (p, e, m))) => {
            //Check nothing remaining in iterator, nothing after the end of the root node.
            if input1.is_empty() {
                let pr = p.unwrap_or((None, vec![]));

                let mut a = DocumentBuilder::new()
                    .prologue(pr.1)
                    .content(vec![e])
                    .epilogue(m.unwrap_or_default())
                    .build();
                if let Some(x) = pr.0 {
                    a.set_xmldecl(x)
                };
                Ok(((input1, state1), a))
            } else {
                Err(ParseError::NotWellFormed)
            }
        }
    }
}

// prolog ::= XMLDecl misc* (doctypedecl Misc*)?
fn prolog() -> impl Fn(ParseInput) -> ParseResult<(Option<XMLDecl>, Vec<RNode>)> {
    map(
        tuple4(opt(xmldecl()), misc(), opt(doctypedecl()), misc()),
        |(xmld, mut m1, _dtd, mut m2)| {
            m1.append(&mut m2);
            (xmld, m1)
        },
    )
}
