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
use crate::parser::{ParseError, ParseInput};
use crate::qname::Interner;
use crate::transform::Transform;
use crate::value::Value;

use rust_decimal::Decimal;

// Literal ::= NumericLiteral | StringLiteral
pub(crate) fn literal<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(alt2(numeric_literal::<I, N>(), string_literal::<I, N>()))
}

// NumericLiteral ::= IntegerLiteral | DecimalLiteral | DoubleLiteral
fn numeric_literal<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(alt3(
        double_literal::<I, N>(),
        decimal_literal::<I, N>(),
        integer_literal::<I, N>(),
    ))
}
// IntegerLiteral ::= Digits
fn integer_literal<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(map(digit1(), |s: String| {
        let n = s.parse::<i64>().unwrap();
        Transform::Literal(Item::Value(Rc::new(Value::Integer(n))))
    }))
}
// DecimalLiteral ::= ('.' Digits) | (Digits '.' [0-9]*)
// Construct a double, but if that fails fall back to decimal
fn decimal_literal<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(alt2(
        decimal_literal_frac::<I, N>(),
        decimal_literal_comp::<I, N>(),
    ))
}
fn decimal_literal_frac<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(map(pair(tag("."), digit1()), |(_, mut f)| {
        f.insert(0, '.');
        let n = f.parse::<f64>();
        let i = match n {
            Ok(m) => Value::Double(m),
            Err(_) => {
                f.insert_str(0, "0");
                Value::Decimal(Decimal::from_str(&f).unwrap())
            }
        };
        Transform::Literal(Item::Value(Rc::new(i)))
    }))
}
fn decimal_literal_comp<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(map(tuple3(digit1(), tag("."), digit0()), |(w, _, f)| {
        let s = format!("{}.{}", w, f);
        let n = s.parse::<f64>();
        let i = match n {
            Ok(m) => Value::Double(m),
            Err(_) => Value::Decimal(Decimal::from_str(&s).unwrap()),
        };
        Transform::Literal(Item::Value(Rc::new(i)))
    }))
}

// DoubleLiteral ::= (('.' Digits) | (Digits ('.' [0-9]*)?)) [eE] [+-]? Digits
// Construct a double
fn double_literal<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(alt2(
        double_literal_frac::<I, N>(),
        double_literal_comp::<I, N>(),
    ))
}

fn double_literal_frac<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(map(
        tuple4(
            pair(tag("."), digit1()),
            alt2(tag("e"), tag("E")),
            opt(alt2(map(tag("+"), |_| "+"), map(tag("-"), |_| "-"))),
            digit1(),
        ),
        |((_, f), _, s, e)| {
            let n = format!("0.{}e{}{}", f, s.unwrap_or(""), e).parse::<f64>();
            let i = match n {
                Ok(m) => Value::Double(m),
                Err(_) => panic!("unable to convert to double"),
            };
            Transform::Literal(Item::Value(Rc::new(i)))
        },
    ))
}
fn double_literal_comp<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(map(
        tuple4(
            tuple3(digit1(), tag("."), digit1()),
            alt2(tag("e"), tag("E")),
            opt(alt2(map(tag("+"), |_| "+"), map(tag("-"), |_| "-"))),
            digit1(),
        ),
        |((c, _, f), _, s, e)| {
            let n = format!("{}.{}e{}{}", c, f, s.unwrap_or(""), e).parse::<f64>();
            let i = match n {
                Ok(m) => Value::Double(m),
                Err(_) => panic!("unable to convert to double"),
            };
            Transform::Literal(Item::Value(Rc::new(i)))
        },
    ))
}

// StringLiteral ::= double- or single-quote delimited with double-delimiter escape
fn string_literal_double<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
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
fn string_literal_single<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
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
fn string_literal<'a, 'i: 'a, I: Interner, N: Node + 'a>() -> Box<
    dyn Fn(
            ParseInput<'a, 'i, I, N>,
        ) -> Result<(ParseInput<'a, 'i, I, N>, Transform<'i, I, N>), ParseError>
        + 'a,
> {
    Box::new(alt2(
        string_literal_double::<I, N>(),
        string_literal_single::<I, N>(),
    ))
}
