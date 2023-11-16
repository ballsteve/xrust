use crate::trees::intmuttree::{NodeBuilder, RNode};
use crate::item::NodeType;
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
use crate::parser::{ParseInput, ParseResult};
use crate::{Node, Value};

// PI ::= '<?' PITarget (char* - '?>') '?>'
pub(crate) fn processing_instruction() -> impl Fn(ParseInput) -> ParseResult<RNode> {
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
                None => NodeBuilder::new(NodeType::ProcessingInstruction)
                    .pi_name(n)
                    .value(Value::String("".to_string()))
                    .build(),
                Some((_, v)) => NodeBuilder::new(NodeType::ProcessingInstruction)
                    .pi_name(n)
                    .value(Value::String(v))
                    .build(),
            },
        ),
        //XML 1.0
        |v| match v.node_type() {
            NodeType::ProcessingInstruction => {
                if v.to_string().contains(|c: char| !is_char10(&c)) {
                    false
                } else if v.pi_name().unwrap().contains(':') {
                    //"No entity names, processing instruction targets, or notation names contain any colons."
                    false
                } else {
                    v.pi_name().unwrap().to_lowercase() != *"xml"
                }
            }
            _ => false,
        },
        //XML 1.1
        |v| match v.node_type() {
            NodeType::ProcessingInstruction => {
                if v.to_string().contains(|c: char| !is_char11(&c)) {
                    false
                } else if v.pi_name().unwrap().contains(':') {
                    //"No entity names, processing instruction targets, or notation names contain any colons."
                    false
                } else {
                    v.pi_name().unwrap().to_lowercase() != *"xml"
                }
            }
            _ => false,
        },
    )
}

// Comment ::= '<!--' (char* - '--') '-->'
pub(crate) fn comment() -> impl Fn(ParseInput) -> ParseResult<RNode> {
    wellformed_ver(
        map(
            delimited(tag("<!--"), take_until("--"), tag("-->")),
            |v: String| {
                NodeBuilder::new(NodeType::Comment)
                    .value(Value::String(v))
                    .build()
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
    )
}

// Misc ::= Comment | PI | S
pub(crate) fn misc() -> impl Fn(ParseInput) -> ParseResult<Vec<RNode>> {
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
