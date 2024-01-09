//! Functions for functions.

use crate::item::Node;
use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::list::separated_list0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple3, tuple6};
use crate::parser::combinators::whitespace::xpwhitespace;
//use crate::parser::combinators::debug::inspect;
use crate::parser::xml::qname::qualname;
use crate::parser::xpath::expr_single_wrapper;
use crate::parser::xpath::expressions::parenthesized_expr;
use crate::parser::xpath::nodetests::qualname_test;
use crate::parser::xpath::numbers::unary_expr;
use crate::parser::{ParseError, ParseInput};
use crate::transform::{NameTest, NodeTest, Transform, WildcardOrName};
use crate::xdmerror::ErrorKind;

// ArrowExpr ::= UnaryExpr ( '=>' ArrowFunctionSpecifier ArgumentList)*
pub(crate) fn arrow_expr<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(
        pair(
            unary_expr::<N>(),
            opt(tuple6(
                xpwhitespace(),
                tag("=>"),
                xpwhitespace(),
                arrowfunctionspecifier::<N>(),
                xpwhitespace(),
                opt(argumentlist::<N>()),
            )),
        ),
        |(v, o)| {
            if o.is_none() {
                v
            } else {
                Transform::NotImplemented("arrow_expr".to_string())
            }
        },
    ))
}

// ArrowFunctionSpecifier ::= EQName | VarRef | ParenthesizedExpr
// TODO: finish this parser with EQName and VarRef
fn arrowfunctionspecifier<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(
        alt2(
            map(qualname(), |_| ()),
            map(parenthesized_expr::<N>(), |_| ()),
        ),
        |_| Transform::NotImplemented("arrowfunctionspecifier".to_string()),
    ))
}

