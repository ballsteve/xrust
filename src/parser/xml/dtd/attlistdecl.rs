use crate::intmuttree::DTDDecl;
use crate::parser::combinators::alt::{alt2, alt3, alt4, alt7};
use crate::parser::combinators::delimited::delimited;
use crate::parser::{ParseInput, ParseResult};
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_while;
use crate::parser::combinators::tuple::{tuple2, tuple6};
use crate::parser::combinators::value::value;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::xml::chardata::chardata_unicode_codepoint;
use crate::parser::xml::dtd::enumerated::enumeratedtype;
use crate::parser::xml::dtd::pereference::petextreference;
use crate::parser::xml::reference::textreference;
use crate::parser::xml::qname::{name, qualname};

//AttlistDecl ::= '<!ATTLIST' S Name AttDef* S? '>'
pub(crate) fn attlistdecl() -> impl Fn(ParseInput) -> ParseResult<()> {
    move |(input, state)| {
        match tuple6(
            tag("<!ATTLIST"),
            whitespace1(),
            qualname(),
            many0(attdef()),
            whitespace0(),
            tag(">"),
        )((input, state))
        {
            Ok(((input2, mut state2), (_, _, n, _, _, _))) => {
                state2
                    .dtd
                    .attlists
                    .insert(n.to_string(), DTDDecl::Attlist(n, "".to_string()));
                Ok(((input2, state2), ()))
            }
            Err(err) => Err(err),
        }
    }
}


//AttDef ::= S Name S AttType S DefaultDecl
fn attdef() -> impl Fn(ParseInput) -> ParseResult<String> {
    map(
        tuple6(
            whitespace1(),
            name(),
            whitespace1(),
            atttype(),
            whitespace1(),
            defaultdecl(),
        ),
        |_x| "".to_string(),
    )
}

//AttType ::= StringType | TokenizedType | EnumeratedType
fn atttype() -> impl Fn(ParseInput) -> ParseResult<()> {
    alt4(
        map(petextreference(), |_| { () }),  //TODO
        tag("CDATA"), //Stringtype
        alt7(
            //tokenizedtype
            tag("IDREFS"),
            tag("IDREF"),
            tag("ID"),
            tag("ENTITY"),
            tag("ENTITIES"),
            tag("NMTOKENS"),
            tag("NMTOKEN"),
        ),
        enumeratedtype(),
    )
}

//DefaultDecl ::= '#REQUIRED' | '#IMPLIED' | (('#FIXED' S)? AttValue)
fn defaultdecl() -> impl Fn(ParseInput) -> ParseResult<()> {
    map(
        alt3(
            value(tag("#REQUIRED"), "#REQUIRED".to_string()),
            value(tag("#IMPLIED"), "#IMPLIED".to_string()),
            map(
                tuple2(
                    opt(tuple2(
                        value(tag("#FIXED"), "#FIXED".to_string()),
                        whitespace1(),
                    )),
                    attvalue(),
                ),
                |(x, y)| match x {
                    None => {
                        y
                    },
                    Some((mut f, _)) => {
                        f.push_str(&y);
                        f
                    }
                },
            ),
        ),
        |_x| (),
    )
}

//AttValue ::= '"' ([^<&"] | Reference)* '"' | "'" ([^<&'] | Reference)* "'"
fn attvalue() -> impl Fn(ParseInput) -> ParseResult<String> {
    alt2(
        delimited(
            tag("\'"),
            map(
                many0(
                    alt3(
                    chardata_unicode_codepoint(),
                    take_while(|c| !"&\'<".contains(c)),
                    textreference()
                    )
                ),
                |v| {
                    v.join("")
                },
            ),
            tag("\'"),
        ),
        delimited(
            tag("\""),
            map(
                many0(
                    alt3(
                        chardata_unicode_codepoint(),
                        take_while(|c| !"&\"<".contains(c)),
                        textreference()
                    )
                ),
                |v| {
                    v.join("")
                },
            ),
            tag("\""),
        ),
    )
}
