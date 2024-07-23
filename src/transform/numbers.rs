//! These functions are for features defined in XPath Functions 1.0 and 2.0.

use std::rc::Rc;
use url::Url;

use formato::Formato;
use english_numbers::{Formatting, convert};
use italian_numbers::roman_converter;

use crate::item::{Item, Node, Sequence, SequenceTrait, NodeType};
use crate::pattern::{Pattern, PathBuilder};
use crate::qname::QualifiedName;
use crate::transform::context::{Context, StaticContext};
use crate::transform::{ArithmeticOperand, ArithmeticOperator, Axis, Transform, NodeTest, KindTest, NameTest, WildcardOrName};
use crate::value::Value;
use crate::xdmerror::{Error, ErrorKind};

/// Level value for xsl:number. See XSLT 12.3.
#[derive(Copy, Clone, Debug, Default)]
pub enum Level {
    #[default] Single,
    Multiple,
    Any,
}

/// Specification for generating numbers. This is avoid recursive types in [Transform] and [Pattern].
#[derive(Clone, Debug)]
pub struct Numbering<N: Node> {
    level: Level,
    count: Option<Pattern<N>>,
    from: Option<Pattern<N>>,
}
impl<N: Node> Numbering<N> {
    pub fn new(level: Level, count: Option<Pattern<N>>, from: Option<Pattern<N>>) -> Self {
        Numbering{level, count, from}
    }
}

/// Generate a sequence of integers
pub fn generate_integers<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    _start_at: &Transform<N>,
    select: &Transform<N>,
    num: &Numbering<N>,
) -> Result<Sequence<N>, Error> {
    // This implements "single" level. "multiple" and "any" are TODO

    // The select expression must evaluate to a single node item (XSLT error XTTE1000)
    let n = ctxt.dispatch(stctxt, select)?;
    if n.len() == 1 {
        if let Item::Node(m) = &n[0] {

            // Determine the count pattern
            let count_pat = (&num.count).clone().unwrap_or(Pattern::Selection(
                match m.node_type() {
                    NodeType::Element => {
                        PathBuilder::new()
                            .step(
                                Axis::SelfAxis,
                                Axis::SelfAxis,
                                NodeTest::Name(NameTest::new(m.name().get_nsuri().map(|ns| WildcardOrName::Name(ns)), None, Some(WildcardOrName::Name(m.name().get_localname()))))
                            )
                            .build()
                    }
                    NodeType::Text => {
                        PathBuilder::new()
                            .step(Axis::SelfAxis, Axis::SelfAxis, NodeTest::Kind(KindTest::Text))
                            .build()
                    }
                    _ => return Err(Error::new(ErrorKind::TypeError, "cannot match this type of node"))
                }
            ));

            // let a = $S/ancestor-or-self::node()[matches-count(.)][1]
            // TODO: Don't Panic
            let a = if count_pat.matches(ctxt, stctxt, &Item::Node(m.clone())) {
                vec![m.clone()]
            } else {
                m.ancestor_iter()
                    .filter(|i| count_pat.matches(ctxt, stctxt, &Item::Node(i.clone())))
                    .take(1)
                    .collect()
            };
            if a.is_empty() {
                return Ok(vec![])
            }
            // let f = $S/ancestor-or-self::node()[matches-from(.)][1]
            // TODO: Don't Panic
            let f: Vec<N> = if let Some(fr) = &num.from.clone() {
                m.ancestor_iter()
                    .filter(|i| {if i.node_type() == NodeType::Document {true} else {
                        fr.matches(ctxt, stctxt, &Item::Node(i.clone()))
                    }})
                    .take(1)
                    .collect()
            } else {
                // When there is no from pattern specified then use the root node
                vec![m.owner_document().clone()]
            };
            if f.is_empty() {
                return Ok(vec![])
            }
            // let af = $a[ancestor-or-self::node()[. is $f]]
            let af_test: Vec<N> = if a[0].is_same(&f[0]) {
                vec![a[0].clone()]
            } else {
                a[0].ancestor_iter()
                    .filter(|i| i.is_same(&f[0]))
                    .collect()
            };
            let af = if af_test.is_empty() {vec![]} else {a};
            if af.is_empty() {
                return Ok(vec![])
            }
            // 1 + count($af/preceding-sibling::node()[matches-count(.)])
            let result: Vec<N> = af[0].prev_iter()
                .filter(|i| count_pat.matches(ctxt, stctxt, &Item::Node(i.clone())))
                .collect();
            Ok(vec![Item::Value(Rc::new(Value::from(1 + result.len())))])
        } else {
            return Err(Error::new_with_code(ErrorKind::TypeError, "not a singleton node", Some(QualifiedName::new(None, None, "XTTE1000"))))
        }
    } else {
        return Err(Error::new_with_code(ErrorKind::TypeError, "not a singleton node", Some(QualifiedName::new(None, None, "XTTE1000"))))
    }
}

