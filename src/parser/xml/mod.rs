mod attribute;
mod chardata;
mod dtd;
mod element;
mod misc;
pub mod qname;
mod reference;
mod strings;
mod xmldecl;

use crate::item::Node;
use crate::namespace::NamespaceMap;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple4;
use crate::parser::xml::dtd::doctypedecl;
use crate::parser::xml::element::element;
use crate::parser::xml::misc::misc;
use crate::parser::xml::xmldecl::xmldecl;
use crate::parser::{ParseError, ParseInput, ParserConfig, ParserState};
use crate::qname::Interner;
use crate::xdmerror::{Error, ErrorKind};
use crate::xmldecl::XMLDecl;
use std::rc::Rc;

pub fn parse<'i, I: Interner, N: Node>(
    doc: N,
    input: &str,
    config: Option<ParserConfig>,
    interner: &'i I,
) -> Result<N, Error> {
    let (xmldoc, _) = parse_with_ns(doc, input, config, interner)?;
    Ok(xmldoc)
}

pub fn parse_with_ns<'i, I: Interner, N: Node>(
    doc: N,
    input: &str,
    config: Option<ParserConfig>,
    interner: &'i I,
) -> Result<(N, Rc<NamespaceMap>), Error> {
    let state = ParserState::new(Some(doc), None, config, interner);
    match document((input, state)) {
        Ok(((_, state1), xmldoc)) => Ok((xmldoc, state1.namespace.clone())),
        Err(err) => {
            match err {
                ParseError::Combinator => Err(Error::new(
                    ErrorKind::ParseError,
                    format!(
                        "Unrecoverable parser error while parsing XML \"{}\"",
                        input.chars().take(80).collect::<String>()
                    ),
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
                ParseError::NotWellFormed(s) => Err(Error::new(
                    ErrorKind::ParseError,
                    format!("XML document not well formed at \"{}\".", s),
                )),
                ParseError::ExtDTDLoadError => Err(Error::new(
                    ErrorKind::ParseError,
                    "Unable to open external DTD.".to_string(),
                )),
                ParseError::Notimplemented => Err(Error::new(
                    ErrorKind::ParseError,
                    "Unimplemented feature.".to_string(),
                )),
                _ => Err(Error::new(ErrorKind::Unknown, "Unknown error.".to_string())),
            }
        }
    }
}

fn document<'a, 'i, I: Interner, N: Node>(
    input: ParseInput<'a, 'i, I, N>,
) -> Result<(ParseInput<'a, 'i, I, N>, N), ParseError> {
    match tuple4(opt(utf8bom()), opt(prolog()), element(), opt(misc()))(input) {
        Err(err) => Err(err),
        Ok(((input1, state1), (_, p, e, m))) => {
            //Check nothing remaining in iterator, nothing after the end of the root node.
            if input1.is_empty() {
                /*
                   We were checking XML IDRefs as we parsed, but sometimes an ID comes after the IDREF,
                   we now check those cases to ensure that all IDs needed were reported.
                */
                if state1.id_tracking {
                    for idref in state1.ids_pending.iter() {
                        match state1.ids_read.get(idref) {
                            None => return Err(ParseError::IDError(String::from("ID missing"))),
                            Some(_) => {}
                        }
                    }
                }

                let pr = p.unwrap_or((None, vec![]));

                let mut d = state1.doc.clone().unwrap();

                pr.1.iter()
                    .for_each(|n| d.push(n.clone()).expect("unable to add node"));
                d.push(e).expect("unable to add node");
                m.unwrap_or_default()
                    .iter()
                    .for_each(|n| d.push(n.clone()).expect("unable to add node"));
                if let Some(x) = pr.0 {
                    let _ = d.set_xmldecl(x);
                }

                if !state1.dtd.patterns.is_empty() {
                    let _ = d.set_dtd(state1.dtd.clone());
                };

                Ok((
                    (input1, state1.clone()),
                    state1.doc.clone().unwrap().clone(),
                ))
            } else {
                Err(ParseError::NotWellFormed(format!(
                    "unexpected extra characters: \"{}\"",
                    input1
                )))
            }
        }
    }
}

// prolog ::= XMLDecl misc* (doctypedecl Misc*)?
fn prolog<'a, 'i, I: Interner + 'i, N: Node>() -> impl Fn(
    ParseInput<'a, 'i, I, N>,
) -> Result<
    (ParseInput<'a, 'i, I, N>, (Option<XMLDecl>, Vec<N>)),
    ParseError,
> {
    map(
        tuple4(opt(xmldecl()), misc(), opt(doctypedecl()), misc()),
        |(xmld, mut m1, _dtd, mut m2)| {
            m1.append(&mut m2);
            (xmld, m1)
        },
    )
}

fn utf8bom<'a, 'i, I: Interner, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, ()), ParseError> {
    tag("\u{feff}")
}
