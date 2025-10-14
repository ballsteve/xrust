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
use crate::parser::{ParseError, ParseInput, StaticState};
use crate::value::Value;
use qualname::{NamespacePrefix, NamespaceUri};
use std::rc::Rc;

// PI ::= '<?' PITarget (char* - '?>') '?>'
// PITarget ::= Name - 'X' 'M' 'L'
// In other words, the name must not start with the three characters 'X' 'M' 'L' in any capitalisation.
// XML Namespaces 1.0 S7 states that processing instruction targets may not contain a colon.
pub(crate) fn processing_instruction<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, N), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    move |(input, state), ss| {
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
                            Rc::new(Value::from(n)),
                            Rc::new(Value::from("".to_string())),
                        )
                        .expect("unable to create processing instruction"),
                    Some((_, v)) => state
                        .doc
                        .as_ref()
                        .unwrap()
                        .new_processing_instruction(
                            Rc::new(Value::from(n)),
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
                    } else {
                        v.name().map_or_else(
                            || false, // "No entity names, processing instruction targets, or notation names contain any colons."
                            |nm| !nm.to_string().to_lowercase().starts_with("xml"),
                        )
                    }
                }
                _ => false,
            },
            //XML 1.1
            |v| match v.node_type() {
                NodeType::ProcessingInstruction => {
                    if v.to_string().contains(|c: char| !is_char11(&c)) {
                        false
                    } else {
                        v.name().map_or_else(
                            || false, // "No entity names, processing instruction targets, or notation names contain any colons."
                            |nm| !nm.to_string().to_lowercase().starts_with("xml"),
                        )
                    }
                }
                _ => false,
            },
        )((input, state.clone()), ss)
    }
}

// Comment ::= '<!--' (char* - '--') '-->'
pub(crate) fn comment<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, N), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
    |(input, state), ss| {
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
        )((input, state.clone()), ss)
    }
}

// Misc ::= Comment | PI | S
pub(crate) fn misc<'a, N: Node, L>()
-> impl Fn(ParseInput<'a, N>, &mut StaticState<L>) -> Result<(ParseInput<'a, N>, Vec<N>), ParseError>
where
    L: FnMut(&NamespacePrefix) -> Result<NamespaceUri, ParseError>,
{
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
