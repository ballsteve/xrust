use crate::item::{Node, NodeType};
use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::combinators::tuple::{tuple2, tuple5};
use crate::parser::combinators::wellformed::wellformed_ver;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::common::{is_char10, is_char11};
use crate::parser::xml::qname::name;
use crate::parser::{ParseError, ParseInput};
use crate::qname::{Interner, QualifiedName};
use crate::value::Value;
use std::rc::Rc;

// PI ::= '<?' PITarget (char* - '?>') '?>'
pub(crate) fn processing_instruction<'a, 'i, I: Interner, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, N), ParseError> {
    move |(input, state)| {
        wellformed_ver(
            map(
                tuple5(
                    tag("<?"),
                    name(),
                    opt(tuple2(whitespace1(), take_until("?>"))),
                    whitespace0(),
                    tag("?>"),
                ),
                |(_, n, vt, _, _)| match vt {
                    None => state
                        .doc
                        .as_ref()
                        .unwrap()
                        .new_processing_instruction(
                            QualifiedName::new(n, None, None, state.interner),
                            Rc::new(Value::from("")),
                        )
                        .expect("unable to create processing instruction"),
                    Some((_, v)) => state
                        .doc
                        .as_ref()
                        .unwrap()
                        .new_processing_instruction(
                            QualifiedName::new(n, None, None, state.interner),
                            Rc::new(Value::from(v)),
                        )
                        .expect("unable to create processing instruction"),
                },
            ),
            //XML 1.0
            |v| match v.node_type() {
                NodeType::ProcessingInstruction => {
                    if v.to_string().contains(|c: char| !is_char10(&c)) {
                        false
                    } else if v.name::<I>().unwrap().prefix().is_some() {
                        //"No entity names, processing instruction targets, or notation names contain any colons."
                        false
                    } else {
                        v.name::<I>().unwrap().to_string().to_lowercase() != *"xml"
                    }
                }
                _ => false,
            },
            //XML 1.1
            |v| match v.node_type() {
                NodeType::ProcessingInstruction => {
                    if v.to_string().contains(|c: char| !is_char11(&c)) {
                        false
                    } else if v.name::<I>().unwrap().prefix().is_some() {
                        // "No entity names, processing instruction targets, or notation names contain any colons."
                        false
                    } else {
                        v.name::<I>().unwrap().to_string().to_lowercase() != *"xml"
                    }
                }
                _ => false,
            },
        )((input, state.clone()))
    }
}

// Comment ::= '<!--' (char* - '--') '-->'
pub(crate) fn comment<'a, 'i, I: Interner, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, N), ParseError> {
    |(input, state)| {
        wellformed_ver(
            map(
                delimited(tag("<!--"), take_until("--"), tag("-->")),
                |v: String| {
                    state
                        .doc
                        .as_ref()
                        .unwrap()
                        .new_comment(Rc::new(Value::from(v)))
                        .expect("unable to create comment")
                },
            ),
            //XML 1.0
            |v| match v.node_type() {
                NodeType::Comment => !v.to_string().contains(|c: char| !is_char10(&c)),
                _ => false,
            },
            //XML 1.1
            |v| match v.node_type() {
                NodeType::Comment => !v.to_string().contains(|c: char| !is_char11(&c)),
                _ => false,
            },
        )((input, state.clone()))
    }
}

// Misc ::= Comment | PI | S
pub(crate) fn misc<'a, 'i, I: Interner + 'i, N: Node>(
) -> impl Fn(ParseInput<'a, 'i, I, N>) -> Result<(ParseInput<'a, 'i, I, N>, Vec<N>), ParseError> {
    map(
        tuple2(
            many0(map(
                alt2(
                    tuple2(whitespace0(), comment()),
                    tuple2(whitespace0(), processing_instruction()),
                ),
                |(_ws, xn)| xn,
            )),
            whitespace0(),
        ),
        |(v, _)| v,
    )
}
