mod attlistdecl;
mod conditionals;
mod elementdecl;
mod enumerated;
pub(crate) mod extsubset;
mod gedecl;
mod intsubset;
mod misc;
mod notation;
mod pedecl;
pub(crate) mod pereference;
mod textdecl;
mod externalid;

use crate::item::Node;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{ tuple8};
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::xml::dtd::extsubset::extsubset;
use crate::parser::xml::dtd::intsubset::intsubset;
use crate::parser::xml::qname::name;
use crate::parser::xml::reference::reference;
use crate::parser::{ParseError, ParseInput};
use crate::parser::xml::dtd::externalid::externalid;
use crate::qname::QualifiedName;
use crate::xmldecl::{AttType, Contentspec, DefaultDecl, DTDPattern};


#[derive(Clone)]
pub(crate) enum Occurances {
    ZeroOrMore,
    OneOrMore,
    One,
    ZeroOrOne
}

pub(crate) fn doctypedecl<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, ()), ParseError> {
    move |input| match tuple8(
        tag("<!DOCTYPE"),
        whitespace1(),
        name(),
        whitespace1(),
        opt(externalid()),
        whitespace0(),
        opt(delimited(tag("["), intsubset(), tag("]"))),
        tag(">"),
    )(input)
    {
        Ok(((input1, mut state1), (_, _, n, _, _, _, _inss, _))) => {
            let q: QualifiedName = if n.contains(':'){
                let mut nameparts = n.split(':');
                QualifiedName::new(None, Some(nameparts.next().unwrap().parse().unwrap()), nameparts.next().unwrap())
            } else {
                QualifiedName::new(None, None, n)
            };
            state1.dtd.name = Some(q);
            /*  We're doing nothing with the below, just evaluating the external entity to check its well formed */
            let exdtd = state1.ext_entities_to_parse.clone().pop();
            match exdtd {
                None => {}
                Some(s) => match state1.clone().resolve(state1.docloc.clone(), s) {
                    Err(_) => return Err(ParseError::ExtDTDLoadError),
                    Ok(s) => {
                        if let Err(e) = extsubset()((s.as_str(), state1.clone())) {
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
                    if let Err(ParseError::NotWellFormed(v)) = reference()((
                        ["&".to_string(), k, ";".to_string()].join("").as_str(),
                        state1.clone(),
                    )) {
                        return Err(ParseError::NotWellFormed(v));
                    }
                }
            }

            // Construction of DTDPatterns for validation

            // First, we construct our "ANY" pattern. It can be any text or comment or declared element, so we loop through the DTD to build it.
            let mut anypattern = DTDPattern::Text;

            for el in &state1.dtd.elements {
                anypattern = DTDPattern::Choice(
                    Box::new(DTDPattern::Ref(el.0.clone())),
                    Box::new(anypattern)
                );
            }
            anypattern = DTDPattern::Choice(
                Box::new(DTDPattern::Empty),
                Box::new(
                    DTDPattern::OneOrMore(
                        Box::new(anypattern)
                    )
                ),
            );

            //println!("dtdelements");
            //println!("{:?}", &state1.dtd.elements);
            //println!("{:?}", &state1.dtd.patterns);

            //let mut patternrefs = HashMap::new();

            for (elname, eldecl) in &state1.dtd.elements {

                let mut pat = match eldecl {
                    Contentspec::ANY => {
                        //println!("element-{:?}", elname);
                        //println!("elementp-{:?}", anypattern.clone());
                        anypattern.clone()
                    }
                    Contentspec::DTDPattern(d) => {
                        //println!("element-{:?}", elname);
                        //println!("elementp-{:?}", d.clone());
                        d.clone()
                    }
                };
                match &state1.dtd.attlists.get(elname){
                    None => {
                        pat = DTDPattern::Element(elname.clone(), Box::new(pat));
                        state1.dtd.patterns.insert(elname.clone(), pat);
                    }
                    Some(attlist) => {
                        let mut attpat = None;
                        for (attname, (at,dd,_)) in attlist.iter() {
                            let mut ap = match at {
                                AttType::CDATA => { DTDPattern::Text }
                                AttType::ID => { DTDPattern::Text }
                                AttType::IDREF => { DTDPattern::Text }
                                AttType::IDREFS => { DTDPattern::Text }
                                AttType::ENTITY => { DTDPattern::Text }
                                AttType::ENTITIES => {DTDPattern::Text }
                                AttType::NMTOKEN => { DTDPattern::Text }
                                AttType::NMTOKENS => { DTDPattern::Text }
                                AttType::NOTATION(_) => { DTDPattern::Text }
                                AttType::ENUMERATION(el) => {
                                    let mut enumers = el.iter();
                                    let mut pat = DTDPattern::Value(enumers.next().unwrap().clone());
                                    for s in enumers {
                                        pat = DTDPattern::Group(
                                            Box::new(pat),
                                            Box::new(DTDPattern::Value(s.clone()))
                                        )
                                    };
                                    pat
                                }
                            };

                            match dd {
                                DefaultDecl::Implied => {
                                    ap = DTDPattern::Choice(
                                        Box::new(DTDPattern::Attribute(
                                            attname.clone(),
                                            Box::new(ap)
                                        )),
                                        Box::new(DTDPattern::Empty)
                                    )
                                }
                                _ => {
                                    ap = DTDPattern::Attribute(
                                        attname.clone(),
                                        Box::new(ap)
                                    )
                                }
                            }

                            match attpat {
                                None => {
                                    attpat = Some(ap);
                                },
                                Some(ap2) => {
                                    attpat = Some(
                                        DTDPattern::Group(
                                            Box::new(
                                                ap
                                            ),
                                            Box::new(ap2)
                                        )
                                    );
                                }
                            }
                        };
                        state1.dtd.patterns.insert(elname.clone(),
                            DTDPattern::Element(elname.clone(),
                               Box::new(DTDPattern::Group(
                                    Box::new(pat),
                                    Box::new(
                                        attpat.unwrap())
                               ))
                            )
                        );
                    }
                }
            }
            //println!("{:?}", patternrefs);
            Ok(((input1, state1), ()))
        }
        Err(err) => {
            Err(err)
        },
    }
}

