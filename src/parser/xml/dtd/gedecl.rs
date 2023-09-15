use crate::parser::combinators::alt::{alt2, alt3, alt4};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::{take_until, take_until_either_or, take_until_either_or_min1, take_until_end};
use crate::parser::combinators::tuple::{tuple2, tuple7};
use crate::parser::combinators::wellformed::wellformed;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::common::is_char;
use crate::parser::xml::chardata::chardata_unicode_codepoint;
use crate::parser::xml::qname::qualname;
use crate::parser::{ParseError, ParseInput, ParseResult};
use crate::parser::xml::dtd::intsubset::intsubset;
use crate::parser::xml::dtd::pereference::{pereference, petextreference};
use crate::parser::xml::dtd::textexternalid;
use crate::parser::xml::reference::{reference, textreference};

pub(crate) fn gedecl() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |input| match wellformed(
        tuple7(
            tag("<!ENTITY"),
            whitespace1(),
            wellformed(qualname(),|n| !n.to_string().contains(':') ),
            whitespace1(),
            alt3(
                textexternalid(),
                delimited(tag("'"), take_until("'"), tag("'")),
                delimited(tag("\""), take_until("\""), tag("\"")),
            ),
            whitespace0(),
            tag(">"),
        ),
        |(_, _, _, _, s, _, _)| !s.contains(|c: char| !is_char(&c)),
    )(input)
    {
        Ok(((input2, mut state2), (_, _, n, _, s, _, _))) => {
            /*
            Numeric and other entities expanded immediately, since there'll be namespaces and the like to
            deal with later, after that we just store the entity as a string and parse again when called.
             */
            if !state2.currentlyexternal && s.contains('%'){
                return Err(ParseError::NotWellFormed)
            }

            let entityparse = map(
                tuple2(
                    map(
                        many0(alt4(
                            chardata_unicode_codepoint(),

                            petextreference(),
                                //General entity is ignored.
                                map(
                                    delimited(tag("&"),
                                    take_until(";"),
                                    tag(";")
                                    ), |s|
                                    ["&".to_string(), s , ";".to_string()].concat()
                                ),
                            //textreference(),
                            //map(tag("&"), |_| "&".to_string()),
                            //take_until("&"),
                            take_until_either_or_min1("&","%")
                        )),
                        |ve| ve.concat(),
                    ),
                    wellformed(take_until_end(),|s| !s.contains("&")&&!s.contains("%")),
                ),
                |(a, b)| [a, b].concat(),
            )((s.as_str(), state2.clone()));

            match entityparse {
                Ok(((_, _), res)) => {
                    if !state2.currentlyexternal {
                        match intsubset()((res.as_str(), state2.clone())){
                            Ok(_) => {}
                            Err(_) => {
                                return Err(ParseError::NotWellFormed)
                            }
                        }
                    };


                    /* Entities should always bind to the first value */
                    let replaceable = state2.currentlyexternal;
                    match state2.dtd.generalentities.get(n.to_string().as_str()) {
                        None => {
                            state2
                                .dtd
                                .generalentities
                                .insert(n.to_string(), (res, replaceable));
                            Ok(((input2, state2), ()))
                        }
                        Some((_, true)) => {
                            state2
                                .dtd
                                .generalentities
                                .entry(n.to_string())
                                .or_insert((res, replaceable));
                            Ok(((input2, state2), ()))
                        }
                        _ => Ok(((input2, state2), ())),
                    }
                    //state2.dtd
                    //    .generalentities.entry(n.to_string())
                    //    .or_insert(res);
                }
                Err(e) => Err(e),
            }
        }
        Err(err) => Err(err),
    }
}