/// XPath number function.
pub fn number<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    num: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let n = ctxt.dispatch(stctxt, num)?;
    match n.len() {
        1 => {
            // First try converting to an integer
            match n[0].to_int() {
                Ok(i) => Ok(vec![Item::Value(Rc::new(Value::Integer(i)))]),
                _ => {
                    // Otherwise convert to double.
                    // NB. This can't fail. At worst it returns NaN.
                    Ok(vec![Item::Value(Rc::new(Value::Double(n[0].to_double())))])
                }
            }
        }
        _ => Err(Error::new(
            ErrorKind::TypeError,
            String::from("not a singleton sequence"),
        )),
    }
}

/// XPath sum function.
pub fn sum<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    s: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    Ok(vec![Item::Value(Rc::new(Value::Double(
        ctxt.dispatch(stctxt, s)?.iter().fold(0.0, |mut acc, i| {
            acc += i.to_double();
            acc
        }),
    )))])
}

/// XPath floor function.
pub fn floor<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    f: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let n = ctxt.dispatch(stctxt, f)?;
    match n.len() {
        1 => Ok(vec![Item::Value(Rc::new(Value::Double(
            n[0].to_double().floor(),
        )))]),
        _ => Err(Error::new(
            ErrorKind::TypeError,
            String::from("not a singleton sequence"),
        )),
    }
}

/// XPath ceiling function.
pub fn ceiling<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    c: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let n = ctxt.dispatch(stctxt, c)?;
    match n.len() {
        1 => Ok(vec![Item::Value(Rc::new(Value::Double(
            n[0].to_double().ceil(),
        )))]),
        _ => Err(Error::new(
            ErrorKind::TypeError,
            String::from("not a singleton sequence"),
        )),
    }
}

/// XPath round function.
pub fn round<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    r: &Transform<N>,
    pr: &Option<Box<Transform<N>>>,
) -> Result<Sequence<N>, Error> {
    match pr {
        Some(p) => {
            let n = ctxt.dispatch(stctxt, r)?;
            let m = ctxt.dispatch(stctxt, p)?;
            match (n.len(), m.len()) {
                (1, 1) => Ok(vec![Item::Value(Rc::new(Value::Double(
                    ((n[0].to_double() * (10.0_f64).powi(m[0].to_int().unwrap() as i32)).round())
                        * (10.0_f64).powi(-m[0].to_int().unwrap() as i32),
                )))]),
                _ => Err(Error::new(
                    ErrorKind::TypeError,
                    String::from("not a singleton sequence"),
                )),
            }
        }
        None => {
            // precision is 0, i.e. round to nearest whole number
            let n = ctxt.dispatch(stctxt, r)?;
            match n.len() {
                1 => Ok(vec![Item::Value(Rc::new(Value::Double(
                    n[0].to_double().round(),
                )))]),
                _ => Err(Error::new(
                    ErrorKind::TypeError,
                    String::from("not a singleton sequence"),
                )),
            }
        }
    }
}

