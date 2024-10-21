use crate::item::Node;
use crate::parser::combinators::alt::{alt2, alt3, alt9};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_while;
use crate::parser::combinators::tuple::{tuple2, tuple6};
use crate::parser::combinators::value::value;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::common::{is_ncnamechar, is_ncnamestartchar};
use crate::parser::xml::chardata::chardata_unicode_codepoint;
use crate::parser::xml::dtd::enumerated::enumeratedtype;
use crate::parser::xml::qname::{name, qualname};
use crate::parser::xml::reference::textreference;
use crate::parser::{ParseError, ParseInput};
use crate::qname::QualifiedName;
use crate::xmldecl::{AttType, DefaultDecl};
use std::collections::HashMap;

//AttlistDecl ::= '<!ATTLIST' S Name AttDef* S? '>'
pub(crate) fn attlistdecl<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, ()), ParseError> {
    move |(input, state)| match tuple6(
        tag("<!ATTLIST"),
        whitespace1(),
        qualname(),
        many0(attdef()),
        whitespace0(),
        tag(">"),
    )((input, state))
    {
        Ok(((input2, mut state2), (_, _, n, ats, _, _))) => {
            /*

            "3.3 Attribute-List Declarations
            When more than one AttlistDecl is provided for a given element type, the contents of all those provided are merged.
            When more than one definition is provided for the same attribute of a given element type, the first declaration
            is binding and later declarations are ignored."

            So we're going to do some checks for existing ATTLIST declarations, but each one has a boolean flag to confirm
            if it was created by an external or internal DTD. If its external we can happily overwrite, but not for internal.

             */
            /* Entities should always bind to the first value */
            let replaceable = state2.currentlyexternal;

            let mut atts = HashMap::new();

            let mut count_id_attrs = 0;
            for (qn, att, dfd) in ats {
                match &dfd {
                    //We need to make sure that the default is valid, even if its not used.
                    DefaultDecl::FIXED(s) | DefaultDecl::Default(s) => {
                        match att {
                            AttType::ID => {
                                count_id_attrs += 1;
                                let mut ch = s.chars();
                                match ch.next() {
                                    None => {}
                                    Some(c) => {
                                        if is_ncnamestartchar(&c) {
                                            for cha in ch {
                                                if !is_ncnamechar(&cha) {
                                                    return Err(ParseError::NotWellFormed(
                                                        String::from(
                                                            "DTD Attvalue default is invalid",
                                                        ),
                                                    ));
                                                }
                                            }
                                        } else {
                                            return Err(ParseError::NotWellFormed(String::from(
                                                "DTD Attvalue default is invalid",
                                            )));
                                        }
                                    }
                                }
                            }
                            AttType::IDREF => {
                                let mut ch = s.chars();
                                match ch.next() {
                                    None => {}
                                    Some(c) => {
                                        if is_ncnamestartchar(&c) {
                                            for cha in ch {
                                                if !is_ncnamechar(&cha) {
                                                    return Err(ParseError::NotWellFormed(
                                                        String::from(
                                                            "DTD Attvalue default is invalid",
                                                        ),
                                                    ));
                                                }
                                            }
                                        } else {
                                            return Err(ParseError::NotWellFormed(String::from(
                                                "DTD Attvalue default is invalid",
                                            )));
                                        }
                                    }
                                }
                            }
                            AttType::IDREFS => {
                                let names = s.split(" ");
                                for name in names {
                                    let mut ch = name.chars();
                                    match ch.next() {
                                        None => {}
                                        Some(c) => {
                                            if is_ncnamestartchar(&c) {
                                                for cha in ch {
                                                    if !is_ncnamechar(&cha) {
                                                        return Err(ParseError::NotWellFormed(
                                                            String::from(
                                                                "DTD Attvalue default is invalid",
                                                            ),
                                                        ));
                                                    }
                                                }
                                            } else {
                                                return Err(ParseError::NotWellFormed(
                                                    String::from("DTD Attvalue default is invalid"),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                            AttType::ENTITY => {
                                let mut ch = s.chars();
                                match ch.next() {
                                    None => {}
                                    Some(c) => {
                                        if is_ncnamestartchar(&c) {
                                            for cha in ch {
                                                if !is_ncnamechar(&cha) {
                                                    return Err(ParseError::NotWellFormed(
                                                        String::from(
                                                            "DTD Attvalue default is invalid",
                                                        ),
                                                    ));
                                                }
                                            }
                                        } else {
                                            return Err(ParseError::NotWellFormed(String::from(
                                                "DTD Attvalue default is invalid",
                                            )));
                                        }
                                    }
                                }
                            }
                            AttType::ENTITIES => {
                                let entities = s.split(" ");
                                for entity in entities {
                                    let mut ch = entity.chars();
                                    match ch.next() {
                                        None => {}
                                        Some(c) => {
                                            if is_ncnamestartchar(&c) {
                                                for cha in ch {
                                                    if !is_ncnamechar(&cha) {
                                                        return Err(ParseError::NotWellFormed(
                                                            String::from(
                                                                "DTD Attvalue default is invalid",
                                                            ),
                                                        ));
                                                    }
                                                }
                                            } else {
                                                return Err(ParseError::NotWellFormed(
                                                    String::from("DTD Attvalue default is invalid"),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                            _ => { /*TODO complete the rest of these */ }
                        }
                    }
                    //else do nothing
                    _ => {}
                }
                atts.insert(qn, (att, dfd, replaceable));
            }
            if count_id_attrs > 1 {
                return Err(ParseError::NotWellFormed(String::from(
                    "Duplicate ID attribute declarations",
                )));
            }

            match state2.dtd.attlists.get(&n) {
                None => {
                    state2.dtd.attlists.insert(n, atts);
                    Ok(((input2, state2), ()))
                }
                Some(al) => {
                    let mut newal = al.clone();
                    for (attname, (atttype, defaultdecl, is_editable)) in atts.iter() {
                        match newal.get(attname) {
                            None => {
                                newal.insert(
                                    attname.clone(),
                                    (atttype.clone(), defaultdecl.clone(), *is_editable),
                                );
                            }
                            Some((_, _, existing_is_editable)) => {
                                if *existing_is_editable {
                                    newal.insert(
                                        attname.clone(),
                                        (atttype.clone(), defaultdecl.clone(), *is_editable),
                                    );
                                }
                            }
                        }
                    }
                    state2.dtd.attlists.insert(n, newal);
                    Ok(((input2, state2), ()))
                }
            }

            /*
                   state2
                       .dtd
                       .attlists
                       .insert(n, atts);
                   Ok(((input2, state2), ()))

            */
        }
        Err(err) => Err(err),
    }
}

//AttDef ::= S Name S AttType S DefaultDecl
fn attdef<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, (QualifiedName, AttType, DefaultDecl)), ParseError>
{
    map(
        tuple6(
            whitespace1(),
            name(),
            whitespace1(),
            atttype(),
            whitespace1(),
            defaultdecl(),
        ),
        |(_, an, _, at, _, dd)| {
            let qn = if an.contains(':') {
                let mut attnamesplit = an.split(":");
                let prefix = Some(attnamesplit.next().unwrap().to_string());
                let local = attnamesplit.collect::<String>();
                QualifiedName::new(None, prefix, local)
            } else {
                QualifiedName::new(None, None, an)
            };
            (qn, at, dd)
        },
    )
}

//AttType ::= StringType | TokenizedType | EnumeratedType
fn atttype<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, AttType), ParseError> {
    alt9(
        //map(petextreference(), |_| {}), //TODO
        value(tag("CDATA"), AttType::CDATA), //Stringtype
        //tokenizedtype
        value(tag("IDREFS"), AttType::IDREFS),
        value(tag("IDREF"), AttType::IDREF),
        value(tag("ID"), AttType::ID),
        value(tag("ENTITY"), AttType::ENTITY),
        value(tag("ENTITIES"), AttType::ENTITIES),
        value(tag("NMTOKENS"), AttType::NMTOKENS),
        value(tag("NMTOKEN"), AttType::NMTOKEN),
        enumeratedtype(),
    )
}

//DefaultDecl ::= '#REQUIRED' | '#IMPLIED' | (('#FIXED' S)? AttValue)
fn defaultdecl<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, DefaultDecl), ParseError> {
    alt3(
        value(tag("#REQUIRED"), DefaultDecl::Required),
        value(tag("#IMPLIED"), DefaultDecl::Implied),
        map(
            tuple2(
                opt(tuple2(
                    value(tag("#FIXED"), "#FIXED".to_string()),
                    whitespace1(),
                )),
                attvalue(),
            ),
            |(x, y)| match x {
                None => DefaultDecl::Default(y),
                Some(_) => DefaultDecl::FIXED(y),
            },
        ),
    )
}

//AttValue ::= '"' ([^<&"] | Reference)* '"' | "'" ([^<&'] | Reference)* "'"
fn attvalue<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> {
    alt2(
        delimited(
            tag("\'"),
            map(
                many0(alt3(
                    map(chardata_unicode_codepoint(), |c| c.to_string()),
                    take_while(|c| !"&\'<".contains(c)),
                    textreference(),
                )),
                |v| v.join(""),
            ),
            tag("\'"),
        ),
        delimited(
            tag("\""),
            map(
                many0(alt3(
                    map(chardata_unicode_codepoint(), |c| c.to_string()),
                    take_while(|c| !"&\"<".contains(c)),
                    textreference(),
                )),
                |v| v.join(""),
            ),
            tag("\""),
        ),
    )
}
