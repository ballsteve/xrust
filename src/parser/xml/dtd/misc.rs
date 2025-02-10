use crate::item::Node;
use crate::parser::combinators::alt::{alt2, alt3, alt4};
use crate::parser::combinators::many::{many0, many1};
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_while;
use crate::parser::combinators::tuple::{tuple2, tuple3, tuple4, tuple5, tuple6};
use crate::parser::combinators::value::value;
use crate::parser::combinators::whitespace::whitespace0;
use crate::parser::common::is_namechar;
use crate::parser::xml::dtd::pereference::petextreference;
use crate::parser::xml::dtd::Occurances;
use crate::parser::xml::qname::name;
use crate::parser::{ParseError, ParseInput};
use crate::qname::QualifiedName;
use crate::xmldecl::DTDPattern;

pub(crate) fn nmtoken<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, String), ParseError> {
    map(many1(take_while(|c| is_namechar(&c))), |x| x.join(""))
}

pub(crate) fn contentspec<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, DTDPattern), ParseError> {
    alt4(
        value(tag("EMPTY"), DTDPattern::Empty),
        value(tag("ANY"), DTDPattern::Any),
        mixed(),
        children(),
    )
}

//Mixed	   ::=   	'(' S? '#PCDATA' (S? '|' S? Name)* S? ')*' | '(' S? '#PCDATA' S? ')'
pub(crate) fn mixed<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, DTDPattern), ParseError> {
    alt2(
        map(
            tuple6(
                tag("("),
                whitespace0(),
                tag("#PCDATA"),
                many0(tuple4(
                    whitespace0(),
                    tag("|"),
                    whitespace0(),
                    alt2(petextreference(), name()),
                )),
                whitespace0(),
                tag(")*"),
            ),
            |(_, _, _, vn, _, _)| {
                let mut r = DTDPattern::Text;
                for (_, _, _, name) in vn {
                    let q: QualifiedName = if name.contains(':') {
                        let mut nameparts = name.split(':');
                        QualifiedName::new(
                            None,
                            Some(nameparts.next().unwrap().parse().unwrap()),
                            nameparts.next().unwrap(),
                        )
                    } else {
                        QualifiedName::new(None, None, name)
                    };
                    r = DTDPattern::Choice(Box::new(DTDPattern::Ref(q)), Box::new(r))
                }
                //Zero or More
                DTDPattern::Choice(
                    Box::new(DTDPattern::OneOrMore(Box::new(r))),
                    Box::new(DTDPattern::Empty),
                )
            },
        ),
        map(
            tuple5(
                tag("("),
                whitespace0(),
                tag("#PCDATA"),
                whitespace0(),
                tag(")"),
            ),
            |_x| DTDPattern::Text,
        ),
    )
}

// children	   ::=   	(choice | seq) ('?' | '*' | '+')?
pub(crate) fn children<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, DTDPattern), ParseError> {
    move |(input, state)| {
        map(
            tuple2(
                alt3(
                    |(input1, state1)| match petextreference()((input1, state1)) {
                        Err(e) => Err(e),
                        Ok(((input2, state2), s)) => {
                            match tuple3(whitespace0(), alt2(choice(), seq()), whitespace0())((
                                s.as_str(),
                                state2,
                            )) {
                                Err(e) => Err(e),
                                Ok(((_input3, state3), (_, d, _))) => Ok(((input2, state3), d)),
                            }
                        }
                    },
                    choice(),
                    seq(),
                ),
                opt(alt3(
                    value(tag("?"), Occurances::ZeroOrOne),
                    value(tag("*"), Occurances::ZeroOrMore),
                    value(tag("+"), Occurances::OneOrMore),
                )),
            ),
            |(dtdp, occ)| match occ {
                None => dtdp,
                Some(o) => match o {
                    Occurances::ZeroOrMore => DTDPattern::Choice(
                        Box::new(DTDPattern::OneOrMore(Box::new(dtdp))),
                        Box::new(DTDPattern::Empty),
                    ),
                    Occurances::OneOrMore => DTDPattern::OneOrMore(Box::new(dtdp)),
                    Occurances::One => dtdp,
                    Occurances::ZeroOrOne => {
                        DTDPattern::Choice(Box::new(dtdp), Box::new(DTDPattern::Empty))
                    }
                },
            },
        )((input, state))
    }
}

