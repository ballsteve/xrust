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
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple4;
use crate::parser::xml::dtd::doctypedecl;
use crate::parser::xml::element::element;
use crate::parser::xml::misc::misc;
use crate::parser::xml::xmldecl::xmldecl;
use crate::parser::{
    ParseError, ParseInput, ParserState, ParserStateBuilder, StaticState, StaticStateBuilder,
};
use crate::xdmerror::{Error, ErrorKind};
use crate::xmldecl::XMLDecl;
use qualname::{NamespaceMap, NamespacePrefix, NamespaceUri};

pub fn parse_with_state<N: Node, L>(
    input: &str,
    ps: ParserState<N>,
    mut ss: StaticState<L>,
) -> Result<N, Error>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    match document((input, ps), &mut ss) {
        Ok(((_, _), xmldoc)) => Ok(xmldoc),
        Err(err) => {
            match err {
                ParseError::Combinator(f) => Err(Error::new(
                    ErrorKind::ParseError,
                    format!(
                        "Unrecoverable parser error ({}) while parsing XML \"{}\"",
                        f,
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

// TODO: remove Option<L> argument
pub fn parse<L, N: Node>(doc: N, input: &str, r: Option<L>) -> Result<N, Error>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    let (xmldoc, _) = parse_with_ns(doc, input, r)?;
    Ok(xmldoc)
}

// TODO: Review need for this function.
// Is returning a NamespaceMap really necessary?
pub fn parse_with_ns<L, N: Node>(
    doc: N,
    input: &str,
    r: Option<L>,
) -> Result<(N, Option<NamespaceMap>), Error>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    let state = ParserStateBuilder::new().doc(doc).build();
    let static_state = r.map_or(StaticState::new(), |f| {
        StaticStateBuilder::new().namespace(f).build()
    });
    Ok((parse_with_state(input, state, static_state)?, None))
}

fn document<'a, N: Node, L>(
    input: ParseInput<'a, N>,
    ss: &mut StaticState<L>,
) -> Result<(ParseInput<'a, N>, N), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    match tuple4(
        opt(utf8bom()),
        opt(prolog()),
        element(),
        opt(misc()),
        "document",
    )(input, ss)
    {
        Err(err) => Err(err),
        Ok(((input1, state1), (_, p, e, m))) => {
            //Check nothing remaining in iterator, nothing after the end of the root node.
            if input1.is_empty() {
                /*
                   We were checking XML IDRefs as we parsed, but sometimes an ID comes after the IDREF,
                   we now check those cases to ensure that all IDs needed were reported.
                */
                if state1.id_tracking {
                    for idref in ss.ids_pending.iter() {
                        if ss.ids_read.get(idref).is_none() {
                            return Err(ParseError::IDError(String::from("ID missing")))
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
fn prolog<'a, N: Node, L>() -> impl Fn(
    ParseInput<'a, N>,
    &mut StaticState<L>,
) -> Result<
    (ParseInput<'a, N>, (Option<XMLDecl>, Vec<N>)),
    ParseError,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    map(
        tuple4(opt(xmldecl()), misc(), opt(doctypedecl()), misc(), "prolog"),
        |(xmld, mut m1, _dtd, mut m2)| {
            m1.append(&mut m2);
            (xmld, m1)
        },
    )
}

fn utf8bom<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, ()), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    tag("\u{feff}")
}
