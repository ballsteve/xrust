use crate::intmuttree::{NodeBuilder, RNode};
use crate::item::NodeType;
use crate::parser::combinators::alt::alt2;
use crate::parser::combinators::delimited::delimited;
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::take::take_until;
use crate::parser::combinators::tuple::{tuple2, tuple5};
use crate::parser::combinators::wellformed::wellformed;
use crate::parser::combinators::whitespace::{whitespace0, whitespace1};
use crate::parser::common::is_char;
use crate::parser::xml::qname::name;
use crate::parser::{ParseInput, ParseResult};
use crate::{Node, Value};

// PI ::= '<?' PITarget (char* - '?>') '?>'
pub(crate) fn processing_instruction() -> impl Fn(ParseInput) -> ParseResult<RNode> {
    wellformed(
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
        |v| match v.node_type() {
            NodeType::ProcessingInstruction => {
                if v.to_string().contains(|c: char| !is_char(&c)) {
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
    wellformed(
        map(
            delimited(tag("<!--"), take_until("--"), tag("-->")),
            |v: String| {
                NodeBuilder::new(NodeType::Comment)
                    .value(Value::String(v))
                    .build()
            },
        ),
        |v| match v.node_type() {
            NodeType::Comment => !v.to_string().contains(|c: char| !is_char(&c)),
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