// cp	   ::=   	(Name | choice | seq) ('?' | '*' | '+')?
fn cp<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, DTDPattern), ParseError> {
    move |(input1, state1)| {
        map(
            tuple2(
                alt4(
                    |(input1, state1)| match petextreference()((input1, state1)) {
                        Err(e) => Err(e),
                        Ok(((input2, state2), s)) => {
                            match tuple3(
                                whitespace0(),
                                alt3(
                                    map(name(), |n| {
                                        if n.contains(':') {
                                            let mut nameparts = n.split(':');
                                            DTDPattern::Ref(QualifiedName::new(
                                                None,
                                                Some(nameparts.next().unwrap().to_string()),
                                                nameparts.next().unwrap(),
                                            ))
                                        } else {
                                            DTDPattern::Ref(QualifiedName::new(None, None, n))
                                        }
                                    }),
                                    choice(),
                                    seq(),
                                ),
                                whitespace0(),
                            )((s.as_str(), state2))
                            {
                                Err(e) => Err(e),
                                Ok(((_input3, state3), (_, d, _))) => Ok(((input2, state3), d)),
                            }
                        }
                    },
                    map(name(), |n| {
                        if n.contains(':') {
                            let mut nameparts = n.split(':');
                            DTDPattern::Ref(QualifiedName::new(
                                None,
                                Some(nameparts.next().unwrap().to_string()),
                                nameparts.next().unwrap(),
                            ))
                        } else {
                            DTDPattern::Ref(QualifiedName::new(None, None, n))
                        }
                    }),
                    choice(),
                    seq(),
                ),
                map(
                    opt(alt3(
                        value(tag("?"), Occurances::ZeroOrOne),
                        value(tag("*"), Occurances::ZeroOrMore),
                        value(tag("+"), Occurances::OneOrMore),
                    )),
                    |o| match o {
                        None => Occurances::One,
                        Some(oc) => oc,
                    },
                ),
            ),
            |(cs, occ)| match occ {
                Occurances::ZeroOrMore => DTDPattern::Choice(
                    Box::new(DTDPattern::OneOrMore(Box::new(cs))),
                    Box::new(DTDPattern::Empty),
                ),
                Occurances::OneOrMore => DTDPattern::OneOrMore(Box::new(cs)),
                Occurances::One => cs,
                Occurances::ZeroOrOne => {
                    DTDPattern::Choice(Box::new(cs), Box::new(DTDPattern::Empty))
                }
            },
        )((input1, state1))
    }
}
//choice	   ::=   	'(' S? cp ( S? '|' S? cp )+ S? ')'
fn choice<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, DTDPattern), ParseError> {
    move |(input, state)| {
        map(
            tuple6(
                tag("("),
                whitespace0(),
                cp(),
                many0(alt2(
                    |(input1, state1)| match petextreference()((input1, state1)) {
                        Err(e) => Err(e),
                        Ok(((input2, state2), s)) => {
                            match tuple3(whitespace0(), cp(), whitespace0())((s.as_str(), state2)) {
                                Err(e) => Err(e),
                                Ok(((_input3, state3), (_, d, _))) => {
                                    Ok(((input2, state3), ((), (), (), d)))
                                }
                            }
                        }
                    },
                    tuple4(whitespace0(), tag("|"), whitespace0(), cp()),
                )),
                whitespace0(),
                tag(")"),
            ),
            |(_, _, c1, vc1, _, _)| {
                let mut res = c1;
                for (_, _, _, c) in vc1 {
                    res = DTDPattern::Choice(Box::new(res), Box::new(c))
                }
                res
            },
        )((input, state))
    }
}

//seq	   ::=   	'(' S? cp ( S? ',' S? cp )* S? ')'
fn seq<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, DTDPattern), ParseError> {
    map(
        tuple6(
            tag("("),
            whitespace0(),
            cp(),
            many0(tuple4(whitespace0(), tag(","), whitespace0(), cp())),
            whitespace0(),
            tag(")"),
        ),
        |(_, _, cp, mut veccp, _, _)| {
            let groupstart = cp;
            let mut prev: Option<DTDPattern> = None;
            veccp.reverse();
            for (_, _, _, c) in veccp {
                if prev.is_none() {
                    prev = Some(c);
                } else {
                    prev = Some(DTDPattern::Group(Box::new(c), Box::new(prev.unwrap())))
                }
            }
            if prev.is_none() {
                groupstart
            } else {
                DTDPattern::Group(Box::new(groupstart), Box::new(prev.unwrap()))
            }
        },
    )
}
