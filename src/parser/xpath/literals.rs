//! Functions that produce literal values or nodes.

use std::rc::Rc;
use std::str::FromStr;

use crate::item::{Item, Node};
use crate::parser::combinators::alt::{alt2, alt3};
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::pair::pair;
use crate::parser::combinators::support::{digit0, digit1, none_of};
use crate::parser::combinators::tag::{anychar, tag};
use crate::parser::combinators::tuple::{tuple3, tuple4};
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::transform::Transform;
use crate::value::Value;
use crate::xdmerror::ErrorKind;
use qualname::{NamespacePrefix, NamespaceUri};

use rust_decimal::Decimal;

// Literal ::= NumericLiteral | StringLiteral
pub(crate) fn literal<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt2(numeric_literal::<N, L>(), string_literal::<N, L>()))
}

// NumericLiteral ::= IntegerLiteral | DecimalLiteral | DoubleLiteral
fn numeric_literal<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt3(
        double_literal::<N, L>(),
        decimal_literal::<N, L>(),
        integer_literal::<N, L>(),
    ))
}
// IntegerLiteral ::= Digits
fn integer_literal<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(digit1(), |s: String| {
        s.parse::<i64>().map_or(
            Transform::Error(ErrorKind::ParseError, String::from("not a valid integer")),
            |n| Transform::Literal(Item::Value(Rc::new(Value::from(n)))),
        )
    }))
}
// DecimalLiteral ::= ('.' Digits) | (Digits '.' [0-9]*)
// Construct a double, but if that fails fall back to decimal
fn decimal_literal<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt2(
        decimal_literal_frac::<N, L>(),
        decimal_literal_comp::<N, L>(),
    ))
}
fn decimal_literal_frac<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(pair(tag("."), digit1()), |(_, mut f)| {
        f.insert(0, '.');
        let n = f.parse::<f64>();
        let i = match n {
            Ok(m) => Value::from(m),
            Err(_) => {
                f.insert(0, '0');
                Value::from(Decimal::from_str(&f).unwrap())
            }
        };
        Transform::Literal(Item::Value(Rc::new(i)))
    }))
}
fn decimal_literal_comp<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(map(tuple3(digit1(), tag("."), digit0()), |(w, _, f)| {
        let s = format!("{}.{}", w, f);
        let n = s.parse::<f64>();
        let i = match n {
            Ok(m) => Value::from(m),
            Err(_) => Value::from(Decimal::from_str(&s).unwrap()),
        };
        Transform::Literal(Item::Value(Rc::new(i)))
    }))
}

// DoubleLiteral ::= (('.' Digits) | (Digits ('.' [0-9]*)?)) [eE] [+-]? Digits
// Construct a double
fn double_literal<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt2(
        double_literal_frac::<N, L>(),
        double_literal_comp::<N, L>(),
    ))
}

fn double_literal_frac<'a, N: Node + 'a, L>() -> Box<
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
        tuple4(
            pair(tag("."), digit1()),
            alt2(tag("e"), tag("E")),
            opt(alt2(map(tag("+"), |_| "+"), map(tag("-"), |_| "-"))),
            digit1(),
            "xpath literal",
        ),
        |((_, f), _, s, e)| {
            let n = format!("0.{}e{}{}", f, s.unwrap_or(""), e).parse::<f64>();
            let i = match n {
                Ok(m) => Value::from(m),
                Err(_) => panic!("unable to convert to double"),
            };
            Transform::Literal(Item::Value(Rc::new(i)))
        },
    ))
}
fn double_literal_comp<'a, N: Node + 'a, L>() -> Box<
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
        tuple4(
            tuple3(digit1(), tag("."), digit1()),
            alt2(tag("e"), tag("E")),
            opt(alt2(map(tag("+"), |_| "+"), map(tag("-"), |_| "-"))),
            digit1(),
            "xpath literal",
        ),
        |((c, _, f), _, s, e)| {
            let n = format!("{}.{}e{}{}", c, f, s.unwrap_or(""), e).parse::<f64>();
            let i = match n {
                Ok(m) => Value::from(m),
                Err(_) => panic!("unable to convert to double"),
            };
            Transform::Literal(Item::Value(Rc::new(i)))
        },
    ))
}

// StringLiteral ::= double- or single-quote delimited with double-delimiter escape
fn string_literal_double<'a, N: Node + 'a, L>() -> Box<
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
        delimited(
            anychar('"'),
            map(many0(alt2(map(tag("\"\""), |_| '"'), none_of("\""))), |v| {
                v.iter().collect::<String>()
            }),
            anychar('"'),
        ),
        |s| Transform::Literal(Item::Value(Rc::new(Value::from(s)))),
    ))
}
fn string_literal_single<'a, N: Node + 'a, L>() -> Box<
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
        delimited(
            anychar('\''),
            map(many0(alt2(map(tag("''"), |_| '\''), none_of("'"))), |v| {
                v.iter().collect::<String>()
            }),
            anychar('\''),
        ),
        |s| Transform::Literal(Item::Value(Rc::new(Value::from(s)))),
    ))
}
fn string_literal<'a, N: Node + 'a, L>() -> Box<
    dyn Fn(
            ParseInput<'a, N>,
            &mut StaticState<L>,
        ) -> Result<(ParseInput<'a, N>, Transform<N>), ParseError>
        + 'a,
>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError> + 'a,
{
    Box::new(alt2(
        string_literal_double::<N, L>(),
        string_literal_single::<N, L>(),
    ))
}
