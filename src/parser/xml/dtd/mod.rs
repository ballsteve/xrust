mod attlistdecl;
mod conditionals;
mod elementdecl;
mod enumerated;
mod externalid;
pub(crate) mod extsubset;
mod gedecl;
mod intsubset;
mod misc;
mod notation;
mod pedecl;
pub(crate) mod pereference;
mod textdecl;

use crate::item::Node;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::tuple8;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::xml::dtd::externalid::externalid;
use crate::parser::xml::dtd::extsubset::extsubset;
use crate::parser::xml::dtd::intsubset::intsubset;
use crate::parser::xml::qname::name;
use crate::parser::xml::reference::reference;
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::xmldecl::{AttType, DTDPattern, DefaultDecl};
use qualname::{NamespacePrefix, NamespaceUri};

#[derive(Clone)]
pub(crate) enum Occurances {
    ZeroOrMore,
    OneOrMore,
    One,
    ZeroOrOne,
}

pub(crate) fn doctypedecl<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, ()), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |input, ss| match tuple8(
        tag("<!DOCTYPE"),
        whitespace1(),
        name(),
        whitespace1(),
        opt(externalid()),
        whitespace0(),
        opt(delimited(tag("["), intsubset(), tag("]"))),
        tag(">"),
    )(input, ss)
    {
        Ok(((input1, mut state1), (_, _, n, _, _, _, _inss, _))) => {
            eprintln!("doctypedecl");
            if n.contains(':') {
                let mut nameparts = n.split(':');
                let prefix = nameparts.next().unwrap();
                let local_part = nameparts.next().unwrap();
                state1.dtd.name = Some((Some(String::from(prefix)), String::from(local_part)));
            } else {
                state1.dtd.name = Some((None, n));
            }
            /*  We're doing nothing with the below, just evaluating the external entity to check its well formed */
            let exdtd = state1.ext_entities_to_parse.clone().pop();
            match exdtd {
                None => {}
                Some(s) => match ss.resolve(state1.docloc.clone(), s) {
                    Err(_) => return Err(ParseError::ExtDTDLoadError),
                    Ok(s) => {
                        if let Err(e) = extsubset()((s.as_str(), state1.clone()), ss) {
                            return Err(e);
                        }
                    }
                },
            }
            /*
            Same again, with Internal subset */
            for (k, (v, _)) in state1.clone().dtd.generalentities {
                if v != *"<" {
                    /* A single < on its own will generate an error if used, but doesn't actually generate a not well formed error! */
                    let i = ["&".to_string(), k, ";".to_string()].join("");
                    if let Err(ParseError::NotWellFormed(w)) =
                        reference()((i.as_str(), state1.clone()), ss)
                    {
                        return Err(ParseError::NotWellFormed(w));
                    }
                }
            }

            for (elname, eldecl) in &state1.dtd.elements {
                match &state1.dtd.attlists.get(elname) {
                    None => {
                        state1.dtd.patterns.insert(
                            elname.clone(),
                            DTDPattern::Element(elname.clone(), Box::new(eldecl.clone())),
                        );
                    }
                    Some(attlist) => {
                        let mut attpat = None;
                        for (attname, (at, dd, _)) in attlist.iter() {
                            let mut ap = match at {
                                AttType::CDATA => DTDPattern::Text,
                                AttType::ID => DTDPattern::Text,
                                AttType::IDREF => DTDPattern::Text,
                                AttType::IDREFS => DTDPattern::Text,
                                AttType::ENTITY => DTDPattern::Text,
                                AttType::ENTITIES => DTDPattern::Text,
                                AttType::NMTOKEN => DTDPattern::Text,
                                AttType::NMTOKENS => DTDPattern::Text,
                                AttType::NOTATION(_) => DTDPattern::Text,
                                AttType::ENUMERATION(el) => {
                                    let mut enumers = el.iter();
                                    let mut pat =
                                        DTDPattern::Value(enumers.next().unwrap().clone());
                                    for s in enumers {
                                        pat = DTDPattern::Group(
                                            Box::new(pat),
                                            Box::new(DTDPattern::Value(s.clone())),
                                        )
                                    }
                                    pat
                                }
                            };

                            match dd {
                                DefaultDecl::Implied => {
                                    ap = DTDPattern::Choice(
                                        Box::new(DTDPattern::Attribute(
                                            attname.clone(),
                                            Box::new(ap),
                                        )),
                                        Box::new(DTDPattern::Empty),
                                    )
                                }
                                _ => ap = DTDPattern::Attribute(attname.clone(), Box::new(ap)),
                            }

                            match attpat {
                                None => {
                                    attpat = Some(ap);
                                }
                                Some(ap2) => {
                                    attpat = Some(DTDPattern::Group(Box::new(ap), Box::new(ap2)));
                                }
                            }
                        }
                        state1.dtd.patterns.insert(
                            elname.clone(),
                            DTDPattern::Element(
                                elname.clone(),
                                Box::new(DTDPattern::Group(
                                    Box::new(eldecl.clone()),
                                    Box::new(attpat.unwrap()),
                                )),
                            ),
                        );
                    }
                }
            }
            //println!("{:?}", patternrefs);
            eprintln!("doctypedecl done");
            Ok(((input1, state1), ()))
        }
        Err(err) => Err(err),
    }
}
