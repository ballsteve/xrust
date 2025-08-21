//! Functions for functions.

use crate::item::Node;
use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::list::separated_list0;
use crate::parser::combinators::map::{map, map_with_state};
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple3, tuple6};
use crate::parser::combinators::whitespace::xpwhitespace;
//use crate::parser::combinators::debug::inspect;
use crate::parser::xml::qname::qualname_to_qname;
use crate::parser::xpath::expr_single_wrapper;
use crate::parser::xpath::expressions::parenthesized_expr;
use crate::parser::xpath::nodetests::qualname_test;
use crate::parser::xpath::numbers::unary_expr;
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::transform::callable::ActualParameters;
use crate::transform::{
    NameTest, NodeTest, Transform, WildcardOrName, WildcardOrNamespaceUri, in_scope_namespaces,
};
use crate::xdmerror::ErrorKind;
use qualname::{NamespacePrefix, NamespaceUri, NcName, QName};

// ArrowExpr ::= UnaryExpr ( '=>' ArrowFunctionSpecifier ArgumentList)*
pub(crate) fn arrow_expr<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(
        pair(
            unary_expr::<N, L>(),
            opt(tuple6(
                xpwhitespace(),
                tag("=>"),
                xpwhitespace(),
                arrowfunctionspecifier::<N, L>(),
                xpwhitespace(),
                opt(argumentlist::<N, L>()),
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
fn arrowfunctionspecifier<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(
        alt2(
            map(qualname_to_qname(), |_| ()),
            map(parenthesized_expr::<N, L>(), |_| ()),
        ),
        |_| Transform::NotImplemented("arrowfunctionspecifier".to_string()),
    ))
}

// FunctionCall ::= EQName ArgumentList
pub(crate) fn function_call<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map_with_state(
        pair(qualname_test(), argumentlist::<N, L>()),
        |(qn, mut a), state, _ss| match qn {
            NodeTest::Name(NameTest {
                name: Some(WildcardOrName::Name(ref localpart)),
                ns: None,
                //prefix: None,
            }) => match localpart.to_string().as_str() {
                "current" => Transform::CurrentItem,
                "position" => Transform::Position,
                "last" => Transform::Last,
                "count" => {
                    if a.is_empty() {
                        Transform::Count(Box::new(Transform::Empty))
                    } else if a.len() == 1 {
                        Transform::Count(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "local-name" => {
                    if a.is_empty() {
                        Transform::LocalName(None)
                    } else if a.len() == 1 {
                        Transform::LocalName(Some(Box::new(a.pop().unwrap())))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "name" => {
                    if a.is_empty() {
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
                    if a.is_empty() {
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
                "generate-id" => {
                    if a.is_empty() {
                        Transform::GenerateId(None)
                    } else if a.len() == 1 {
                        Transform::GenerateId(Some(Box::new(a.pop().unwrap())))
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
                    if a.is_empty() {
                        Transform::True
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "false" => {
                    if a.is_empty() {
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
                "avg" => {
                    if a.len() == 0 {
                        Transform::Empty
                    } else if a.len() == 1 {
                        Transform::Avg(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "min" => {
                    if a.len() == 0 {
                        Transform::Empty
                    } else if a.len() == 1 {
                        Transform::Min(Box::new(a.pop().unwrap()))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "max" => {
                    if a.len() == 0 {
                        Transform::Empty
                    } else if a.len() == 1 {
                        Transform::Max(Box::new(a.pop().unwrap()))
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
                    if a.is_empty() {
                        Transform::CurrentDateTime
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "current-date" => {
                    if a.is_empty() {
                        Transform::CurrentDate
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "current-time" => {
                    if a.is_empty() {
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
                "format-number" => {
                    if a.is_empty() || a.len() == 1 {
                        // Too few arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too few arguments"))
                    } else if a.len() == 2 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        Transform::FormatNumber(Box::new(c), Box::new(b), None)
                    } else if a.len() == 3 {
                        let b = a.pop().unwrap();
                        let c = a.pop().unwrap();
                        let d = a.pop().unwrap();
                        Transform::FormatNumber(Box::new(d), Box::new(c), Some(Box::new(b)))
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "current-group" => {
                    if a.is_empty() {
                        Transform::CurrentGroup
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "current-grouping-key" => {
                    if a.is_empty() {
                        Transform::CurrentGroupingKey
                    } else {
                        // Too many arguments
                        Transform::Error(ErrorKind::ParseError, String::from("too many arguments"))
                    }
                }
                "key" => {
                    if a.len() == 2 {
                        let m = a.pop().unwrap();
                        let name = a.pop().unwrap();
                        Transform::Key(
                            Box::new(name),
                            Box::new(m),
                            None,
                            in_scope_namespaces(state.cur.clone()),
                        )
                    } else if a.len() == 3 {
                        let u = a.pop().unwrap();
                        let m = a.pop().unwrap();
                        let name = a.pop().unwrap();
                        Transform::Key(
                            Box::new(name),
                            Box::new(m),
                            Some(Box::new(u)),
                            in_scope_namespaces(state.cur.clone()),
                        )
                    } else {
                        // Wrong # arguments
                        Transform::Error(
                            ErrorKind::ParseError,
                            String::from("wrong number of arguments"),
                        )
                    }
                }
                "system-property" => {
                    eprintln!(
                        "\n\tsystem-property with current node {:?}\n",
                        state.cur.clone()
                    );
                    if a.len() == 1 {
                        let p = a.pop().unwrap();
                        Transform::SystemProperty(
                            Box::new(p),
                            in_scope_namespaces(state.cur.clone()),
                        )
                    } else {
                        // Wrong # arguments
                        Transform::Error(
                            ErrorKind::ParseError,
                            String::from("wrong number of arguments"),
                        )
                    }
                }
                "available-system-properties" => {
                    if a.is_empty() {
                        Transform::AvailableSystemProperties
                    } else {
                        // Wrong # arguments
                        Transform::Error(
                            ErrorKind::ParseError,
                            String::from("wrong number of arguments"),
                        )
                    }
                }
                "document" => match a.len() {
                    0 => Transform::Document(Box::new(Transform::Empty), None),
                    1 => {
                        let u = a.pop().unwrap();
                        Transform::Document(Box::new(u), None)
                    }
                    2 => {
                        let b = a.pop().unwrap();
                        let u = a.pop().unwrap();
                        Transform::Document(Box::new(u), Some(Box::new(b)))
                    }
                    _ => Transform::Error(
                        ErrorKind::ParseError,
                        String::from("wrong number of arguments"),
                    ),
                },
                _ => Transform::Error(
                    ErrorKind::ParseError,
                    format!("undefined function \"{}\"", qn),
                ), // TODO: user-defined functions
            },
            NodeTest::Name(NameTest {
                name: Some(WildcardOrName::Name(localpart)),
                ns: Some(WildcardOrNamespaceUri::NamespaceUri(nsuri)),
                //prefix: p,
            }) => Transform::Invoke(
                QName::new_from_parts(
                    NcName::try_from(localpart.local_name().as_str()).unwrap(),
                    Some(nsuri),
                ),
                ActualParameters::Positional(a),
                in_scope_namespaces(state.cur.clone()),
            ),
            /*NodeTest::Name(NameTest {
                name: Some(WildcardOrName::Name(localpart)),
                ns: None,
                //prefix: p,
            }) => Transform::Invoke(
                localpart,
                ActualParameters::Positional(a),
                in_scope_namespaces(state.cur.clone()),
            ),*/
            _ => Transform::Error(ErrorKind::Unknown, format!("unknown function \"{}\"", qn)),
        },
    ))
}

// ArgumentList ::= '(' (Argument (',' Argument)*)? ')'
// TODO: finish this parser with actual arguments
fn argumentlist<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Vec<Transform<N>>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(
        tuple3(
            tag("("),
            separated_list0(
                map(tuple3(xpwhitespace(), tag(","), xpwhitespace()), |_| ()),
                argument::<N, L>(),
            ),
            tag(")"),
        ),
        |(_, a, _)| a,
    ))
}

// Argument ::= ExprSingle | ArgumentPlaceHolder
// TODO: ArgumentPlaceHolder
fn argument<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(expr_single_wrapper::<N, L>(true))
}