// FunctionCall ::= EQName ArgumentList
pub(crate) fn function_call<'a, N: Node + 'a>(
) -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(map(
        pair(qualname_test(), argumentlist::<N>()),
        |(qn, mut a)| match qn {
            NodeTest::Name(NameTest {
                name: Some(WildcardOrName::Name(localpart)),
                ns: None,
                prefix: None,
            }) => match localpart.as_str() {
                "position" => Transform::Position,
                "last" => Transform::Last,
                "count" => {
                    if a.len() == 0 {
                        Transform::Count(Box::new(Transform::Empty))
                    } else if a.len() == 1 {
                        Transform::Count(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "local-name" => {
                    if a.len() == 0 {
                        Transform::LocalName(None)
                    } else if a.len() == 1 {
                        Transform::LocalName(Some(Box::new(a.pop().unwrap())))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "name" => {
                    if a.len() == 0 {
                        Transform::Name(None)
                    } else if a.len() == 1 {
                        Transform::Name(Some(Box::new(a.pop().unwrap())))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "string" => {
                    if a.len() == 1 {
                        Transform::String(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "concat" => Transform::Concat(a),
                "starts-with" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::StartsWith(Box::new(c), Box::new(b))
                    } else {
                        // Incorrect arguments
                        Transform::Error(ErrorKind::ParseError, String::from("incorrect arguments"))
                    }
                }
                "contains" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::Contains(Box::new(c), Box::new(b))
                    } else {
                        // Incorrect arguments
                        Transform::Error(ErrorKind::ParseError, String::from("incorrect arguments"))
                    }
                }
                "substring" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::Substring(Box::new(c), Box::new(b), None)
                    } else if a.len() == 3 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        Transform::Substring(Box::new(d), Box::new(c), Some(Box::new(b)))
                    } else {
                        // Wrong number of arguments
                        Transform::Error(
                            ErrorKind::ParseError,
                            String::from("wrong number of arguments"),
                        )
                    }
                }
                "substring-before" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::SubstringBefore(Box::new(c), Box::new(b))
                    } else {
                        // Incorrect arguments
                        Transform::Error(ErrorKind::ParseError, String::from("incorrect arguments"))
                    }
                }
                "substring-after" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::SubstringAfter(Box::new(c), Box::new(b))
                    } else {
                        // Incorrect arguments
                        Transform::Error(ErrorKind::ParseError, String::from("incorrect arguments"))
                    }
                }
                "normalize-space" => {
                    if a.len() == 0 {
                        Transform::NormalizeSpace(None)
                    } else if a.len() == 1 {
                        Transform::NormalizeSpace(Some(Box::new(a.pop().unwrap())))
                    } else {
                        // Wrong number of arguments
                        Transform::Error(
                            ErrorKind::ParseError,
                            String::from("wrong number of arguments"),
                        )
                    }
                }
                "translate" => {
                    if a.len() == 3 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        Transform::Translate(Box::new(d), Box::new(c), Box::new(b))
                    } else {
                        // Wrong number of arguments
                        Transform::Error(
                            ErrorKind::ParseError,
                            String::from("wrong number of arguments"),
                        )
                    }
                }
                "boolean" => {
                    if a.len() == 1 {
                        Transform::Boolean(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "not" => {
                    if a.len() == 1 {
                        Transform::Not(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "true" => {
                    if a.len() == 0 {
                        Transform::True
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "false" => {
                    if a.len() == 0 {
                        Transform::False
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "number" => {
                    if a.len() == 1 {
                        Transform::Number(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "sum" => {
                    if a.len() == 1 {
                        Transform::Sum(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "floor" => {
                    if a.len() == 1 {
                        Transform::Floor(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "ceiling" => {
                    if a.len() == 1 {
                        Transform::Ceiling(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "round" => {
                    if a.len() == 1 {
                        let b = a.pop().unwrap();
                        Transform::Round(Box::new(b), None)
                    } else if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::Round(Box::new(c), Some(Box::new(b)))
                    } else {
                        // Wrong number of arguments
                        Transform::Error(
                            ErrorKind::ParseError,
                            String::from("wrong number of arguments"),
                        )
                    }
                }
                "current-date-time" => {
                    if a.len() == 0 {
                        Transform::CurrentDateTime
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "current-date" => {
                    if a.len() == 0 {
                        Transform::CurrentDate
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "current-time" => {
                    if a.len() == 0 {
                        Transform::CurrentTime
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "format-date-time" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::FormatDateTime(Box::new(c), Box::new(b), None, None, None)
                    } else if a.len() == 5 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        let e = a.pop().unwrap();
                        let f = a.pop().unwrap();
                        Transform::FormatDateTime(
                            Box::new(f),
                            Box::new(e),
                            Some(Box::new(d)),
                            Some(Box::new(c)),
                            Some(Box::new(b)),
                        )
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "format-date" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::FormatDate(Box::new(c), Box::new(b), None, None, None)
                    } else if a.len() == 5 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        let e = a.pop().unwrap();
                        let f = a.pop().unwrap();
                        Transform::FormatDate(
                            Box::new(f),
                            Box::new(e),
                            Some(Box::new(d)),
                            Some(Box::new(c)),
                            Some(Box::new(b)),
                        )
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "format-time" => {
                    if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::FormatTime(Box::new(c), Box::new(b), None, None, None)
                    } else if a.len() == 5 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        let e = a.pop().unwrap();
                        let f = a.pop().unwrap();
                        Transform::FormatTime(
                            Box::new(f),
                            Box::new(e),
                            Some(Box::new(d)),
                            Some(Box::new(c)),
                            Some(Box::new(b)),
                        )
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "current-group" => {
                    if a.len() == 0 {
                        Transform::CurrentGroup
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "current-grouping-key" => {
                    if a.len() == 0 {
                        Transform::CurrentGroupingKey
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                _ => Transform::Error(ErrorKind::ParseError, String::from("undefined function")), // TODO: user-defined functions
            },
            _ => Transform::Error(ErrorKind::ParseError, String::from("unknown function")),
        },
    ))
}

// ArgumentList ::= '(' (Argument (',' Argument)*)? ')'
// TODO: finish this parser with actual arguments
fn argumentlist<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Vec<Transform<N>>), ParseError> + 'a>
{
    Box::new(map(
        tuple3(
            tag("("),
            separated_list0(
                map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
                argument::<N>(),
            ),
            tag(")"),
        ),
        |(_, a, _)| a,
    ))
}

// Argument ::= ExprSingle | ArgumentPlaceHolder
// TODO: ArgumentPlaceHolder
fn argument<'a, N: Node + 'a>() -> Box<dyn Fn(ParseInput<N>) -> Result<(ParseInput<N>, Transform<N>), ParseError> + 'a> {
    Box::new(expr_single_wrapper::<N>(true))
}
