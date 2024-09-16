use crate::item::{Node, NodeType};
use crate::xdmerror::{Error, ErrorKind};
use crate::parser::combinators::alt::{alt2, alt4};
use crate::parser::combinators::many::many0;
use crate::parser::combinators::map::map;
use crate::parser::combinators::opt::opt;
use crate::parser::combinators::tag::tag;
use crate::parser::combinators::tuple::{tuple10, tuple2, tuple5};
use crate::parser::combinators::wellformed::wellformed;
use crate::parser::combinators::whitespace::whitespace0;
use crate::parser::xml::attribute::attributes;
use crate::parser::xml::chardata::chardata;
use crate::parser::xml::misc::{comment, processing_instruction};
use crate::parser::xml::qname::qualname;
use crate::parser::xml::reference::reference;
use crate::parser::{ParseError, ParseInput};
use crate::qname::QualifiedName;
use crate::value::Value;
use std::rc::Rc;

// Element ::= EmptyElemTag | STag content ETag
pub(crate) fn element<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, N), ParseError>
{
    move |input| alt2(emptyelem(), taggedelem())(input)
}

// EmptyElemTag ::= '<' Name (Attribute)* '/>'
fn emptyelem<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, N), ParseError> {
    move |input| {
        match tuple5(
            tag("<"),
            wellformed(qualname(), |qn| {
                qn.prefix_to_string() != Some("xmlns".to_string())
            }),
            attributes(), //many0(attribute),
            whitespace0(),
            tag("/>"),
        )(input)
        {
            Ok(((input1, state1), (_, mut n, (av, namespaces), _, _))) => {
                if n.resolve(|p| state1.namespace.get(&p).map_or(
                    Err(Error::new(ErrorKind::DynamicAbsent, "no namespace for prefix")),
                    |r| Ok(r.clone())
                )).is_err() {
                    return Err(ParseError::MissingNameSpace);
                }
                let e = state1
                    .doc
                    .clone()
                    .unwrap()
                    .new_element(state1.get_qualified_name(n.namespace_uri(), n.prefix(), n.localname()))
                    .expect("unable to create element");
                av.iter()
                    .for_each(|b| e.add_attribute(b.clone()).expect("unable to add attribute"));
                namespaces.iter().for_each(|nn| e.add_namespace(nn.clone()).expect("unable to add namespace node"));

                Ok(((input1, state1.clone()), e))
            }
            Err(err) => Err(err),
        }
    }
}

// STag ::= '<' Name (Attribute)* '>'
// ETag ::= '</' Name '>'
// TODO: Check that names match and throw meaningful error
fn taggedelem<N: Node>() -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, N), ParseError> {
    move |input| {
        match wellformed(
            tuple10(
                tag("<"),
                wellformed(qualname(), |qn| {
                    qn.prefix_to_string() != Some("xmlns".to_string())
                }),
                attributes(), //many0(attribute),
                whitespace0(),
                tag(">"),
                content(),
                tag("</"),
                wellformed(qualname(), |qn| {
                    qn.prefix_to_string() != Some("xmlns".to_string())
                }),
                whitespace0(),
                tag(">"),
            ),
            |(_, n, _a, _, _, _c, _, e, _, _)| n.to_string() == e.to_string(),
        )(input)
        {
            Ok(((input1, state1), (_, mut n, (av, namespaces), _, _, c, _, _, _, _))) => {
                if n.resolve(|p| state1.namespace.get(&p).map_or(
                    Err(Error::new(ErrorKind::DynamicAbsent, "no namespace for prefix")),
                    |r| Ok(r.clone())
                )).is_err() {
                    return Err(ParseError::MissingNameSpace);
                }
                let d = state1.doc.clone().unwrap();
                let mut e = d
                    .new_element(state1.get_qualified_name(n.namespace_uri(), n.prefix(), n.localname()))
                    .expect("unable to create element");
                av.iter()
                    .for_each(|b| e.add_attribute(b.clone()).expect("unable to add attribute"));
                namespaces.iter()
                    .for_each(|b| e.add_namespace(b.clone()).expect("unable to add namespace"));
                // Add child nodes
                c.iter().for_each(|d| {
                    e.push(d.clone()).expect("unable to add node");
                });
                Ok(((input1, state1.clone()), e))
            }
            Err(err) => Err(err),
        }
    }
}

// content ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
pub(crate) fn content<N: Node>(
) -> impl Fn(ParseInput<N>) -> Result<(ParseInput<N>, Vec<N>), ParseError> {
    move |(input, state)| match tuple2(
        opt(chardata()),
        many0(tuple2(
            alt4(
                map(processing_instruction(), |e| vec![e]),
                map(comment(), |e| vec![e]),
                map(element(), |e| vec![e]),
                reference(),
            ),
            opt(chardata()),
        )),
    )((input, state.clone()))
    {
        Ok((state1, (c, v))) => {
            let mut new: Vec<N> = Vec::new();
            let mut notex: Vec<String> = Vec::new();
            if c.is_some() {
                notex.push(c.unwrap());
            }
            if !v.is_empty() {
                for (w, d) in v {
                    for x in w {
                        match x.node_type() {
                            NodeType::Text => notex.push(x.to_string()),
                            _ => {
                                if !notex.is_empty() {
                                    new.push(
                                        state
                                            .doc
                                            .clone()
                                            .unwrap()
                                            .new_text(Rc::new(Value::String(notex.concat())))
                                            .expect("unable to create text node"),
                                    );
                                    notex.clear();
                                }
                                new.push(x);
                            }
                        }
                    }
                    if d.is_some() {
                        notex.push(d.unwrap())
                    }
                }
            }
            if !notex.is_empty() {
                new.push(
                    state
                        .doc
                        .clone()
                        .unwrap()
                        .new_text(Rc::new(Value::String(notex.concat())))
                        .expect("unable to create text node"),
                );
            }
            Ok((state1, new))
        }

        Err(e) => Err(e),
    }
}