/// Generate a sequence with a range of integers.
pub(crate) fn tr_range<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    start: &Transform<N>,
    end: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let s = ctxt.dispatch(stctxt, start)?;
    let e = ctxt.dispatch(stctxt, end)?;
    if s.len() == 0 || e.len() == 0 {
        // Empty sequence is the result
        return Ok(vec![]);
    }
    if s.len() != 1 || e.len() != 1 {
        return Err(Error::new(
            ErrorKind::TypeError,
            String::from("operands must be singleton sequence"),
        ));
    }
    let i = s[0].to_int()?;
    let j = e[0].to_int()?;
    if i > j {
        // empty sequence result
        Ok(vec![])
    } else if i == j {
        let mut seq = Sequence::new();
        seq.push_value(&Rc::new(Value::Integer(i)));
        Ok(seq)
    } else {
        let mut result = Sequence::new();
        for k in i..=j {
            result.push_value(&Rc::new(Value::from(k)))
        }
        Ok(result)
    }
}

/// Perform an arithmetic operation.
pub(crate) fn arithmetic<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    ops: &Vec<ArithmeticOperand<N>>,
) -> Result<Sequence<N>, Error> {
    // Type: the result will be a number, but integer or double?
    // If all of the operands are integers, then the result is integer otherwise double
    // TODO: check the type of all operands to determine type of result (can probably do this in static analysis phase)
    // In the meantime, let's assume the result will be double and convert any integers
    let mut acc = 0.0;
    for o in ops {
        let j = match ctxt.dispatch(stctxt, &o.operand) {
            Ok(s) => s,
            Err(_) => {
                acc = f64::NAN;
                break;
            }
        };
        if j.len() != 1 {
            acc = f64::NAN;
            break;
        }
        let u = j[0].to_double();
        match o.op {
            ArithmeticOperator::Noop => acc = u,
            ArithmeticOperator::Add => acc += u,
            ArithmeticOperator::Subtract => acc -= u,
            ArithmeticOperator::Multiply => acc *= u,
            ArithmeticOperator::Divide => acc /= u,
            ArithmeticOperator::IntegerDivide => acc /= u, // TODO: convert to integer
            ArithmeticOperator::Modulo => acc = acc % u,
        }
    }
    Ok(vec![Item::Value(Rc::new(Value::from(acc)))])
}

/// XPath format-number function.
pub fn format_number<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    num: &Transform<N>,
    picture: &Transform<N>,
    _name: &Option<Box<Transform<N>>>,
) -> Result<Sequence<N>, Error> {
    let p = ctxt.dispatch(stctxt, picture)?.to_string();
    let n = ctxt.dispatch(stctxt, num)?;
    match n.len() {
        1 => {
            // First try converting to an integer
            match n[0].to_int() {
                Ok(i) => {
                    Ok(vec![Item::Value(Rc::new(Value::String(i.formato(p.as_str()))))])
                }
                _ => {
                    // Otherwise convert to double.
                    // NB. This can't fail. At worst it returns NaN.
                    Ok(vec![Item::Value(Rc::new(Value::String(n[0].to_double().formato(p.as_str()))))])
                }
            }
        }
        _ => Err(Error::new(
            ErrorKind::TypeError,
            String::from("not a singleton sequence"),
        )),
    }
}

/// XSLT xsl:number and XPath format-integer function.
pub fn format_integer<
    N: Node,
    F: FnMut(&str) -> Result<(), Error>,
    G: FnMut(&str) -> Result<N, Error>,
    H: FnMut(&Url) -> Result<String, Error>,
