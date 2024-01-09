use std::rc::Rc;
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
use crate::value::Value;
use crate::qname::QualifiedName;

// PI ::= '<?' PITarget (char* - '?>') '?>'
pub(crate) fn processing_instruction<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, N), ParseError> {
    move |(input, state)| wellformed_ver(
        map(
            tuple5(
                tag("<?"),
                name(),
                opt(tuple2(whitespace1(), take_until("?>"))),
                whitespace0(),
                tag("?>"),
            ),
            |(_, n, vt, _, _)| match vt {
                None => state.doc.as_ref().unwrap().new_processing_instruction(QualifiedName::new(None, None, n), Rc::new(Value::String("".to_string()))).expect("unable to create processing instruction"),
                Some((_, v)) => state.doc.as_ref().unwrap().new_processing_instruction(QualifiedName::new(None, None, n), Rc::new(Value::String(v))).expect("unable to create processing instruction"),
            },
        ),
        //XML 1.0
        |v| match v.node_type() {
                NodeType::ProcessingInstruction => {
                    if v.to_string().contains(|c: char| !is_char10(&c)) {
                        false
                    } else if v.name().to_string().contains(':') {
                        //"No entity names, processing instruction targets, or notation names contain any colons."
                        false
                    } else {
                        v.name().to_string().to_lowercase() != *"xml"
                    }
                }
            _ => false,
        },
        //XML 1.1
        |v| match v.node_type() {
                NodeType::ProcessingInstruction => {
                    if v.to_string().contains(|c: char| !is_char11(&c)) {
                        false
                    } else if v.name().to_string().contains(':') {
                        // "No entity names, processing instruction targets, or notation names contain any colons."
                        false
                    } else {
                        v.name().to_string().to_lowercase() != *"xml"
                    }
                }
                _ => false,
        },
    )((input, state.clone()))
}

// Comment ::= '<!--' (char* - '--') '-->'
pub(crate) fn comment<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, N), ParseError> {
    |(input, state)| wellformed_ver(
        map(
            delimited(tag("<!--"), take_until("--"), tag("-->")),
            |v: String| {
                state.doc.as_ref().unwrap().new_comment(Rc::new(Value::String(v))).expect("unable to create comment")
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

// Misc ::= Comment | PI | S
pub(crate) fn misc<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, Vec<N>), ParseError> {
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
