mod attribute;
mod chardata;
mod dtd;
mod element;
mod misc;
pub mod qname;
mod reference;
mod strings;
mod xmldecl;

use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tuple::{tuple3, tuple4};
use crate::parser::xml::dtd::doctypedecl;
use crate::parser::xml::element::element;
use crate::parser::xml::misc::misc;
use crate::parser::xml::xmldecl::xmldecl;
use crate::parser::{ParseError, ParseInput, ParserState};
use crate::xdmerror::{Error, ErrorKind};
use crate::externals::URLResolver;
use crate::item::Node;
use crate::xmldecl::XMLDecl;

// For backward compatibility
//pub type XMLDocument = Document;

pub fn parse<N: Node>(
    doc: N,
    input: &str,
    entityresolver: Option<URLResolver>,
    docloc: Option<String>,
) -> Result<N, Error> {
    let state = ParserState::new(Some(doc), entityresolver, docloc);
    match document((input, state)) {
        Ok((_, xmldoc)) => Result::Ok(xmldoc),
        Err(err) => {
            match err {
                ParseError::Combinator => Err(Error::new(
                    ErrorKind::ParseError,
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
                ParseError::MissingGenEntity { .. } => Err(Error::new(
                    ErrorKind::ParseError,
                    "Missing Gen Entity.".to_string(),
                )),
                ParseError::MissingParamEntity { .. } => Err(Error::new(
                    ErrorKind::ParseError,
                    "Missing Param Entity.".to_string(),
                )),
                ParseError::EntityDepth { .. } => Err(Error::new(
                    ErrorKind::ParseError,
                    "Entity depth limit exceeded".to_string(),
                )),
                ParseError::Validation { .. } => Err(Error::new(
                    ErrorKind::ParseError,
                    "Validation error.".to_string(),
                )),
                ParseError::MissingNameSpace => Err(Error::new(
                    ErrorKind::ParseError,
                    "Missing namespace declaration.".to_string(),
                )),
                ParseError::NotWellFormed => Err(Error::new(
                    ErrorKind::ParseError,
                    "XML document not well formed.".to_string(),
                )),
                ParseError::ExtDTDLoadError => Err(Error::new(
                    ErrorKind::ParseError,
                    "Unable to open external DTD.".to_string(),
                )),
                ParseError::Notimplemented => Err(Error::new(
                    ErrorKind::ParseError,
                    "Unimplemented feature.".to_string(),
                )),
                _ => Err(Error::new(
                    ErrorKind::Unknown,
                    "Unknown error.".to_string(),
                )),
            }
        }
    }
}

fn document<N: Node>(input: ParseInput<N>) -> Result<(ParseInput<N>, N), ParseError> {
    match tuple3(opt(prolog()), element(), opt(misc()))(input) {
        Err(err) => Err(err),
        Ok(((input1, state1), (p, e, m))) => {
            //Check nothing remaining in iterator, nothing after the end of the root node.
            if input1.is_empty() {
                let pr = p.unwrap_or((None, vec![]));

                pr.1.iter().for_each(|n| state1.doc.clone().unwrap().push(n.clone()).expect("unable to add node"));
                state1.doc.clone().unwrap().push(e).expect("unable to add node");
                m.unwrap_or_default().iter().for_each(|n| state1.doc.clone().unwrap().push(n.clone()).expect("unable to add node"));
                if let Some(x) = pr.0 {
                    let _ = state1.doc.clone().unwrap().set_xmldecl(x);
                }
                Ok(((input1, state1.clone()), state1.doc.clone().unwrap().clone()))
            } else {
                Err(ParseError::NotWellFormed)
            }
        }
    }
}

// prolog ::= XMLDecl misc* (doctypedecl Misc*)?
fn prolog<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, (Option<XMLDecl>, Vec<N>)), ParseError> {
    map(
        tuple4(opt(xmldecl()), misc(), opt(doctypedecl()), misc()),
        |(xmld, mut m1, _dtd, mut m2)| {
            m1.append(&mut m2);
            (xmld, m1)
        },
    )
}