>(
    ctxt: &Context<N>,
    stctxt: &mut StaticContext<N, F, G, H>,
    num: &Transform<N>,
    picture: &Transform<N>,
) -> Result<Sequence<N>, Error> {
    let p = ctxt.dispatch(stctxt, picture)?.to_string();
    let numbers = ctxt.dispatch(stctxt, num)?;
    let mut nit = numbers.iter();

    let mut result = String::new();

    // Interpret the picture string.
    // Most of the tokens are one character, except for 'Ww'.
    let mut pit = p.chars().peekable();
    loop {
        let c = pit.next();
        if let Some(d) = c {
            if d.is_alphanumeric() {
                match d {
                    '0' => {
                        // 01, 02, 03, 04, ...
                        // length specification
                        // TODO: non-arabic-roman numerals
                        let mut token = String::from(d);
                        loop {
                            if let Some(p) = pit.peek() {
                                if p.eq(&'0') {
                                    pit.next();
                                    token.push('0');
                                } else if p.eq(&'1'){
                                    pit.next();
                                    token.push('1');
                                } else {
                                    break
                                }
                            } else {
                                break
                            }
                        }
                        if let Some(num) = nit.next() {
                            result.push_str(format!("{:0>1$}", num.to_int()?.to_string(), token.len()).as_str());
                        }  else {
                            break
                        }
                    }
                    '1' => {
                        // 1, 2, 3, ...
                        if let Some(num) = nit.next() {
                            result.push_str(num.to_int()?.to_string().as_str())
                        }  else {
                            break
                        }
                    }
                    'A' => {
                        // A, B, C, ..., AA, BB, CC, ...
                    }
                    'a' => {
                        // a, b, c, ..., aa, bb, cc, ...
                    }
                    'i' => {
                        // i, ii, iii, iv, v, vi, ...
                        if let Some(num) = nit.next() {
                            result.push_str(
                                roman_converter(
                                u16::try_from(num.to_int()?).map_err(|e| Error::new(ErrorKind::ParseError, e.to_string()))?
                                ).map_err(|e| Error::new(ErrorKind::ParseError, e))?
                                .to_lowercase().as_str()
                            )
                        }  else {
                            break
                        }
                    }
                    'I' => {
                        // I, II, III, IV, V, VI, ...
                        if let Some(num) = nit.next() {
                            result.push_str(
                                roman_converter(
                                    u16::try_from(num.to_int()?).map_err(|e| Error::new(ErrorKind::ParseError, e.to_string()))?
                                ).map_err(|e| Error::new(ErrorKind::ParseError, e))?
                                    .as_str()
                            )
                        }  else {
                            break
                        }
                    }
                    'w' => {
                        // one, two, three, ...
                        if let Some(num) = nit.next() {
                            result.push_str(convert(num.to_int()?,
                                                    Formatting{title_case: false, spaces: true, conjunctions: false, commas: false, dashes: false}
                            ).to_string().as_str())
                        } else {
                            break
                        }
                    }
                    'W' => {
                        // 'Ww'
                        if let Some('w') = pit.peek() {
                            // One, Two, Three, ...
                            pit.next();
                            if let Some(num) = nit.next() {
                                result.push_str(convert(num.to_int()?,
                                                        Formatting{title_case: true, spaces: true, conjunctions: false, commas: false, dashes: false}
                                ).to_string().as_str())
                            } else {
                                break
                            }
                        } else {
                            // ONE, TWO, THREE, ...
                            if let Some(num) = nit.next() {
                                result.push_str(convert(num.to_int()?,
                                                        Formatting{title_case: false, spaces: true, conjunctions: false, commas: false, dashes: false}
                                ).to_string().to_uppercase().as_str())
                            } else {
                                break
                            }
                        }
                    }
                    // TODO: non-English words
                    // Use french-numbers crate
                    // Use italian-numbers crate
                    _ => {}
                }
            } else {
                result.push(d)
            }
        } else {
            break
        }
    }

    Ok(vec![Item::Value(Rc::new(Value::from(result)))])
}
